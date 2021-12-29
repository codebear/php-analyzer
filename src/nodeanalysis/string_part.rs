use crate::autonodes::string_part::StringPartNode;
use crate::types::union::DiscreteType;
use crate::types::union::UnionType;
use crate::{analysis::state::AnalysisState, issue::IssueEmitter, value::PHPValue};

impl StringPartNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        ()
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        let str = self.get_raw();
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
