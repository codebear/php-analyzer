use crate::autotree::NodeAccess;
use crate::types::union::PHPType;
use crate::{
    analysis::state::AnalysisState,
    autonodes::union_type::{UnionTypeChildren, UnionTypeNode},
    issue::{Issue, IssueEmitter},
    types::union::{DiscreteType, UnionType},
};

impl UnionTypeNode {
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
        for child in &self.children {
            if let Some(sometype) = match &**child {
                UnionTypeChildren::NamedType(nt) => nt.get_utype(state, emitter),
                UnionTypeChildren::OptionalType(ot) => ot.get_utype(state, emitter),
                UnionTypeChildren::PrimitiveType(pt) => pt.get_utype(state, emitter),

                UnionTypeChildren::Extra(_) => continue,
            } {
                utype.append(sometype)
            } else {
                emitter.emit(Issue::UnknownType(
                    state.pos_from_range(child.range()),
                    r"unable to  extract valid type".into(),
                ));
                utype.push(DiscreteType::Unknown);
            }
        }
        if utype.len() > 0 {
            Some(utype.into())
        } else {
            None
        }
    }
}
