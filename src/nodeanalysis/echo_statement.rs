use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, echo_statement::EchoStatementNode},
    issue::IssueEmitter,
    types::union::PHPType,
};

use super::analysis::ThirdPassAnalyzeableNode;
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
    ) -> Option<PHPType> {
        None
    }
}

impl ThirdPassAnalyzeableNode for EchoStatementNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &[AnyNodeRef],
    ) -> bool {
        self.child.read_from(state, emitter);

        // todo
        self.analyze_third_pass_children(&self.as_any(), state, emitter, path)
    }
}
