use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, switch_statement::SwitchStatementNode},
    issue::IssueEmitter,
    types::union::UnionType,
};

use super::analysis::AnalyzeableRoundTwoNode;

use crate::autotree::NodeAccess;

impl SwitchStatementNode {
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
impl AnalyzeableRoundTwoNode for SwitchStatementNode {
    fn analyze_round_two(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn crate::issue::IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        self.condition.read_from(state, emitter);
        crate::missing!("analyze_round_two of switch");
        // Vi kjører på som om ingenting har hendt enn så lenge
        // Her må vi inn med noe semikomplisert scope/branch-analysis-greier
        self.analyze_round_two_children(&self.as_any(), state, emitter, path)
    }
}
