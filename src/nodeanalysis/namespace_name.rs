use crate::{
    analysis::state::AnalysisState, autonodes::namespace_name::NamespaceNameNode,
    issue::IssueEmitter, symbols::FullyQualifiedName, types::union::PHPType,
};

impl NamespaceNameNode {
    pub fn get_name(&self) -> FullyQualifiedName {
        let mut fq_name = FullyQualifiedName::new();
        for part in &self.children {
            fq_name.push(part.get_name());
        }
        fq_name
    }

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
        crate::missing_none!("{}.get_utype(..)", self.kind())
    }
}
