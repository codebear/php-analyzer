use crate::{
    analysis::state::AnalysisState, autonodes::_primary_expression::_PrimaryExpressionNode,
    issue::IssueEmitter, types::union::PHPType, value::PHPValue,
};

impl _PrimaryExpressionNode {
    pub fn write_to(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        val_type: Option<PHPType>,
        value: Option<PHPValue>,
    ) {
        match self {
            _PrimaryExpressionNode::_Literal(_) => crate::missing!("_Literal.write_to(..)"),
            _PrimaryExpressionNode::AnonymousFunctionCreationExpression(_) => {
                crate::missing!("AnonymousFunctionCreationExpression.write_to(..)")
            }
            _PrimaryExpressionNode::ArrayCreationExpression(_) => {
                crate::missing!("ArrayCreateExpression.write_to(..)")
            }
            _PrimaryExpressionNode::ArrowFunction(_) => crate::missing!("Arrow.write_to(..)"),
            _PrimaryExpressionNode::CastExpression(_) => crate::missing!("Cast.write_to(..)"),
            _PrimaryExpressionNode::ClassConstantAccessExpression(_) => {
                crate::missing!("ClassConstantAccessExpression.write_to(..)")
            }
            _PrimaryExpressionNode::DynamicVariableName(_) => {
                crate::missing!("DynamicVariableName.write_to(..)")
            }
            _PrimaryExpressionNode::FunctionCallExpression(_) => {
                crate::missing!("FunctionCallExpression.write_to(..)")
            }
            _PrimaryExpressionNode::MemberAccessExpression(_) => {
                crate::missing!("MemberAccessExpression.write_to(..)")
            }
            _PrimaryExpressionNode::MemberCallExpression(_) => {
                crate::missing!("MemberCallExpression.write_to(..)")
            }
            _PrimaryExpressionNode::Name(_) => crate::missing!("Name.write_to(..)"),
            _PrimaryExpressionNode::NullsafeMemberAccessExpression(_) => {
                crate::missing!("NullsafeMemberAccessExpression.write_to(..)")
            }
            _PrimaryExpressionNode::NullsafeMemberCallExpression(_) => {
                crate::missing!("NullsafeMemberCallExpression.write_to(..)")
            }
            _PrimaryExpressionNode::ObjectCreationExpression(_) => {
                crate::missing!("ObjectCreationExpression.write_to(..)")
            }
            _PrimaryExpressionNode::ParenthesizedExpression(_) => {
                crate::missing!("ParenthesizedExpression.write_to(..)")
            }
            _PrimaryExpressionNode::PrintIntrinsic(_) => {
                crate::missing!("PrintIntrinsic.write_to(..)")
            }
            _PrimaryExpressionNode::QualifiedName(_) => {
                crate::missing!("QualifiedName.write_to(..)")
            }
            _PrimaryExpressionNode::ScopedCallExpression(_) => {
                crate::missing!("ScopedCallExpression.write_to(..)")
            }
            _PrimaryExpressionNode::ScopedPropertyAccessExpression(_) => {
                crate::missing!("ScopedPropertyAccessExpression.write_to(..)")
            }
            _PrimaryExpressionNode::ShellCommandExpression(_) => {
                crate::missing!("ShellCommandExpression.write_to(..)")
            }
            _PrimaryExpressionNode::SubscriptExpression(_) => {
                crate::missing!("SubscriptExpression.write_to(..)")
            }
            _PrimaryExpressionNode::ThrowExpression(_) => {
                crate::missing!("ThrowExpression.write_to(..)")
            }
            _PrimaryExpressionNode::UpdateExpression(_) => {
                crate::missing!("UpdateExpression.write_to(..)")
            }
            _PrimaryExpressionNode::VariableName(vn) => {
                vn.write_to(state, emitter, val_type, value)
            }

            _PrimaryExpressionNode::Extra(_) => (),
        }
    }
}
