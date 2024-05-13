use crate::{
    analysis::state::AnalysisState,
    autonodes::heredoc::HeredocNode,
    issue::IssueEmitter,
    types::union::{DiscreteType, UnionType},
};

impl HeredocNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        // Tree-sitters heredoc-parsing is incomplete

        //        crate::missing!("{}.read_from(..)", self.kind());
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
        Some(DiscreteType::String.into())
    }
}
