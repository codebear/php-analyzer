use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::integer::IntegerNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autotree::NodeAccess;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;

use crate::operators::add::AddOperator;
use crate::operators::binary_not::BinaryNotOperator;
use crate::operators::not::NotOperator;
use crate::operators::operator::Operator;
use crate::operators::sub::SubOperator;

use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub enum UnaryOpExpressionOperator {
    Not(NotOperator),
    Add(AddOperator),
    Sub(SubOperator),
    BinaryNot(BinaryNotOperator),
    Extra(ExtraChild),
}

impl UnaryOpExpressionOperator {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => UnaryOpExpressionOperator::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                UnaryOpExpressionOperator::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => UnaryOpExpressionOperator::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "!" => UnaryOpExpressionOperator::Not(NotOperator(node.range())),
            "+" => UnaryOpExpressionOperator::Add(AddOperator(node.range())),
            "-" => UnaryOpExpressionOperator::Sub(SubOperator(node.range())),
            "~" => UnaryOpExpressionOperator::BinaryNot(BinaryNotOperator(node.range())),

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
            "comment" => UnaryOpExpressionOperator::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                UnaryOpExpressionOperator::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => UnaryOpExpressionOperator::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "!" => UnaryOpExpressionOperator::Not(NotOperator(node.range())),
            "+" => UnaryOpExpressionOperator::Add(AddOperator(node.range())),
            "-" => UnaryOpExpressionOperator::Sub(SubOperator(node.range())),
            "~" => UnaryOpExpressionOperator::BinaryNot(BinaryNotOperator(node.range())),

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
    pub expr: Option<_ExpressionNode>,
    pub operator: Option<Box<UnaryOpExpressionOperator>>,
    pub child: Option<Box<IntegerNode>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl UnaryOpExpressionNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
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
        let mut skip_nodes: Vec<usize> = vec![];
        let expr: Option<_ExpressionNode> = node
            .children_by_field_name("expr", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode1| _ExpressionNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        let operator: Option<Box<UnaryOpExpressionOperator>> = node
            .children_by_field_name("operator", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode2| UnaryOpExpressionOperator::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .into();
        Ok(Self {
            range,
            expr,
            operator,
            child: node
                .named_children(&mut node.walk())
                .filter(|node| !skip_nodes.contains(&node.id()))
                .filter(|node| node.kind() != "comment")
                .map(|k| IntegerNode::parse(k, source))
                .collect::<Result<Vec<IntegerNode>, ParseError>>()?
                .drain(..)
                .map(|j| Box::new(j))
                .next(),
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
        "unary_op_expression"
    }
}

impl NodeAccess for UnaryOpExpressionNode {
    fn brief_desc(&self) -> String {
        "UnaryOpExpressionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::UnaryOpExpression(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.expr {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.operator {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.child {
            child_vec.push(x.as_any());
        }
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
