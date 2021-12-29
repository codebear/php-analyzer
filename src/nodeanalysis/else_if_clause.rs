use crate::{
    analysis::state::AnalysisState, autonodes::else_if_clause::ElseIfClauseNode,
    issue::IssueEmitter, types::union::UnionType,
};

// use super::analysis::AnalyzeableRoundTwoNode;
impl ElseIfClauseNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        crate::missing!("{}.read_from(..)", self.kind());
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
/*
impl AnalyzeableRoundTwoNode for ElseIfClauseNode {
    fn analyze_round_two(&self, _state: &mut crate::analysis::AnalysisState, _emitter: &dyn crate::issue::IssueEmitter) {
        todo!()
    }
}*/
