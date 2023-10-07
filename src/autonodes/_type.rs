use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::intersection_type::IntersectionTypeNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::union_type::UnionTypeNode;
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
pub enum _TypeNode {
    IntersectionType(Box<IntersectionTypeNode>),
    UnionType(Box<UnionTypeNode>),
    Extra(ExtraChild),
}

impl _TypeNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => _TypeNode::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "text_interpolation" => _TypeNode::Extra(ExtraChild::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            ))),
            "ERROR" => {
                _TypeNode::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "intersection_type" => {
                _TypeNode::IntersectionType(Box::new(IntersectionTypeNode::parse(node, source)?))
            }
            "union_type" => _TypeNode::UnionType(Box::new(UnionTypeNode::parse(node, source)?)),

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
            "comment" => _TypeNode::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "text_interpolation" => _TypeNode::Extra(ExtraChild::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            ))),
            "ERROR" => {
                _TypeNode::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "intersection_type" => {
                _TypeNode::IntersectionType(Box::new(IntersectionTypeNode::parse(node, source)?))
            }
            "union_type" => _TypeNode::UnionType(Box::new(UnionTypeNode::parse(node, source)?)),

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
            _TypeNode::Extra(x) => x.get_utype(state, emitter),
            _TypeNode::IntersectionType(x) => x.get_utype(state, emitter),
            _TypeNode::UnionType(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            _TypeNode::Extra(x) => x.get_php_value(state, emitter),
            _TypeNode::IntersectionType(x) => x.get_php_value(state, emitter),
            _TypeNode::UnionType(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            _TypeNode::Extra(x) => x.read_from(state, emitter),
            _TypeNode::IntersectionType(x) => x.read_from(state, emitter),
            _TypeNode::UnionType(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for _TypeNode {
    fn brief_desc(&self) -> String {
        match self {
            _TypeNode::Extra(x) => format!("_TypeNode::extra({})", x.brief_desc()),
            _TypeNode::IntersectionType(x) => {
                format!("_TypeNode::intersection_type({})", x.brief_desc())
            }
            _TypeNode::UnionType(x) => format!("_TypeNode::union_type({})", x.brief_desc()),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            _TypeNode::Extra(x) => x.as_any(),
            _TypeNode::IntersectionType(x) => x.as_any(),
            _TypeNode::UnionType(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            _TypeNode::Extra(x) => x.children_any(),
            _TypeNode::IntersectionType(x) => x.children_any(),
            _TypeNode::UnionType(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            _TypeNode::Extra(x) => x.range(),
            _TypeNode::IntersectionType(x) => x.range(),
            _TypeNode::UnionType(x) => x.range(),
        }
    }
}
