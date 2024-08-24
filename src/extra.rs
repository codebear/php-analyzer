use tree_sitter::Node;

use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, comment::CommentNode, text_interpolation::TextInterpolationNode},
    autotree::{NodeAccess, NodeParser, ParseError},
    errornode::ErrorNode,
    issue::IssueEmitter,
    types::union::PHPType,
    value::PHPValue,
};

#[derive(Debug, Clone)]
pub enum ExtraChild {
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl ExtraChild {
    pub fn parse_vec<'a, I>(children: I, source: &[u8]) -> Result<Vec<Box<Self>>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
    {
        let mut res: Vec<Box<Self>> = vec![];
        for child in children {
            res.push(Box::new(Self::parse(child, source)?));
        }
        Ok(res)
    }

    pub fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ExtraChild::Comment(Box::new(CommentNode::parse(node, source)?)),
            "text_interpolation" => {
                ExtraChild::TextInterpolation(Box::new(TextInterpolationNode::parse(node, source)?))
            }
            "ERROR" => ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)),
            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }
    pub fn kind(&self) -> &'static str {
        match self {
            ExtraChild::Comment(c) => c.kind(),
            ExtraChild::TextInterpolation(c) => c.kind(),
            ExtraChild::Error(e) => e.kind(),
        }
    }
}
impl NodeAccess for ExtraChild {
    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            ExtraChild::Comment(c) => c.as_any(),
            ExtraChild::TextInterpolation(c) => c.as_any(),
            ExtraChild::Error(e) => e.as_any(),
        }
    }

    fn brief_desc(&self) -> String {
        match self {
            ExtraChild::Comment(c) => c.brief_desc(),
            ExtraChild::TextInterpolation(c) => c.brief_desc(),
            ExtraChild::Error(e) => e.brief_desc(),
        }
    }

    fn range(&self) -> crate::parser::Range {
        match self {
            ExtraChild::Comment(c) => c.range(),
            ExtraChild::TextInterpolation(c) => c.range(),
            ExtraChild::Error(e) => e.range(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            ExtraChild::Comment(c) => c.children_any(),
            ExtraChild::TextInterpolation(c) => c.children_any(),
            ExtraChild::Error(e) => e.children_any(),
        }
    }
}

impl ExtraChild {
    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPType> {
        match self {
            ExtraChild::Comment(c) => c.get_utype(state, emitter),
            ExtraChild::TextInterpolation(c) => c.get_utype(state, emitter),
            ExtraChild::Error(e) => e.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ExtraChild::Comment(c) => c.get_php_value(state, emitter),
            ExtraChild::TextInterpolation(c) => c.get_php_value(state, emitter),
            ExtraChild::Error(e) => e.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ExtraChild::Comment(c) => c.read_from(state, emitter),
            ExtraChild::TextInterpolation(c) => c.read_from(state, emitter),
            ExtraChild::Error(e) => e.read_from(state, emitter),
        }
    }
}
