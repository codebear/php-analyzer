use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, try_statement::TryStatementNode},
    issue::IssueEmitter,
    types::union::PHPType,
};

use super::analysis::ThirdPassAnalyzeableNode;
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
    ) -> Option<PHPType> {
        crate::missing_none!("{}.get_utype(..)", self.kind())
    }
}

impl ThirdPassAnalyzeableNode for TryStatementNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &[AnyNodeRef],
    ) -> bool {
        // FIXME noe scope-greier
        crate::missing!();
        self.analyze_third_pass_children(&self.as_any(), state, emitter, path)
    }
}
