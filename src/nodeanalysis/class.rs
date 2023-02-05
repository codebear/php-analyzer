use crate::analysis::state::AnalysisState;
use crate::issue::{Issue, IssueEmitter};
use crate::symboldata::class::ClassName;

use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::base_clause::BaseClauseChildren;
use crate::autotree::NodeAccess;
use crate::symbols::{FullyQualifiedName, Name};

pub trait AnalysisOfDeclaredNameNode {
    ///
    /// The declared name of the class/interface/trait
    ///
    fn get_declared_name(&self) -> Name;
}

pub trait AnalysisOfClassBaseLikeNode: NodeAccess {
    ///
    /// extends <Something>
    ///
    fn get_declared_base_class_name(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<ClassName> {
        let mut names = if let Some(names) = self.get_declared_base_class_names(state, emitter) {
            names
        } else {
            return None;
        };

        if names.len() > 1 {
            emitter.emit(Issue::ParseAnomaly(
                self.pos(state),
                "Can't extend multiple bases".into(),
            ));
            return None;
        }
        let name = names.drain(..).next();
        name
    }
    ///
    /// extends <Something>(, <SomethingElse>)*
    ///
    fn get_declared_base_class_names(
        &self,
        state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<Vec<ClassName>> {
        for base_entry in self.named_children("base_clause") {
            let base_node = match base_entry {
                AnyNodeRef::BaseClause(base) => base,
                _ => continue,
            };
            let mut res = vec![];
            for base_child in &base_node.children {
                match &**base_child {
                    BaseClauseChildren::Name(n) => {
                        res.push(ClassName::new_with_analysis_state(&n.get_name(), state));
                    }
                    BaseClauseChildren::QualifiedName(n) => {
                        res.push(ClassName {
                            name: n.get_name(),
                            fq_name: n.get_fq_name(state),
                        });
                    }
                    BaseClauseChildren::Comment(c) => {
                        todo!("{:?}", c);
                    }
                    _ => continue,
                }
            }
            return if res.len() > 0 { Some(res) } else { None };
        }
        None
    }
}

pub trait AnalysisOfClassLikeNode: AnalysisOfClassBaseLikeNode {
    ///
    /// implements Some, Things
    ///
    fn get_interfaces(&self, _state: &mut AnalysisState) -> Option<Vec<FullyQualifiedName>> {
        None
    }
}
