use std::os::unix::prelude::OsStrExt;

use crate::autotree::NodeAccess;
use crate::issue::{Issue, VoidEmitter};
use crate::symbols::Name;
use crate::types::union::{DiscreteType, SpecialType};
use crate::{
    analysis::state::AnalysisState,
    autonodes::class_constant_access_expression::{
        ClassConstantAccessExpressionClass, ClassConstantAccessExpressionNode,
    },
    issue::IssueEmitter,
    symboldata::class::ClassName,
    types::union::UnionType,
    value::PHPValue,
};

use super::analysis::SecondPassAnalyzeableNode;

impl ClassConstantAccessExpressionNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        // FIXME marker en konstant som brukt
        ()
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        // this should return a `class-string<_>` type if
        // it is a ::class-constant-reference
        let constant_name = self.constant.get_name();

        if constant_name == b"class" as &[u8] {
            return Some(
                if let Some(cname) = self.get_class_name(state) {
                    DiscreteType::Special(SpecialType::ClassString(Some(
                        cname.get_fq_name().clone(),
                    )))
                } else {
                    DiscreteType::String
                }
                .into(),
            );
        }

        if let Some(val) = self.get_php_value(state, emitter) {
            val.get_utype()
        } else {
            crate::missing_none!("{}.get_utype(..)", self.kind())
        }
    }

    pub fn get_class_name(&self, state: &mut AnalysisState) -> Option<ClassName> {
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
                let fq_name = n.get_fq_name(state);
                Some(ClassName::new_with_names(
                    fq_name.get_name().unwrap_or_else(|| Name::new()),
                    fq_name,
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
            ClassConstantAccessExpressionClass::VariableName(n) => {
                // The variable in question might have a known type of class-string<Something>-type
                // or actually, even a union of multiple class-string<Foo>|class-string<Bar>-types
                // and then we will and should be able to determine if the class-constant-access is valid

                if let Some(t) = n.get_utype(state, &VoidEmitter::new()) {
                    todo!("Fant {:?}", t);
                };

                crate::missing_none!(
                    "Hvordan hente en verdi ut av {:?} at {}",
                    n,
                    state.pos_as_string(self.range())
                )
            }
            ClassConstantAccessExpressionClass::Nowdoc(n) => crate::missing_none!(
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
        _emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        let class_name = self.get_class_name(state)?;

        let constant_name = self.constant.get_name();

        if constant_name == b"class" as &[u8] {
            return Some(PHPValue::String(class_name.to_os_string()));
        }

        // FIXME handle SomeClass::class-constants

        let class_data = state.symbol_data.get_class(&class_name)?;
        let classish = class_data.read().unwrap();

        classish.get_constant_value(&state.symbol_data, &constant_name)
    }
}

impl SecondPassAnalyzeableNode for ClassConstantAccessExpressionNode {
    fn analyze_second_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        if let Some(class_name) = self.get_class_name(state) {
            let constant_name = self.constant.get_name();

            // FIXME handle SomeClass::class-constants

            if let Some(class_data) = state.symbol_data.get_class(&class_name) {
                if constant_name == b"class" as &[u8] {
                    return;
                }

                let classish = class_data.read().unwrap();
                let const_val = classish.get_constant_value(&state.symbol_data, &constant_name);

                if let None = const_val {
                    /*eprintln!(
                        "BALLE Unknown class constant {:?}::{:?}",
                        class_name, constant_name
                    );*/
                    emitter.emit(Issue::UnknownClassConstant(
                        self.pos(state),
                        class_name.get_fq_name().clone(),
                        constant_name,
                    ));
                }
            } else {
                // FIXME move this emitting to the analysis-pass
                emitter.emit(Issue::UnknownClass(
                    self.pos(state),
                    class_name.get_fq_name().clone(),
                ));

                /*
                let fqname: String = format!("{}", class_name.get_fq_name());

                eprintln!(
                    "BALLE Unknown class {}, {:?}",
                    fqname,
                    class_name.get_fq_name()
                );*/
            }
        }
        self.analyze_second_pass_children(&self.as_any(), state, emitter);
        //state.in_class = None;
    }
}
