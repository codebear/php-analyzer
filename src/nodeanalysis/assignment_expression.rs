use crate::{
    analysis::state::AnalysisState,
    autonodes::{
        any::AnyNodeRef,
        assignment_expression::{AssignmentExpressionLeft, AssignmentExpressionNode},
    },
    issue::IssueEmitter,
    types::union::UnionType,
    value::PHPValue,
};

use super::analysis::ThirdPassAnalyzeableNode;

// use crate::nodeanalysis::lang::GetValueFromNode;
use crate::autotree::NodeAccess;
impl AssignmentExpressionNode {
    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        self.right.read_from(state, emitter);
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        self.right.get_php_value(state, emitter)
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        self.right.get_utype(state, emitter)
    }
}
/*
impl GetValueFromNode for AssignmentExpressionNode {

}*/

impl AssignmentExpressionLeft {
    pub fn write_to(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn IssueEmitter,
        val_type: Option<UnionType>,
        value: Option<PHPValue>,
    ) {
        match self {
            AssignmentExpressionLeft::CastExpression(_) => crate::missing!(),
            AssignmentExpressionLeft::DynamicVariableName(_) => crate::missing!(),
            AssignmentExpressionLeft::FunctionCallExpression(_) => crate::missing!(),
            AssignmentExpressionLeft::ListLiteral(ll) => {
                ll.write_to(state, emitter, val_type, value)
            }
            AssignmentExpressionLeft::MemberAccessExpression(man) => {
                man.write_to(state, emitter, val_type, value)
            }
            AssignmentExpressionLeft::MemberCallExpression(_) => crate::missing!(),
            AssignmentExpressionLeft::NullsafeMemberAccessExpression(_) => crate::missing!(),
            AssignmentExpressionLeft::NullsafeMemberCallExpression(_) => crate::missing!(),
            AssignmentExpressionLeft::ScopedCallExpression(_) => crate::missing!(),
            AssignmentExpressionLeft::ScopedPropertyAccessExpression(sp) => {
                sp.write_to(state, emitter, val_type, value)
            }
            AssignmentExpressionLeft::SubscriptExpression(se) => {
                se.write_to(state, emitter, val_type, value)
            }
            AssignmentExpressionLeft::VariableName(vn) => {
                vn.write_to(state, emitter, val_type, value)
            }

            AssignmentExpressionLeft::Comment(_)
            | AssignmentExpressionLeft::TextInterpolation(_)
            | AssignmentExpressionLeft::Error(_) => (),
        }
    }
}

impl ThirdPassAnalyzeableNode for AssignmentExpressionNode {
    fn analyze_third_pass(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn crate::issue::IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        if !self.analyze_third_pass_children(&self.as_any(), state, emitter, path) {
            return false;
        }
        let right_type = self.right.get_utype(state, emitter);
        let right_val = self.right.get_php_value(state, emitter);
        self.left.write_to(state, emitter, right_type, right_val);

        // Make sure we tag any rval as read_from
        self.right.read_from(state, emitter);

        true
    }
}
