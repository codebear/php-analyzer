use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, for_statement::ForStatementNode},
    autotree::NodeAccess,
    issue::IssueEmitter,
    types::union::PHPType,
};

use super::analysis::ThirdPassAnalyzeableNode;

impl ForStatementNode {
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
    ) -> Option<PHPType> {
        crate::missing_none!("{}.get_utype(..)", self.kind())
    }
}

impl ThirdPassAnalyzeableNode for ForStatementNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &[AnyNodeRef],
    ) -> bool {
        // first analyze initializer, to make sure the condition-block doesn't falsely emit missing variables or similar
        if let Some(false) = self
            .initialize
            .as_ref()
            .map(|x| x.as_any().analyze_third_pass(state, emitter, path))
        {
            return false;
        }

        if let Some(false) = self.condition.as_ref().map(|x| {
            x.read_from(state, emitter);
            x.as_any().analyze_third_pass(state, emitter, path)
        }) {
            return false;
        }

        //        self.initialize.map(|x| x.read_from(state, emitter));
        crate::missing!("for-loop analysis needs attention");
        for child in &self.children {
            if !child.as_any().analyze_third_pass(state, emitter, path) {
                return false;
            }
        }
        self.increment
            .as_ref()
            .map(|x| x.as_any().analyze_third_pass(state, emitter, path))
            .unwrap_or(true)
    }
}
