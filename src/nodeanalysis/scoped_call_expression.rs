use crate::{
    analysis::state::AnalysisState,
    autonodes::scoped_call_expression::{
        ScopedCallExpressionName, ScopedCallExpressionNode, ScopedCallExpressionScope,
    },
    issue::{IssueEmitter, VoidEmitter},
    symboldata::class::{ClassName, MethodData},
    symbols::{Name, SymbolClass, SymbolMethod},
    types::union::{DiscreteType, SpecialType, UnionType},
    value::PHPValue,
};

///
/// Static method call
impl ScopedCallExpressionNode {
    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        self.scope.read_from(state, emitter);
        self.name.read_from(state, emitter);
        self.arguments.read_from(state, emitter);
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        // Method calls are very unlikely to return a static value
        // but a static method call might be able to?
        None
        //         crate::missing_none!("{}.get_php_value(..)", self.kind())
    }

    pub fn get_class_name(&self, state: &mut AnalysisState) -> Option<ClassName> {
        match &*self.scope {
            ScopedCallExpressionScope::ArrayCreationExpression(_) => {
                crate::missing_none!("{}", self.kind())
            }
            ScopedCallExpressionScope::CastExpression(_) => crate::missing_none!("{}", self.kind()),
            ScopedCallExpressionScope::ClassConstantAccessExpression(_) => {
                crate::missing_none!("{}", self.kind())
            }
            ScopedCallExpressionScope::DynamicVariableName(_) => {
                crate::missing_none!("{}", self.kind())
            }
            ScopedCallExpressionScope::EncapsedString(_) => crate::missing_none!("{}", self.kind()),
            ScopedCallExpressionScope::FunctionCallExpression(_) => {
                crate::missing_none!("{}", self.kind())
            }
            ScopedCallExpressionScope::Heredoc(_) => crate::missing_none!("{}", self.kind()),
            ScopedCallExpressionScope::MemberAccessExpression(_) => {
                crate::missing_none!("{}", self.kind())
            }
            ScopedCallExpressionScope::MemberCallExpression(_) => {
                crate::missing_none!("{}", self.kind())
            }
            ScopedCallExpressionScope::Name(name) => {
                Some(ClassName::new_with_analysis_state(&name.get_name(), state))
            }
            ScopedCallExpressionScope::NullsafeMemberAccessExpression(_) => {
                crate::missing_none!("{}", self.kind())
            }
            ScopedCallExpressionScope::NullsafeMemberCallExpression(_) => {
                crate::missing_none!("{}", self.kind())
            }
            ScopedCallExpressionScope::ParenthesizedExpression(_) => {
                crate::missing_none!("{}", self.kind())
            }
            ScopedCallExpressionScope::QualifiedName(qname) => Some(ClassName::new_with_names(
                qname.get_name(),
                qname.get_fq_name(state),
            )),
            ScopedCallExpressionScope::RelativeScope(_) => crate::missing_none!("{}", self.kind()),
            ScopedCallExpressionScope::ScopedCallExpression(sc) => {
                if let Some(utype) = sc.get_utype(state, &VoidEmitter::new()) {
                    match utype.single_type() {
                        Some(DiscreteType::Named(_, fq_name)) => {
                            return Some(ClassName::new_with_fq_name(fq_name));
                        }
                        _ => (),
                    }
                }

                crate::missing_none!("{}", self.kind())
            }
            ScopedCallExpressionScope::ScopedPropertyAccessExpression(_) => {
                crate::missing_none!("{}", self.kind())
            }
            ScopedCallExpressionScope::String(_) => crate::missing_none!("{}", self.kind()),
            ScopedCallExpressionScope::SubscriptExpression(_) => {
                crate::missing_none!("{}", self.kind())
            }
            ScopedCallExpressionScope::VariableName(_) => crate::missing_none!("{}", self.kind()),
            ScopedCallExpressionScope::Nowdoc(_) => crate::missing_none!("{}", self.kind()),

            ScopedCallExpressionScope::Comment(_)
            | ScopedCallExpressionScope::TextInterpolation(_)
            | ScopedCallExpressionScope::Error(_) => None,
        }
    }

    pub fn get_method_name(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<Name> {
        match &*self.name {
            ScopedCallExpressionName::_Expression(_) => {
                crate::missing_none!("{}.get_method_name(..)", self.kind())
            }
            ScopedCallExpressionName::DynamicVariableName(_) => {
                crate::missing_none!("{}.get_method_name(..)", self.kind())
            }
            ScopedCallExpressionName::Name(name) => Some(name.get_name()),
            ScopedCallExpressionName::VariableName(vn) => {
                if let Some(val) = vn.get_php_value(state, emitter) {
                    match val {
                        PHPValue::String(name) => return Some(Name::from(name)),
                        _ => {
                            return crate::missing_none!(
                                "{}.get_method_name(..) find method name from a {:?}",
                                self.kind(),
                                val.get_utype()
                            )
                        }
                    }
                }
                crate::missing_none!("{}.get_method_name(..)", self.kind())
            }
            ScopedCallExpressionName::Comment(_)
            | ScopedCallExpressionName::TextInterpolation(_)
            | ScopedCallExpressionName::Error(_) => None,
        }
    }

    pub fn get_method_data(&self, state: &mut AnalysisState) -> Option<(ClassName, MethodData)> {
        let class_name = self.get_class_name(state)?;

        let class_data_handle = state.symbol_data.get_class(&class_name)?;

        let method_name = self.get_method_name(state, &VoidEmitter::new())?;

        let noe = class_data_handle.read().unwrap();

        if let Some(mdata) = noe.get_method(&method_name, state.symbol_data.clone()) {
            Some((class_name, mdata))
        } else {
            None
        }
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        let (class_name, method_data) = self.get_method_data(state)?;

        // FIXME sjekk etter `static`, og konverter til fq_class_name
        let utype = if let Some(t) = method_data.inferred_return_type {
            t
        } else if let Some(c) = method_data.comment_return_type {
            c.0
        } else if let Some(t) = method_data.php_return_type {
            t
        } else {
            return None;
        };
        let mut ret_type = UnionType::new();
        for ut in utype.types {
            match ut {
                DiscreteType::Special(SpecialType::Static) => {
                    // Find static called class
                    ret_type.push(DiscreteType::Named(
                        class_name.get_name().clone(),
                        class_name.get_fq_name().clone(),
                    ))
                }
                _ => ret_type.push(ut),
            }
        }
        Some(ret_type)
    }

    pub fn get_method_symbol(&self, state: &mut AnalysisState) -> Option<SymbolMethod> {
        // FIXME should this check for existence, or just return symbol?

        let cname = self.get_class_name(state)?;
        let mname = self.get_method_name(state, &VoidEmitter::new())?;

        let locked_cdata = state.symbol_data.get_class(&cname)?;

        let cdata = locked_cdata.read().ok()?;
        cdata.get_method(&mname, state.symbol_data.clone())?;

        Some(SymbolMethod::new(mname, SymbolClass::new_from_cname(cname)))
    }
}
