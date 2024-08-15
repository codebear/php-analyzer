#![allow(clippy::from_over_into)]
use std::marker::PhantomData;

use crate::autonodes::program::ProgramNode;
use crate::issue::IssuePosition;
use crate::{analysis::state::AnalysisState, autonodes::any::AnyNodeRef};
// use tree_sitter::Range;
use crate::parser::Range;
use tree_sitter::Tree;
use tree_sitter::{Node, Point};

#[derive(Clone, Debug)]
pub struct ParseError {
    pub range: Range,
    pub error: String,
}

impl ParseError {
    pub fn new<R>(range: R, error: String) -> Self
    where
        R: Into<Range>,
    {
        Self {
            range: range.into(),
            error,
        }
    }
}

/// Tullenavn enn så lenge
#[derive(Debug)]
pub struct AutoTree {
    pub tree: Tree,
    pub root: Box<ProgramNode>,
}

impl AutoTree {
    pub fn new(tree: Tree, source: Vec<u8>) -> Result<Self, ParseError> {
        let root_node = tree.root_node();
        if root_node.kind() != "program" {
            panic!("Root node is not a program?");
        }
        // Usefull if parsing failes:
        // Self::debug_dump_tree(&tree);
        let root = Box::new(ProgramNode::parse(root_node, &source)?);

        Ok(AutoTree { tree, root })
    }

    pub fn debug_dump(&self) {
        Self::debug_dump_tree(&self.tree);
    }

    pub fn debug_dump_tree(tree: &Tree) {
        eprintln!("ROOT: {:?}", tree);
        Self::debug_dump_node(&tree.root_node(), None, 0);
    }

    pub fn debug_dump_node(node: &Node, field_name: Option<&'static str>, level: usize) {
        let prefix = format!(
            "{}:{}-{}:{}-{}:",
            node.range().start_point.row + 1,
            node.range().start_point.column,
            node.range().end_point.row + 1,
            node.range().end_point.column,
            level
        );
        eprintln!(
            "{:20}{} {}{} ",
            prefix,
            "   ".repeat(level),
            if let Some(fname) = field_name {
                format!("{}: ", fname)
            } else {
                "".to_string()
            },
            if node.is_named() {
                format!("\"{}\"", node.kind())
            } else {
                node.kind().to_string()
            }
        );
        for (idx, child) in node.children(&mut node.walk()).enumerate() {
            let field_name = if level > 0 {
                let u32_idx: u32 = idx as u32;
                node.field_name_for_child(u32_idx)
            } else {
                None
            };

            Self::debug_dump_node(&child, field_name, level + 1);
        }
    }
}

pub trait NodeAccess {
    fn brief_desc(&self) -> String;

    fn range(&self) -> Range;

