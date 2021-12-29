use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, return_statement::ReturnStatementNode},
    issue::{Issue, IssueEmitter},
    types::union::UnionType,
};

use super::analysis::AnalyzeableRoundTwoNode;
use crate::autotree::NodeAccess;

impl ReturnStatementNode {
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

impl AnalyzeableRoundTwoNode for ReturnStatementNode {
    fn analyze_round_two(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        if let Some(child) = &self.child {
            child.read_from(state, emitter);

            let ret_type = child.get_utype(state, emitter);
            let ret_value = child.get_php_value(state, emitter);

            if let Some(func_state) = state.in_function_stack.last() {
                func_state.add_return(ret_type, ret_value);
            } else {
                emitter.emit(Issue::ParseAnomaly(
                    self.pos(state),
                    "return statement not in function".into(),
                ));
            }
        }

        self.analyze_round_two_children(&self.as_any(), state, emitter, path)
    }
}
