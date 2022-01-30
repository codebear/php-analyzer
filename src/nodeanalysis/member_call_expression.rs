use crate::{
    autonodes::any::AnyNodeRef,
    autotree::NodeAccess,
    issue::{Issue, VoidEmitter},
    symboldata::class::MethodData,
    symbols::{Name, Symbol, SymbolClass, SymbolMethod},
};

use crate::{
    analysis::state::AnalysisState,
    autonodes::member_call_expression::{MemberCallExpressionName, MemberCallExpressionNode},
    issue::IssueEmitter,
    symboldata::class::ClassName,
    types::union::{DiscreteType, UnionType},
    value::PHPValue,
};

use super::analysis::ThirdPassAnalyzeableNode;

impl MemberCallExpressionNode {
    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        self.object.read_from(state, emitter);
        self.name.read_from(state, emitter);

        // Mark any arguments as read from
        self.arguments.read_from(state, emitter);
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        // FIXME
        // Find out what the return-type of the method is
        // If the method is marked for overload-analysis, and we
        // have concrete argument values, register them with method-data for analysis in next pass

        let method_name = self.name.get_method_name(state)?;

        let class_names = self.get_object_class_names(state, emitter)?;

        let mut types = UnionType::new();

        for class_name in class_names {
            // If we have one missing class-type, abandon generating a result
            let class_name = class_name?;

            let cdata_handle = state.symbol_data.get_class(&class_name)?;

            let method_data = if let Some(md) = {
                let unlocked = cdata_handle.read().unwrap();
                unlocked.get_method(&method_name, state.symbol_data.clone())
            } {
                md
            } else {
                // FIXME This should be emittet in an analyze-three-pass, and not here

                return None;
            };
            let call_return_type = method_data
                .comment_return_type.map(|x|x.0)
                .or(method_data.php_return_type)
                .or(method_data.inferred_return_type)?;

            types.merge_into(call_return_type);
        }
        if types.len() > 0 {
            Some(types)
        } else {
            None
        }
    }

    pub fn get_methods_data(
        &self,
        state: &mut AnalysisState,
    ) -> Vec<Option<(ClassName, MethodData)>> {
        let mut methods = vec![];

        let method_name = if let Some(mname) = self.name.get_method_name(state) {
            mname
        } else {
            return methods;
        };

        let class_names =
            if let Some(cnames) = self.get_object_class_names(state, &VoidEmitter::new()) {
                cnames
            } else {
                return methods;
            };

        for class_name in class_names {
            // If we have one missing class-type, abandon generating a result
            let class_name = if let Some(cname) = class_name {
                cname
            } else {
                methods.push(None);
                continue;
            };

            let cdata_handle = if let Some(ch) = state.symbol_data.get_class(&class_name) {
                ch
            } else {
                methods.push(None);
                continue;
            };

            let method_data = if let Some(md) = {
                let unlocked = cdata_handle.read().unwrap();
                unlocked.get_method(&method_name, state.symbol_data.clone())
            } {
                md
            } else {
                methods.push(None);
                continue;
            };

            methods.push(Some((class_name, method_data)));
        }

        methods
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        None
        //         crate::missing_none!("{}.get_php_value(..)", self.kind())
    }

    fn get_class_name_from_discrete_type(&self, dtype: &DiscreteType) -> Option<ClassName> {
        match dtype {
            DiscreteType::Named(lname, fq_name) => {
                //eprintln!("1. Calling method on type: {}", fq_name);
                let cname = ClassName::new_with_names(lname.clone(), fq_name.clone());
                Some(cname)
            }
            DiscreteType::Generic(btype, _) => {
                crate::missing!("Trying to get class-name from a generic type which a method is being called on, for now ignoring generic arguments");
                self.get_class_name_from_discrete_type(&**btype)
            }
            DiscreteType::NULL => None,
            _ => {
                crate::missing_none!("Trying to get class-name from a {} which a method is being called on", dtype)
                //eprintln!("2. Calling method on type: {}", t);
//                    None
            }
        }
    }

    /// Return if the object has one single type
    fn get_object_class_names(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<Vec<Option<ClassName>>> {
        let object_utype = self.object.get_utype(state, emitter)?;

        let mut cnames = vec![];
        for dtype in object_utype.types {
            if let DiscreteType::NULL = dtype {
                continue;
            }
            let class_name = self.get_class_name_from_discrete_type(&dtype);
            cnames.push(class_name);
        }
        Some(cnames)
    }

    pub fn get_symbols(&self, state: &mut AnalysisState) -> Option<Vec<Symbol>> {
        let cnames = self.get_object_class_names(state, &VoidEmitter::new())?;
        let mut symbols: Vec<_> = vec![];
        for cname in cnames {
            let cname = cname?;
            let name = self.name.get_method_name(state)?;
            let class = SymbolClass {
                name: cname.name.clone(),
                ns: cname.get_namespace(),
            };
            symbols.push(Symbol::Method(SymbolMethod { name, class }))
        }
        Some(symbols)
    }
}

