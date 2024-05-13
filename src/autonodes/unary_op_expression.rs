use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::operators::add::AddOperator;
use crate::operators::binary_not::BinaryNotOperator;
use crate::operators::not::NotOperator;
use crate::operators::operator::Operator;
use crate::operators::sub::SubOperator;
use crate::parser::Range;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub enum UnaryOpExpressionOperator {
    Not(NotOperator),
    Add(AddOperator),
    Sub(SubOperator),
    BinaryNot(BinaryNotOperator),
    Extra(ExtraChild),
}

impl NodeParser for UnaryOpExpressionOperator {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => UnaryOpExpressionOperator::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => UnaryOpExpressionOperator::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "!" => UnaryOpExpressionOperator::Not(NotOperator(node.range().into())),
            "+" => UnaryOpExpressionOperator::Add(AddOperator(node.range().into())),
            "-" => UnaryOpExpressionOperator::Sub(SubOperator(node.range().into())),
            "~" => UnaryOpExpressionOperator::BinaryNot(BinaryNotOperator(node.range().into())),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }
}

impl UnaryOpExpressionOperator {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => UnaryOpExpressionOperator::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => UnaryOpExpressionOperator::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "!" => UnaryOpExpressionOperator::Not(NotOperator(node.range().into())),
            "+" => UnaryOpExpressionOperator::Add(AddOperator(node.range().into())),
            "-" => UnaryOpExpressionOperator::Sub(SubOperator(node.range().into())),
            "~" => UnaryOpExpressionOperator::BinaryNot(BinaryNotOperator(node.range().into())),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            UnaryOpExpressionOperator::Extra(y) => y.kind(),
            UnaryOpExpressionOperator::Not(y) => y.kind(),
            UnaryOpExpressionOperator::Add(y) => y.kind(),
            UnaryOpExpressionOperator::Sub(y) => y.kind(),
            UnaryOpExpressionOperator::BinaryNot(y) => y.kind(),
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
}

#[derive(Debug, Clone)]
pub struct UnaryOpExpressionNode {
    pub range: Range,
    pub argument: _ExpressionNode,
    pub operator: Box<UnaryOpExpressionOperator>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for UnaryOpExpressionNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "unary_op_expression" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [unary_op_expression] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let argument: _ExpressionNode = Result::from(node.parse_child("argument", source).into())?;
        let operator: Box<UnaryOpExpressionOperator> =
            Result::from(node.parse_child("operator", source).into())?;
        Ok(Self {
            range,
            argument,
            operator,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl UnaryOpExpressionNode {
    pub fn kind(&self) -> &'static str {
        "unary_op_expression"
    }
}

impl NodeAccess for UnaryOpExpressionNode {
    fn brief_desc(&self) -> String {
        "UnaryOpExpressionNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::UnaryOpExpression(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.argument.as_any());
        child_vec.push(self.operator.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
