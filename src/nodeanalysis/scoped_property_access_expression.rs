use crate::{
    analysis::state::AnalysisState,
    autonodes::{
        any::AnyNodeRef,
        scoped_property_access_expression::{
            ScopedPropertyAccessExpressionName, ScopedPropertyAccessExpressionNode,
        },
    },
    issue::IssueEmitter,
    types::union::UnionType,
    value::PHPValue,
};

use super::analysis::ThirdPassAnalyzeableNode;
use crate::autotree::NodeAccess;

impl ScopedPropertyAccessExpressionNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        //FIXME mark reading from static class vars
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        None
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        crate::missing_none!("Finn ut av lesing fra en ScopedPropertyAccessExpressionNode")
    }

    pub fn write_to(
        &self,
        _state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn IssueEmitter,
        _val_type: Option<UnionType>,
        _value: Option<PHPValue>,
    ) {
        crate::missing!();
    }
}

impl ThirdPassAnalyzeableNode for ScopedPropertyAccessExpressionNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        if !self.scope.as_any().analyze_third_pass(state, emitter, path) {
            return false;
        }

        self.name.analyze_third_pass(state, emitter, path)
    }
}

impl ThirdPassAnalyzeableNode for ScopedPropertyAccessExpressionName {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        match self {
            ScopedPropertyAccessExpressionName::DynamicVariableName(dv) => {
                // a statement like { Foo:$$bar; } $bar needs to be found, so let's dig down here
                dv.as_any().analyze_third_pass(state, emitter, path)
            }
            ScopedPropertyAccessExpressionName::VariableName(_) => {
                // The VariableName node thinks it's a regular variable
                // so we don't want to analyze it here
                true
            }

            ScopedPropertyAccessExpressionName::Comment(_)
            | ScopedPropertyAccessExpressionName::TextInterpolation(_)
            | ScopedPropertyAccessExpressionName::Error(_) => true,
        }
    }
}
