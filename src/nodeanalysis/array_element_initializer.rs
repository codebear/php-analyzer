use crate::{
    analysis::state::AnalysisState,
    autonodes::array_element_initializer::ArrayElementInitializerNode, issue::IssueEmitter,
    types::union::UnionType,
};

impl ArrayElementInitializerNode {
    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        if let Some(key_ref) = &self.key {
            key_ref.read_from(state, emitter);
        }

        if let Some(value_ref) = &self.value {
            value_ref.read_from(state, emitter);
        }

        if let Some(spread) = &self.spread {
            spread.read_from(state, emitter);
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
}