impl MemberCallExpressionName {
    pub fn get_method_name(&self, state: &mut AnalysisState) -> Option<Name> {
        match self {
            MemberCallExpressionName::_Expression(_) => crate::missing_none!(),
            MemberCallExpressionName::DynamicVariableName(_) => crate::missing_none!(),
            MemberCallExpressionName::Name(n) => Some(n.get_name()),
            MemberCallExpressionName::VariableName(vname) => {
                let noe = vname.get_php_value(state, &VoidEmitter::new())?;
                if let PHPValue::String(s) = noe {
                    let name = Name::from(s);
                    Some(name)
                } else {
                    crate::missing_none!(
                        "Hente ut metode-navn fra noe som ikke er en PHPValue::String(..)?"
                    )
                }
            }

            MemberCallExpressionName::Comment(_)
            | MemberCallExpressionName::TextInterpolation(_)
            | MemberCallExpressionName::Error(_) => None,
        }
    }
}

impl ThirdPassAnalyzeableNode for MemberCallExpressionNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        self.object.read_from(state, emitter);
        self.name.read_from(state, emitter);
        if !self.analyze_third_pass_children(&self.as_any(), state, emitter, path) {
            return false;
        }

        let maybe_method_name = self.name.get_method_name(state);

        if let Some(object_utype) = self.object.get_utype(state, emitter) {
            if object_utype.is_nullable() {
                emitter.emit(Issue::MethodCallOnNullableType(
                    self.object.pos(state),
                    maybe_method_name.clone(),
                ));
            }
        }

        if let Some(method_name) = maybe_method_name {
            if let Some(cnames) = self.get_object_class_names(state, emitter) {
                for cname in cnames {
                    if let Some(cname) = cname {
                        if let Some(cdata_handle) = state.symbol_data.get_class(&cname) {
                            let cdata = cdata_handle.read().unwrap();
                            if cdata
                                .get_method(&method_name, state.symbol_data.clone())
                                .is_none()
                            {
                                let fq_cname = cdata.get_fq_name();
                                emitter.emit(Issue::UnknownMethod(
                                    self.name.pos(state),
                                    fq_cname,
                                    method_name.clone(),
                                ));
                            }
                        } else {
                            emitter.emit(Issue::MethodCallOnUnknownType(
                                self.object.pos(state),
                                Some(cname.get_fq_name().clone()),
                                Some(method_name.clone()),
                            ));
                        }
                    } else {
                        emitter.emit(Issue::MethodCallOnUnknownType(
                            self.object.pos(state),
                            None,
                            Some(method_name.clone()),
                        ));
                    }
                }
            } else {
                emitter.emit(Issue::MethodCallOnUnknownType(
                    self.object.pos(state),
                    None,
                    Some(method_name),
                ));
            }
        }

        true
    }
}
