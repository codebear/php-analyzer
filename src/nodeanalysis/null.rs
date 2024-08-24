use crate::{
    analysis::state::AnalysisState,
    autonodes::null::NullNode,
    issue::IssueEmitter,
    types::union::{DiscreteType, PHPType},
    value::PHPValue,
};

impl NullNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {}

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        Some(PHPValue::NULL)
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<PHPType> {
        Some(DiscreteType::NULL.into())
    }
}
