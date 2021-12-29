use crate::{
    analysis::state::AnalysisState, autonodes::type_list::TypeListNode, issue::IssueEmitter,
    types::union::UnionType,
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
    ) -> Option<UnionType> {
        let mut utype = UnionType::new();
        for tchild in &self.children {
            if let Some(t) = tchild.get_utype(state, emitter) {
                utype.merge_into(t);
            }
        }
        if utype.len() > 0 {
            Some(utype)
        } else {
            None
        }
    }
}
