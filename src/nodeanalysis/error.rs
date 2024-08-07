use std::{ffi::OsString, str::FromStr};

use crate::{
    analysis::state::AnalysisState,
    autonodes::any::AnyNodeRef,
    autotree::NodeAccess,
    errornode::ErrorNode,
    issue::{Issue, IssueEmitter},
    types::union::UnionType,
};

use super::analysis::ErrorPassAnalyzableNode;

impl ErrorNode {
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

impl ErrorPassAnalyzableNode for ErrorNode {
    fn analyze_errors(
        &self,
        node_ref: &AnyNodeRef,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) {
        let mut err =
            OsString::from_str("Parse error near ").expect("This should succeed in all cases");
        err.push(self.get_raw());

        emitter.emit(Issue::ParseError(
            state.pos_from_range(node_ref.range()),
            err,
        ));
    }
}
