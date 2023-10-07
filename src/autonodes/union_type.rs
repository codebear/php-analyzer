use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::named_type::NamedTypeNode;
use crate::autonodes::optional_type::OptionalTypeNode;
use crate::autonodes::primitive_type::PrimitiveTypeNode;
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
pub enum UnionTypeChildren {
    NamedType(Box<NamedTypeNode>),
    OptionalType(Box<OptionalTypeNode>),
    PrimitiveType(Box<PrimitiveTypeNode>),
    Extra(ExtraChild),
}

impl UnionTypeChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => UnionTypeChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => UnionTypeChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => UnionTypeChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "named_type" => {
                UnionTypeChildren::NamedType(Box::new(NamedTypeNode::parse(node, source)?))
            }
            "optional_type" => {
                UnionTypeChildren::OptionalType(Box::new(OptionalTypeNode::parse(node, source)?))
            }
            "primitive_type" => {
                UnionTypeChildren::PrimitiveType(Box::new(PrimitiveTypeNode::parse(node, source)?))
            }

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
            "comment" => UnionTypeChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => UnionTypeChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => UnionTypeChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "named_type" => {
                UnionTypeChildren::NamedType(Box::new(NamedTypeNode::parse(node, source)?))
            }
            "optional_type" => {
                UnionTypeChildren::OptionalType(Box::new(OptionalTypeNode::parse(node, source)?))
            }
            "primitive_type" => {
                UnionTypeChildren::PrimitiveType(Box::new(PrimitiveTypeNode::parse(node, source)?))
            }

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
            UnionTypeChildren::Extra(x) => x.get_utype(state, emitter),
            UnionTypeChildren::NamedType(x) => x.get_utype(state, emitter),
            UnionTypeChildren::OptionalType(x) => x.get_utype(state, emitter),
            UnionTypeChildren::PrimitiveType(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            UnionTypeChildren::Extra(x) => x.get_php_value(state, emitter),
            UnionTypeChildren::NamedType(x) => x.get_php_value(state, emitter),
            UnionTypeChildren::OptionalType(x) => x.get_php_value(state, emitter),
            UnionTypeChildren::PrimitiveType(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            UnionTypeChildren::Extra(x) => x.read_from(state, emitter),
            UnionTypeChildren::NamedType(x) => x.read_from(state, emitter),
            UnionTypeChildren::OptionalType(x) => x.read_from(state, emitter),
            UnionTypeChildren::PrimitiveType(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for UnionTypeChildren {
    fn brief_desc(&self) -> String {
        match self {
            UnionTypeChildren::Extra(x) => format!("UnionTypeChildren::extra({})", x.brief_desc()),
            UnionTypeChildren::NamedType(x) => {
                format!("UnionTypeChildren::named_type({})", x.brief_desc())
            }
            UnionTypeChildren::OptionalType(x) => {
                format!("UnionTypeChildren::optional_type({})", x.brief_desc())
            }
            UnionTypeChildren::PrimitiveType(x) => {
                format!("UnionTypeChildren::primitive_type({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            UnionTypeChildren::Extra(x) => x.as_any(),
            UnionTypeChildren::NamedType(x) => x.as_any(),
            UnionTypeChildren::OptionalType(x) => x.as_any(),
            UnionTypeChildren::PrimitiveType(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            UnionTypeChildren::Extra(x) => x.children_any(),
            UnionTypeChildren::NamedType(x) => x.children_any(),
            UnionTypeChildren::OptionalType(x) => x.children_any(),
            UnionTypeChildren::PrimitiveType(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            UnionTypeChildren::Extra(x) => x.range(),
            UnionTypeChildren::NamedType(x) => x.range(),
            UnionTypeChildren::OptionalType(x) => x.range(),
            UnionTypeChildren::PrimitiveType(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct UnionTypeNode {
    pub range: Range,
    pub children: Vec<Box<UnionTypeChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl UnionTypeNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "union_type" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [union_type] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            children: UnionTypeChildren::parse_vec(
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
        "union_type"
    }
}

impl NodeAccess for UnionTypeNode {
    fn brief_desc(&self) -> String {
        "UnionTypeNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::UnionType(self)
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
