use crate::{
    analysis::state::AnalysisState, autonodes::silence_expression::SilenceExpressionNode,
    issue::IssueEmitter, types::union::UnionType,
};

impl SilenceExpressionNode {
    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        self.expr.read_from(state, emitter)
        //crate::missing!("{}.read_from(..)", self.kind());
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        self.expr.get_php_value(state, emitter)
        //crate::missing_none!("{}.get_php_value(..)", self.kind())
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        self.expr.get_utype(state, emitter)
        //crate::missing_none!("{}.get_utype(..)", self.kind())
    }
}
