use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, namespace_definition::NamespaceDefinitionNode},
    autotree::NodeAccess,
    issue::{Issue, IssueEmitter},
    symbols::FullyQualifiedName,
    types::union::PHPType,
};

use super::analysis::{
    FirstPassAnalyzeableNode, SecondPassAnalyzeableNode, ThirdPassAnalyzeableNode,
};

impl NamespaceDefinitionNode {
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

    fn get_namespace(&self) -> FullyQualifiedName {
        if let Some(name) = &self.name {
            let mut fq_name = FullyQualifiedName::new();
            for ns in &name.children {
                fq_name.push(ns.get_name());
            }
            fq_name
        } else {
            FullyQualifiedName::new()
        }
    }
}

impl FirstPassAnalyzeableNode for NamespaceDefinitionNode {
    fn analyze_first_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        if self.name.is_some() {
            let namespace = self.get_namespace();
            state.namespace = Some(namespace);
        } else {
            emitter.emit(Issue::ParseAnomaly(
                self.pos(state),
                "Couldn't resolve namespace".into(),
            ))
        }
    }
}

impl SecondPassAnalyzeableNode for NamespaceDefinitionNode {
    fn analyze_second_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        if self.name.is_some() {
            let namespace = self.get_namespace();
            state.namespace = Some(namespace);
        } else {
            emitter.emit(Issue::ParseAnomaly(
                self.pos(state),
                "Couldn't resolve namespace".into(),
            ))
        }
    }
}

impl ThirdPassAnalyzeableNode for NamespaceDefinitionNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        _path: &[AnyNodeRef],
    ) -> bool {
        if self.name.is_some() {
            let namespace = self.get_namespace();
            state.namespace = Some(namespace);
        } else {
            emitter.emit(Issue::ParseAnomaly(
                self.pos(state),
                "Couldn't resolve namespace".into(),
            ))
        }
        true
    }
}
