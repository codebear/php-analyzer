use std::os::unix::prelude::OsStrExt;

use crate::autotree::NodeAccess;
use crate::issue::Issue;
use crate::symbols::Name;
use crate::{
    analysis::state::AnalysisState,
    autonodes::class_constant_access_expression::{
        ClassConstantAccessExpressionClass, ClassConstantAccessExpressionNode,
    },
    issue::IssueEmitter,
    symboldata::class::{ClassName, ClassType},
    types::union::UnionType,
    value::PHPValue,
};

impl ClassConstantAccessExpressionNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        // FIXME marker en konstant som brukt
        ()
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        crate::missing_none!("{}.get_utype(..)", self.kind())
    }

    pub fn get_class_name(&self, state: &AnalysisState) -> Option<ClassName> {
        match &*self.class {
            ClassConstantAccessExpressionClass::ArrayCreationExpression(n) => crate::missing_none!(
                "Hvordan hente en verdi ut av {:?} at {}",
                n,
                state.pos_as_string(self.range())
            ),
            ClassConstantAccessExpressionClass::CastExpression(n) => crate::missing_none!(
                "Hvordan hente en verdi ut av {:?} at {}",
                n,
                state.pos_as_string(self.range())
            ),
            ClassConstantAccessExpressionClass::ClassConstantAccessExpression(n) => {
                crate::missing_none!(
                    "Hvordan hente en verdi ut av {:?} at {}",
                    n,
                    state.pos_as_string(self.range())
                )
            }
            ClassConstantAccessExpressionClass::DynamicVariableName(n) => crate::missing_none!(
                "Hvordan hente en verdi ut av {:?} at {}",
                n,
                state.pos_as_string(self.range())
            ),
            ClassConstantAccessExpressionClass::EncapsedString(n) => crate::missing_none!(
                "Hvordan hente en verdi ut av {:?} at {}",
                n,
                state.pos_as_string(self.range())
            ),
            ClassConstantAccessExpressionClass::FunctionCallExpression(n) => crate::missing_none!(
                "Hvordan hente en verdi ut av {:?} at {}",
                n,
                state.pos_as_string(self.range())
            ),
            ClassConstantAccessExpressionClass::Heredoc(n) => crate::missing_none!(
                "Hvordan hente en verdi ut av {:?} at {}",
                n,
                state.pos_as_string(self.range())
            ),
            ClassConstantAccessExpressionClass::MemberAccessExpression(n) => crate::missing_none!(
                "Hvordan hente en verdi ut av {:?} at {}",
                n,
                state.pos_as_string(self.range())
            ),
            ClassConstantAccessExpressionClass::MemberCallExpression(n) => crate::missing_none!(
                "Hvordan hente en verdi ut av {:?} at {}",
                n,
                state.pos_as_string(self.range())
            ),
            ClassConstantAccessExpressionClass::Name(n) => {
                Some(ClassName::new_with_analysis_state(&n.get_name(), state))
            }

            ClassConstantAccessExpressionClass::NullsafeMemberAccessExpression(n) => {
                crate::missing_none!(
                    "Hvordan hente en verdi ut av {:?} at {}",
                    n,
                    state.pos_as_string(self.range())
                )
            }
            ClassConstantAccessExpressionClass::NullsafeMemberCallExpression(n) => {
                crate::missing_none!(
                    "Hvordan hente en verdi ut av {:?} at {}",
                    n,
                    state.pos_as_string(self.range())
                )
            }
            ClassConstantAccessExpressionClass::ParenthesizedExpression(n) => crate::missing_none!(
                "Hvordan hente en verdi ut av {:?} at {}",
                n,
                state.pos_as_string(self.range())
            ),
            ClassConstantAccessExpressionClass::QualifiedName(n) => {
                Some(ClassName::new_with_names(
                    n.get_fq_name().get_name().unwrap_or_else(|| Name::new()),
                    n.get_fq_name(),
                ))
            }
            ClassConstantAccessExpressionClass::RelativeScope(n) => {
                let rel_scope = n.get_raw();

                match rel_scope.to_ascii_lowercase().as_bytes() {
                    b"static" => {
                        if let Some(cstate) = &state.in_class {
                            // This one is actually wrong...
                            // emitter.emit(self.range, format!("Refering to a constant with `static` is ilogical, as the constant can't be abstract, missing nor redefined in a subclass").into());
                            // FIXME how to ve analyze this?
                            Some(cstate.get_name())
                        } else {
                            // error-state?
                            None
                        }
                    }
                    b"self" => {
                        if let Some(cstate) = &state.in_class {
                            Some(cstate.get_name())
                        } else {
                            // error-state?
                            None
                        }
                    }
                    b"parent" => crate::missing_none!(),
                    _ => panic!("Should not get here"),
                }
            }
            ClassConstantAccessExpressionClass::ScopedCallExpression(n) => crate::missing_none!(
                "Hvordan hente en verdi ut av {:?} at {}",
                n,
                state.pos_as_string(self.range())
            ),
            ClassConstantAccessExpressionClass::ScopedPropertyAccessExpression(n) => {
                crate::missing_none!(
                    "Hvordan hente en verdi ut av {:?} at {}",
                    n,
                    state.pos_as_string(self.range())
                )
            }
            ClassConstantAccessExpressionClass::String(n) => crate::missing_none!(
                "Hvordan hente en verdi ut av {:?} at {}",
                n,
                state.pos_as_string(self.range())
            ),
            ClassConstantAccessExpressionClass::SubscriptExpression(n) => crate::missing_none!(
                "Hvordan hente en verdi ut av {:?} at {}",
                n,
                state.pos_as_string(self.range())
            ),
            ClassConstantAccessExpressionClass::VariableName(n) => crate::missing_none!(
                "Hvordan hente en verdi ut av {:?} at {}",
                n,
                state.pos_as_string(self.range())
            ),

            ClassConstantAccessExpressionClass::Comment(_)
            | ClassConstantAccessExpressionClass::TextInterpolation(_)
            | ClassConstantAccessExpressionClass::Error(_) => None,
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        let class_name = self.get_class_name(state)?;

        let constant_name = self.constant.get_name();

        // FIXME handle SomeClass::class-constants

        if let Some(class_data) = state.symbol_data.get_class(&class_name) {
            let classish = class_data.read().unwrap();
            match &*classish {
                ClassType::None => crate::missing_none!(),
                ClassType::Class(cdata) => cdata.constants.get(&constant_name).cloned(),
                ClassType::Interface(_) => crate::missing_none!(),
                ClassType::Trait(_) => crate::missing_none!(),
            }
        } else {
            // FIXME move this emitting to the analysis-pass
            emitter.emit(Issue::UnknownClass(
                self.pos(state),
                class_name.get_fq_name().clone(),
            ));

            None
        }
    }
}
