use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, return_statement::ReturnStatementNode},
    issue::{Issue, IssueEmitter},
    types::union::{UnionType, DiscreteType},
};

use super::analysis::ThirdPassAnalyzeableNode;
use crate::autotree::NodeAccess;

impl ReturnStatementNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        crate::missing!("{}.read_from(..)", self.kind());
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        self.child.as_ref()?.get_php_value(state, emitter)
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        if let Some(child) = &self.child {
            child.get_utype(state, emitter)
        } else {
            Some(DiscreteType::Void.into())
        }
    }
}

impl ThirdPassAnalyzeableNode for ReturnStatementNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        let (ret_type, ret_value) = if let Some(child) = &self.child {
            child.read_from(state, emitter);

            let ret_type = child.get_utype(state, emitter);
            let ret_value = child.get_php_value(state, emitter);
            (ret_type, ret_value)
        } else {
            (Some(DiscreteType::Void.into()), None)
        };
        if let Some(func_state) = state.in_function_stack.last() {
            func_state.add_return(ret_type, ret_value);
        } else {
            emitter.emit(Issue::ParseAnomaly(
                self.pos(state),
                "return statement not in function".into(),
            ));
        }

        self.analyze_third_pass_children(&self.as_any(), state, emitter, path)
    }
}
