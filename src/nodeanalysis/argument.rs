use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, argument::ArgumentNode},
    issue::IssueEmitter,
    types::union::UnionType,
    value::PHPValue,
};

use super::analysis::ThirdPassAnalyzeableNode;
use crate::autotree::NodeAccess;

impl ArgumentNode {
    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        self.child.read_from(state, emitter);
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        self.child.get_php_value(state, emitter)
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        self.child.get_utype(state, emitter)
    }
}

impl ThirdPassAnalyzeableNode for ArgumentNode {
    fn analyze_third_pass(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn crate::issue::IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        if !self.analyze_third_pass_children(&self.as_any(), state, emitter, path) {
            return false;
        }
        self.child.read_from(state, emitter);
        true
    }
}
