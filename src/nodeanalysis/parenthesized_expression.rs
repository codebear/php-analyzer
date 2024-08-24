use crate::{
    analysis::state::AnalysisState,
    autonodes::parenthesized_expression::ParenthesizedExpressionNode, issue::IssueEmitter,
    types::union::PHPType,
};

impl ParenthesizedExpressionNode {
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
}
