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
use crate::autonodes::variable_name::VariableNameNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::operators::add_assign::AddAssignOperator;
use crate::operators::and_assign::AndAssignOperator;
use crate::operators::concat_assign::ConcatAssignOperator;
use crate::operators::div_assign::DivAssignOperator;
use crate::operators::left_shift_assign::LeftShiftAssignOperator;
use crate::operators::mod_assign::ModAssignOperator;
use crate::operators::mult_assign::MultAssignOperator;
use crate::operators::nullsafe_assign::NullsafeAssignOperator;
use crate::operators::operator::Operator;
use crate::operators::or_assign::OrAssignOperator;
use crate::operators::pow_assign::PowAssignOperator;
use crate::operators::right_shift_assign::RightShiftAssignOperator;
use crate::operators::sub_assign::SubAssignOperator;
use crate::operators::xor_assign::XorAssignOperator;
use crate::parser::Range;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use tree_sitter::Node;

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
    Extra(ExtraChild),
}

impl NodeParser for AugmentedAssignmentExpressionLeft {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => AugmentedAssignmentExpressionLeft::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => AugmentedAssignmentExpressionLeft::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
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
}

impl AugmentedAssignmentExpressionLeft {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => AugmentedAssignmentExpressionLeft::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => AugmentedAssignmentExpressionLeft::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
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
        match self {
            AugmentedAssignmentExpressionLeft::Extra(y) => y.kind(),
            AugmentedAssignmentExpressionLeft::CastExpression(y) => y.kind(),
            AugmentedAssignmentExpressionLeft::DynamicVariableName(y) => y.kind(),
            AugmentedAssignmentExpressionLeft::FunctionCallExpression(y) => y.kind(),
            AugmentedAssignmentExpressionLeft::MemberAccessExpression(y) => y.kind(),
            AugmentedAssignmentExpressionLeft::MemberCallExpression(y) => y.kind(),
            AugmentedAssignmentExpressionLeft::NullsafeMemberAccessExpression(y) => y.kind(),
            AugmentedAssignmentExpressionLeft::NullsafeMemberCallExpression(y) => y.kind(),
            AugmentedAssignmentExpressionLeft::ScopedCallExpression(y) => y.kind(),
            AugmentedAssignmentExpressionLeft::ScopedPropertyAccessExpression(y) => y.kind(),
            AugmentedAssignmentExpressionLeft::SubscriptExpression(y) => y.kind(),
            AugmentedAssignmentExpressionLeft::VariableName(y) => y.kind(),
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
            AugmentedAssignmentExpressionLeft::Extra(x) => x.get_utype(state, emitter),
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
            AugmentedAssignmentExpressionLeft::Extra(x) => x.get_php_value(state, emitter),
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
            AugmentedAssignmentExpressionLeft::Extra(x) => x.read_from(state, emitter),
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
            AugmentedAssignmentExpressionLeft::Extra(x) => format!(
                "AugmentedAssignmentExpressionLeft::extra({})",
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

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            AugmentedAssignmentExpressionLeft::Extra(x) => x.as_any(),
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

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            AugmentedAssignmentExpressionLeft::Extra(x) => x.children_any(),
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
            AugmentedAssignmentExpressionLeft::Extra(x) => x.range(),
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
    ModAssign(ModAssignOperator),
    AndAssign(AndAssignOperator),
    PowAssign(PowAssignOperator),
    MultAssign(MultAssignOperator),
    AddAssign(AddAssignOperator),
    SubAssign(SubAssignOperator),
    ConcatAssign(ConcatAssignOperator),
    DivAssign(DivAssignOperator),
    LeftShiftAssign(LeftShiftAssignOperator),
    RightShiftAssign(RightShiftAssignOperator),
    NullsafeAssign(NullsafeAssignOperator),
    XorAssign(XorAssignOperator),
    OrAssign(OrAssignOperator),
    Extra(ExtraChild),
}

impl NodeParser for AugmentedAssignmentExpressionOperator {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => AugmentedAssignmentExpressionOperator::Extra(ExtraChild::Comment(
                Box::new(CommentNode::parse(node, source)?),
            )),
            "ERROR" => AugmentedAssignmentExpressionOperator::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "%=" => AugmentedAssignmentExpressionOperator::ModAssign(ModAssignOperator(
                node.range().into(),
            )),
            "&=" => AugmentedAssignmentExpressionOperator::AndAssign(AndAssignOperator(
                node.range().into(),
            )),
            "**=" => AugmentedAssignmentExpressionOperator::PowAssign(PowAssignOperator(
                node.range().into(),
            )),
            "*=" => AugmentedAssignmentExpressionOperator::MultAssign(MultAssignOperator(
                node.range().into(),
            )),
            "+=" => AugmentedAssignmentExpressionOperator::AddAssign(AddAssignOperator(
                node.range().into(),
            )),
            "-=" => AugmentedAssignmentExpressionOperator::SubAssign(SubAssignOperator(
                node.range().into(),
            )),
            ".=" => AugmentedAssignmentExpressionOperator::ConcatAssign(ConcatAssignOperator(
                node.range().into(),
            )),
            "/=" => AugmentedAssignmentExpressionOperator::DivAssign(DivAssignOperator(
                node.range().into(),
            )),
            "<<=" => AugmentedAssignmentExpressionOperator::LeftShiftAssign(
                LeftShiftAssignOperator(node.range().into()),
            ),
            ">>=" => AugmentedAssignmentExpressionOperator::RightShiftAssign(
                RightShiftAssignOperator(node.range().into()),
            ),
            "??=" => AugmentedAssignmentExpressionOperator::NullsafeAssign(NullsafeAssignOperator(
                node.range().into(),
            )),
            "^=" => AugmentedAssignmentExpressionOperator::XorAssign(XorAssignOperator(
                node.range().into(),
            )),
            "|=" => AugmentedAssignmentExpressionOperator::OrAssign(OrAssignOperator(
                node.range().into(),
            )),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }
}

impl AugmentedAssignmentExpressionOperator {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => AugmentedAssignmentExpressionOperator::Extra(ExtraChild::Comment(
                Box::new(CommentNode::parse(node, source)?),
            )),
            "ERROR" => AugmentedAssignmentExpressionOperator::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "%=" => AugmentedAssignmentExpressionOperator::ModAssign(ModAssignOperator(
                node.range().into(),
            )),
            "&=" => AugmentedAssignmentExpressionOperator::AndAssign(AndAssignOperator(
                node.range().into(),
            )),
            "**=" => AugmentedAssignmentExpressionOperator::PowAssign(PowAssignOperator(
                node.range().into(),
            )),
            "*=" => AugmentedAssignmentExpressionOperator::MultAssign(MultAssignOperator(
                node.range().into(),
            )),
            "+=" => AugmentedAssignmentExpressionOperator::AddAssign(AddAssignOperator(
                node.range().into(),
            )),
            "-=" => AugmentedAssignmentExpressionOperator::SubAssign(SubAssignOperator(
                node.range().into(),
            )),
            ".=" => AugmentedAssignmentExpressionOperator::ConcatAssign(ConcatAssignOperator(
                node.range().into(),
            )),
            "/=" => AugmentedAssignmentExpressionOperator::DivAssign(DivAssignOperator(
                node.range().into(),
            )),
            "<<=" => AugmentedAssignmentExpressionOperator::LeftShiftAssign(
                LeftShiftAssignOperator(node.range().into()),
            ),
            ">>=" => AugmentedAssignmentExpressionOperator::RightShiftAssign(
                RightShiftAssignOperator(node.range().into()),
            ),
            "??=" => AugmentedAssignmentExpressionOperator::NullsafeAssign(NullsafeAssignOperator(
                node.range().into(),
            )),
            "^=" => AugmentedAssignmentExpressionOperator::XorAssign(XorAssignOperator(
                node.range().into(),
            )),
            "|=" => AugmentedAssignmentExpressionOperator::OrAssign(OrAssignOperator(
                node.range().into(),
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            AugmentedAssignmentExpressionOperator::Extra(y) => y.kind(),
            AugmentedAssignmentExpressionOperator::ModAssign(y) => y.kind(),
            AugmentedAssignmentExpressionOperator::AndAssign(y) => y.kind(),
            AugmentedAssignmentExpressionOperator::PowAssign(y) => y.kind(),
            AugmentedAssignmentExpressionOperator::MultAssign(y) => y.kind(),
            AugmentedAssignmentExpressionOperator::AddAssign(y) => y.kind(),
            AugmentedAssignmentExpressionOperator::SubAssign(y) => y.kind(),
            AugmentedAssignmentExpressionOperator::ConcatAssign(y) => y.kind(),
            AugmentedAssignmentExpressionOperator::DivAssign(y) => y.kind(),
            AugmentedAssignmentExpressionOperator::LeftShiftAssign(y) => y.kind(),
            AugmentedAssignmentExpressionOperator::RightShiftAssign(y) => y.kind(),
            AugmentedAssignmentExpressionOperator::NullsafeAssign(y) => y.kind(),
            AugmentedAssignmentExpressionOperator::XorAssign(y) => y.kind(),
            AugmentedAssignmentExpressionOperator::OrAssign(y) => y.kind(),
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
pub struct AugmentedAssignmentExpressionNode {
    pub range: Range,
    pub left: Box<AugmentedAssignmentExpressionLeft>,
    pub operator: Box<AugmentedAssignmentExpressionOperator>,
    pub right: _ExpressionNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for AugmentedAssignmentExpressionNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "augmented_assignment_expression" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [augmented_assignment_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let left: Box<AugmentedAssignmentExpressionLeft> =
            Into::<Result<_, _>>::into(node.parse_child("left", source))?;
        let operator: Box<AugmentedAssignmentExpressionOperator> =
            Into::<Result<_, _>>::into(node.parse_child("operator", source))?;
        let right: _ExpressionNode = Into::<Result<_, _>>::into(node.parse_child("right", source))?;
        Ok(Self {
            range,
            left,
            operator,
            right,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl AugmentedAssignmentExpressionNode {
    pub fn kind(&self) -> &'static str {
        "augmented_assignment_expression"
    }
}

impl NodeAccess for AugmentedAssignmentExpressionNode {
    fn brief_desc(&self) -> String {
        "AugmentedAssignmentExpressionNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::AugmentedAssignmentExpression(self)
    }

    #[allow(clippy::vec_init_then_push)]
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
