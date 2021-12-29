use crate::{
    analysis::state::AnalysisState, autonodes::const_element::ConstElementNode,
    issue::IssueEmitter, symbols::Name, types::union::UnionType,
};

impl ConstElementNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        crate::missing!("{}.read_from(..)", self.kind());
    }

    /// the main part of the const analysis is in `crate::nodeanalysis::const_declaration::ConstDeclarationNode`
    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        self.value.get_php_value(state, emitter)
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        crate::missing_none!("{}.get_utype(..)", self.kind())
    }

    pub fn get_const_name(&self) -> Name {
        self.name.get_name()
    }
}