    fn as_any(&self) -> AnyNodeRef<'_>;

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        vec![]
    }

    fn named_children<'a>(&'a self, name: &str) -> Vec<AnyNodeRef<'a>> {
        self.children_any()
            .iter()
            .filter(|node| node.kind() == name)
            // Klarer vi å bli kvitt denne klonen?
            .cloned()
            .collect::<Vec<AnyNodeRef<'a>>>()
    }

    fn pos(&self, state: &AnalysisState) -> IssuePosition {
        IssuePosition::new(&state.filename, self.range())
    }

    fn with_node_ref_at_position<T, CB>(
        &self,
        line: usize,
        character: usize,
        cb: &mut CB,
    ) -> Option<T>
    where
        CB: FnMut(AnyNodeRef) -> T,
    {
        for child in self.children_any() {
            if child.contains(line, character) {
                return child.with_node_ref_at_position(line, character, cb);
            }
        }
        if self.contains(line, character) {
            Some(self.with_any_node_ref(cb))
        } else {
            None
        }
    }

    fn with_node_ref_path_at_position<T, CB>(
        &self,
        line: usize,
        character: usize,
        cb: &mut CB,
    ) -> Option<T>
    where
        CB: FnMut(&Vec<AnyNodeRef>) -> T,
    {
        self.with_node_ref_path_at_position_from(vec![], line, character, cb)
    }

    fn with_node_ref_path_at_position_from<T, CB>(
        &self,
        vec: Vec<AnyNodeRef>,
        line: usize,
        character: usize,
        cb: &mut CB,
    ) -> Option<T>
    where
        CB: FnMut(&Vec<AnyNodeRef>) -> T,
    {
        for child in self.children_any() {
            if child.contains(line, character) {
                let mut vec = vec.clone();
                vec.push(child.clone());
                return child.with_node_ref_path_at_position_from(vec, line, character, cb);
            }
        }

        if self.contains(line, character) {
            Some(self.with_any_node_ref(&mut |node| {
                let mut vec = vec.clone();
                vec.push(node);
                cb(&vec)
            }))
        } else {
            None
        }
    }

    fn with_any_node_ref<'a, T, CB>(&'a self, cb: &mut CB) -> T
    where
        CB: FnMut(AnyNodeRef<'a>) -> T,
    {
        cb(self.as_any())
    }

    ///
    /// check if the position is contained in this node, and not contained in any of the child nodes
    /// A lot less efficient than contains/contains_pos, which might be what you need
    ///
    fn is_owner_of_position(&self, point: Point) -> bool {
        if !self.contains(point.row, point.column) {
            return false;
        }
        for child in self.children_any() {
            if child.contains(point.row, point.column) {
                return false;
            }
        }
        true
    }

    fn contains_pos(&self, point: Point) -> bool {
        self.contains(point.row, point.column)
    }

    fn contains(&self, line: usize, character: usize) -> bool {
        let pos = self.range();

        if line < pos.start_point.row {
            return false;
        }

        if line > pos.end_point.row {
            return false;
        }

        if line == pos.start_point.row && character < pos.start_point.column {
            return false;
        }
        if line == pos.end_point.row && character > pos.end_point.column {
            return false;
        }
        true
    }
}

pub trait NodeParser {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError>
    where
        Self: Sized;

    fn parse_vec<'a, I>(children: I, source: &[u8]) -> Result<Vec<Box<Self>>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
        Self: Sized,
    {
        let mut res: Vec<Box<Self>> = vec![];
        for child in children {
            if child.kind() == "comment" {
                continue;
            }
            res.push(Box::new(Self::parse(child, source)?));
        }
        Ok(res)
    }
}

pub struct ChildNodeParserHelper<'node, 'source, 'skipped, T> {
    node: &'node Node<'node>,
    fieldname: &'static str,
    source: &'source [u8],
    mark_skipped_node: Option<&'skipped mut Vec<usize>>,
    _marker: std::marker::PhantomData<T>,
}

impl<'node, 'source, 'skipped, T> ChildNodeParserHelper<'node, 'source, 'skipped, T> {
    fn maybe_mark_skipped_node(&mut self, node_id: usize) {
        if let Some(ref mut skip_nodes) = self.mark_skipped_node.as_mut() {
            skip_nodes.push(node_id);
        }
    }

    pub fn mark_skipped_node(mut self, skip_nodes: &'skipped mut Vec<usize>) -> Self {
        self.mark_skipped_node = Some(skip_nodes);
        self
    }
}

pub trait ChildNodeParser<'node, 'source, 'skipped> {
    fn parse_child<T>(
        &'node self,
        fieldname: &'static str,
        source: &'source [u8],
    ) -> ChildNodeParserHelper<'node, 'source, 'skipped, T>
    where
        T: NodeParser;
}

impl<'node, 'source, 'skipped> ChildNodeParser<'node, 'source, 'skipped> for Node<'node> {
    fn parse_child<T>(
        &'node self,
        fieldname: &'static str,
        source: &'source [u8],
    ) -> ChildNodeParserHelper<'node, 'source, 'skipped, T>
    where
        T: NodeParser,
    {
        ChildNodeParserHelper {
            node: self,
            fieldname,
            source,
            mark_skipped_node: None,
            _marker: PhantomData,
        }
    }
}
/*
impl<'node, 'source, T> ChildNodeParserHelper<'node, 'source, T> {
    // void
}*/

impl<T> Into<Result<Option<T>, ParseError>> for ChildNodeParserHelper<'_, '_, '_, T>
where
    T: NodeParser,
{
    fn into(mut self) -> Result<Option<T>, ParseError> {
        let cursor = &mut self.node.walk();

        let mut traversable = self.node.children_by_field_name(self.fieldname, cursor);

        let Some(first) = traversable.next() else {
            return Ok(None);
        };
        self.maybe_mark_skipped_node(first.id());

        Ok(Some(T::parse(first, self.source)?))
    }
}

