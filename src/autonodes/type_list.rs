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
pub enum TypeListChildren {
    NamedType(Box<NamedTypeNode>),
    OptionalType(Box<OptionalTypeNode>),
    PrimitiveType(Box<PrimitiveTypeNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl TypeListChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => TypeListChildren::Comment(Box::new(CommentNode::parse(node, source)?)),
            "text_interpolation" => TypeListChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => TypeListChildren::Error(Box::new(ErrorNode::parse(node, source)?)),
            "named_type" => {
                TypeListChildren::NamedType(Box::new(NamedTypeNode::parse(node, source)?))
            }
            "optional_type" => {
                TypeListChildren::OptionalType(Box::new(OptionalTypeNode::parse(node, source)?))
            }
            "primitive_type" => {
                TypeListChildren::PrimitiveType(Box::new(PrimitiveTypeNode::parse(node, source)?))
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
            "comment" => TypeListChildren::Comment(Box::new(CommentNode::parse(node, source)?)),
            "text_interpolation" => TypeListChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => TypeListChildren::Error(Box::new(ErrorNode::parse(node, source)?)),
            "named_type" => {
                TypeListChildren::NamedType(Box::new(NamedTypeNode::parse(node, source)?))
            }
            "optional_type" => {
                TypeListChildren::OptionalType(Box::new(OptionalTypeNode::parse(node, source)?))
            }
            "primitive_type" => {
                TypeListChildren::PrimitiveType(Box::new(PrimitiveTypeNode::parse(node, source)?))
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
            TypeListChildren::Comment(x) => x.get_utype(state, emitter),
            TypeListChildren::TextInterpolation(x) => x.get_utype(state, emitter),
            TypeListChildren::Error(x) => x.get_utype(state, emitter),
            TypeListChildren::NamedType(x) => x.get_utype(state, emitter),
            TypeListChildren::OptionalType(x) => x.get_utype(state, emitter),
            TypeListChildren::PrimitiveType(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            TypeListChildren::Comment(x) => x.get_php_value(state, emitter),
            TypeListChildren::TextInterpolation(x) => x.get_php_value(state, emitter),
            TypeListChildren::Error(x) => x.get_php_value(state, emitter),
            TypeListChildren::NamedType(x) => x.get_php_value(state, emitter),
            TypeListChildren::OptionalType(x) => x.get_php_value(state, emitter),
            TypeListChildren::PrimitiveType(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            TypeListChildren::Comment(x) => x.read_from(state, emitter),
            TypeListChildren::TextInterpolation(x) => x.read_from(state, emitter),
            TypeListChildren::Error(x) => x.read_from(state, emitter),
            TypeListChildren::NamedType(x) => x.read_from(state, emitter),
            TypeListChildren::OptionalType(x) => x.read_from(state, emitter),
            TypeListChildren::PrimitiveType(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for TypeListChildren {
    fn brief_desc(&self) -> String {
        match self {
            TypeListChildren::Comment(x) => {
                format!("TypeListChildren::comment({})", x.brief_desc())
            }
            TypeListChildren::TextInterpolation(x) => {
                format!("TypeListChildren::text_interpolation({})", x.brief_desc())
            }
            TypeListChildren::Error(x) => format!("TypeListChildren::ERROR({})", x.brief_desc()),
            TypeListChildren::NamedType(x) => {
                format!("TypeListChildren::named_type({})", x.brief_desc())
            }
            TypeListChildren::OptionalType(x) => {
                format!("TypeListChildren::optional_type({})", x.brief_desc())
            }
            TypeListChildren::PrimitiveType(x) => {
                format!("TypeListChildren::primitive_type({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            TypeListChildren::Comment(x) => x.as_any(),
            TypeListChildren::TextInterpolation(x) => x.as_any(),
            TypeListChildren::Error(x) => x.as_any(),
            TypeListChildren::NamedType(x) => x.as_any(),
            TypeListChildren::OptionalType(x) => x.as_any(),
            TypeListChildren::PrimitiveType(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            TypeListChildren::Comment(x) => x.children_any(),
            TypeListChildren::TextInterpolation(x) => x.children_any(),
            TypeListChildren::Error(x) => x.children_any(),
            TypeListChildren::NamedType(x) => x.children_any(),
            TypeListChildren::OptionalType(x) => x.children_any(),
            TypeListChildren::PrimitiveType(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            TypeListChildren::Comment(x) => x.range(),
            TypeListChildren::TextInterpolation(x) => x.range(),
            TypeListChildren::Error(x) => x.range(),
            TypeListChildren::NamedType(x) => x.range(),
            TypeListChildren::OptionalType(x) => x.range(),
            TypeListChildren::PrimitiveType(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TypeListNode {
    pub range: Range,
    pub children: Vec<Box<TypeListChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl TypeListNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "type_list" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [type_list] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            children: TypeListChildren::parse_vec(
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
        "type_list"
    }
}

impl NodeAccess for TypeListNode {
    fn brief_desc(&self) -> String {
        "TypeListNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::TypeList(self)
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
