use crate::{
    analysis::state::{AnalysisState, ClassState},
    autonodes::{any::AnyNodeRef, interface_declaration::InterfaceDeclarationNode},
    issue::{Issue, IssueEmitter},
    symboldata::{
        class::{ClassName, ClassType, InterfaceData},
        FileLocation,
    },
    symbols::Name,
    types::union::UnionType,
};

use super::{
    analysis::{AnalyzeableNode, AnalyzeableRoundTwoNode},
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
    ) -> Option<UnionType> {
        crate::missing_none!("{}.get_utype(..)", self.kind())
    }

    pub fn get_interface_name(&self, state: &mut AnalysisState) -> ClassName {
        let decl_if_name = self.get_declared_name();
        ClassName::new_with_analysis_state_without_aliasing(&decl_if_name, state)
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

impl AnalyzeableNode for InterfaceDeclarationNode {
    fn analyze_round_one(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
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
        state.in_class = Some(ClassState::Interface(if_name));
        self.analyze_round_one_children(&self.as_any(), state, emitter);
        state.in_class = None;
    }
}

impl AnalyzeableRoundTwoNode for InterfaceDeclarationNode {
    fn analyze_round_two(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        let if_name = self.get_interface_name(state);
        state.last_doc_comment = None;
        state.in_class = Some(ClassState::Interface(if_name));
        let carry_on = self.analyze_round_two_children(&self.as_any(), state, emitter, path);
        state.in_class = None;

        carry_on
    }
}
