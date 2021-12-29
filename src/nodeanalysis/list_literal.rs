use crate::{
    analysis::state::AnalysisState, autonodes::list_literal::ListLiteralNode, issue::IssueEmitter,
    types::union::UnionType, value::PHPValue,
};

impl ListLiteralNode {
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
        crate::missing_none!("{}.get_utype(..)", self.kind())
    }

    pub fn write_to(
        &self,
        _state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn IssueEmitter,
        _val_type: Option<UnionType>,
        _value: Option<PHPValue>,
    ) {
        crate::missing!("list literal write_to");
    }
}
