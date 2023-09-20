use crate::{
    analysis::state::AnalysisState,
    autonodes::qualified_name::{QualifiedNameChildren, QualifiedNameNode},
    autotree::NodeAccess,
    issue::{Issue, IssueEmitter},
    symbols::{FullyQualifiedName, Name},
    types::union::UnionType,
    value::PHPValue,
};

impl QualifiedNameNode {
    pub fn get_prefix(&self) -> FullyQualifiedName {
        for x in &self.children {
            match &**x {
                QualifiedNameChildren::NamespaceNameAsPrefix(pf) => {
                    return pf.get_prefix();
                }
                QualifiedNameChildren::Name(_) => (),
                _ => todo!("Hva gjør vi med en {:?} her?", x),
            }
        }
        FullyQualifiedName::new()
    }

    pub fn is_root_anchored(&self) -> bool {
        for child in &self.children {
            match &**child {
                QualifiedNameChildren::Name(n) => {
                    todo!("CRAP {:?} {:?}", self.range.start_byte, n.range.start_byte);
                    // return false;
                }
                QualifiedNameChildren::NamespaceNameAsPrefix(nn) => {
                    return nn.is_root_anchored();
                }
                QualifiedNameChildren::Comment(_)
                | QualifiedNameChildren::TextInterpolation(_)
                | QualifiedNameChildren::Error(_) => (),
            }
        }
        false
    }

    /// This must not be used and relied upon in `use` statements, as
    /// those are always fully-qualified originally
    /// @see get_raw_fq_name
    pub fn get_fq_name(&self, state: &AnalysisState) -> FullyQualifiedName {
        let mut qn = if self.is_root_anchored() {
            self.get_prefix()
        } else if let Some(ns) = &state.namespace {
            let mut qn = ns.clone();
            qn.append_fq(self.get_prefix());
            qn
        } else {
            self.get_prefix()
        };
        qn.push(self.get_name());
        qn
    }

    pub fn get_raw_fq_name(&self) -> FullyQualifiedName {
        let mut qn = self.get_prefix();

        qn.push(self.get_name());
        qn
    }
    pub fn get_name(&self) -> Name {
        let mut name: Option<Name> = None;
        for x in &self.children {
            match &**x {
                QualifiedNameChildren::NamespaceNameAsPrefix(_) => (),
                QualifiedNameChildren::Name(n) => {
                    if let Some(_) = name {
                        panic!("Vi har funnet flere name-noder?");
                    }
                    name = Some(n.get_name())
                }
                _ => todo!("Hva gjør vi med en {:?} her?", x),
            }
        }

        name.take().expect("Den her burde funnet et name?")
    }

    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        // FIXME report usage of constant
        ()
        //        crate::missing!("{}.read_from(..)", self.kind());
    }

    ///
    /// A QualifiedNameNode MIGHT be in a constant-usage-context. So make sure that the node is in such a context before relying on the output from this
    ///
    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        if let Some(x) = state
            .global
            .constants
            .read()
            .unwrap()
            .get(&self.get_fq_name(state))
        {
            x.get_value()
        } else {
            // FIXME this should not emit here, but in an analysis-pass
            emitter.emit(Issue::UnknownConstant(
                self.pos(state),
                self.get_fq_name(state),
            ));
            None
        }
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        crate::missing_none!("{}.get_utype(..)", self.kind())
    }
}
