use std::os::unix::prelude::OsStrExt;

use crate::{
    analysis::state::AnalysisState,
    autonodes::cast_expression::CastExpressionNode,
    issue::IssueEmitter,
    types::union::{DiscreteType, UnionType},
};

impl CastExpressionNode {
    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        self.value.read_from(state, emitter)
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        let val = self.value.get_php_value(state, emitter)?;
        let cast = self.type_.get_raw().to_ascii_lowercase(); // .as_bytes();
        match cast.as_bytes() {
            b"int" => val.as_php_int(),
            b"string" => val.as_php_string(),
            b"bool" => val.as_php_bool(),
            b"float" => val.as_php_float(),
            _ => crate::missing_none!("{}.get_php_value(..) cast to {:?}", self.kind(), cast),
        }
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        let cast = self.type_.get_raw().to_ascii_lowercase(); // .as_bytes();
        match cast.as_bytes() {
            b"string" => Some(DiscreteType::String.into()),
            b"int" => Some(DiscreteType::Int.into()),
            b"float" |
            b"double" => Some(DiscreteType::Float.into()),
            b"array" => Some(DiscreteType::Array.into()),
            b"bool" => Some(DiscreteType::Bool.into()),
            b"object" | // What!?
            _ => crate::missing_none!("{}.get_utype(..) cast to {:?}", self.kind(), cast),
        }
    }
}
