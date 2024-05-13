use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, namespace_use_group::NamespaceUseGroupNode},
    issue::IssueEmitter,
    types::union::UnionType,
};

use super::analysis::{
    FirstPassAnalyzeableNode, SecondPassAnalyzeableNode, ThirdPassAnalyzeableNode,
};

impl NamespaceUseGroupNode {
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

impl FirstPassAnalyzeableNode for NamespaceUseGroupNode {
    fn analyze_first_pass(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        todo!("WHAT: {:?}", self);
    }
}

impl SecondPassAnalyzeableNode for NamespaceUseGroupNode {
    fn analyze_second_pass(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        todo!("WHAT: {:?}", self);
    }
}

impl ThirdPassAnalyzeableNode for NamespaceUseGroupNode {
    fn analyze_third_pass(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
        _path: &Vec<AnyNodeRef>,
    ) -> bool {
        todo!("WHAT: {:?}", self);
    }
}
