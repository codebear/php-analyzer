use crate::{
    analysis::state::{AnalysisState, FunctionState},
    autonodes::{
        anonymous_function_creation_expression::AnonymousFunctionCreationExpressionNode,
        any::AnyNodeRef,
    },
    issue::IssueEmitter,
    types::union::UnionType,
};

use super::analysis::ThirdPassAnalyzeableNode;
use crate::autotree::NodeAccess;

impl AnonymousFunctionCreationExpressionNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        // eprintln!("Det leses fra en closure-definisjon. Vi må vel finne ut om noen variabler blir `use`-deklarert");

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

impl ThirdPassAnalyzeableNode for AnonymousFunctionCreationExpressionNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        let function = FunctionState::new_anonymous();
        state.in_function_stack.push(function);
        let ret = self.analyze_third_pass_children(&self.as_any(), state, emitter, path);
        state.in_function_stack.pop();
        ret
    }
}
