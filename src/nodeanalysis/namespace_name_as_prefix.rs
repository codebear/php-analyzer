use crate::{
    analysis::state::AnalysisState, autonodes::namespace_name_as_prefix::NamespaceNameAsPrefixNode,
    issue::IssueEmitter, symbols::FullyQualifiedName, types::union::UnionType,
};

impl NamespaceNameAsPrefixNode {
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

    pub fn get_prefix(&self) -> FullyQualifiedName {
        if let Some(ch) = &self.child {
            ch.get_name()
        } else {
            let range = self.range;
            let len = range.end_byte - range.start_byte;
            if len != 1 {
                panic!("Lengde er ikke 1??: {}, {:?}", len, self);
            }
            FullyQualifiedName::new()
        }
    }
}
