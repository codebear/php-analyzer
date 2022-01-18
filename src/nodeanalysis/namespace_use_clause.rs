use crate::{
    analysis::state::AnalysisState,
    autonodes::{
        any::AnyNodeRef,
        namespace_use_clause::{NamespaceUseClauseChildren, NamespaceUseClauseNode},
    },
    autotree::NodeAccess,
    issue::{Issue, IssueEmitter},
    symbols::{FullyQualifiedName, Name},
    types::union::UnionType,
};

use super::analysis::{FirstPassAnalyzeableNode, ThirdPassAnalyzeableNode};

impl NamespaceUseClauseNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        ()
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        None
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        None
    }
}

impl ThirdPassAnalyzeableNode for NamespaceUseClauseNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        _path: &Vec<AnyNodeRef>,
    ) -> bool {
        self.analyze_first_pass(state, emitter);

        true
    }
}

impl FirstPassAnalyzeableNode for NamespaceUseClauseNode {
    fn analyze_first_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        // eprintln!("\n\n************************");
        let mut use_fq_name: Option<FullyQualifiedName> = None;
        let mut use_name: Option<Name> = None;
        for child in &self.children {
            //eprintln!("child: {:?}", child);
            match &**child {
                NamespaceUseClauseChildren::Name(n) => {
                    // Box<NameNode>),
                    if use_name.is_some() {
                        // We didn't expect us to arrive here multiple times
                        todo!("Something askew with parsing");
                    }
                    let name = n.get_name();
                    use_fq_name = Some(FullyQualifiedName::from(&name));
                    use_name = Some(name);
                    //
                    /*                     eprintln!("NAVN: {:?}", n.get_name());
                    let name = n.get_name();
                    if let Some(_) = state.use_map.get(&name) {
                        emitter.emit(n.range(), format!("Duplicate symbol {:?}", name));
                    } else {
                        let fq_name = &[b"\\", name.as_bytes()].concat();
                        state.use_map.insert(name, OsStr::from_bytes(fq_name).to_os_string());
                    }
                    */
                    //    eprintln!("Usemap: {:?}", state.use_map);
                }
                NamespaceUseClauseChildren::NamespaceAliasingClause(use_clause) => {
                    // (), // Box<NamespaceAliasingClauseNode>),
                    //                    eprintln!("use_name={:?}, use_fq_name={:?}", use_name, use_fq_name);
                    //                    let use_clause = &**n;
                    use_name = Some(use_clause.child.get_name());
                    //                    eprintln!("NamespaceUseClauseChildren::NamespaceAliasingClause: {:?}", x.child.get_name());
                }
                NamespaceUseClauseChildren::QualifiedName(qn) => {
                    if use_name.is_some() {
                        // We didn't expect us to arrive here multiple times
                        todo!("Something askew with parsing");
                    }
                    use_name = Some(qn.get_name());
                    use_fq_name = Some(qn.get_fq_name());
                    // eprintln!("USE FQ: {:?}", use_fq_name);
                    //let fq_name = qn.get_fq_name();

                    /*                     eprintln!("FQNAVN: {:?}", &fq_name);
                    let name = qn.get_name();
                    if let Some(_) = state.use_map.get(&name) {
                        emitter.emit(qn.range(), format!("Duplicate symbol {:?}", name));
                    } else {
                        state.use_map.insert(name, fq_name);
                    }*/
                    //        eprintln!("Usemap: {:?}", state.use_map);
                }
                _ => continue,
            }
        }

        match (use_fq_name, use_name) {
            (Some(fq_name), Some(name)) => {
                if let Some(_) = state.use_map.get(&name) {
                    if state.pass <= 2 {
                        emitter.emit(Issue::DuplicateSymbol(self.pos(state), name));
                    }
                } else {
                    // eprintln!("INSERT INTO USE_MAP: {:?}", &fq_name);
                    state.use_map.insert(name, fq_name);
                }
            }
            _ => eprintln!("Someting strange, in the neighbourhood"),
        }
    }
}
