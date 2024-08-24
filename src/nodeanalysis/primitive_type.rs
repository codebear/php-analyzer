use std::os::unix::prelude::OsStrExt;

use crate::{
    analysis::state::AnalysisState,
    autonodes::primitive_type::PrimitiveTypeNode,
    issue::IssueEmitter,
    types::union::{DiscreteType, PHPType},
};

impl PrimitiveTypeNode {
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
        let raw = self.get_raw().to_ascii_lowercase();

        let dtype = match raw.as_bytes() {
            b"callable" => DiscreteType::Callable,
            b"string" => DiscreteType::String,
            b"int" => DiscreteType::Int,
            b"array" => DiscreteType::Array,
            b"bool" => DiscreteType::Bool,
            b"void" => DiscreteType::Void,
            b"float" => DiscreteType::Float,
            _ => {
                return crate::missing_none!(
                    "Finn type from PrimitiveType like {:?}",
                    self.get_raw()
                )
            }
        };

        Some(PHPType::Discrete(Box::new(dtype)))
    }
}
