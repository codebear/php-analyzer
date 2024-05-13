use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::named_type::NamedTypeNode;
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
pub enum OptionalTypeChildren {
    NamedType(Box<NamedTypeNode>),
    PrimitiveType(Box<PrimitiveTypeNode>),
    Extra(ExtraChild),
}

impl NodeParser for OptionalTypeChildren {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => OptionalTypeChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => OptionalTypeChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "named_type" => {
                OptionalTypeChildren::NamedType(Box::new(NamedTypeNode::parse(node, source)?))
            }
            "primitive_type" => OptionalTypeChildren::PrimitiveType(Box::new(
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

impl OptionalTypeChildren {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => OptionalTypeChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => OptionalTypeChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "named_type" => {
                OptionalTypeChildren::NamedType(Box::new(NamedTypeNode::parse(node, source)?))
            }
            "primitive_type" => OptionalTypeChildren::PrimitiveType(Box::new(
                PrimitiveTypeNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            OptionalTypeChildren::Extra(y) => y.kind(),
            OptionalTypeChildren::NamedType(y) => y.kind(),
            OptionalTypeChildren::PrimitiveType(y) => y.kind(),
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
            OptionalTypeChildren::Extra(x) => x.get_utype(state, emitter),
            OptionalTypeChildren::NamedType(x) => x.get_utype(state, emitter),
            OptionalTypeChildren::PrimitiveType(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            OptionalTypeChildren::Extra(x) => x.get_php_value(state, emitter),
            OptionalTypeChildren::NamedType(x) => x.get_php_value(state, emitter),
            OptionalTypeChildren::PrimitiveType(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            OptionalTypeChildren::Extra(x) => x.read_from(state, emitter),
            OptionalTypeChildren::NamedType(x) => x.read_from(state, emitter),
            OptionalTypeChildren::PrimitiveType(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for OptionalTypeChildren {
    fn brief_desc(&self) -> String {
        match self {
            OptionalTypeChildren::Extra(x) => {
                format!("OptionalTypeChildren::extra({})", x.brief_desc())
            }
            OptionalTypeChildren::NamedType(x) => {
                format!("OptionalTypeChildren::named_type({})", x.brief_desc())
            }
            OptionalTypeChildren::PrimitiveType(x) => {
                format!("OptionalTypeChildren::primitive_type({})", x.brief_desc())
            }
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            OptionalTypeChildren::Extra(x) => x.as_any(),
            OptionalTypeChildren::NamedType(x) => x.as_any(),
            OptionalTypeChildren::PrimitiveType(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            OptionalTypeChildren::Extra(x) => x.children_any(),
            OptionalTypeChildren::NamedType(x) => x.children_any(),
            OptionalTypeChildren::PrimitiveType(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            OptionalTypeChildren::Extra(x) => x.range(),
            OptionalTypeChildren::NamedType(x) => x.range(),
            OptionalTypeChildren::PrimitiveType(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OptionalTypeNode {
    pub range: Range,
    pub child: Box<OptionalTypeChildren>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for OptionalTypeNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "optional_type" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [optional_type] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            child: node
                .named_children(&mut node.walk())
                .filter(|node| node.kind() != "comment")
                .map(|k| OptionalTypeChildren::parse(k, source))
                .collect::<Result<Vec<OptionalTypeChildren>, ParseError>>()?
                .drain(..)
                .map(Box::new)
                .next()
                .expect("Should be a child"),
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )?,
        })
    }
}

impl OptionalTypeNode {
    pub fn kind(&self) -> &'static str {
        "optional_type"
    }
}

impl NodeAccess for OptionalTypeNode {
    fn brief_desc(&self) -> String {
        "OptionalTypeNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::OptionalType(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.child.as_any());
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
