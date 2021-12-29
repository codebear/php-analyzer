use std::os::unix::prelude::OsStrExt;

use crate::{
    analysis::state::AnalysisState,
    autonodes::integer::IntegerNode,
    issue::IssueEmitter,
    types::union::{DiscreteType, UnionType},
    value::PHPValue,
};

impl IntegerNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        ()
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        let raw = self.get_raw();
        let bytes = raw.as_bytes();
        let istr = String::from_utf8_lossy(raw.as_bytes());
        let parsed = if bytes.len() > 2 && bytes[1] == b'x' {
            i64::from_str_radix(&istr[2..], 16)
            // 0x noe
        } else {
            istr.parse::<i64>()
        };

        match parsed {
            Ok(i) => Some(PHPValue::Int(i)),
            Err(e) => crate::missing_none!(
                "get_php_value from {} found invalid integer: {:?}, error: {}",
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
        Some(DiscreteType::Int.into())
    }
}
