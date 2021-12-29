use std::os::unix::prelude::OsStrExt;

use crate::{
    analysis::state::AnalysisState,
    autonodes::primitive_type::PrimitiveTypeNode,
    issue::IssueEmitter,
    types::union::{DiscreteType, UnionType},
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
    ) -> Option<UnionType> {
        let raw = self.get_raw().to_ascii_lowercase();
        Some(match raw.as_bytes() {
            b"callable" => UnionType::from(DiscreteType::Callable),
            b"string" => UnionType::from(DiscreteType::String),
            b"int" => UnionType::from(DiscreteType::Int),
            b"array" => UnionType::from(DiscreteType::Array),
            b"bool" => UnionType::from(DiscreteType::Bool),
            b"void" => UnionType::from(DiscreteType::Void),
            b"float" => UnionType::from(DiscreteType::Float),
            _ => {
                return crate::missing_none!(
                    "Finn type from PrimitiveType like {:?}",
                    self.get_raw()
                )
            }
        })
    }
}
