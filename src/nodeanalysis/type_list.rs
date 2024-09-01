use crate::{
    analysis::state::AnalysisState,
    autonodes::type_list::TypeListNode,
    issue::IssueEmitter,
    types::union::{PHPType, UnionType},
};

impl TypeListNode {
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
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPType> {
        let mut utype = UnionType::new();
        for tchild in &self.children {
            if let Some(t) = tchild.get_utype(state, emitter) {
                utype.append(t);
            }
        }
        if !utype.is_empty() {
            Some(utype.into())
        } else {
            None
        }
    }
}
