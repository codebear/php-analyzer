use crate::{
    analysis::state::AnalysisState,
    autonodes::const_declaration::{ConstDeclarationChildren, ConstDeclarationNode},
    issue::{Issue, IssueEmitter},
    symboldata::class::ClassType,
    types::union::UnionType,
};

use super::analysis::AnalyzeableNode;
use crate::autotree::NodeAccess;

impl ConstDeclarationNode {
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
}

impl AnalyzeableNode for ConstDeclarationNode {
    fn analyze_round_one(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        // Finn ut av
        // self.attributes;
        // Finn ut av
        if let Some(modi) = &self.modifier {
            eprintln!("Const har en {:?}, hva gjør vi med det?", modi.get_raw());
        }

        for child in &self.children {
            match &**child {
                ConstDeclarationChildren::ConstElement(c) => {
                    let name = c.get_const_name();
                    let value = c.get_php_value(state, emitter);
                    if let Some(val) = c.get_php_value(state, emitter) {
                        if let Some(class_state) = &state.in_class {
                            if let Some(class_data) =
                                state.symbol_data.get_class(&class_state.get_name())
                            {
                                let mut mutable = class_data.write().unwrap();
                                match &mut (*mutable) {
                                    ClassType::Class(c) => {
                                        if let Some(_) = c.constants.get(&name) {
                                            emitter.emit(Issue::DuplicateClassConstant(
                                                self.pos(state),
                                                class_state.get_name().get_fq_name().clone(),
                                                name,
                                            ));
                                        } else {
                                            c.constants.insert(name, val);
                                        }
                                    }
                                    ClassType::None => todo!(),
                                    ClassType::Interface(intf) => {
                                        if let Some(_) = intf.constants.get(&name) {
                                            emitter.emit(Issue::DuplicateClassConstant(
                                                self.pos(state),
                                                class_state.get_name().get_fq_name().clone(),
                                                name,
                                            ));
                                        } else {
                                            intf.constants.insert(name, val);
                                        }
                                    }
                                    ClassType::Trait(_) => todo!(),
                                }
                            } else {
                                eprintln!("Missing class: {:?}", class_state.get_name());
                                // Finner ikke klassen?
                            }
                        } else {
                            // Global const?
                            eprintln!("Global const decls?");
                            todo!("Const: self::{:?} = {:?} ({:?})", name, value, c);
                        }
                    } else {
                        // emitter.emit(self.range(), format!("Couldn't resolve class const content for {:?}", name).into());
                    }
                }
                ConstDeclarationChildren::VisibilityModifier(v) => todo!("analysere: {:?}", v),
                _ => continue,
            }
        }
    }
}
