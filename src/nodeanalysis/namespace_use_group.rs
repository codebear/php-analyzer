use crate::{
    analysis::state::AnalysisState, autonodes::namespace_use_group::NamespaceUseGroupNode,
    issue::IssueEmitter, types::union::UnionType,
};

use super::analysis::AnalyzeableNode;

impl NamespaceUseGroupNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        ()
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

impl AnalyzeableNode for NamespaceUseGroupNode {
    fn analyze_round_one(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        todo!("WHAT: {:?}", self);
    }
}
