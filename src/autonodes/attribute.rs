use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::arguments::ArgumentsNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::qualified_name::QualifiedNameNode;
use crate::autotree::ChildNodeParser;
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
pub enum AttributeChildren {
    Name(Box<NameNode>),
    QualifiedName(Box<QualifiedNameNode>),
    Extra(ExtraChild),
}

impl NodeParser for AttributeChildren {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => AttributeChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => AttributeChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "name" => AttributeChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "qualified_name" => {
                AttributeChildren::QualifiedName(Box::new(QualifiedNameNode::parse(node, source)?))
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

impl AttributeChildren {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => AttributeChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => AttributeChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "name" => AttributeChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "qualified_name" => {
                AttributeChildren::QualifiedName(Box::new(QualifiedNameNode::parse(node, source)?))
            }

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            AttributeChildren::Extra(y) => y.kind(),
            AttributeChildren::Name(y) => y.kind(),
            AttributeChildren::QualifiedName(y) => y.kind(),
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
            AttributeChildren::Extra(x) => x.get_utype(state, emitter),
            AttributeChildren::Name(x) => x.get_utype(state, emitter),
            AttributeChildren::QualifiedName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            AttributeChildren::Extra(x) => x.get_php_value(state, emitter),
            AttributeChildren::Name(x) => x.get_php_value(state, emitter),
            AttributeChildren::QualifiedName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            AttributeChildren::Extra(x) => x.read_from(state, emitter),
            AttributeChildren::Name(x) => x.read_from(state, emitter),
            AttributeChildren::QualifiedName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for AttributeChildren {
    fn brief_desc(&self) -> String {
        match self {
            AttributeChildren::Extra(x) => format!("AttributeChildren::extra({})", x.brief_desc()),
            AttributeChildren::Name(x) => format!("AttributeChildren::name({})", x.brief_desc()),
            AttributeChildren::QualifiedName(x) => {
                format!("AttributeChildren::qualified_name({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            AttributeChildren::Extra(x) => x.as_any(),
            AttributeChildren::Name(x) => x.as_any(),
            AttributeChildren::QualifiedName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            AttributeChildren::Extra(x) => x.children_any(),
            AttributeChildren::Name(x) => x.children_any(),
            AttributeChildren::QualifiedName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            AttributeChildren::Extra(x) => x.range(),
            AttributeChildren::Name(x) => x.range(),
            AttributeChildren::QualifiedName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AttributeNode {
    pub range: Range,
    pub parameters: Option<ArgumentsNode>,
    pub child: Box<AttributeChildren>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for AttributeNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "attribute" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [attribute] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let mut skip_nodes: Vec<usize> = vec![];
        let parameters: Option<ArgumentsNode> = Result::from(
            node.parse_child("parameters", source)
                .mark_skipped_node(&mut skip_nodes)
                .into(),
        )?;
        Ok(Self {
            range,
            parameters,
            child: node
                .named_children(&mut node.walk())
                .filter(|node| !skip_nodes.contains(&node.id()))
                .filter(|node| node.kind() != "comment")
                .map(|k| AttributeChildren::parse(k, source))
                .collect::<Result<Vec<AttributeChildren>, ParseError>>()?
                .drain(..)
                .map(|j| Box::new(j))
                .next()
                .expect("Should be a child"),
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment")
                    .filter(|node| !skip_nodes.contains(&node.id())),
                source,
            )?,
        })
    }
}

impl AttributeNode {
    pub fn kind(&self) -> &'static str {
        "attribute"
    }
}

impl NodeAccess for AttributeNode {
    fn brief_desc(&self) -> String {
        "AttributeNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::Attribute(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.parameters {
            child_vec.push(x.as_any());
        }
        child_vec.push(self.child.as_any());
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
