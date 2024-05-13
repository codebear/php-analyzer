use crate::{
    analysis::state::AnalysisState, autonodes::else_clause::ElseClauseNode, issue::IssueEmitter,
    types::union::UnionType,
};

// use super::analysis::AnalyzeableRoundTwoNode;
impl ElseClauseNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {

        // crate::missing!("{}.read_from(..)", self.kind());
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        // crate::missing_none!("{}.get_php_value(..)", self.kind())
        None
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        // crate::missing_none!("{}.get_utype(..)", self.kind())
        None
    }
}

/*impl AnalyzeableRoundTwoNode for ElseClauseNode {
    fn analyze_round_two(&self, _state: &mut crate::analysis::AnalysisState, _emitter: &dyn crate::issue::IssueEmitter) {
        crate::missing!()
    }
}*/
