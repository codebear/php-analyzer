use std::sync::{Arc, RwLock};

use crate::{
    analysis::state::{AnalysisState, ClassState},
    autonodes::{any::AnyNodeRef, interface_declaration::InterfaceDeclarationNode},
    issue::{Issue, IssueEmitter},
    symboldata::{
        class::{ClassName, ClassType, InterfaceData},
        FileLocation,
    },
    symbols::Name,
    types::union::PHPType,
};

use super::{
    analysis::{FirstPassAnalyzeableNode, SecondPassAnalyzeableNode, ThirdPassAnalyzeableNode},
    class::AnalysisOfDeclaredNameNode,
};
use crate::autotree::NodeAccess;
use crate::nodeanalysis::class::AnalysisOfClassBaseLikeNode;

impl InterfaceDeclarationNode {
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

    pub fn get_interface_name(&self, state: &mut AnalysisState) -> ClassName {
        let decl_if_name = self.get_declared_name();
        ClassName::new_with_analysis_state_without_aliasing(&decl_if_name, state)
    }

    fn get_interface_data(&self, state: &mut AnalysisState) -> Arc<RwLock<ClassType>> {
        let if_name = self.get_interface_name(state);
        state.symbol_data.get_or_create_class(&if_name)
    }
}
///
/// INTERFACES
///

impl AnalysisOfDeclaredNameNode for InterfaceDeclarationNode {
    fn get_declared_name(&self) -> Name {
        self.name.get_name()
    }
}

impl AnalysisOfClassBaseLikeNode for InterfaceDeclarationNode {}

impl FirstPassAnalyzeableNode for InterfaceDeclarationNode {
    fn analyze_first_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        let if_name = self.get_interface_name(state);
        let base_name = self.get_declared_base_class_names(state, emitter);

        let mut if_data =
            InterfaceData::new(FileLocation::new(self.name.pos(state)), if_name.clone());
        if_data.base_interface_names = base_name;

        let symbol_data = state.symbol_data.get_or_create_class(&if_name);
        {
            let mut unlocked = symbol_data.write().unwrap();
            match *unlocked {
                ClassType::None => {
                    *unlocked = ClassType::Interface(if_data);
                }
                _ => emitter.emit(Issue::DuplicateClass(
                    self.name.pos(state),
                    if_name.get_fq_name().clone(),
                )),
            }
        }

        state.last_doc_comment = None;
        state.in_class = Some(ClassState::Interface(if_name, symbol_data));
        self.analyze_first_pass_children(&self.as_any(), state, emitter);
        state.in_class = None;
    }
}

impl SecondPassAnalyzeableNode for InterfaceDeclarationNode {
    fn analyze_second_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        let if_name = self.get_interface_name(state);
        state.in_class = Some(ClassState::Interface(
            if_name,
            self.get_interface_data(state),
        ));
        self.analyze_second_pass_children(&self.as_any(), state, emitter);
        state.in_class = None;
    }
}

impl ThirdPassAnalyzeableNode for InterfaceDeclarationNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &[AnyNodeRef],
    ) -> bool {
        let if_name = self.get_interface_name(state);
        state.last_doc_comment = None;
        state.in_class = Some(ClassState::Interface(
            if_name,
            self.get_interface_data(state),
        ));
        let carry_on = self.analyze_third_pass_children(&self.as_any(), state, emitter, path);
        state.in_class = None;

        carry_on
    }
}
