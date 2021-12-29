use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, try_statement::TryStatementNode},
    issue::IssueEmitter,
    types::union::UnionType,
};

use super::analysis::AnalyzeableRoundTwoNode;
use crate::autotree::NodeAccess;

impl TryStatementNode {
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

impl AnalyzeableRoundTwoNode for TryStatementNode {
    fn analyze_round_two(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        // FIXME noe scope-greier
        crate::missing!();
        self.analyze_round_two_children(&self.as_any(), state, emitter, path)
    }
}
