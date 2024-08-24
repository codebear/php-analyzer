use crate::{analysis::state::AnalysisState, issue::IssueEmitter, parser::Range};

use super::union::SpecialType;

impl SpecialType {
    pub(crate) fn ensure_valid(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
        _range: &Range,
        _allow_unforfilled_templates: bool,
    ) {
        crate::missing!("Ensure that self and static only are used in usable contexts");
    }
}
