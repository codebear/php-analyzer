use crate::analysis::state::AnalysisState;
use crate::issue::IssueEmitter;

use crate::types::union::PHPType;

pub trait AnalysisOfType {
    fn get_declared_type(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<PHPType>;

    fn get_inferred_type(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<PHPType>;
}
