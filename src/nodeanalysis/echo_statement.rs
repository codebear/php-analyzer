use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, echo_statement::EchoStatementNode},
    issue::IssueEmitter,
    types::union::UnionType,
};

use super::analysis::AnalyzeableRoundTwoNode;
use crate::autotree::NodeAccess;

impl EchoStatementNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        // void
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        None
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        None
    }
}

impl AnalyzeableRoundTwoNode for EchoStatementNode {
    fn analyze_round_two(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        self.child.read_from(state, emitter);

        // todo
        self.analyze_round_two_children(&self.as_any(), state, emitter, path)
    }
}
