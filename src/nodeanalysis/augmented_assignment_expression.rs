use crate::{
    analysis::state::AnalysisState,
    autonodes::{
        any::AnyNodeRef,
        augmented_assignment_expression::{
            AugmentedAssignmentExpressionLeft, AugmentedAssignmentExpressionNode,
            AugmentedAssignmentExpressionOperator,
        },
    },
    issue::IssueEmitter,
    types::union::{DiscreteType, UnionType},
    value::PHPValue,
};

use super::analysis::ThirdPassAnalyzeableNode;
use crate::autotree::NodeAccess;

impl AugmentedAssignmentExpressionNode {
    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        self.left.read_from(state, emitter);
        self.right.read_from(state, emitter);
        /*         match &*self.operator {
            AugmentedAssignmentExpressionOperator::ModAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::AndAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::PowAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::MultAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::AddAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::SubAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::ConcatAssign(_, _) => self.right.read_from(state, emitter),
            AugmentedAssignmentExpressionOperator::DivAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::LeftShiftAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::RightShiftAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::NullsafeAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::XorAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::OrAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),

            AugmentedAssignmentExpressionOperator::Comment(_) |
            AugmentedAssignmentExpressionOperator::TextInterpolation(_) |
            AugmentedAssignmentExpressionOperator::Error(_) => (),
        }*/
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        crate::missing_none!("{}.get_php_value(..)", self.kind())
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        crate::missing_none!("{}.get_utype(..)", self.kind())
    }
}

// FIXME den her må sjekk at right-operanden har rett type

impl ThirdPassAnalyzeableNode for AugmentedAssignmentExpressionNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        self.right.read_from(state, emitter);
        //let right_val = self.right.get_php_value(state, emitter);
        //let left_val = self.left.get_php_value(state, emitter);
        let (val_type, value) = match *self.operator {
            AugmentedAssignmentExpressionOperator::ModAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),
            AugmentedAssignmentExpressionOperator::AndAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),
            AugmentedAssignmentExpressionOperator::PowAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),
            AugmentedAssignmentExpressionOperator::MultAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),
            AugmentedAssignmentExpressionOperator::AddAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),
            AugmentedAssignmentExpressionOperator::SubAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),
            AugmentedAssignmentExpressionOperator::ConcatAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),
            AugmentedAssignmentExpressionOperator::DivAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),

            AugmentedAssignmentExpressionOperator::LeftShiftAssign(_, _) => (
                Some(DiscreteType::Int.into()),
                crate::missing_none!("{}.analyze_round_two(..)", self.kind()),
            ),
            AugmentedAssignmentExpressionOperator::RightShiftAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),
            AugmentedAssignmentExpressionOperator::NullsafeAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),
            AugmentedAssignmentExpressionOperator::XorAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),
            AugmentedAssignmentExpressionOperator::OrAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),

            AugmentedAssignmentExpressionOperator::Comment(_)
            | AugmentedAssignmentExpressionOperator::TextInterpolation(_)
            | AugmentedAssignmentExpressionOperator::Error(_) => (None, None),
        };
        // let val_type = self.right.get_utype(state, emitter);
        self.left.write_to(state, emitter, val_type, value);
        self.analyze_third_pass_children(&self.as_any(), state, emitter, path)
    }
}

impl AugmentedAssignmentExpressionLeft {
    pub fn write_to(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn IssueEmitter,
        val_type: Option<UnionType>,
        value: Option<PHPValue>,
    ) {
        match self {
            AugmentedAssignmentExpressionLeft::CastExpression(_) => {
                crate::missing!("{}.write_to(..)", self.brief_desc())
            }
            AugmentedAssignmentExpressionLeft::DynamicVariableName(_) => {
                crate::missing!("{}.write_to(..)", self.brief_desc())
            }
            AugmentedAssignmentExpressionLeft::FunctionCallExpression(_) => {
                crate::missing!("{}.write_to(..)", self.brief_desc())
            }
            AugmentedAssignmentExpressionLeft::MemberAccessExpression(ma) => {
                ma.write_to(state, emitter, val_type, value)
            }
            AugmentedAssignmentExpressionLeft::MemberCallExpression(_) => {
                crate::missing!("{}.write_to(..)", self.brief_desc())
            }
            AugmentedAssignmentExpressionLeft::NullsafeMemberAccessExpression(_) => {
                crate::missing!("{}.write_to(..)", self.brief_desc())
            }
            AugmentedAssignmentExpressionLeft::NullsafeMemberCallExpression(_) => {
                crate::missing!("{}.write_to(..)", self.brief_desc())
            }
            AugmentedAssignmentExpressionLeft::ScopedCallExpression(_) => {
                crate::missing!("{}.write_to(..)", self.brief_desc())
            }
            AugmentedAssignmentExpressionLeft::ScopedPropertyAccessExpression(sp) => {
                sp.write_to(state, emitter, val_type, value)
            }
            AugmentedAssignmentExpressionLeft::SubscriptExpression(se) => {
                se.write_to(state, emitter, val_type, value)
            }
            AugmentedAssignmentExpressionLeft::VariableName(vn) => {
                vn.write_to(state, emitter, val_type, value)
            }

            AugmentedAssignmentExpressionLeft::Comment(_)
            | AugmentedAssignmentExpressionLeft::TextInterpolation(_)
            | AugmentedAssignmentExpressionLeft::Error(_) => (),
        }
    }
}
