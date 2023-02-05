use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    analysis::state::{AnalysisState, ClassState, FunctionState},
    autonodes::{
        any::AnyNodeRef,
        method_declaration::{MethodDeclarationChildren, MethodDeclarationNode},
    },
    issue::{Issue, IssueEmitter},
    phpdoc::types::{PHPDocComment, PHPDocEntry},
    symboldata::{
        class::{ClassMemberVisibility, ClassModifier, ClassName, MethodData},
        FileLocation,
    },
    symbols::Name,
    types::union::{DiscreteType, SpecialType, UnionType},
};

use super::{
    analysis::{FirstPassAnalyzeableNode, SecondPassAnalyzeableNode, ThirdPassAnalyzeableNode},
    class::AnalysisOfDeclaredNameNode,
};
use crate::autotree::NodeAccess;

use crate::analysis::scope::BranchableScope;

impl MethodDeclarationNode {
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
///
/// METHODS
///
///
impl AnalysisOfDeclaredNameNode for MethodDeclarationNode {
    fn get_declared_name(&self) -> Name {
        self.name.get_name()
    }
}

trait AnalysisOfFunctionLike {
    fn get_php_declared_return_type(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType>;

    fn get_inferred_return_type(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType>;

    // Her kan vi lag en get_overloaded_inferred_return_type_with_arguments(...) for overload-sjekking
}

impl AnalysisOfFunctionLike for MethodDeclarationNode {
    fn get_php_declared_return_type(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        let ret = &self.return_type.as_ref()?;
        let utype = ret.get_utype(state, emitter)?;
        let utype: UnionType = utype
            .types
            .iter()
            .map(|x| match x {
                DiscreteType::Special(SpecialType::Self_) => {
                    let _cname = self.get_class_name(state);
                    todo!()
                }
                x @ _ => x,
            })
            .collect();
        Some(utype)
    }

    fn get_inferred_return_type(
        &self,
        _state: &mut AnalysisState,
        _: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        todo!()
    }
}

impl MethodDeclarationNode {
    pub fn get_class_name(&self, state: &mut AnalysisState) -> Option<ClassName> {
        state.in_class.as_ref().map(|x| x.get_name())
    }

    pub fn get_method_data(&self, state: &mut AnalysisState) -> Option<Arc<RwLock<MethodData>>> {
        let method_name = self.get_declared_name();

        let class = self.get_class_name(state)?;

        Some(
            state
                .symbol_data
                .get_or_create_method(
                    &class,
                    &method_name,
                    FileLocation::new(self.name.pos(state)),
                )
                .clone(),
        )
    }

    fn get_doc_comment_declared_templates(
        &self,
        doc_comment: &PHPDocComment,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Vec<Name> {
        let mut method_template_params = vec![];

        for entry in &doc_comment.entries {
            match entry {
                PHPDocEntry::Template(range, t, _) => {
                    let generic_templates =
                        state.get_generic_templates(Some(&method_template_params));
                    let temp_name: Name = t.into();
                    if let Some(gen) = generic_templates {
                        if gen.contains(&temp_name) {
                            emitter.emit(Issue::DuplicateTemplate(
                                state.pos_from_range(range.clone()),
                                temp_name,
                            ))
                        } else {
                            // eprintln!("fant 1 {}", temp_name);
                            method_template_params.push(temp_name);
                        }
                    } else {
                        // eprintln!("fant 2 {}", temp_name);
                        method_template_params.push(temp_name);
                    }
                }
                _ => (),
            }
        }
        method_template_params
    }
}

impl FirstPassAnalyzeableNode for MethodDeclarationNode {
    fn analyze_first_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        //eprintln!("TEMPLATES: {:?}", state.get_generic_templates());
        if state.in_class.is_none() {
            self.analyze_first_pass_children(&self.as_any(), state, emitter);
            emitter.emit(Issue::ParseAnomaly(
                self.pos(state),
                "Got method declaration, while not in a class".into(),
            ));
            return;
        };
        let method_name = self.get_declared_name();
        let php_return_type = self.get_php_declared_return_type(state, emitter);

        let mut modifier = ClassModifier::None;
        let mut is_static = false;
        let mut visibility = ClassMemberVisibility::Public;
        let mut phpdoc = None;
        let mut comment_return_type = None;
        let mut param_map = HashMap::new();
        let mut method_template_params = vec![];
        if let Some((doc_comment, range)) = &state.last_doc_comment {
            match PHPDocComment::parse(doc_comment, range) {
                Ok(doc_comment) => {
                    // First check for templates
                    method_template_params =
                        self.get_doc_comment_declared_templates(&doc_comment, state, emitter);
                    // TODO type-parsing expects generic-information in FunctionData in state...

                    // Then check the remaining entries
                    for entry in &doc_comment.entries {
                        match entry {
                            PHPDocEntry::Return(range, ptype, _desc) => {
                                comment_return_type = UnionType::from_parsed_type(
                                    ptype.clone(),
                                    state,
                                    emitter,
                                    Some(&method_template_params),
                                )
                                .map(|x| (x, range.clone()));
                            }
                            PHPDocEntry::Param(range, vtype, osstr_name, desc) => {
                                if let Some(osstr_name) = osstr_name {
                                    let name: Name = osstr_name.into();
                                    if param_map.contains_key(&name) {
                                        emitter.emit(Issue::InvalidPHPDocEntry(
                                            state.pos_from_range(range.clone()),
                                            "Duplicate @param-entry".into(),
                                        ))
                                    } else {
                                        param_map.insert(name, entry.clone());
                                    }
                                } else {
                                    emitter.emit(Issue::InvalidPHPDocEntry(
                                        state.pos_from_range(range.clone()),
                                        format!(
                                            "@param-entry is missing $param-name, [{:?}] [{:?}] [{:?}]",
                                            vtype, osstr_name, desc
                                        )
                                        .into(),
                                    ))
                                }
                            }

                            PHPDocEntry::Var(range, _, _, _) => {
                                emitter.emit(Issue::MisplacedPHPDocEntry(
                                    state.pos_from_range(range.clone()),
                                    "@var can't be used on a method-declaration".into(),
                                ));
                            }
                            _ => (),
                        }
                    }
                    phpdoc = Some(doc_comment);
                }
                Err(_) => {
                    emitter.emit(Issue::PHPDocParseError(state.pos_from_range(range.clone())))
                }
            }
        }

        for child in &self.children {
            match &**child {
                MethodDeclarationChildren::AbstractModifier(_) => {
                    modifier = ClassModifier::Abstract
                }
                MethodDeclarationChildren::FinalModifier(_) => modifier = ClassModifier::Final,
                MethodDeclarationChildren::StaticModifier(_) => is_static = true,
                MethodDeclarationChildren::VisibilityModifier(v) => visibility = v.get_visibility(),
                MethodDeclarationChildren::VarModifier(v) => {
                    todo!("HWA I ALLE DAGER ER DETTE: {:?}", v);
                }
                _ => continue,
            }
        }

        let method_data = self.get_method_data(state).unwrap();
        let arguments = self.parameters.analyze_first_pass_parameters(
            state,
            emitter,
            &param_map,
            Some(&method_template_params),
        );

        {
            // We scope the locked state to make it as short as possible
            let mut unlocked = method_data.write().unwrap();
            unlocked.name = method_name.clone();
            unlocked.php_return_type = php_return_type;
            unlocked.comment_return_type = comment_return_type;
            unlocked.modifier = modifier;
            unlocked.is_static = is_static;
            unlocked.visibility = visibility;
            unlocked.phpdoc = phpdoc;
            unlocked.arguments = arguments;
            unlocked.generic_templates = Some(method_template_params);
        }

        // eprintln!("Tolket metode: {:?}", method_data);
        state.last_doc_comment = None;
        state
            .in_function_stack
            .push(FunctionState::new_method(method_name, method_data));
        self.analyze_first_pass_children(&self.as_any(), state, emitter);
        state.in_function_stack.pop();
    }
}

impl SecondPassAnalyzeableNode for MethodDeclarationNode {
    fn analyze_second_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        // Check types used in phpdoc
        let locked_data = self.get_method_data(state).unwrap();
        {
            let method_data = locked_data.read().unwrap();
            if let Some(phpdoc) = &method_data.phpdoc {
                // First check for templates
                let method_template_params_list =
                    self.get_doc_comment_declared_templates(&phpdoc, state, emitter);
                let method_template_params = if method_template_params_list.len() > 0 {
                    Some(&method_template_params_list)
                } else {
                    None
                };

                for entry in &phpdoc.entries {
                    let (range, concrete_types) = match entry {
                        PHPDocEntry::Param(range, ptype, _pname, _pdesc) => (range, ptype),
                        PHPDocEntry::Return(range, rtype, _rdesc) => (range, rtype),
                        _ => continue,
                    };

                    if let Some(utype) = UnionType::from_parsed_type(
                        concrete_types.clone(),
                        state,
                        emitter,
                        method_template_params,
                    ) {
                        utype.ensure_valid(state, emitter, range, true);
                    } else {
                        emitter.emit(Issue::InvalidPHPDocEntry(
                            state.pos_from_range(range.clone()),
                            "Invalid type".into(),
                        ));
                    }
                }
            }
            if let Some((utype, range)) = &method_data.comment_return_type {
                utype.ensure_valid(state, emitter, range, true);
            }
        }
        let function = FunctionState::new_method(self.get_declared_name(), locked_data);
        state.in_function_stack.push(function);

        self.analyze_second_pass_children(&self.as_any(), state, emitter);

        let _func = state
            .in_function_stack
            .pop()
            .expect("There must be a state");
    }
}

impl ThirdPassAnalyzeableNode for MethodDeclarationNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        let _pdata = match &state.in_class {
            Some(ClassState::Interface(_, _)) => {
                // Drop third-pass-analyse av interfacer-metoder
                return true;
            }
            Some(ClassState::Class(_, cdata)) => cdata,
            Some(ClassState::Trait(_, tdata)) => tdata,
            None => {
                emitter.emit(Issue::ParseAnomaly(
                    self.pos(state),
                    "Missing class/trait/interface-data".into(),
                ));
                return true;
            }
        };

