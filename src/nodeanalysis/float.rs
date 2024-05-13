use std::os::unix::prelude::OsStrExt;

use crate::{
    analysis::state::AnalysisState,
    autonodes::float::FloatNode,
    issue::IssueEmitter,
    types::union::{DiscreteType, UnionType},
    value::{PHPFloat, PHPValue},
};

impl FloatNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {}

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        let raw = self.get_raw();
        let fval = String::from_utf8_lossy(raw.as_bytes());
        match fval.parse::<f64>() {
            Ok(f) => Some(PHPValue::Float(PHPFloat::new(f))),
            Err(e) => crate::missing_none!(
                "get_php_value from {:?} found invalid float: {:?}, error: {}",
                self.kind(),
                self.get_raw(),
                e
            ),
        }
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        Some(DiscreteType::Float.into())
    }
}
