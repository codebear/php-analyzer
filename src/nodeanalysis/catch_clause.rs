use crate::autonodes::any::AnyNodeRef;
use crate::{
    analysis::state::AnalysisState, autonodes::catch_clause::CatchClauseNode, issue::IssueEmitter,
    types::union::UnionType,
};

use super::analysis::ThirdPassAnalyzeableNode;
use crate::analysis::scope::BranchableScope;
use crate::autotree::NodeAccess;

impl CatchClauseNode {
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

impl ThirdPassAnalyzeableNode for CatchClauseNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        crate::missing!();
        let scope = state.current_scope();
        let branch = scope.branch();
        state.push_scope(branch);
        if let Some(catch_var) = &self.name {
            let utype = self.type_.get_utype(state, emitter);
            catch_var.write_to(state, emitter, utype, None);
        }
        let carry_on = self.analyze_third_pass_children(&self.body.as_any(), state, emitter, path);
        state.pop_scope();

        carry_on
    }
}
