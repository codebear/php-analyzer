use crate::{
    analysis::state::{AnalysisState, ClassState},
    autonodes::{
        any::AnyNodeRef,
        class_declaration::{ClassDeclarationModifier, ClassDeclarationNode},
        class_interface_clause::ClassInterfaceClauseChildren,
    },
    autotree::NodeAccess,
    issue::{Issue, IssueEmitter},
    symboldata::{
        class::{ClassData, ClassModifier, ClassName, ClassType},
        FileLocation,
    },
    symbols::{FullyQualifiedName, Name},
    types::union::UnionType,
};

use super::{
    analysis::{AnalyzeableNode, AnalyzeableRoundTwoNode},
    class::{AnalysisOfClassBaseLikeNode, AnalysisOfDeclaredNameNode},
};
use crate::nodeanalysis::class::AnalysisOfClassLikeNode;

impl ClassDeclarationNode {
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

    fn get_class_name(&self, state: &mut AnalysisState) -> ClassName {
        let decl_class_name = self.get_declared_name();
        // new_with_analysis_state går nok via use-map, og deklarert klassenavn bør ikke det...
        let class_name =
            ClassName::new_with_analysis_state_without_aliasing(&decl_class_name, state);
        class_name
    }
}

impl AnalysisOfDeclaredNameNode for ClassDeclarationNode {
    fn get_declared_name(&self) -> Name {
        self.name.get_name()
    }
}

impl AnalysisOfClassBaseLikeNode for ClassDeclarationNode {}

impl AnalysisOfClassLikeNode for ClassDeclarationNode {
    fn get_interfaces(&self, state: &mut AnalysisState) -> Option<Vec<FullyQualifiedName>> {
        let mut ifs = vec![];
        for any_intf_claus in self.named_children("class_interface_clause") {
            if let AnyNodeRef::ClassInterfaceClause(intf_claus) = any_intf_claus {
                for intf in &intf_claus.children {
                    match &**intf {
                        ClassInterfaceClauseChildren::Name(n) => {
                            ifs.push(state.get_fq_symbol_name_from_local_name(&n.get_name()))
                        }
                        ClassInterfaceClauseChildren::QualifiedName(qn) => {
                            ifs.push(qn.get_fq_name())
                        }
                        _ => (),
                    }
                }
            }
        }
        if ifs.len() > 0 {
            Some(ifs)
        } else {
            None
        }
    }
}

impl AnalyzeableNode for ClassDeclarationNode {
    fn analyze_round_one(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        let class_name = self.get_class_name(state);
        let base_name = self.get_declared_base_class_name(state, emitter);

        let modifier = if let Some(modifier) = &self.modifier {
            match &**modifier {
                ClassDeclarationModifier::AbstractModifier(_) => ClassModifier::Abstract,
                ClassDeclarationModifier::FinalModifier(_) => ClassModifier::Final,
                _ => ClassModifier::None,
            }
        } else {
            ClassModifier::None
        };

        let interfaces = self.get_interfaces(state);

        let mut class_data =
            ClassData::new(FileLocation::new(self.name.pos(state)), class_name.clone());
        class_data.modifier = modifier;
        if let Some(_) = &base_name {
            class_data.base_class_name = base_name;
        }
        if let Some(int) = interfaces {
            class_data.interfaces = int
                .iter()
                .map(|iname| {
                    ClassName::new_with_names(
                        iname.get_name().unwrap_or_else(|| Name::new()),
                        iname.clone(),
                    )
                })
                .collect();
        }

        let symbol_data = state.symbol_data.get_or_create_class(&class_name);
        {
            let mut unlocked = symbol_data.write().unwrap();
            match *unlocked {
                ClassType::None => {
                    *unlocked = ClassType::Class(class_data);
                }
                _ => {
                    emitter.emit(Issue::DuplicateClass(
                        self.pos(state),
                        class_name.get_fq_name().clone(),
                    ));
                    //                     emitter.emit(state.filename.as_ref(), self.range, format!("Duplicate class {:?}. Not analyzing interior.", class_name.get_fq_name()).into());
                    return;
                }
            }
        }
        // eprintln!("ClassDeclarationNode.analyze_round_one(): Analyzed os fram til {:?}", class_data);
        state.in_class = Some(ClassState::Class(class_name));
        state.last_doc_comment = None;
        self.analyze_round_one_children(&self.as_any(), state, emitter);
        state.in_class = None;
    }
}

impl AnalyzeableRoundTwoNode for ClassDeclarationNode {
    fn analyze_round_two(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        let class_name = self.get_class_name(state);
        state.in_class = Some(ClassState::Class(class_name));
        state.last_doc_comment = None;
        let res = self.analyze_round_two_children(&self.as_any(), state, emitter, path);
        state.in_class = None;
        res
    }
}
