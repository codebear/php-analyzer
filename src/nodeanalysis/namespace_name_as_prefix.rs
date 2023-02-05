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

    pub fn is_root_anchored(&self) -> bool {
        let Some(child) = &self.child else {
            // An namespace_name_as_prefix_node without content represents \name
            return true;
        };
        // The ast is somewhat lacking here. So
        // the only fix we've found is to sneak in the byte offsets
        // If the first element starts after self, there is a \ at the beginning
        child.range.start_byte > self.range.start_byte
    }
}
