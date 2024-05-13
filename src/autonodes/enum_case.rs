use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::attribute_list::AttributeListNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::encapsed_string::EncapsedStringNode;
use crate::autonodes::heredoc::HeredocNode;
use crate::autonodes::integer::IntegerNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::nowdoc::NowdocNode;
use crate::autonodes::string::StringNode;
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
pub enum EnumCaseValue {
    EncapsedString(Box<EncapsedStringNode>),
    Heredoc(Box<HeredocNode>),
    Integer(Box<IntegerNode>),
    Nowdoc(Box<NowdocNode>),
    String(Box<StringNode>),
    Extra(ExtraChild),
}

impl NodeParser for EnumCaseValue {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => EnumCaseValue::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "ERROR" => {
                EnumCaseValue::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "encapsed_string" => {
                EnumCaseValue::EncapsedString(Box::new(EncapsedStringNode::parse(node, source)?))
            }
            "heredoc" => EnumCaseValue::Heredoc(Box::new(HeredocNode::parse(node, source)?)),
            "integer" => EnumCaseValue::Integer(Box::new(IntegerNode::parse(node, source)?)),
            "nowdoc" => EnumCaseValue::Nowdoc(Box::new(NowdocNode::parse(node, source)?)),
            "string" => EnumCaseValue::String(Box::new(StringNode::parse(node, source)?)),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }
}

impl EnumCaseValue {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => EnumCaseValue::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "ERROR" => {
                EnumCaseValue::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "encapsed_string" => {
                EnumCaseValue::EncapsedString(Box::new(EncapsedStringNode::parse(node, source)?))
            }
            "heredoc" => EnumCaseValue::Heredoc(Box::new(HeredocNode::parse(node, source)?)),
            "integer" => EnumCaseValue::Integer(Box::new(IntegerNode::parse(node, source)?)),
            "nowdoc" => EnumCaseValue::Nowdoc(Box::new(NowdocNode::parse(node, source)?)),
            "string" => EnumCaseValue::String(Box::new(StringNode::parse(node, source)?)),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            EnumCaseValue::Extra(y) => y.kind(),
            EnumCaseValue::EncapsedString(y) => y.kind(),
            EnumCaseValue::Heredoc(y) => y.kind(),
            EnumCaseValue::Integer(y) => y.kind(),
            EnumCaseValue::Nowdoc(y) => y.kind(),
            EnumCaseValue::String(y) => y.kind(),
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
            EnumCaseValue::Extra(x) => x.get_utype(state, emitter),
            EnumCaseValue::EncapsedString(x) => x.get_utype(state, emitter),
            EnumCaseValue::Heredoc(x) => x.get_utype(state, emitter),
            EnumCaseValue::Integer(x) => x.get_utype(state, emitter),
            EnumCaseValue::Nowdoc(x) => x.get_utype(state, emitter),
            EnumCaseValue::String(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            EnumCaseValue::Extra(x) => x.get_php_value(state, emitter),
            EnumCaseValue::EncapsedString(x) => x.get_php_value(state, emitter),
            EnumCaseValue::Heredoc(x) => x.get_php_value(state, emitter),
            EnumCaseValue::Integer(x) => x.get_php_value(state, emitter),
            EnumCaseValue::Nowdoc(x) => x.get_php_value(state, emitter),
            EnumCaseValue::String(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            EnumCaseValue::Extra(x) => x.read_from(state, emitter),
            EnumCaseValue::EncapsedString(x) => x.read_from(state, emitter),
            EnumCaseValue::Heredoc(x) => x.read_from(state, emitter),
            EnumCaseValue::Integer(x) => x.read_from(state, emitter),
            EnumCaseValue::Nowdoc(x) => x.read_from(state, emitter),
            EnumCaseValue::String(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for EnumCaseValue {
    fn brief_desc(&self) -> String {
        match self {
            EnumCaseValue::Extra(x) => format!("EnumCaseValue::extra({})", x.brief_desc()),
            EnumCaseValue::EncapsedString(x) => {
                format!("EnumCaseValue::encapsed_string({})", x.brief_desc())
            }
            EnumCaseValue::Heredoc(x) => format!("EnumCaseValue::heredoc({})", x.brief_desc()),
            EnumCaseValue::Integer(x) => format!("EnumCaseValue::integer({})", x.brief_desc()),
            EnumCaseValue::Nowdoc(x) => format!("EnumCaseValue::nowdoc({})", x.brief_desc()),
            EnumCaseValue::String(x) => format!("EnumCaseValue::string({})", x.brief_desc()),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            EnumCaseValue::Extra(x) => x.as_any(),
            EnumCaseValue::EncapsedString(x) => x.as_any(),
            EnumCaseValue::Heredoc(x) => x.as_any(),
            EnumCaseValue::Integer(x) => x.as_any(),
            EnumCaseValue::Nowdoc(x) => x.as_any(),
            EnumCaseValue::String(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            EnumCaseValue::Extra(x) => x.children_any(),
            EnumCaseValue::EncapsedString(x) => x.children_any(),
            EnumCaseValue::Heredoc(x) => x.children_any(),
            EnumCaseValue::Integer(x) => x.children_any(),
            EnumCaseValue::Nowdoc(x) => x.children_any(),
            EnumCaseValue::String(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            EnumCaseValue::Extra(x) => x.range(),
            EnumCaseValue::EncapsedString(x) => x.range(),
            EnumCaseValue::Heredoc(x) => x.range(),
            EnumCaseValue::Integer(x) => x.range(),
            EnumCaseValue::Nowdoc(x) => x.range(),
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

impl NodeParser for EnumCaseNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
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
        let attributes: Option<AttributeListNode> =
            Into::<Result<_, _>>::into(node.parse_child("attributes", source))?;
        let name: NameNode = Into::<Result<_, _>>::into(node.parse_child("name", source))?;
        let value: Option<Box<EnumCaseValue>> =
            Into::<Result<_, _>>::into(node.parse_child("value", source))?;
        Ok(Self {
            range,
            attributes,
            name,
            value,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl EnumCaseNode {
    pub fn kind(&self) -> &'static str {
        "enum_case"
    }
}

impl NodeAccess for EnumCaseNode {
    fn brief_desc(&self) -> String {
        "EnumCaseNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
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
