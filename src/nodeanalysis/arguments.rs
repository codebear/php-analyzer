use crate::{
    analysis::state::AnalysisState, autonodes::arguments::ArgumentsNode, issue::IssueEmitter,
    types::union::UnionType, value::PHPValue,
};

impl ArgumentsNode {
    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        for child in &self.children {
            child.read_from(state, emitter);
        }
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

    pub fn get_argument_values(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Vec<Option<PHPValue>> {
        self.children
            .iter()
            .map(|x| x.get_php_value(state, emitter))
            .collect()
    }

    pub fn get_argument_types(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Vec<Option<UnionType>> {
        self.children
            .iter()
            .map(|x| x.get_utype(state, emitter))
            .collect()
    }
}