impl<T> Into<Result<T, ParseError>> for ChildNodeParserHelper<'_, '_, '_, T>
where
    T: NodeParser,
{
    fn into(mut self) -> Result<T, ParseError> {
        let cursor = &mut self.node.walk();

        let mut traversable = self.node.children_by_field_name(self.fieldname, cursor);

        let Some(first) = traversable.next() else {
            return Err(ParseError::new(
                self.node.range(),
                format!("Expected child node with fieldname {}", self.fieldname),
            ));
        };

        let parsed = T::parse(first, self.source)?;
        self.maybe_mark_skipped_node(first.id());
        Ok(parsed)
    }
}

impl<T> Into<Result<Box<T>, ParseError>> for ChildNodeParserHelper<'_, '_, '_, T>
where
    T: NodeParser,
{
    fn into(mut self) -> Result<Box<T>, ParseError> {
        let cursor = &mut self.node.walk();

        let mut traversable = self.node.children_by_field_name(self.fieldname, cursor);

        let Some(first) = traversable.next() else {
            return Err(ParseError::new(
                self.node.range(),
                format!("Expected child node with fieldname {}", self.fieldname),
            ));
        };

        let parsed = T::parse(first, self.source)?;
        self.maybe_mark_skipped_node(first.id());
        Ok(Box::new(parsed))
    }
}

impl<T> Into<Result<Vec<T>, ParseError>> for ChildNodeParserHelper<'_, '_, '_, T>
where
    T: NodeParser,
{
    fn into(mut self) -> Result<Vec<T>, ParseError> {
        let mut result = vec![];
        for noe in self
            .node
            .children_by_field_name(self.fieldname, &mut self.node.walk())
        {
            let parsed = T::parse(noe, self.source)?;
            self.maybe_mark_skipped_node(noe.id());
            result.push(parsed);
        }
        Ok(result)
    }
}

impl<T> Into<Result<Vec<Box<T>>, ParseError>> for ChildNodeParserHelper<'_, '_, '_, T>
where
    T: NodeParser,
{
    fn into(mut self) -> Result<Vec<Box<T>>, ParseError> {
        let mut result = vec![];
        for noe in self
            .node
            .children_by_field_name(self.fieldname, &mut self.node.walk())
        {
            let parsed = T::parse(noe, self.source)?;
            self.maybe_mark_skipped_node(noe.id());

            result.push(Box::new(parsed));
        }
        Ok(result)
    }
}

