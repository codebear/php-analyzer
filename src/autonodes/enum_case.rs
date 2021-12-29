use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::attribute_list::AttributeListNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::integer::IntegerNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::string::StringNode;
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
pub enum EnumCaseValue {
    Integer(Box<IntegerNode>),
    String(Box<StringNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl EnumCaseValue {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => EnumCaseValue::Comment(Box::new(CommentNode::parse(node, source)?)),
            "text_interpolation" => EnumCaseValue::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => EnumCaseValue::Error(Box::new(ErrorNode::parse(node, source)?)),
            "integer" => EnumCaseValue::Integer(Box::new(IntegerNode::parse(node, source)?)),
            "string" => EnumCaseValue::String(Box::new(StringNode::parse(node, source)?)),

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
            "comment" => EnumCaseValue::Comment(Box::new(CommentNode::parse(node, source)?)),
            "text_interpolation" => EnumCaseValue::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => EnumCaseValue::Error(Box::new(ErrorNode::parse(node, source)?)),
            "integer" => EnumCaseValue::Integer(Box::new(IntegerNode::parse(node, source)?)),
            "string" => EnumCaseValue::String(Box::new(StringNode::parse(node, source)?)),

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
            EnumCaseValue::Comment(x) => x.get_utype(state, emitter),
            EnumCaseValue::TextInterpolation(x) => x.get_utype(state, emitter),
            EnumCaseValue::Error(x) => x.get_utype(state, emitter),
            EnumCaseValue::Integer(x) => x.get_utype(state, emitter),
            EnumCaseValue::String(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            EnumCaseValue::Comment(x) => x.get_php_value(state, emitter),
            EnumCaseValue::TextInterpolation(x) => x.get_php_value(state, emitter),
            EnumCaseValue::Error(x) => x.get_php_value(state, emitter),
            EnumCaseValue::Integer(x) => x.get_php_value(state, emitter),
            EnumCaseValue::String(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            EnumCaseValue::Comment(x) => x.read_from(state, emitter),
            EnumCaseValue::TextInterpolation(x) => x.read_from(state, emitter),
            EnumCaseValue::Error(x) => x.read_from(state, emitter),
            EnumCaseValue::Integer(x) => x.read_from(state, emitter),
            EnumCaseValue::String(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for EnumCaseValue {
    fn brief_desc(&self) -> String {
        match self {
            EnumCaseValue::Comment(x) => format!("EnumCaseValue::comment({})", x.brief_desc()),
            EnumCaseValue::TextInterpolation(x) => {
                format!("EnumCaseValue::text_interpolation({})", x.brief_desc())
            }
            EnumCaseValue::Error(x) => format!("EnumCaseValue::ERROR({})", x.brief_desc()),
            EnumCaseValue::Integer(x) => format!("EnumCaseValue::integer({})", x.brief_desc()),
            EnumCaseValue::String(x) => format!("EnumCaseValue::string({})", x.brief_desc()),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            EnumCaseValue::Comment(x) => x.as_any(),
            EnumCaseValue::TextInterpolation(x) => x.as_any(),
            EnumCaseValue::Error(x) => x.as_any(),
            EnumCaseValue::Integer(x) => x.as_any(),
            EnumCaseValue::String(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            EnumCaseValue::Comment(x) => x.children_any(),
            EnumCaseValue::TextInterpolation(x) => x.children_any(),
            EnumCaseValue::Error(x) => x.children_any(),
            EnumCaseValue::Integer(x) => x.children_any(),
            EnumCaseValue::String(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            EnumCaseValue::Comment(x) => x.range(),
            EnumCaseValue::TextInterpolation(x) => x.range(),
            EnumCaseValue::Error(x) => x.range(),
            EnumCaseValue::Integer(x) => x.range(),
            EnumCaseValue::String(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct EnumCaseNode {
    pub range: Range,
    pub attributes: Option<AttributeListNode>,
    pub name: NameNode,
    pub value: Option<Box<EnumCaseValue>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl EnumCaseNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "enum_case" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [enum_case] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let attributes: Option<AttributeListNode> = node
            .children_by_field_name("attributes", &mut node.walk())
            .map(|chnode1| AttributeListNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        let name: NameNode = node
            .children_by_field_name("name", &mut node.walk())
            .map(|chnode1| NameNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field name should exist");
        let value: Option<Box<EnumCaseValue>> = node
            .children_by_field_name("value", &mut node.walk())
            .map(|chnode2| EnumCaseValue::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .into();
        Ok(Self {
            range,
            attributes,
            name,
            value,
            extras: vec![], // todo lookup unused nodes
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
        "enum_case"
    }
}

impl NodeAccess for EnumCaseNode {
    fn brief_desc(&self) -> String {
        "EnumCaseNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::EnumCase(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.attributes {
            child_vec.push(x.as_any());
        }
        child_vec.push(self.name.as_any());
        if let Some(x) = &self.value {
            child_vec.push(x.as_any());
        }

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
