use crate::{
    analysis::state::AnalysisState, autonodes::relative_scope::RelativeScopeNode,
    issue::IssueEmitter, types::union::UnionType,
};

/// `self`, `static` og `parent`
impl RelativeScopeNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {}

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
