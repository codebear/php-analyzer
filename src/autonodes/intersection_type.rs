use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::named_type::NamedTypeNode;
use crate::autonodes::optional_type::OptionalTypeNode;
use crate::autonodes::primitive_type::PrimitiveTypeNode;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::parser::Range;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub enum IntersectionTypeChildren {
    NamedType(Box<NamedTypeNode>),
    OptionalType(Box<OptionalTypeNode>),
    PrimitiveType(Box<PrimitiveTypeNode>),
    Extra(ExtraChild),
}

impl NodeParser for IntersectionTypeChildren {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => IntersectionTypeChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => IntersectionTypeChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "named_type" => {
                IntersectionTypeChildren::NamedType(Box::new(NamedTypeNode::parse(node, source)?))
            }
            "optional_type" => IntersectionTypeChildren::OptionalType(Box::new(
                OptionalTypeNode::parse(node, source)?,
            )),
            "primitive_type" => IntersectionTypeChildren::PrimitiveType(Box::new(
                PrimitiveTypeNode::parse(node, source)?,
            )),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }
}

impl IntersectionTypeChildren {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => IntersectionTypeChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => IntersectionTypeChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "named_type" => {
                IntersectionTypeChildren::NamedType(Box::new(NamedTypeNode::parse(node, source)?))
            }
            "optional_type" => IntersectionTypeChildren::OptionalType(Box::new(
                OptionalTypeNode::parse(node, source)?,
            )),
            "primitive_type" => IntersectionTypeChildren::PrimitiveType(Box::new(
                PrimitiveTypeNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            IntersectionTypeChildren::Extra(y) => y.kind(),
            IntersectionTypeChildren::NamedType(y) => y.kind(),
            IntersectionTypeChildren::OptionalType(y) => y.kind(),
            IntersectionTypeChildren::PrimitiveType(y) => y.kind(),
        }
    }

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

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        match self {
            IntersectionTypeChildren::Extra(x) => x.get_utype(state, emitter),
            IntersectionTypeChildren::NamedType(x) => x.get_utype(state, emitter),
            IntersectionTypeChildren::OptionalType(x) => x.get_utype(state, emitter),
            IntersectionTypeChildren::PrimitiveType(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            IntersectionTypeChildren::Extra(x) => x.get_php_value(state, emitter),
            IntersectionTypeChildren::NamedType(x) => x.get_php_value(state, emitter),
            IntersectionTypeChildren::OptionalType(x) => x.get_php_value(state, emitter),
            IntersectionTypeChildren::PrimitiveType(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            IntersectionTypeChildren::Extra(x) => x.read_from(state, emitter),
            IntersectionTypeChildren::NamedType(x) => x.read_from(state, emitter),
            IntersectionTypeChildren::OptionalType(x) => x.read_from(state, emitter),
            IntersectionTypeChildren::PrimitiveType(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for IntersectionTypeChildren {
    fn brief_desc(&self) -> String {
        match self {
            IntersectionTypeChildren::Extra(x) => {
                format!("IntersectionTypeChildren::extra({})", x.brief_desc())
            }
            IntersectionTypeChildren::NamedType(x) => {
                format!("IntersectionTypeChildren::named_type({})", x.brief_desc())
            }
            IntersectionTypeChildren::OptionalType(x) => format!(
                "IntersectionTypeChildren::optional_type({})",
                x.brief_desc()
            ),
            IntersectionTypeChildren::PrimitiveType(x) => format!(
                "IntersectionTypeChildren::primitive_type({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            IntersectionTypeChildren::Extra(x) => x.as_any(),
            IntersectionTypeChildren::NamedType(x) => x.as_any(),
            IntersectionTypeChildren::OptionalType(x) => x.as_any(),
            IntersectionTypeChildren::PrimitiveType(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            IntersectionTypeChildren::Extra(x) => x.children_any(),
            IntersectionTypeChildren::NamedType(x) => x.children_any(),
            IntersectionTypeChildren::OptionalType(x) => x.children_any(),
            IntersectionTypeChildren::PrimitiveType(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            IntersectionTypeChildren::Extra(x) => x.range(),
            IntersectionTypeChildren::NamedType(x) => x.range(),
            IntersectionTypeChildren::OptionalType(x) => x.range(),
            IntersectionTypeChildren::PrimitiveType(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntersectionTypeNode {
    pub range: Range,
    pub children: Vec<Box<IntersectionTypeChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for IntersectionTypeNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "intersection_type" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [intersection_type] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            children: IntersectionTypeChildren::parse_vec(
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
}

impl IntersectionTypeNode {
    pub fn kind(&self) -> &'static str {
        "intersection_type"
    }
}

impl NodeAccess for IntersectionTypeNode {
    fn brief_desc(&self) -> String {
        "IntersectionTypeNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::IntersectionType(self)
    }

    #[allow(clippy::vec_init_then_push)]
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
