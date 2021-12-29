use crate::{
    autonodes::any::AnyNodeRef,
    autotree::NodeAccess,
    issue::{Issue, VoidEmitter},
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

use super::analysis::AnalyzeableRoundTwoNode;

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

        let method_name = self.name.get_method_name()?;

        let class_names = self.get_object_class_name(state, emitter)?;

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
                .comment_return_type
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

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        None
        //         crate::missing_none!("{}.get_php_value(..)", self.kind())
    }

    /// Return if the object has one single type
    fn get_object_class_name(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<Vec<Option<ClassName>>> {
        let object_utype = self.object.get_utype(state, emitter)?;

        Some(
            object_utype
                .types
                .iter()
                .map(|object_type| match &object_type {
                    DiscreteType::Named(lname, fq_name) => {
                        let cname = ClassName::new_with_names(lname.clone(), fq_name.clone());
                        Some(cname)
                    }
                    _ => None,
                })
                .collect(),
        )
    }

    pub fn get_symbols(&self, state: &mut AnalysisState) -> Option<Vec<Symbol>> {
        let cnames = self.get_object_class_name(state, &VoidEmitter::new())?;
        let mut symbols: Vec<_> = vec![];
        for cname in cnames {
            let cname = cname?;
            let name = self.name.get_method_name()?;
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
    pub fn get_method_name(&self) -> Option<Name> {
        match self {
            MemberCallExpressionName::_Expression(_) => crate::missing_none!(),
            MemberCallExpressionName::DynamicVariableName(_) => crate::missing_none!(),
            MemberCallExpressionName::Name(n) => Some(n.get_name()),
            MemberCallExpressionName::VariableName(_) => crate::missing_none!(),

            MemberCallExpressionName::Comment(_)
            | MemberCallExpressionName::TextInterpolation(_)
            | MemberCallExpressionName::Error(_) => None,
        }
    }
}

impl AnalyzeableRoundTwoNode for MemberCallExpressionNode {
    fn analyze_round_two(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        self.object.read_from(state, emitter);
        self.name.read_from(state, emitter);
        if !self.analyze_round_two_children(&self.as_any(), state, emitter, path) {
            return false;
        }

        if let Some(method_name) = self.name.get_method_name() {
            if let Some(cnames) = self.get_object_class_name(state, emitter) {
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
                        }
                    } else {
                        emitter.emit(Issue::MethodCallOnUnknownType(
                            self.object.pos(state),
                            Some(method_name.clone()),
                        ));
                    }
                }
            } else {
                emitter.emit(Issue::MethodCallOnUnknownType(
                    self.object.pos(state),
                    Some(method_name),
                ));
            }
        }

        true
    }
}
