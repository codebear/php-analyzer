use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::intersection_type::IntersectionTypeNode;
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
pub enum DisjunctiveNormalFormTypeChildren {
    IntersectionType(Box<IntersectionTypeNode>),
    NamedType(Box<NamedTypeNode>),
    OptionalType(Box<OptionalTypeNode>),
    PrimitiveType(Box<PrimitiveTypeNode>),
    Extra(ExtraChild),
}

impl NodeParser for DisjunctiveNormalFormTypeChildren {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => DisjunctiveNormalFormTypeChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => DisjunctiveNormalFormTypeChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "intersection_type" => DisjunctiveNormalFormTypeChildren::IntersectionType(Box::new(
                IntersectionTypeNode::parse(node, source)?,
            )),
            "named_type" => DisjunctiveNormalFormTypeChildren::NamedType(Box::new(
                NamedTypeNode::parse(node, source)?,
            )),
            "optional_type" => DisjunctiveNormalFormTypeChildren::OptionalType(Box::new(
                OptionalTypeNode::parse(node, source)?,
            )),
            "primitive_type" => DisjunctiveNormalFormTypeChildren::PrimitiveType(Box::new(
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

impl DisjunctiveNormalFormTypeChildren {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => DisjunctiveNormalFormTypeChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => DisjunctiveNormalFormTypeChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "intersection_type" => DisjunctiveNormalFormTypeChildren::IntersectionType(Box::new(
                IntersectionTypeNode::parse(node, source)?,
            )),
            "named_type" => DisjunctiveNormalFormTypeChildren::NamedType(Box::new(
                NamedTypeNode::parse(node, source)?,
            )),
            "optional_type" => DisjunctiveNormalFormTypeChildren::OptionalType(Box::new(
                OptionalTypeNode::parse(node, source)?,
            )),
            "primitive_type" => DisjunctiveNormalFormTypeChildren::PrimitiveType(Box::new(
                PrimitiveTypeNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            DisjunctiveNormalFormTypeChildren::Extra(y) => y.kind(),
            DisjunctiveNormalFormTypeChildren::IntersectionType(y) => y.kind(),
            DisjunctiveNormalFormTypeChildren::NamedType(y) => y.kind(),
            DisjunctiveNormalFormTypeChildren::OptionalType(y) => y.kind(),
            DisjunctiveNormalFormTypeChildren::PrimitiveType(y) => y.kind(),
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
            DisjunctiveNormalFormTypeChildren::Extra(x) => x.get_utype(state, emitter),
            DisjunctiveNormalFormTypeChildren::IntersectionType(x) => x.get_utype(state, emitter),
            DisjunctiveNormalFormTypeChildren::NamedType(x) => x.get_utype(state, emitter),
            DisjunctiveNormalFormTypeChildren::OptionalType(x) => x.get_utype(state, emitter),
            DisjunctiveNormalFormTypeChildren::PrimitiveType(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            DisjunctiveNormalFormTypeChildren::Extra(x) => x.get_php_value(state, emitter),
            DisjunctiveNormalFormTypeChildren::IntersectionType(x) => {
                x.get_php_value(state, emitter)
            }
            DisjunctiveNormalFormTypeChildren::NamedType(x) => x.get_php_value(state, emitter),
            DisjunctiveNormalFormTypeChildren::OptionalType(x) => x.get_php_value(state, emitter),
            DisjunctiveNormalFormTypeChildren::PrimitiveType(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            DisjunctiveNormalFormTypeChildren::Extra(x) => x.read_from(state, emitter),
            DisjunctiveNormalFormTypeChildren::IntersectionType(x) => x.read_from(state, emitter),
            DisjunctiveNormalFormTypeChildren::NamedType(x) => x.read_from(state, emitter),
            DisjunctiveNormalFormTypeChildren::OptionalType(x) => x.read_from(state, emitter),
            DisjunctiveNormalFormTypeChildren::PrimitiveType(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for DisjunctiveNormalFormTypeChildren {
    fn brief_desc(&self) -> String {
        match self {
            DisjunctiveNormalFormTypeChildren::Extra(x) => format!(
                "DisjunctiveNormalFormTypeChildren::extra({})",
                x.brief_desc()
            ),
            DisjunctiveNormalFormTypeChildren::IntersectionType(x) => format!(
                "DisjunctiveNormalFormTypeChildren::intersection_type({})",
                x.brief_desc()
            ),
            DisjunctiveNormalFormTypeChildren::NamedType(x) => format!(
                "DisjunctiveNormalFormTypeChildren::named_type({})",
                x.brief_desc()
            ),
            DisjunctiveNormalFormTypeChildren::OptionalType(x) => format!(
                "DisjunctiveNormalFormTypeChildren::optional_type({})",
                x.brief_desc()
            ),
            DisjunctiveNormalFormTypeChildren::PrimitiveType(x) => format!(
                "DisjunctiveNormalFormTypeChildren::primitive_type({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            DisjunctiveNormalFormTypeChildren::Extra(x) => x.as_any(),
            DisjunctiveNormalFormTypeChildren::IntersectionType(x) => x.as_any(),
            DisjunctiveNormalFormTypeChildren::NamedType(x) => x.as_any(),
            DisjunctiveNormalFormTypeChildren::OptionalType(x) => x.as_any(),
            DisjunctiveNormalFormTypeChildren::PrimitiveType(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            DisjunctiveNormalFormTypeChildren::Extra(x) => x.children_any(),
            DisjunctiveNormalFormTypeChildren::IntersectionType(x) => x.children_any(),
            DisjunctiveNormalFormTypeChildren::NamedType(x) => x.children_any(),
            DisjunctiveNormalFormTypeChildren::OptionalType(x) => x.children_any(),
            DisjunctiveNormalFormTypeChildren::PrimitiveType(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            DisjunctiveNormalFormTypeChildren::Extra(x) => x.range(),
            DisjunctiveNormalFormTypeChildren::IntersectionType(x) => x.range(),
            DisjunctiveNormalFormTypeChildren::NamedType(x) => x.range(),
            DisjunctiveNormalFormTypeChildren::OptionalType(x) => x.range(),
            DisjunctiveNormalFormTypeChildren::PrimitiveType(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DisjunctiveNormalFormTypeNode {
    pub range: Range,
    pub children: Vec<Box<DisjunctiveNormalFormTypeChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for DisjunctiveNormalFormTypeNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "disjunctive_normal_form_type" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [disjunctive_normal_form_type] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            children: DisjunctiveNormalFormTypeChildren::parse_vec(
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

impl DisjunctiveNormalFormTypeNode {
    pub fn kind(&self) -> &'static str {
        "disjunctive_normal_form_type"
    }
}

impl NodeAccess for DisjunctiveNormalFormTypeNode {
    fn brief_desc(&self) -> String {
        "DisjunctiveNormalFormTypeNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::DisjunctiveNormalFormType(self)
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
