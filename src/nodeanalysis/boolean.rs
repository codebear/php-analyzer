use std::os::unix::prelude::OsStrExt;

use crate::{
    analysis::state::AnalysisState,
    autonodes::boolean::BooleanNode,
    issue::IssueEmitter,
    types::union::{DiscreteType, UnionType},
    value::PHPValue,
};

impl BooleanNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        ()
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        let raw = self.get_raw().to_ascii_lowercase();
        match raw.as_bytes() {
            b"false" => Some(PHPValue::Boolean(false)),
            b"true" => Some(PHPValue::Boolean(true)),
            _ => crate::missing_none!("Unknown boolean value? {:?}", self.get_raw()),
        }
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        Some(DiscreteType::Bool.into())
    }
}
