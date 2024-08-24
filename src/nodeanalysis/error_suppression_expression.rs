use crate::{
    analysis::state::AnalysisState,
    autonodes::error_suppression_expression::ErrorSuppressionExpressionNode, issue::IssueEmitter,
    types::union::PHPType,
};

impl ErrorSuppressionExpressionNode {
    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        self.child.read_from(state, emitter)
        //crate::missing!("{}.read_from(..)", self.kind());
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        self.child.get_php_value(state, emitter)
        //crate::missing_none!("{}.get_php_value(..)", self.kind())
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPType> {
        self.child.get_utype(state, emitter)
        //crate::missing_none!("{}.get_utype(..)", self.kind())
    }
}