impl<T> Into<Result<Option<Vec<T>>, ParseError>> for ChildNodeParserHelper<'_, '_, '_, T>
where
    T: NodeParser,
{
    fn into(mut self) -> Result<Option<Vec<T>>, ParseError> {
        let mut result = vec![];
        for noe in self
            .node
            .children_by_field_name(self.fieldname, &mut self.node.walk())
        {
            let parsed = T::parse(noe, self.source)?;
            self.maybe_mark_skipped_node(noe.id());

            result.push(parsed);
        }
        if !result.is_empty() {
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}

impl<T> Into<Result<Option<Vec<Box<T>>>, ParseError>> for ChildNodeParserHelper<'_, '_, '_, T>
where
    T: NodeParser,
{
    fn into(mut self) -> Result<Option<Vec<Box<T>>>, ParseError> {
        let mut result = vec![];
        for noe in self
            .node
            .children_by_field_name(self.fieldname, &mut self.node.walk())
        {
            let parsed = T::parse(noe, self.source)?;
            self.maybe_mark_skipped_node(noe.id());

            result.push(Box::new(parsed));
        }
        if !result.is_empty() {
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}

impl<T> Into<Result<Option<Box<T>>, ParseError>> for ChildNodeParserHelper<'_, '_, '_, T>
where
    T: NodeParser,
{
    fn into(mut self) -> Result<Option<Box<T>>, ParseError> {
        let cursor = &mut self.node.walk();
        let mut traversable = self.node.children_by_field_name(self.fieldname, cursor);
        let Some(first) = traversable.next() else {
            return Ok(None);
        };

        let parsed = T::parse(first, self.source)?;
        self.maybe_mark_skipped_node(first.id());

        Ok(Some(Box::new(parsed)))
    }
}

pub struct NodeParserBuilder<'node, 'skip_nodes, TParseResult, TFMap> {
    node: &'node Node<'node>,
    fieldname: &'static str,
    map: Box<TFMap>,
    skip_nodes: Option<&'skip_nodes mut Vec<usize>>,
    _marker: std::marker::PhantomData<TParseResult>,
}

impl<'node, 'skip_nodes, TParseResult, TFMap>
    NodeParserBuilder<'node, 'skip_nodes, TParseResult, TFMap>
where
    TFMap: FnMut(Node<'node>) -> Result<TParseResult, ParseError>, // TFBoxMap: FnMut(Node<'node>) -> Result<Box<TParseResult>, ParseError>,
{
    pub fn mark_skipped_node(mut self, skip_nodes: &'skip_nodes mut Vec<usize>) -> Self {
        self.skip_nodes = Some(skip_nodes);
        self
    }

    pub fn boxed(self) -> NodeParserBuilder<'node, 'skip_nodes, Box<TParseResult>, TFMap> {
        todo!();
    }
    /*  pub fn boxed(mut self) -> NodeParserBuilder<'node, 'skip_nodes, Box<TParseResult>, TFBoxMap>
    where {
            let mut mapper = self.map;

            let new_mapper = Box::new(move |childnode| -> Result<Box<TParseResult>, ParseError> {
                let res = mapper(childnode)?;

                let x = Box::new(res);
                Ok(x)
            });

            NodeParserBuilder {
                node: self.node,
                fieldname: self.fieldname,
                map: new_mapper,
                skip_nodes: self.skip_nodes,
                _marker: PhantomData,
            }
        }*/

    pub fn one(self) -> Result<TParseResult, ParseError> {
        let node = self.node;
        let mut items = self.many()?;
        if items.len() != 1 {
            return Err(ParseError::new(
                node.range(),
                format!("Expected exactly one node, got {}", items.len()),
            ));
        }
        let item = items.drain(..).next().expect("This must have one element");
        Ok(item)
    }

    pub fn maybe_one(self) -> Result<Option<TParseResult>, ParseError> {
        let node = self.node;
        let mut items = self.many()?;
        if items.len() > 1 {
            return Err(ParseError::new(
                node.range(),
                format!("Expected at most one node, got {}", items.len()),
            ));
        }
        let maybe_item = items.drain(..).next();
        Ok(maybe_item)
    }

    pub fn many(self) -> Result<Vec<TParseResult>, ParseError> {
        let fieldname = self.fieldname;
        let node = self.node;
        let tree_cursor = &mut node.walk();
        let iter = node.children_by_field_name(fieldname, tree_cursor);

        let nodevec = if let Some(maybe_mark_skipped_node) = self.skip_nodes {
            iter.map(|chnode| {
                maybe_mark_skipped_node.push(chnode.id());
                chnode
            })
            .map(self.map)
            .collect::<Result<Vec<_>, ParseError>>()?
        } else {
            iter.map(self.map).collect::<Result<Vec<_>, ParseError>>()?
        };

        Ok(nodevec)
    }
    pub fn maybe_many(self) -> Result<Option<Vec<TParseResult>>, ParseError> {
        let nodevec = self.many()?;
        if !nodevec.is_empty() {
            Ok(Some(nodevec))
        } else {
            Ok(None)
        }
    }
}

/*
pub(crate) trait NodeParserHelper<'node> {
    fn parse_child<'noskipnodes, TFMap, TParseResult>(
        &'node self,
        fieldname: &'static str,
        map: TFMap,
    ) -> NodeParserBuilder<'node, 'noskipnodes, TParseResult, TFMap>
    where
        TFMap: FnMut(Node<'node>) -> Result<TParseResult, ParseError>;
}
*/
/*
impl<'node> NodeParserHelper<'node> for Node<'node> {
    fn parse_child<'noskipnodes, TFMap, TParseResult>(
        &'node self,
        fieldname: &'static str,
        map: TFMap,
    ) -> NodeParserBuilder<'node, 'noskipnodes, TParseResult, TFMap>
    where
        TFMap: FnMut(Node<'node>) -> Result<TParseResult, ParseError>,
    {
        NodeParserBuilder {
            node: self,
            fieldname,
            map: Box::new(map),
            skip_nodes: None,
            _marker: PhantomData,
        }
    }
}*/
