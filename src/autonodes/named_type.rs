use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::qualified_name::QualifiedNameNode;
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
pub enum NamedTypeChildren {
    Name(Box<NameNode>),
    QualifiedName(Box<QualifiedNameNode>),
    Extra(ExtraChild),
}

impl NodeParser for NamedTypeChildren {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => NamedTypeChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => NamedTypeChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "name" => NamedTypeChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "qualified_name" => {
                NamedTypeChildren::QualifiedName(Box::new(QualifiedNameNode::parse(node, source)?))
            }

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }
}

impl NamedTypeChildren {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => NamedTypeChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => NamedTypeChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "name" => NamedTypeChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "qualified_name" => {
                NamedTypeChildren::QualifiedName(Box::new(QualifiedNameNode::parse(node, source)?))
            }

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            NamedTypeChildren::Extra(y) => y.kind(),
            NamedTypeChildren::Name(y) => y.kind(),
            NamedTypeChildren::QualifiedName(y) => y.kind(),
        }
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
            NamedTypeChildren::Extra(x) => x.get_utype(state, emitter),
            NamedTypeChildren::Name(x) => x.get_utype(state, emitter),
            NamedTypeChildren::QualifiedName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            NamedTypeChildren::Extra(x) => x.get_php_value(state, emitter),
            NamedTypeChildren::Name(x) => x.get_php_value(state, emitter),
            NamedTypeChildren::QualifiedName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            NamedTypeChildren::Extra(x) => x.read_from(state, emitter),
            NamedTypeChildren::Name(x) => x.read_from(state, emitter),
            NamedTypeChildren::QualifiedName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for NamedTypeChildren {
    fn brief_desc(&self) -> String {
        match self {
            NamedTypeChildren::Extra(x) => format!("NamedTypeChildren::extra({})", x.brief_desc()),
            NamedTypeChildren::Name(x) => format!("NamedTypeChildren::name({})", x.brief_desc()),
            NamedTypeChildren::QualifiedName(x) => {
                format!("NamedTypeChildren::qualified_name({})", x.brief_desc())
            }
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            NamedTypeChildren::Extra(x) => x.as_any(),
            NamedTypeChildren::Name(x) => x.as_any(),
            NamedTypeChildren::QualifiedName(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            NamedTypeChildren::Extra(x) => x.children_any(),
            NamedTypeChildren::Name(x) => x.children_any(),
            NamedTypeChildren::QualifiedName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            NamedTypeChildren::Extra(x) => x.range(),
            NamedTypeChildren::Name(x) => x.range(),
            NamedTypeChildren::QualifiedName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NamedTypeNode {
    pub range: Range,
    pub child: Box<NamedTypeChildren>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for NamedTypeNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "named_type" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [named_type] on pos {}:{}",
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
                .map(|k| NamedTypeChildren::parse(k, source))
                .collect::<Result<Vec<NamedTypeChildren>, ParseError>>()?
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

impl NamedTypeNode {
    pub fn kind(&self) -> &'static str {
        "named_type"
    }
}

impl NodeAccess for NamedTypeNode {
    fn brief_desc(&self) -> String {
        "NamedTypeNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::NamedType(self)
    }

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
