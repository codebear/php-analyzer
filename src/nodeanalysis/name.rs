use crate::{
    analysis::state::AnalysisState,
    autonodes::name::NameNode,
    autotree::NodeAccess,
    issue::{Issue, IssueEmitter},
    symbols::{FullyQualifiedName, Name},
    types::union::UnionType,
    value::PHPValue,
};

impl NameNode {
    pub fn get_name(&self) -> Name {
        Name::from(self.get_raw())
    }

    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        // FIXME kanskje registrere lesing fra konstanter, men det er kanskje egen node som bør ta den
        
        // crate::missing!("{}.read_from(..)", self.kind());
    }

    pub fn get_constant_name(&self, state: &mut AnalysisState) -> FullyQualifiedName {
        let name = self.get_name();
        state.get_fq_symbol_name_from_local_name(&name)
    }

    ///
    /// A NameNode MIGHT be in a constant-usage-context. So make sure that the node is in such a context before relying on the output from this method
    ///
    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        let const_name = self.get_constant_name(state);
        if let Some(x) = state.global.constants.read().unwrap().get(&const_name) {
            x.get_value()
        } else {
            // FIXME this should emit in an analysis-method?
            emitter.emit(Issue::UnknownConstant(self.pos(state), const_name));
            None
        }
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        if let Some(v) = self.get_php_value(state, emitter) {
            v.get_utype()
        } else {
            // the constant is unknown
            None
            //            crate::missing_none!("{}[{:?}].get_utype(..)", self.kind(), self.get_raw())
        }
    }
}
