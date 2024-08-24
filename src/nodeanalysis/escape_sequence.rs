use crate::{
    analysis::state::AnalysisState,
    autonodes::escape_sequence::EscapeSequenceNode,
    issue::IssueEmitter,
    types::union::{DiscreteType, PHPType},
    value::PHPValue,
};

impl EscapeSequenceNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {}

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        Some(PHPValue::String(self.get_raw()))
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<PHPType> {
        Some(DiscreteType::String.into())
    }
}
