use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::by_ref::ByRefNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::variadic_unpacking::VariadicUnpackingNode;
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
pub enum ArrayElementInitializerValue {
    _Expression(Box<_ExpressionNode>),
    ByRef(Box<ByRefNode>),
    Extra(ExtraChild),
}

impl ArrayElementInitializerValue {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ArrayElementInitializerValue::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                ArrayElementInitializerValue::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => ArrayElementInitializerValue::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "by_ref" => {
                ArrayElementInitializerValue::ByRef(Box::new(ByRefNode::parse(node, source)?))
            }

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| ArrayElementInitializerValue::_Expression(y))
                {
                    x
                } else {
                    return Err(ParseError::new(
                        node.range(),
                        format!("Parse error, unexpected node-type: {}", node.kind()),
                    ));
                }
            }
        })
    }

    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ArrayElementInitializerValue::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                ArrayElementInitializerValue::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => ArrayElementInitializerValue::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "by_ref" => {
                ArrayElementInitializerValue::ByRef(Box::new(ByRefNode::parse(node, source)?))
            }

            _ => {
                return Ok(
                    if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| ArrayElementInitializerValue::_Expression(y))
                    {
                        Some(x)
                    } else {
                        None
                    },
                )
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            ArrayElementInitializerValue::Extra(y) => y.kind(),
            ArrayElementInitializerValue::_Expression(y) => y.kind(),
            ArrayElementInitializerValue::ByRef(y) => y.kind(),
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
            ArrayElementInitializerValue::Extra(x) => x.get_utype(state, emitter),
            ArrayElementInitializerValue::_Expression(x) => x.get_utype(state, emitter),
            ArrayElementInitializerValue::ByRef(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ArrayElementInitializerValue::Extra(x) => x.get_php_value(state, emitter),
            ArrayElementInitializerValue::_Expression(x) => x.get_php_value(state, emitter),
            ArrayElementInitializerValue::ByRef(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ArrayElementInitializerValue::Extra(x) => x.read_from(state, emitter),
            ArrayElementInitializerValue::_Expression(x) => x.read_from(state, emitter),
            ArrayElementInitializerValue::ByRef(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ArrayElementInitializerValue {
    fn brief_desc(&self) -> String {
        match self {
            ArrayElementInitializerValue::Extra(x) => {
                format!("ArrayElementInitializerValue::extra({})", x.brief_desc())
            }
            ArrayElementInitializerValue::_Expression(x) => format!(
                "ArrayElementInitializerValue::_expression({})",
                x.brief_desc()
            ),
            ArrayElementInitializerValue::ByRef(x) => {
                format!("ArrayElementInitializerValue::by_ref({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ArrayElementInitializerValue::Extra(x) => x.as_any(),
            ArrayElementInitializerValue::_Expression(x) => x.as_any(),
            ArrayElementInitializerValue::ByRef(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ArrayElementInitializerValue::Extra(x) => x.children_any(),
            ArrayElementInitializerValue::_Expression(x) => x.children_any(),
            ArrayElementInitializerValue::ByRef(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ArrayElementInitializerValue::Extra(x) => x.range(),
            ArrayElementInitializerValue::_Expression(x) => x.range(),
            ArrayElementInitializerValue::ByRef(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArrayElementInitializerNode {
    pub range: Range,
    pub key: Option<_ExpressionNode>,
    pub spread: Option<VariadicUnpackingNode>,
    pub value: Option<Box<ArrayElementInitializerValue>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl ArrayElementInitializerNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "array_element_initializer" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [array_element_initializer] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let key: Option<_ExpressionNode> = node
            .children_by_field_name("key", &mut node.walk())
            .map(|chnode1| _ExpressionNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        let spread: Option<VariadicUnpackingNode> = node
            .children_by_field_name("spread", &mut node.walk())
            .map(|chnode1| VariadicUnpackingNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        let value: Option<Box<ArrayElementInitializerValue>> = node
            .children_by_field_name("value", &mut node.walk())
            .map(|chnode2| ArrayElementInitializerValue::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .into();
        Ok(Self {
            range,
            key,
            spread,
            value,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
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
        "array_element_initializer"
    }
}

impl NodeAccess for ArrayElementInitializerNode {
    fn brief_desc(&self) -> String {
        "ArrayElementInitializerNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::ArrayElementInitializer(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.key {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.spread {
            child_vec.push(x.as_any());
        }
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
