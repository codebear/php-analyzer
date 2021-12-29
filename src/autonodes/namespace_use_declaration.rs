use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::namespace_name::NamespaceNameNode;
use crate::autonodes::namespace_use_clause::NamespaceUseClauseNode;
use crate::autonodes::namespace_use_group::NamespaceUseGroupNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autotree::NodeAccess;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub enum NamespaceUseDeclarationChildren {
    NamespaceName(Box<NamespaceNameNode>),
    NamespaceUseClause(Box<NamespaceUseClauseNode>),
    NamespaceUseGroup(Box<NamespaceUseGroupNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl NamespaceUseDeclarationChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => NamespaceUseDeclarationChildren::Comment(Box::new(CommentNode::parse(
                node, source,
            )?)),
            "text_interpolation" => NamespaceUseDeclarationChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => {
                NamespaceUseDeclarationChildren::Error(Box::new(ErrorNode::parse(node, source)?))
            }
            "namespace_name" => NamespaceUseDeclarationChildren::NamespaceName(Box::new(
                NamespaceNameNode::parse(node, source)?,
            )),
            "namespace_use_clause" => NamespaceUseDeclarationChildren::NamespaceUseClause(
                Box::new(NamespaceUseClauseNode::parse(node, source)?),
            ),
            "namespace_use_group" => NamespaceUseDeclarationChildren::NamespaceUseGroup(Box::new(
                NamespaceUseGroupNode::parse(node, source)?,
            )),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }

    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => NamespaceUseDeclarationChildren::Comment(Box::new(CommentNode::parse(
                node, source,
            )?)),
            "text_interpolation" => NamespaceUseDeclarationChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => {
                NamespaceUseDeclarationChildren::Error(Box::new(ErrorNode::parse(node, source)?))
            }
            "namespace_name" => NamespaceUseDeclarationChildren::NamespaceName(Box::new(
                NamespaceNameNode::parse(node, source)?,
            )),
            "namespace_use_clause" => NamespaceUseDeclarationChildren::NamespaceUseClause(
                Box::new(NamespaceUseClauseNode::parse(node, source)?),
            ),
            "namespace_use_group" => NamespaceUseDeclarationChildren::NamespaceUseGroup(Box::new(
                NamespaceUseGroupNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        self.as_any().kind()
    }

    pub fn parse_vec<'a, I>(children: I, source: &Vec<u8>) -> Result<Vec<Box<Self>>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
    {
        let mut res: Vec<Box<Self>> = vec![];
        for child in children {
            res.push(Box::new(Self::parse(child, source)?));
        }
        Ok(res)
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        match self {
            NamespaceUseDeclarationChildren::Comment(x) => x.get_utype(state, emitter),
            NamespaceUseDeclarationChildren::TextInterpolation(x) => x.get_utype(state, emitter),
            NamespaceUseDeclarationChildren::Error(x) => x.get_utype(state, emitter),
            NamespaceUseDeclarationChildren::NamespaceName(x) => x.get_utype(state, emitter),
            NamespaceUseDeclarationChildren::NamespaceUseClause(x) => x.get_utype(state, emitter),
            NamespaceUseDeclarationChildren::NamespaceUseGroup(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            NamespaceUseDeclarationChildren::Comment(x) => x.get_php_value(state, emitter),
            NamespaceUseDeclarationChildren::TextInterpolation(x) => {
                x.get_php_value(state, emitter)
            }
            NamespaceUseDeclarationChildren::Error(x) => x.get_php_value(state, emitter),
            NamespaceUseDeclarationChildren::NamespaceName(x) => x.get_php_value(state, emitter),
            NamespaceUseDeclarationChildren::NamespaceUseClause(x) => {
                x.get_php_value(state, emitter)
            }
            NamespaceUseDeclarationChildren::NamespaceUseGroup(x) => {
                x.get_php_value(state, emitter)
            }
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            NamespaceUseDeclarationChildren::Comment(x) => x.read_from(state, emitter),
            NamespaceUseDeclarationChildren::TextInterpolation(x) => x.read_from(state, emitter),
            NamespaceUseDeclarationChildren::Error(x) => x.read_from(state, emitter),
            NamespaceUseDeclarationChildren::NamespaceName(x) => x.read_from(state, emitter),
            NamespaceUseDeclarationChildren::NamespaceUseClause(x) => x.read_from(state, emitter),
            NamespaceUseDeclarationChildren::NamespaceUseGroup(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for NamespaceUseDeclarationChildren {
    fn brief_desc(&self) -> String {
        match self {
            NamespaceUseDeclarationChildren::Comment(x) => format!(
                "NamespaceUseDeclarationChildren::comment({})",
                x.brief_desc()
            ),
            NamespaceUseDeclarationChildren::TextInterpolation(x) => format!(
                "NamespaceUseDeclarationChildren::text_interpolation({})",
                x.brief_desc()
            ),
            NamespaceUseDeclarationChildren::Error(x) => {
                format!("NamespaceUseDeclarationChildren::ERROR({})", x.brief_desc())
            }
            NamespaceUseDeclarationChildren::NamespaceName(x) => format!(
                "NamespaceUseDeclarationChildren::namespace_name({})",
                x.brief_desc()
            ),
            NamespaceUseDeclarationChildren::NamespaceUseClause(x) => format!(
                "NamespaceUseDeclarationChildren::namespace_use_clause({})",
                x.brief_desc()
            ),
            NamespaceUseDeclarationChildren::NamespaceUseGroup(x) => format!(
                "NamespaceUseDeclarationChildren::namespace_use_group({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            NamespaceUseDeclarationChildren::Comment(x) => x.as_any(),
            NamespaceUseDeclarationChildren::TextInterpolation(x) => x.as_any(),
            NamespaceUseDeclarationChildren::Error(x) => x.as_any(),
            NamespaceUseDeclarationChildren::NamespaceName(x) => x.as_any(),
            NamespaceUseDeclarationChildren::NamespaceUseClause(x) => x.as_any(),
            NamespaceUseDeclarationChildren::NamespaceUseGroup(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            NamespaceUseDeclarationChildren::Comment(x) => x.children_any(),
            NamespaceUseDeclarationChildren::TextInterpolation(x) => x.children_any(),
            NamespaceUseDeclarationChildren::Error(x) => x.children_any(),
            NamespaceUseDeclarationChildren::NamespaceName(x) => x.children_any(),
            NamespaceUseDeclarationChildren::NamespaceUseClause(x) => x.children_any(),
            NamespaceUseDeclarationChildren::NamespaceUseGroup(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            NamespaceUseDeclarationChildren::Comment(x) => x.range(),
            NamespaceUseDeclarationChildren::TextInterpolation(x) => x.range(),
            NamespaceUseDeclarationChildren::Error(x) => x.range(),
            NamespaceUseDeclarationChildren::NamespaceName(x) => x.range(),
            NamespaceUseDeclarationChildren::NamespaceUseClause(x) => x.range(),
            NamespaceUseDeclarationChildren::NamespaceUseGroup(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct NamespaceUseDeclarationNode {
    pub range: Range,
    pub children: Vec<Box<NamespaceUseDeclarationChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NamespaceUseDeclarationNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "namespace_use_declaration" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [namespace_use_declaration] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            children: NamespaceUseDeclarationChildren::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() != "comment"),
                source,
            )?,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )?,
        })
    }

    pub fn parse_vec<'a, I>(children: I, source: &Vec<u8>) -> Result<Vec<Box<Self>>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
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

    pub fn kind(&self) -> &'static str {
        "namespace_use_declaration"
    }
}

impl NodeAccess for NamespaceUseDeclarationNode {
    fn brief_desc(&self) -> String {
        "NamespaceUseDeclarationNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::NamespaceUseDeclaration(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.extend(self.children.iter().map(|n| n.as_any()));
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
