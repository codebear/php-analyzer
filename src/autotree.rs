use crate::autonodes::program::ProgramNode;
use crate::issue::IssuePosition;
use crate::{analysis::state::AnalysisState, autonodes::any::AnyNodeRef};
use tree_sitter::Range;
use tree_sitter::Tree;
use tree_sitter::{Node, Point};

#[derive(Clone, Debug)]
pub struct ParseError {
    pub range: Range,
    pub error: String,
}

impl ParseError {
    pub fn new(range: Range, error: String) -> Self {
        Self { range, error }
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
            "{}:{}-{}:{}:",
            node.range().start_point.row + 1,
            node.range().start_point.column,
            node.range().end_point.row + 1,
            node.range().end_point.column,
        );
        eprintln!(
            "{:10}{} {}{} ",
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
                format!("{}", node.kind())
            }
        );
        let mut idx = 0;
        for child in node.children(&mut node.walk()) {
            let field_name = if level > 0 {
                node.field_name_for_child(idx)
            } else {
                None
            };

            Self::debug_dump_node(&child, field_name, level + 1);
            idx += 1;
        }
    }
}

pub trait NodeAccess {
    fn brief_desc(&self) -> String;

    fn range(&self) -> Range;

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a>;

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>>;

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
        return true;
    }
}