        let locked_data = self.get_method_data(state).unwrap();

        let function = FunctionState::new_method(self.get_declared_name(), locked_data);
        state.in_function_stack.push(function);

        {
            let scope_h = state.current_scope();
            let mut scope = scope_h.write().unwrap();
            let this_data_lock = scope.get_or_create_var(Name::from("this"));
            let mut this_data = this_data_lock.write().unwrap();
            if let Some(cstate) = &state.in_class {
                this_data.php_declared_type = Some(
                    DiscreteType::Named(
                        cstate.get_name().name.clone(),
                        cstate.get_name().fq_name.clone(),
                    )
                    .into(),
                );
            } else {
                emitter.emit(Issue::ParseAnomaly(
                    self.pos(state),
                    "method declaration without class".into(),
                ));
            }
            this_data.written_to += 1;
            this_data.read_from += 1;
        }

        if !self.analyze_third_pass_children(&self.as_any(), state, emitter, path) {
            return false;
        }

        let func = state
            .in_function_stack
            .pop()
            .expect("There must be a state");

        let returns = func.returns.read().unwrap().clone();
        let return_count = returns.len();
        let scope_handle = func.scope_stack.read().unwrap().top();
        scope_handle.analyze_for_unused_vars(state, emitter);

        let mut ret_type = UnionType::new();
        //        let mut ret_value = HashSet::new();
        //        let mut missing_value = false;
        // Handle returns
        // eprintln!("Returns: {:?}", &returns);
        for (r_type, _val) in returns {
            if let Some(t) = r_type {
                // t;
                ret_type.merge_into(t);
            } else {
                ret_type = UnionType::new();
                break;
                //                 return true;
            }
            /*             if let Some(x) = val {
                ret_value.insert(x);
            } else {
                missing_value = true;
            }*/
            //             if ret_type
        }

        /*let val = if missing_value {
            None
        } else if ret_value.len() == 1 {
            ret_value.iter().next().cloned()
        } else {
            None
        };*/
        if let Some(method) = self.get_method_data(state) {
            let mut method_data = method.write().unwrap();
            method_data.return_count = return_count;
            if ret_type.len() > 0 {
                method_data.inferred_return_type = Some(ret_type);
            }
        }
        true
    }
}
