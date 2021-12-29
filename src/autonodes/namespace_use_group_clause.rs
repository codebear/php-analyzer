use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::namespace_aliasing_clause::NamespaceAliasingClauseNode;
use crate::autonodes::namespace_name::NamespaceNameNode;
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
pub enum NamespaceUseGroupClauseChildren {
    NamespaceAliasingClause(Box<NamespaceAliasingClauseNode>),
    NamespaceName(Box<NamespaceNameNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl NamespaceUseGroupClauseChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => NamespaceUseGroupClauseChildren::Comment(Box::new(CommentNode::parse(
                node, source,
            )?)),
            "text_interpolation" => NamespaceUseGroupClauseChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => {
                NamespaceUseGroupClauseChildren::Error(Box::new(ErrorNode::parse(node, source)?))
            }
            "namespace_aliasing_clause" => {
                NamespaceUseGroupClauseChildren::NamespaceAliasingClause(Box::new(
                    NamespaceAliasingClauseNode::parse(node, source)?,
                ))
            }
            "namespace_name" => NamespaceUseGroupClauseChildren::NamespaceName(Box::new(
                NamespaceNameNode::parse(node, source)?,
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
            "comment" => NamespaceUseGroupClauseChildren::Comment(Box::new(CommentNode::parse(
                node, source,
            )?)),
            "text_interpolation" => NamespaceUseGroupClauseChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => {
                NamespaceUseGroupClauseChildren::Error(Box::new(ErrorNode::parse(node, source)?))
            }
            "namespace_aliasing_clause" => {
                NamespaceUseGroupClauseChildren::NamespaceAliasingClause(Box::new(
                    NamespaceAliasingClauseNode::parse(node, source)?,
                ))
            }
            "namespace_name" => NamespaceUseGroupClauseChildren::NamespaceName(Box::new(
                NamespaceNameNode::parse(node, source)?,
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
            NamespaceUseGroupClauseChildren::Comment(x) => x.get_utype(state, emitter),
            NamespaceUseGroupClauseChildren::TextInterpolation(x) => x.get_utype(state, emitter),
            NamespaceUseGroupClauseChildren::Error(x) => x.get_utype(state, emitter),
            NamespaceUseGroupClauseChildren::NamespaceAliasingClause(x) => {
                x.get_utype(state, emitter)
            }
            NamespaceUseGroupClauseChildren::NamespaceName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            NamespaceUseGroupClauseChildren::Comment(x) => x.get_php_value(state, emitter),
            NamespaceUseGroupClauseChildren::TextInterpolation(x) => {
                x.get_php_value(state, emitter)
            }
            NamespaceUseGroupClauseChildren::Error(x) => x.get_php_value(state, emitter),
            NamespaceUseGroupClauseChildren::NamespaceAliasingClause(x) => {
                x.get_php_value(state, emitter)
            }
            NamespaceUseGroupClauseChildren::NamespaceName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            NamespaceUseGroupClauseChildren::Comment(x) => x.read_from(state, emitter),
            NamespaceUseGroupClauseChildren::TextInterpolation(x) => x.read_from(state, emitter),
            NamespaceUseGroupClauseChildren::Error(x) => x.read_from(state, emitter),
            NamespaceUseGroupClauseChildren::NamespaceAliasingClause(x) => {
                x.read_from(state, emitter)
            }
            NamespaceUseGroupClauseChildren::NamespaceName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for NamespaceUseGroupClauseChildren {
    fn brief_desc(&self) -> String {
        match self {
            NamespaceUseGroupClauseChildren::Comment(x) => format!(
                "NamespaceUseGroupClauseChildren::comment({})",
                x.brief_desc()
            ),
            NamespaceUseGroupClauseChildren::TextInterpolation(x) => format!(
                "NamespaceUseGroupClauseChildren::text_interpolation({})",
                x.brief_desc()
            ),
            NamespaceUseGroupClauseChildren::Error(x) => {
                format!("NamespaceUseGroupClauseChildren::ERROR({})", x.brief_desc())
            }
            NamespaceUseGroupClauseChildren::NamespaceAliasingClause(x) => format!(
                "NamespaceUseGroupClauseChildren::namespace_aliasing_clause({})",
                x.brief_desc()
            ),
            NamespaceUseGroupClauseChildren::NamespaceName(x) => format!(
                "NamespaceUseGroupClauseChildren::namespace_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            NamespaceUseGroupClauseChildren::Comment(x) => x.as_any(),
            NamespaceUseGroupClauseChildren::TextInterpolation(x) => x.as_any(),
            NamespaceUseGroupClauseChildren::Error(x) => x.as_any(),
            NamespaceUseGroupClauseChildren::NamespaceAliasingClause(x) => x.as_any(),
            NamespaceUseGroupClauseChildren::NamespaceName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            NamespaceUseGroupClauseChildren::Comment(x) => x.children_any(),
            NamespaceUseGroupClauseChildren::TextInterpolation(x) => x.children_any(),
            NamespaceUseGroupClauseChildren::Error(x) => x.children_any(),
            NamespaceUseGroupClauseChildren::NamespaceAliasingClause(x) => x.children_any(),
            NamespaceUseGroupClauseChildren::NamespaceName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            NamespaceUseGroupClauseChildren::Comment(x) => x.range(),
            NamespaceUseGroupClauseChildren::TextInterpolation(x) => x.range(),
            NamespaceUseGroupClauseChildren::Error(x) => x.range(),
            NamespaceUseGroupClauseChildren::NamespaceAliasingClause(x) => x.range(),
            NamespaceUseGroupClauseChildren::NamespaceName(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct NamespaceUseGroupClauseNode {
    pub range: Range,
    pub children: Vec<Box<NamespaceUseGroupClauseChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NamespaceUseGroupClauseNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "namespace_use_group_clause" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [namespace_use_group_clause] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            children: NamespaceUseGroupClauseChildren::parse_vec(
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
        "namespace_use_group_clause"
    }
}

impl NodeAccess for NamespaceUseGroupClauseNode {
    fn brief_desc(&self) -> String {
        "NamespaceUseGroupClauseNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::NamespaceUseGroupClause(self)
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
