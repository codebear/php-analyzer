use std::{ffi::OsStr, os::unix::prelude::OsStrExt};

use crate::{
    analysis::state::AnalysisState,
    autonodes::string::StringNode,
    issue::IssueEmitter,
    types::union::{DiscreteType, UnionType},
    value::PHPValue,
};

impl StringNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        ()
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        todo!();
        let raw = b""; // &self.child.raw;
        let len = raw.len();
        if len < 2 {
            return None;
        }
        let raw = &raw[1..len - 1];
        let str = OsStr::from_bytes(raw).to_os_string();
        Some(PHPValue::String(str))
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        Some(DiscreteType::String.into())
    }
}
