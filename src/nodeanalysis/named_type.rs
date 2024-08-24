use crate::{
    analysis::state::AnalysisState,
    autonodes::named_type::{NamedTypeChildren, NamedTypeNode},
    issue::IssueEmitter,
    symbols::Name,
    types::union::{DiscreteType, PHPType},
};

impl NamedTypeNode {
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
        _emitter: &dyn IssueEmitter,
    ) -> Option<PHPType> {
        let discrete_type = match &*self.child {
            NamedTypeChildren::Name(n) => {
                let fq_name = state.get_fq_symbol_name_from_local_name(&n.get_name());
                DiscreteType::Named(n.get_name(), fq_name)
            }
            NamedTypeChildren::QualifiedName(fq) => {
                let fq_name = fq.get_fq_name(state);
                DiscreteType::Named(fq_name.get_name().unwrap_or_else(Name::new), fq_name)
            }
            _ => todo!(
                "Fint type from NamedType in {}: {:?}",
                state.pos_as_string(self.range),
                self
            ),
        };

        Some(discrete_type.into())
    }
}
