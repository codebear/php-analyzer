use crate::{
    analysis::state::AnalysisState, autonodes::by_ref::ByRefNode, issue::IssueEmitter,
    types::union::PHPType, value::PHPValue,
};

impl ByRefNode {
    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        self.child.read_from(state, emitter)
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        self.child.get_php_value(state, emitter)
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPType> {
        self.child.get_utype(state, emitter)
    }

    pub fn write_to(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn IssueEmitter,
        val_type: Option<PHPType>,
        value: Option<PHPValue>,
    ) {
        match &*self.child {
            crate::autonodes::by_ref::ByRefChildren::DynamicVariableName(_) => {
                crate::missing!("byref write_to(..)")
            }
            crate::autonodes::by_ref::ByRefChildren::FunctionCallExpression(_) => {
                crate::missing!("byref write_to(..)")
            }
            crate::autonodes::by_ref::ByRefChildren::MemberAccessExpression(_) => {
                crate::missing!("byref write_to(..)")
            }
            crate::autonodes::by_ref::ByRefChildren::MemberCallExpression(_) => {
                crate::missing!("byref write_to(..)")
            }
            crate::autonodes::by_ref::ByRefChildren::NullsafeMemberAccessExpression(_) => {
                crate::missing!("byref write_to(..)")
            }
            crate::autonodes::by_ref::ByRefChildren::NullsafeMemberCallExpression(_) => {
                crate::missing!("byref write_to(..)")
            }
            crate::autonodes::by_ref::ByRefChildren::ScopedCallExpression(_) => {
                crate::missing!("byref write_to(..)")
            }
            crate::autonodes::by_ref::ByRefChildren::SubscriptExpression(se) => {
                se.write_to(state, emitter, val_type, value)
            }
            crate::autonodes::by_ref::ByRefChildren::VariableName(vn) => {
                vn.write_to(state, emitter, val_type, value)
            }

            crate::autonodes::by_ref::ByRefChildren::Extra(_) => (),
        }
    }
}
