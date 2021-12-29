use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::arguments::ArgumentsNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::qualified_name::QualifiedNameNode;
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
pub enum AttributeChildren {
    Name(Box<NameNode>),
    QualifiedName(Box<QualifiedNameNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl AttributeChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => AttributeChildren::Comment(Box::new(CommentNode::parse(node, source)?)),
            "text_interpolation" => AttributeChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => AttributeChildren::Error(Box::new(ErrorNode::parse(node, source)?)),
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

    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => AttributeChildren::Comment(Box::new(CommentNode::parse(node, source)?)),
            "text_interpolation" => AttributeChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => AttributeChildren::Error(Box::new(ErrorNode::parse(node, source)?)),
            "name" => AttributeChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "qualified_name" => {
                AttributeChildren::QualifiedName(Box::new(QualifiedNameNode::parse(node, source)?))
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
            AttributeChildren::Comment(x) => x.get_utype(state, emitter),
            AttributeChildren::TextInterpolation(x) => x.get_utype(state, emitter),
            AttributeChildren::Error(x) => x.get_utype(state, emitter),
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
            AttributeChildren::Comment(x) => x.get_php_value(state, emitter),
            AttributeChildren::TextInterpolation(x) => x.get_php_value(state, emitter),
            AttributeChildren::Error(x) => x.get_php_value(state, emitter),
            AttributeChildren::Name(x) => x.get_php_value(state, emitter),
            AttributeChildren::QualifiedName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            AttributeChildren::Comment(x) => x.read_from(state, emitter),
            AttributeChildren::TextInterpolation(x) => x.read_from(state, emitter),
            AttributeChildren::Error(x) => x.read_from(state, emitter),
            AttributeChildren::Name(x) => x.read_from(state, emitter),
            AttributeChildren::QualifiedName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for AttributeChildren {
    fn brief_desc(&self) -> String {
        match self {
            AttributeChildren::Comment(x) => {
                format!("AttributeChildren::comment({})", x.brief_desc())
            }
            AttributeChildren::TextInterpolation(x) => {
                format!("AttributeChildren::text_interpolation({})", x.brief_desc())
            }
            AttributeChildren::Error(x) => format!("AttributeChildren::ERROR({})", x.brief_desc()),
            AttributeChildren::Name(x) => format!("AttributeChildren::name({})", x.brief_desc()),
            AttributeChildren::QualifiedName(x) => {
                format!("AttributeChildren::qualified_name({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            AttributeChildren::Comment(x) => x.as_any(),
            AttributeChildren::TextInterpolation(x) => x.as_any(),
            AttributeChildren::Error(x) => x.as_any(),
            AttributeChildren::Name(x) => x.as_any(),
            AttributeChildren::QualifiedName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            AttributeChildren::Comment(x) => x.children_any(),
            AttributeChildren::TextInterpolation(x) => x.children_any(),
            AttributeChildren::Error(x) => x.children_any(),
            AttributeChildren::Name(x) => x.children_any(),
            AttributeChildren::QualifiedName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            AttributeChildren::Comment(x) => x.range(),
            AttributeChildren::TextInterpolation(x) => x.range(),
            AttributeChildren::Error(x) => x.range(),
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

impl AttributeNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
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
        let parameters: Option<ArgumentsNode> = node
            .children_by_field_name("parameters", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode1| ArgumentsNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
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
