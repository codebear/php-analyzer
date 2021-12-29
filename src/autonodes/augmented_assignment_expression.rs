use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::cast_expression::CastExpressionNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::dynamic_variable_name::DynamicVariableNameNode;
use crate::autonodes::function_call_expression::FunctionCallExpressionNode;
use crate::autonodes::member_access_expression::MemberAccessExpressionNode;
use crate::autonodes::member_call_expression::MemberCallExpressionNode;
use crate::autonodes::nullsafe_member_access_expression::NullsafeMemberAccessExpressionNode;
use crate::autonodes::nullsafe_member_call_expression::NullsafeMemberCallExpressionNode;
use crate::autonodes::scoped_call_expression::ScopedCallExpressionNode;
use crate::autonodes::scoped_property_access_expression::ScopedPropertyAccessExpressionNode;
use crate::autonodes::subscript_expression::SubscriptExpressionNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::variable_name::VariableNameNode;
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
pub enum AugmentedAssignmentExpressionLeft {
    CastExpression(Box<CastExpressionNode>),
    DynamicVariableName(Box<DynamicVariableNameNode>),
    FunctionCallExpression(Box<FunctionCallExpressionNode>),
    MemberAccessExpression(Box<MemberAccessExpressionNode>),
    MemberCallExpression(Box<MemberCallExpressionNode>),
    NullsafeMemberAccessExpression(Box<NullsafeMemberAccessExpressionNode>),
    NullsafeMemberCallExpression(Box<NullsafeMemberCallExpressionNode>),
    ScopedCallExpression(Box<ScopedCallExpressionNode>),
    ScopedPropertyAccessExpression(Box<ScopedPropertyAccessExpressionNode>),
    SubscriptExpression(Box<SubscriptExpressionNode>),
    VariableName(Box<VariableNameNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl AugmentedAssignmentExpressionLeft {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => AugmentedAssignmentExpressionLeft::Comment(Box::new(CommentNode::parse(
                node, source,
            )?)),
            "text_interpolation" => AugmentedAssignmentExpressionLeft::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => {
                AugmentedAssignmentExpressionLeft::Error(Box::new(ErrorNode::parse(node, source)?))
            }
            "cast_expression" => AugmentedAssignmentExpressionLeft::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "dynamic_variable_name" => AugmentedAssignmentExpressionLeft::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "function_call_expression" => {
                AugmentedAssignmentExpressionLeft::FunctionCallExpression(Box::new(
                    FunctionCallExpressionNode::parse(node, source)?,
                ))
            }
            "member_access_expression" => {
                AugmentedAssignmentExpressionLeft::MemberAccessExpression(Box::new(
                    MemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "member_call_expression" => AugmentedAssignmentExpressionLeft::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "nullsafe_member_access_expression" => {
                AugmentedAssignmentExpressionLeft::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                AugmentedAssignmentExpressionLeft::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "scoped_call_expression" => AugmentedAssignmentExpressionLeft::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "scoped_property_access_expression" => {
                AugmentedAssignmentExpressionLeft::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "subscript_expression" => AugmentedAssignmentExpressionLeft::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => AugmentedAssignmentExpressionLeft::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

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
            "comment" => AugmentedAssignmentExpressionLeft::Comment(Box::new(CommentNode::parse(
                node, source,
            )?)),
            "text_interpolation" => AugmentedAssignmentExpressionLeft::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => {
                AugmentedAssignmentExpressionLeft::Error(Box::new(ErrorNode::parse(node, source)?))
            }
            "cast_expression" => AugmentedAssignmentExpressionLeft::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "dynamic_variable_name" => AugmentedAssignmentExpressionLeft::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "function_call_expression" => {
                AugmentedAssignmentExpressionLeft::FunctionCallExpression(Box::new(
                    FunctionCallExpressionNode::parse(node, source)?,
                ))
            }
            "member_access_expression" => {
                AugmentedAssignmentExpressionLeft::MemberAccessExpression(Box::new(
                    MemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "member_call_expression" => AugmentedAssignmentExpressionLeft::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "nullsafe_member_access_expression" => {
                AugmentedAssignmentExpressionLeft::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                AugmentedAssignmentExpressionLeft::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "scoped_call_expression" => AugmentedAssignmentExpressionLeft::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "scoped_property_access_expression" => {
                AugmentedAssignmentExpressionLeft::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "subscript_expression" => AugmentedAssignmentExpressionLeft::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => AugmentedAssignmentExpressionLeft::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

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
            AugmentedAssignmentExpressionLeft::Comment(x) => x.get_utype(state, emitter),
            AugmentedAssignmentExpressionLeft::TextInterpolation(x) => x.get_utype(state, emitter),
            AugmentedAssignmentExpressionLeft::Error(x) => x.get_utype(state, emitter),
            AugmentedAssignmentExpressionLeft::CastExpression(x) => x.get_utype(state, emitter),
            AugmentedAssignmentExpressionLeft::DynamicVariableName(x) => {
                x.get_utype(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::FunctionCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::MemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::MemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::NullsafeMemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::NullsafeMemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::ScopedCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::ScopedPropertyAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::SubscriptExpression(x) => {
                x.get_utype(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            AugmentedAssignmentExpressionLeft::Comment(x) => x.get_php_value(state, emitter),
            AugmentedAssignmentExpressionLeft::TextInterpolation(x) => {
                x.get_php_value(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::Error(x) => x.get_php_value(state, emitter),
            AugmentedAssignmentExpressionLeft::CastExpression(x) => x.get_php_value(state, emitter),
            AugmentedAssignmentExpressionLeft::DynamicVariableName(x) => {
                x.get_php_value(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::FunctionCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::MemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::MemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::NullsafeMemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::NullsafeMemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::ScopedCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::ScopedPropertyAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::SubscriptExpression(x) => {
                x.get_php_value(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            AugmentedAssignmentExpressionLeft::Comment(x) => x.read_from(state, emitter),
            AugmentedAssignmentExpressionLeft::TextInterpolation(x) => x.read_from(state, emitter),
            AugmentedAssignmentExpressionLeft::Error(x) => x.read_from(state, emitter),
            AugmentedAssignmentExpressionLeft::CastExpression(x) => x.read_from(state, emitter),
            AugmentedAssignmentExpressionLeft::DynamicVariableName(x) => {
                x.read_from(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::FunctionCallExpression(x) => {
                x.read_from(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::MemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::MemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::NullsafeMemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::NullsafeMemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::ScopedCallExpression(x) => {
                x.read_from(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::ScopedPropertyAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::SubscriptExpression(x) => {
                x.read_from(state, emitter)
            }
            AugmentedAssignmentExpressionLeft::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for AugmentedAssignmentExpressionLeft {
    fn brief_desc(&self) -> String {
        match self {
            AugmentedAssignmentExpressionLeft::Comment(x) => format!(
                "AugmentedAssignmentExpressionLeft::comment({})",
                x.brief_desc()
            ),
            AugmentedAssignmentExpressionLeft::TextInterpolation(x) => format!(
                "AugmentedAssignmentExpressionLeft::text_interpolation({})",
                x.brief_desc()
            ),
            AugmentedAssignmentExpressionLeft::Error(x) => format!(
                "AugmentedAssignmentExpressionLeft::ERROR({})",
                x.brief_desc()
            ),
            AugmentedAssignmentExpressionLeft::CastExpression(x) => format!(
                "AugmentedAssignmentExpressionLeft::cast_expression({})",
                x.brief_desc()
            ),
            AugmentedAssignmentExpressionLeft::DynamicVariableName(x) => format!(
                "AugmentedAssignmentExpressionLeft::dynamic_variable_name({})",
                x.brief_desc()
            ),
            AugmentedAssignmentExpressionLeft::FunctionCallExpression(x) => format!(
                "AugmentedAssignmentExpressionLeft::function_call_expression({})",
                x.brief_desc()
            ),
            AugmentedAssignmentExpressionLeft::MemberAccessExpression(x) => format!(
                "AugmentedAssignmentExpressionLeft::member_access_expression({})",
                x.brief_desc()
            ),
            AugmentedAssignmentExpressionLeft::MemberCallExpression(x) => format!(
                "AugmentedAssignmentExpressionLeft::member_call_expression({})",
                x.brief_desc()
            ),
            AugmentedAssignmentExpressionLeft::NullsafeMemberAccessExpression(x) => format!(
                "AugmentedAssignmentExpressionLeft::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            AugmentedAssignmentExpressionLeft::NullsafeMemberCallExpression(x) => format!(
                "AugmentedAssignmentExpressionLeft::nullsafe_member_call_expression({})",
                x.brief_desc()
            ),
            AugmentedAssignmentExpressionLeft::ScopedCallExpression(x) => format!(
                "AugmentedAssignmentExpressionLeft::scoped_call_expression({})",
                x.brief_desc()
            ),
            AugmentedAssignmentExpressionLeft::ScopedPropertyAccessExpression(x) => format!(
                "AugmentedAssignmentExpressionLeft::scoped_property_access_expression({})",
                x.brief_desc()
            ),
            AugmentedAssignmentExpressionLeft::SubscriptExpression(x) => format!(
                "AugmentedAssignmentExpressionLeft::subscript_expression({})",
                x.brief_desc()
            ),
            AugmentedAssignmentExpressionLeft::VariableName(x) => format!(
                "AugmentedAssignmentExpressionLeft::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            AugmentedAssignmentExpressionLeft::Comment(x) => x.as_any(),
            AugmentedAssignmentExpressionLeft::TextInterpolation(x) => x.as_any(),
            AugmentedAssignmentExpressionLeft::Error(x) => x.as_any(),
            AugmentedAssignmentExpressionLeft::CastExpression(x) => x.as_any(),
            AugmentedAssignmentExpressionLeft::DynamicVariableName(x) => x.as_any(),
            AugmentedAssignmentExpressionLeft::FunctionCallExpression(x) => x.as_any(),
            AugmentedAssignmentExpressionLeft::MemberAccessExpression(x) => x.as_any(),
            AugmentedAssignmentExpressionLeft::MemberCallExpression(x) => x.as_any(),
            AugmentedAssignmentExpressionLeft::NullsafeMemberAccessExpression(x) => x.as_any(),
            AugmentedAssignmentExpressionLeft::NullsafeMemberCallExpression(x) => x.as_any(),
            AugmentedAssignmentExpressionLeft::ScopedCallExpression(x) => x.as_any(),
            AugmentedAssignmentExpressionLeft::ScopedPropertyAccessExpression(x) => x.as_any(),
            AugmentedAssignmentExpressionLeft::SubscriptExpression(x) => x.as_any(),
            AugmentedAssignmentExpressionLeft::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            AugmentedAssignmentExpressionLeft::Comment(x) => x.children_any(),
            AugmentedAssignmentExpressionLeft::TextInterpolation(x) => x.children_any(),
            AugmentedAssignmentExpressionLeft::Error(x) => x.children_any(),
            AugmentedAssignmentExpressionLeft::CastExpression(x) => x.children_any(),
            AugmentedAssignmentExpressionLeft::DynamicVariableName(x) => x.children_any(),
            AugmentedAssignmentExpressionLeft::FunctionCallExpression(x) => x.children_any(),
            AugmentedAssignmentExpressionLeft::MemberAccessExpression(x) => x.children_any(),
            AugmentedAssignmentExpressionLeft::MemberCallExpression(x) => x.children_any(),
            AugmentedAssignmentExpressionLeft::NullsafeMemberAccessExpression(x) => {
                x.children_any()
            }
            AugmentedAssignmentExpressionLeft::NullsafeMemberCallExpression(x) => x.children_any(),
            AugmentedAssignmentExpressionLeft::ScopedCallExpression(x) => x.children_any(),
            AugmentedAssignmentExpressionLeft::ScopedPropertyAccessExpression(x) => {
                x.children_any()
            }
            AugmentedAssignmentExpressionLeft::SubscriptExpression(x) => x.children_any(),
            AugmentedAssignmentExpressionLeft::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            AugmentedAssignmentExpressionLeft::Comment(x) => x.range(),
            AugmentedAssignmentExpressionLeft::TextInterpolation(x) => x.range(),
            AugmentedAssignmentExpressionLeft::Error(x) => x.range(),
            AugmentedAssignmentExpressionLeft::CastExpression(x) => x.range(),
            AugmentedAssignmentExpressionLeft::DynamicVariableName(x) => x.range(),
            AugmentedAssignmentExpressionLeft::FunctionCallExpression(x) => x.range(),
            AugmentedAssignmentExpressionLeft::MemberAccessExpression(x) => x.range(),
            AugmentedAssignmentExpressionLeft::MemberCallExpression(x) => x.range(),
            AugmentedAssignmentExpressionLeft::NullsafeMemberAccessExpression(x) => x.range(),
            AugmentedAssignmentExpressionLeft::NullsafeMemberCallExpression(x) => x.range(),
            AugmentedAssignmentExpressionLeft::ScopedCallExpression(x) => x.range(),
            AugmentedAssignmentExpressionLeft::ScopedPropertyAccessExpression(x) => x.range(),
            AugmentedAssignmentExpressionLeft::SubscriptExpression(x) => x.range(),
            AugmentedAssignmentExpressionLeft::VariableName(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AugmentedAssignmentExpressionOperator {
    ModAssign(&'static str, Range),
    AndAssign(&'static str, Range),
    PowAssign(&'static str, Range),
    MultAssign(&'static str, Range),
    AddAssign(&'static str, Range),
    SubAssign(&'static str, Range),
    ConcatAssign(&'static str, Range),
    DivAssign(&'static str, Range),
    LeftShiftAssign(&'static str, Range),
    RightShiftAssign(&'static str, Range),
    NullsafeAssign(&'static str, Range),
    XorAssign(&'static str, Range),
    OrAssign(&'static str, Range),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl AugmentedAssignmentExpressionOperator {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => AugmentedAssignmentExpressionOperator::Comment(Box::new(
                CommentNode::parse(node, source)?,
            )),
            "text_interpolation" => AugmentedAssignmentExpressionOperator::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            ),
            "ERROR" => AugmentedAssignmentExpressionOperator::Error(Box::new(ErrorNode::parse(
                node, source,
            )?)),
            "%=" => AugmentedAssignmentExpressionOperator::ModAssign("%=", node.range()),
            "&=" => AugmentedAssignmentExpressionOperator::AndAssign("&=", node.range()),
            "**=" => AugmentedAssignmentExpressionOperator::PowAssign("**=", node.range()),
            "*=" => AugmentedAssignmentExpressionOperator::MultAssign("*=", node.range()),
            "+=" => AugmentedAssignmentExpressionOperator::AddAssign("+=", node.range()),
            "-=" => AugmentedAssignmentExpressionOperator::SubAssign("-=", node.range()),
            ".=" => AugmentedAssignmentExpressionOperator::ConcatAssign(".=", node.range()),
            "/=" => AugmentedAssignmentExpressionOperator::DivAssign("/=", node.range()),
            "<<=" => AugmentedAssignmentExpressionOperator::LeftShiftAssign("<<=", node.range()),
            ">>=" => AugmentedAssignmentExpressionOperator::RightShiftAssign(">>=", node.range()),
            "??=" => AugmentedAssignmentExpressionOperator::NullsafeAssign("??=", node.range()),
            "^=" => AugmentedAssignmentExpressionOperator::XorAssign("^=", node.range()),
            "|=" => AugmentedAssignmentExpressionOperator::OrAssign("|=", node.range()),

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
            "comment" => AugmentedAssignmentExpressionOperator::Comment(Box::new(
                CommentNode::parse(node, source)?,
            )),
            "text_interpolation" => AugmentedAssignmentExpressionOperator::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            ),
            "ERROR" => AugmentedAssignmentExpressionOperator::Error(Box::new(ErrorNode::parse(
                node, source,
            )?)),
            "%=" => AugmentedAssignmentExpressionOperator::ModAssign("%=", node.range()),
            "&=" => AugmentedAssignmentExpressionOperator::AndAssign("&=", node.range()),
            "**=" => AugmentedAssignmentExpressionOperator::PowAssign("**=", node.range()),
            "*=" => AugmentedAssignmentExpressionOperator::MultAssign("*=", node.range()),
            "+=" => AugmentedAssignmentExpressionOperator::AddAssign("+=", node.range()),
            "-=" => AugmentedAssignmentExpressionOperator::SubAssign("-=", node.range()),
            ".=" => AugmentedAssignmentExpressionOperator::ConcatAssign(".=", node.range()),
            "/=" => AugmentedAssignmentExpressionOperator::DivAssign("/=", node.range()),
            "<<=" => AugmentedAssignmentExpressionOperator::LeftShiftAssign("<<=", node.range()),
            ">>=" => AugmentedAssignmentExpressionOperator::RightShiftAssign(">>=", node.range()),
            "??=" => AugmentedAssignmentExpressionOperator::NullsafeAssign("??=", node.range()),
            "^=" => AugmentedAssignmentExpressionOperator::XorAssign("^=", node.range()),
            "|=" => AugmentedAssignmentExpressionOperator::OrAssign("|=", node.range()),

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
            AugmentedAssignmentExpressionOperator::Comment(x) => x.get_utype(state, emitter),
            AugmentedAssignmentExpressionOperator::TextInterpolation(x) => {
                x.get_utype(state, emitter)
            }
            AugmentedAssignmentExpressionOperator::Error(x) => x.get_utype(state, emitter),
            AugmentedAssignmentExpressionOperator::ModAssign(_, _) => {
                Some(DiscreteType::String.into())
            }
            AugmentedAssignmentExpressionOperator::AndAssign(_, _) => {
                Some(DiscreteType::String.into())
            }
            AugmentedAssignmentExpressionOperator::PowAssign(_, _) => {
                Some(DiscreteType::String.into())
            }
            AugmentedAssignmentExpressionOperator::MultAssign(_, _) => {
                Some(DiscreteType::String.into())
            }
            AugmentedAssignmentExpressionOperator::AddAssign(_, _) => {
                Some(DiscreteType::String.into())
            }
            AugmentedAssignmentExpressionOperator::SubAssign(_, _) => {
                Some(DiscreteType::String.into())
            }
            AugmentedAssignmentExpressionOperator::ConcatAssign(_, _) => {
                Some(DiscreteType::String.into())
            }
            AugmentedAssignmentExpressionOperator::DivAssign(_, _) => {
                Some(DiscreteType::String.into())
            }
            AugmentedAssignmentExpressionOperator::LeftShiftAssign(_, _) => {
                Some(DiscreteType::String.into())
            }
            AugmentedAssignmentExpressionOperator::RightShiftAssign(_, _) => {
                Some(DiscreteType::String.into())
            }
            AugmentedAssignmentExpressionOperator::NullsafeAssign(_, _) => {
                Some(DiscreteType::String.into())
            }
            AugmentedAssignmentExpressionOperator::XorAssign(_, _) => {
                Some(DiscreteType::String.into())
            }
            AugmentedAssignmentExpressionOperator::OrAssign(_, _) => {
                Some(DiscreteType::String.into())
            }
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            AugmentedAssignmentExpressionOperator::Comment(x) => x.get_php_value(state, emitter),
            AugmentedAssignmentExpressionOperator::TextInterpolation(x) => {
                x.get_php_value(state, emitter)
            }
            AugmentedAssignmentExpressionOperator::Error(x) => x.get_php_value(state, emitter),
            AugmentedAssignmentExpressionOperator::ModAssign(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            AugmentedAssignmentExpressionOperator::AndAssign(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            AugmentedAssignmentExpressionOperator::PowAssign(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            AugmentedAssignmentExpressionOperator::MultAssign(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            AugmentedAssignmentExpressionOperator::AddAssign(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            AugmentedAssignmentExpressionOperator::SubAssign(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            AugmentedAssignmentExpressionOperator::ConcatAssign(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            AugmentedAssignmentExpressionOperator::DivAssign(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            AugmentedAssignmentExpressionOperator::LeftShiftAssign(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            AugmentedAssignmentExpressionOperator::RightShiftAssign(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            AugmentedAssignmentExpressionOperator::NullsafeAssign(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            AugmentedAssignmentExpressionOperator::XorAssign(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            AugmentedAssignmentExpressionOperator::OrAssign(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            AugmentedAssignmentExpressionOperator::Comment(x) => x.read_from(state, emitter),
            AugmentedAssignmentExpressionOperator::TextInterpolation(x) => {
                x.read_from(state, emitter)
            }
            AugmentedAssignmentExpressionOperator::Error(x) => x.read_from(state, emitter),
            AugmentedAssignmentExpressionOperator::ModAssign(_, _) => (),
            AugmentedAssignmentExpressionOperator::AndAssign(_, _) => (),
            AugmentedAssignmentExpressionOperator::PowAssign(_, _) => (),
            AugmentedAssignmentExpressionOperator::MultAssign(_, _) => (),
            AugmentedAssignmentExpressionOperator::AddAssign(_, _) => (),
            AugmentedAssignmentExpressionOperator::SubAssign(_, _) => (),
            AugmentedAssignmentExpressionOperator::ConcatAssign(_, _) => (),
            AugmentedAssignmentExpressionOperator::DivAssign(_, _) => (),
            AugmentedAssignmentExpressionOperator::LeftShiftAssign(_, _) => (),
            AugmentedAssignmentExpressionOperator::RightShiftAssign(_, _) => (),
            AugmentedAssignmentExpressionOperator::NullsafeAssign(_, _) => (),
            AugmentedAssignmentExpressionOperator::XorAssign(_, _) => (),
            AugmentedAssignmentExpressionOperator::OrAssign(_, _) => (),
        }
    }
}

impl NodeAccess for AugmentedAssignmentExpressionOperator {
    fn brief_desc(&self) -> String {
        match self {
            AugmentedAssignmentExpressionOperator::Comment(x) => format!(
                "AugmentedAssignmentExpressionOperator::comment({})",
                x.brief_desc()
            ),
            AugmentedAssignmentExpressionOperator::TextInterpolation(x) => format!(
                "AugmentedAssignmentExpressionOperator::text_interpolation({})",
                x.brief_desc()
            ),
            AugmentedAssignmentExpressionOperator::Error(x) => format!(
                "AugmentedAssignmentExpressionOperator::ERROR({})",
                x.brief_desc()
            ),
            AugmentedAssignmentExpressionOperator::ModAssign(a, _) => a.to_string(),
            AugmentedAssignmentExpressionOperator::AndAssign(a, _) => a.to_string(),
            AugmentedAssignmentExpressionOperator::PowAssign(a, _) => a.to_string(),
            AugmentedAssignmentExpressionOperator::MultAssign(a, _) => a.to_string(),
            AugmentedAssignmentExpressionOperator::AddAssign(a, _) => a.to_string(),
            AugmentedAssignmentExpressionOperator::SubAssign(a, _) => a.to_string(),
            AugmentedAssignmentExpressionOperator::ConcatAssign(a, _) => a.to_string(),
            AugmentedAssignmentExpressionOperator::DivAssign(a, _) => a.to_string(),
            AugmentedAssignmentExpressionOperator::LeftShiftAssign(a, _) => a.to_string(),
            AugmentedAssignmentExpressionOperator::RightShiftAssign(a, _) => a.to_string(),
            AugmentedAssignmentExpressionOperator::NullsafeAssign(a, _) => a.to_string(),
            AugmentedAssignmentExpressionOperator::XorAssign(a, _) => a.to_string(),
            AugmentedAssignmentExpressionOperator::OrAssign(a, _) => a.to_string(),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            AugmentedAssignmentExpressionOperator::Comment(x) => x.as_any(),
            AugmentedAssignmentExpressionOperator::TextInterpolation(x) => x.as_any(),
            AugmentedAssignmentExpressionOperator::Error(x) => x.as_any(),
            AugmentedAssignmentExpressionOperator::ModAssign(a, b) => AnyNodeRef::StaticExpr(a, *b),
            AugmentedAssignmentExpressionOperator::AndAssign(a, b) => AnyNodeRef::StaticExpr(a, *b),
            AugmentedAssignmentExpressionOperator::PowAssign(a, b) => AnyNodeRef::StaticExpr(a, *b),
            AugmentedAssignmentExpressionOperator::MultAssign(a, b) => {
                AnyNodeRef::StaticExpr(a, *b)
            }
            AugmentedAssignmentExpressionOperator::AddAssign(a, b) => AnyNodeRef::StaticExpr(a, *b),
            AugmentedAssignmentExpressionOperator::SubAssign(a, b) => AnyNodeRef::StaticExpr(a, *b),
            AugmentedAssignmentExpressionOperator::ConcatAssign(a, b) => {
                AnyNodeRef::StaticExpr(a, *b)
            }
            AugmentedAssignmentExpressionOperator::DivAssign(a, b) => AnyNodeRef::StaticExpr(a, *b),
            AugmentedAssignmentExpressionOperator::LeftShiftAssign(a, b) => {
                AnyNodeRef::StaticExpr(a, *b)
            }
            AugmentedAssignmentExpressionOperator::RightShiftAssign(a, b) => {
                AnyNodeRef::StaticExpr(a, *b)
            }
            AugmentedAssignmentExpressionOperator::NullsafeAssign(a, b) => {
                AnyNodeRef::StaticExpr(a, *b)
            }
            AugmentedAssignmentExpressionOperator::XorAssign(a, b) => AnyNodeRef::StaticExpr(a, *b),
            AugmentedAssignmentExpressionOperator::OrAssign(a, b) => AnyNodeRef::StaticExpr(a, *b),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            AugmentedAssignmentExpressionOperator::Comment(x) => x.children_any(),
            AugmentedAssignmentExpressionOperator::TextInterpolation(x) => x.children_any(),
            AugmentedAssignmentExpressionOperator::Error(x) => x.children_any(),
            AugmentedAssignmentExpressionOperator::ModAssign(_, _) => todo!("Crap"),
            AugmentedAssignmentExpressionOperator::AndAssign(_, _) => todo!("Crap"),
            AugmentedAssignmentExpressionOperator::PowAssign(_, _) => todo!("Crap"),
            AugmentedAssignmentExpressionOperator::MultAssign(_, _) => todo!("Crap"),
            AugmentedAssignmentExpressionOperator::AddAssign(_, _) => todo!("Crap"),
            AugmentedAssignmentExpressionOperator::SubAssign(_, _) => todo!("Crap"),
            AugmentedAssignmentExpressionOperator::ConcatAssign(_, _) => todo!("Crap"),
            AugmentedAssignmentExpressionOperator::DivAssign(_, _) => todo!("Crap"),
            AugmentedAssignmentExpressionOperator::LeftShiftAssign(_, _) => todo!("Crap"),
            AugmentedAssignmentExpressionOperator::RightShiftAssign(_, _) => todo!("Crap"),
            AugmentedAssignmentExpressionOperator::NullsafeAssign(_, _) => todo!("Crap"),
            AugmentedAssignmentExpressionOperator::XorAssign(_, _) => todo!("Crap"),
            AugmentedAssignmentExpressionOperator::OrAssign(_, _) => todo!("Crap"),
        }
    }

    fn range(&self) -> Range {
        match self {
            AugmentedAssignmentExpressionOperator::Comment(x) => x.range(),
            AugmentedAssignmentExpressionOperator::TextInterpolation(x) => x.range(),
            AugmentedAssignmentExpressionOperator::Error(x) => x.range(),
            AugmentedAssignmentExpressionOperator::ModAssign(_, r) => *r,
            AugmentedAssignmentExpressionOperator::AndAssign(_, r) => *r,
            AugmentedAssignmentExpressionOperator::PowAssign(_, r) => *r,
            AugmentedAssignmentExpressionOperator::MultAssign(_, r) => *r,
            AugmentedAssignmentExpressionOperator::AddAssign(_, r) => *r,
            AugmentedAssignmentExpressionOperator::SubAssign(_, r) => *r,
            AugmentedAssignmentExpressionOperator::ConcatAssign(_, r) => *r,
            AugmentedAssignmentExpressionOperator::DivAssign(_, r) => *r,
            AugmentedAssignmentExpressionOperator::LeftShiftAssign(_, r) => *r,
            AugmentedAssignmentExpressionOperator::RightShiftAssign(_, r) => *r,
            AugmentedAssignmentExpressionOperator::NullsafeAssign(_, r) => *r,
            AugmentedAssignmentExpressionOperator::XorAssign(_, r) => *r,
            AugmentedAssignmentExpressionOperator::OrAssign(_, r) => *r,
        }
    }
}
#[derive(Debug, Clone)]
pub struct AugmentedAssignmentExpressionNode {
    pub range: Range,
    pub left: Box<AugmentedAssignmentExpressionLeft>,
    pub operator: Box<AugmentedAssignmentExpressionOperator>,
    pub right: _ExpressionNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl AugmentedAssignmentExpressionNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "augmented_assignment_expression" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [augmented_assignment_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let left: Box<AugmentedAssignmentExpressionLeft> = node
            .children_by_field_name("left", &mut node.walk())
            .map(|chnode2| AugmentedAssignmentExpressionLeft::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .expect("Field left should exist")
            .into();
        let operator: Box<AugmentedAssignmentExpressionOperator> = node
            .children_by_field_name("operator", &mut node.walk())
            .map(|chnode2| AugmentedAssignmentExpressionOperator::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .expect("Field operator should exist")
            .into();
        let right: _ExpressionNode = node
            .children_by_field_name("right", &mut node.walk())
            .map(|chnode1| _ExpressionNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field right should exist");
        Ok(Self {
            range,
            left,
            operator,
            right,
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
        "augmented_assignment_expression"
    }
}

impl NodeAccess for AugmentedAssignmentExpressionNode {
    fn brief_desc(&self) -> String {
        "AugmentedAssignmentExpressionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::AugmentedAssignmentExpression(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.left.as_any());
        child_vec.push(self.operator.as_any());
        child_vec.push(self.right.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
