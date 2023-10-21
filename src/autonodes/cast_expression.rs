use crate::analysis::state::AnalysisState;
use crate::autonodes::_primary_expression::_PrimaryExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::cast_type::CastTypeNode;
use crate::autonodes::clone_expression::CloneExpressionNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::exponentiation_expression::ExponentiationExpressionNode;
use crate::autonodes::include_expression::IncludeExpressionNode;
use crate::autonodes::include_once_expression::IncludeOnceExpressionNode;
use crate::autonodes::silence_expression::SilenceExpressionNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::unary_op_expression::UnaryOpExpressionNode;
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
pub enum CastExpressionValue {
    _PrimaryExpression(Box<_PrimaryExpressionNode>),
    CloneExpression(Box<CloneExpressionNode>),
    ExponentiationExpression(Box<ExponentiationExpressionNode>),
    IncludeExpression(Box<IncludeExpressionNode>),
    IncludeOnceExpression(Box<IncludeOnceExpressionNode>),
    SilenceExpression(Box<SilenceExpressionNode>),
    UnaryOpExpression(Box<UnaryOpExpressionNode>),
    Extra(ExtraChild),
}

impl CastExpressionValue {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => CastExpressionValue::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => CastExpressionValue::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => CastExpressionValue::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "clone_expression" => CastExpressionValue::CloneExpression(Box::new(
                CloneExpressionNode::parse(node, source)?,
            )),
            "exponentiation_expression" => CastExpressionValue::ExponentiationExpression(Box::new(
                ExponentiationExpressionNode::parse(node, source)?,
            )),
            "include_expression" => CastExpressionValue::IncludeExpression(Box::new(
                IncludeExpressionNode::parse(node, source)?,
            )),
            "include_once_expression" => CastExpressionValue::IncludeOnceExpression(Box::new(
                IncludeOnceExpressionNode::parse(node, source)?,
            )),
            "silence_expression" => CastExpressionValue::SilenceExpression(Box::new(
                SilenceExpressionNode::parse(node, source)?,
            )),
            "unary_op_expression" => CastExpressionValue::UnaryOpExpression(Box::new(
                UnaryOpExpressionNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _PrimaryExpressionNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| CastExpressionValue::_PrimaryExpression(y))
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
            "comment" => CastExpressionValue::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => CastExpressionValue::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => CastExpressionValue::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "clone_expression" => CastExpressionValue::CloneExpression(Box::new(
                CloneExpressionNode::parse(node, source)?,
            )),
            "exponentiation_expression" => CastExpressionValue::ExponentiationExpression(Box::new(
                ExponentiationExpressionNode::parse(node, source)?,
            )),
            "include_expression" => CastExpressionValue::IncludeExpression(Box::new(
                IncludeExpressionNode::parse(node, source)?,
            )),
            "include_once_expression" => CastExpressionValue::IncludeOnceExpression(Box::new(
                IncludeOnceExpressionNode::parse(node, source)?,
            )),
            "silence_expression" => CastExpressionValue::SilenceExpression(Box::new(
                SilenceExpressionNode::parse(node, source)?,
            )),
            "unary_op_expression" => CastExpressionValue::UnaryOpExpression(Box::new(
                UnaryOpExpressionNode::parse(node, source)?,
            )),

            _ => {
                return Ok(
                    if let Some(x) = _PrimaryExpressionNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| CastExpressionValue::_PrimaryExpression(y))
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
            CastExpressionValue::Extra(y) => y.kind(),
            CastExpressionValue::_PrimaryExpression(y) => y.kind(),
            CastExpressionValue::CloneExpression(y) => y.kind(),
            CastExpressionValue::ExponentiationExpression(y) => y.kind(),
            CastExpressionValue::IncludeExpression(y) => y.kind(),
            CastExpressionValue::IncludeOnceExpression(y) => y.kind(),
            CastExpressionValue::SilenceExpression(y) => y.kind(),
            CastExpressionValue::UnaryOpExpression(y) => y.kind(),
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
            CastExpressionValue::Extra(x) => x.get_utype(state, emitter),
            CastExpressionValue::_PrimaryExpression(x) => x.get_utype(state, emitter),
            CastExpressionValue::CloneExpression(x) => x.get_utype(state, emitter),
            CastExpressionValue::ExponentiationExpression(x) => x.get_utype(state, emitter),
            CastExpressionValue::IncludeExpression(x) => x.get_utype(state, emitter),
            CastExpressionValue::IncludeOnceExpression(x) => x.get_utype(state, emitter),
            CastExpressionValue::SilenceExpression(x) => x.get_utype(state, emitter),
            CastExpressionValue::UnaryOpExpression(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            CastExpressionValue::Extra(x) => x.get_php_value(state, emitter),
            CastExpressionValue::_PrimaryExpression(x) => x.get_php_value(state, emitter),
            CastExpressionValue::CloneExpression(x) => x.get_php_value(state, emitter),
            CastExpressionValue::ExponentiationExpression(x) => x.get_php_value(state, emitter),
            CastExpressionValue::IncludeExpression(x) => x.get_php_value(state, emitter),
            CastExpressionValue::IncludeOnceExpression(x) => x.get_php_value(state, emitter),
            CastExpressionValue::SilenceExpression(x) => x.get_php_value(state, emitter),
            CastExpressionValue::UnaryOpExpression(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            CastExpressionValue::Extra(x) => x.read_from(state, emitter),
            CastExpressionValue::_PrimaryExpression(x) => x.read_from(state, emitter),
            CastExpressionValue::CloneExpression(x) => x.read_from(state, emitter),
            CastExpressionValue::ExponentiationExpression(x) => x.read_from(state, emitter),
            CastExpressionValue::IncludeExpression(x) => x.read_from(state, emitter),
            CastExpressionValue::IncludeOnceExpression(x) => x.read_from(state, emitter),
            CastExpressionValue::SilenceExpression(x) => x.read_from(state, emitter),
            CastExpressionValue::UnaryOpExpression(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for CastExpressionValue {
    fn brief_desc(&self) -> String {
        match self {
            CastExpressionValue::Extra(x) => {
                format!("CastExpressionValue::extra({})", x.brief_desc())
            }
            CastExpressionValue::_PrimaryExpression(x) => format!(
                "CastExpressionValue::_primary_expression({})",
                x.brief_desc()
            ),
            CastExpressionValue::CloneExpression(x) => {
                format!("CastExpressionValue::clone_expression({})", x.brief_desc())
            }
            CastExpressionValue::ExponentiationExpression(x) => format!(
                "CastExpressionValue::exponentiation_expression({})",
                x.brief_desc()
            ),
            CastExpressionValue::IncludeExpression(x) => format!(
                "CastExpressionValue::include_expression({})",
                x.brief_desc()
            ),
            CastExpressionValue::IncludeOnceExpression(x) => format!(
                "CastExpressionValue::include_once_expression({})",
                x.brief_desc()
            ),
            CastExpressionValue::SilenceExpression(x) => format!(
                "CastExpressionValue::silence_expression({})",
                x.brief_desc()
            ),
            CastExpressionValue::UnaryOpExpression(x) => format!(
                "CastExpressionValue::unary_op_expression({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            CastExpressionValue::Extra(x) => x.as_any(),
            CastExpressionValue::_PrimaryExpression(x) => x.as_any(),
            CastExpressionValue::CloneExpression(x) => x.as_any(),
            CastExpressionValue::ExponentiationExpression(x) => x.as_any(),
            CastExpressionValue::IncludeExpression(x) => x.as_any(),
            CastExpressionValue::IncludeOnceExpression(x) => x.as_any(),
            CastExpressionValue::SilenceExpression(x) => x.as_any(),
            CastExpressionValue::UnaryOpExpression(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            CastExpressionValue::Extra(x) => x.children_any(),
            CastExpressionValue::_PrimaryExpression(x) => x.children_any(),
            CastExpressionValue::CloneExpression(x) => x.children_any(),
            CastExpressionValue::ExponentiationExpression(x) => x.children_any(),
            CastExpressionValue::IncludeExpression(x) => x.children_any(),
            CastExpressionValue::IncludeOnceExpression(x) => x.children_any(),
            CastExpressionValue::SilenceExpression(x) => x.children_any(),
            CastExpressionValue::UnaryOpExpression(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            CastExpressionValue::Extra(x) => x.range(),
            CastExpressionValue::_PrimaryExpression(x) => x.range(),
            CastExpressionValue::CloneExpression(x) => x.range(),
            CastExpressionValue::ExponentiationExpression(x) => x.range(),
            CastExpressionValue::IncludeExpression(x) => x.range(),
            CastExpressionValue::IncludeOnceExpression(x) => x.range(),
            CastExpressionValue::SilenceExpression(x) => x.range(),
            CastExpressionValue::UnaryOpExpression(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct CastExpressionNode {
    pub range: Range,
    pub type_: CastTypeNode,
    pub value: Box<CastExpressionValue>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl CastExpressionNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "cast_expression" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [cast_expression] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let type_: CastTypeNode = node
            .children_by_field_name("type", &mut node.walk())
            .map(|chnode1| CastTypeNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field type should exist");
        let value: Box<CastExpressionValue> = node
            .children_by_field_name("value", &mut node.walk())
            .map(|chnode2| CastExpressionValue::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .expect("Field value should exist")
            .into();
        Ok(Self {
            range,
            type_,
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
        "cast_expression"
    }
}

impl NodeAccess for CastExpressionNode {
    fn brief_desc(&self) -> String {
        "CastExpressionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::CastExpression(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.type_.as_any());
        child_vec.push(self.value.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
