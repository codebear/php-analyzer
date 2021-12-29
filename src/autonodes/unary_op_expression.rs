use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::integer::IntegerNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autotree::NodeAccess;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::types::union::DiscreteType;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use std::ffi::OsStr;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub enum UnaryOpExpressionOperator {
    Not(&'static str, Range),
    Add(&'static str, Range),
    Sub(&'static str, Range),
    BinaryNot(&'static str, Range),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl UnaryOpExpressionOperator {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => {
                UnaryOpExpressionOperator::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => UnaryOpExpressionOperator::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => UnaryOpExpressionOperator::Error(Box::new(ErrorNode::parse(node, source)?)),
            "!" => UnaryOpExpressionOperator::Not("!", node.range()),
            "+" => UnaryOpExpressionOperator::Add("+", node.range()),
            "-" => UnaryOpExpressionOperator::Sub("-", node.range()),
            "~" => UnaryOpExpressionOperator::BinaryNot("~", node.range()),

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
            "comment" => {
                UnaryOpExpressionOperator::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => UnaryOpExpressionOperator::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => UnaryOpExpressionOperator::Error(Box::new(ErrorNode::parse(node, source)?)),
            "!" => UnaryOpExpressionOperator::Not("!", node.range()),
            "+" => UnaryOpExpressionOperator::Add("+", node.range()),
            "-" => UnaryOpExpressionOperator::Sub("-", node.range()),
            "~" => UnaryOpExpressionOperator::BinaryNot("~", node.range()),

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
            UnaryOpExpressionOperator::Comment(x) => x.get_utype(state, emitter),
            UnaryOpExpressionOperator::TextInterpolation(x) => x.get_utype(state, emitter),
            UnaryOpExpressionOperator::Error(x) => x.get_utype(state, emitter),
            UnaryOpExpressionOperator::Not(_, _) => Some(DiscreteType::String.into()),
            UnaryOpExpressionOperator::Add(_, _) => Some(DiscreteType::String.into()),
            UnaryOpExpressionOperator::Sub(_, _) => Some(DiscreteType::String.into()),
            UnaryOpExpressionOperator::BinaryNot(_, _) => Some(DiscreteType::String.into()),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            UnaryOpExpressionOperator::Comment(x) => x.get_php_value(state, emitter),
            UnaryOpExpressionOperator::TextInterpolation(x) => x.get_php_value(state, emitter),
            UnaryOpExpressionOperator::Error(x) => x.get_php_value(state, emitter),
            UnaryOpExpressionOperator::Not(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            UnaryOpExpressionOperator::Add(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            UnaryOpExpressionOperator::Sub(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            UnaryOpExpressionOperator::BinaryNot(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            UnaryOpExpressionOperator::Comment(x) => x.read_from(state, emitter),
            UnaryOpExpressionOperator::TextInterpolation(x) => x.read_from(state, emitter),
            UnaryOpExpressionOperator::Error(x) => x.read_from(state, emitter),
            UnaryOpExpressionOperator::Not(_, _) => (),
            UnaryOpExpressionOperator::Add(_, _) => (),
            UnaryOpExpressionOperator::Sub(_, _) => (),
            UnaryOpExpressionOperator::BinaryNot(_, _) => (),
        }
    }
}

impl NodeAccess for UnaryOpExpressionOperator {
    fn brief_desc(&self) -> String {
        match self {
            UnaryOpExpressionOperator::Comment(x) => {
                format!("UnaryOpExpressionOperator::comment({})", x.brief_desc())
            }
            UnaryOpExpressionOperator::TextInterpolation(x) => format!(
                "UnaryOpExpressionOperator::text_interpolation({})",
                x.brief_desc()
            ),
            UnaryOpExpressionOperator::Error(x) => {
                format!("UnaryOpExpressionOperator::ERROR({})", x.brief_desc())
            }
            UnaryOpExpressionOperator::Not(a, _) => a.to_string(),
            UnaryOpExpressionOperator::Add(a, _) => a.to_string(),
            UnaryOpExpressionOperator::Sub(a, _) => a.to_string(),
            UnaryOpExpressionOperator::BinaryNot(a, _) => a.to_string(),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            UnaryOpExpressionOperator::Comment(x) => x.as_any(),
            UnaryOpExpressionOperator::TextInterpolation(x) => x.as_any(),
            UnaryOpExpressionOperator::Error(x) => x.as_any(),
            UnaryOpExpressionOperator::Not(a, b) => AnyNodeRef::StaticExpr(a, *b),
            UnaryOpExpressionOperator::Add(a, b) => AnyNodeRef::StaticExpr(a, *b),
            UnaryOpExpressionOperator::Sub(a, b) => AnyNodeRef::StaticExpr(a, *b),
            UnaryOpExpressionOperator::BinaryNot(a, b) => AnyNodeRef::StaticExpr(a, *b),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            UnaryOpExpressionOperator::Comment(x) => x.children_any(),
            UnaryOpExpressionOperator::TextInterpolation(x) => x.children_any(),
            UnaryOpExpressionOperator::Error(x) => x.children_any(),
            UnaryOpExpressionOperator::Not(_, _) => todo!("Crap"),
            UnaryOpExpressionOperator::Add(_, _) => todo!("Crap"),
            UnaryOpExpressionOperator::Sub(_, _) => todo!("Crap"),
            UnaryOpExpressionOperator::BinaryNot(_, _) => todo!("Crap"),
        }
    }

    fn range(&self) -> Range {
        match self {
            UnaryOpExpressionOperator::Comment(x) => x.range(),
            UnaryOpExpressionOperator::TextInterpolation(x) => x.range(),
            UnaryOpExpressionOperator::Error(x) => x.range(),
            UnaryOpExpressionOperator::Not(_, r) => *r,
            UnaryOpExpressionOperator::Add(_, r) => *r,
            UnaryOpExpressionOperator::Sub(_, r) => *r,
            UnaryOpExpressionOperator::BinaryNot(_, r) => *r,
        }
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
