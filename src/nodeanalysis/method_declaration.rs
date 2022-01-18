use std::{
    sync::{Arc, RwLock},
};


use crate::{
    analysis::state::{AnalysisState, ClassState, FunctionState},
    autonodes::{
        _type::_TypeNode,
        any::AnyNodeRef,
        method_declaration::{MethodDeclarationChildren, MethodDeclarationNode},
        type_list::TypeListChildren,
    },
    issue::{Issue, IssueEmitter},
    phpdoc::types::{PHPDocComment, PHPDocEntry},
    symboldata::{
        class::{ClassMemberVisibility, ClassModifier, ClassName, MethodData},
        FileLocation,
    },
    symbols::Name,
    types::union::{DiscreteType, UnionType},
};

use super::{
    analysis::{AnalyzeableNode, AnalyzeableRoundTwoNode},
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
        if let Some(_TypeNode::TypeList(ret)) = &self.return_type {
            let mut utype = UnionType::new();

            for typed in &*ret.children {
                if let Some(child_type) = match &**typed {
                    TypeListChildren::PrimitiveType(t) => t.get_utype(state, emitter),
                    TypeListChildren::NamedType(named_type) => named_type.get_utype(state, emitter),
                    TypeListChildren::OptionalType(_) => todo!(),
                    _ => None,
                } {
                    utype.merge_into(child_type);
                }
            }
            Some(utype)
        } else {
            None
        }
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
    pub fn get_method_data(&self, state: &mut AnalysisState) -> Option<Arc<RwLock<MethodData>>> {
        let method_name = self.get_declared_name();
        let class = if let Some(c) = &state.in_class {
            c
        } else {
            return None;
        };
        Some(
            state
                .symbol_data
                .get_or_create_method(
                    &class.get_name(),
                    &method_name,
                    FileLocation::new(self.name.pos(state)),
                )
                .clone(),
        )
    }
}

impl AnalyzeableNode for MethodDeclarationNode {
    fn analyze_round_one(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        if state.in_class.is_none() {
            self.analyze_round_one_children(&self.as_any(), state, emitter);
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

        let mut phpdoc_entries = None;
        let mut comment_return_type = None;
        if let Some((doc_comment, range)) = &state.last_doc_comment {
            match PHPDocComment::parse(doc_comment, range) {
                Ok(doc_comment) => {
                    for entry in &doc_comment.entries {
                        match entry {
                            PHPDocEntry::Return(ptype, _desc) => {
                                if method_name == Name::from(b"getAddress" as &[u8]) {
                                    eprintln!("ptype: {:?}", ptype);
                                }
                                comment_return_type =
                                    UnionType::from_parsed_type(ptype.clone(), state, emitter);

                                if method_name == Name::from(b"getAddress" as &[u8]) {
                                    eprintln!("comment_return_type: {:?}", comment_return_type);
                                }
                            }
                            _ => (),
                        }
                    }
                    phpdoc_entries = Some(doc_comment);
                }
                Err(_) => emitter.emit(Issue::PHPDocParseError(
                    self.pos_from_range(state, range.clone()),
                )),
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

        let class = &state
            .in_class
            .as_ref()
            .expect("We checked it at the start of the function");

        let method_data = state.symbol_data.get_or_create_method(
            &class.get_name(),
            &method_name,
            FileLocation::new(self.name.pos(state)),
        );

        // eprintln!("ICK parse method {:?}::{:?}", class.name.get_fq_name(), self.get_declared_name());

        {
            // We scope the locked state to make it as short as possible
            let mut unlocked = method_data.write().unwrap();
            unlocked.name = method_name.clone();
            unlocked.php_return_type = php_return_type;
            unlocked.comment_return_type = comment_return_type;
            unlocked.modifier = modifier;
            unlocked.is_static = is_static;
            unlocked.visibility = visibility;
            unlocked.phpdoc_entries = phpdoc_entries;
        }
        // eprintln!("Tolket metode: {:?}", method_data);
        state.last_doc_comment = None;
        state
            .in_function_stack
            .push(FunctionState::new_method(method_name));
        self.analyze_round_one_children(&self.as_any(), state, emitter);
        state.in_function_stack.pop();
    }
}

impl AnalyzeableRoundTwoNode for MethodDeclarationNode {
    fn analyze_round_two(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        if let Some(ClassState::Interface(_)) = state.in_class {
            // Drop round two-analyse av interfacer-metoder
            return true;
        }

        // Check if the doc-comment-type is valid
        crate::missing!("Den her sjekken ska inn i nye phase 2");
        /*        if let Some((utype, range)) = self.get_comment_declared_return_type(state, emitter) {
            todo!("WHAT");
            for ut in utype.types {
                match ut {
                    DiscreteType::Named(name, fq_name) => {
                        let cname = ClassName::new_with_names(name, fq_name.clone());
                        if state.symbol_data.get_class(&cname).is_none() {
                            let mut pos = self.pos(state);
                            pos.range = range;
                            emitter.emit(Issue::UnknownClass(pos, fq_name));
                        }
                    }
                    _ => {}
                }
            }
        }*/

        let function = FunctionState::new_method(self.get_declared_name());
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

        if !self.analyze_round_two_children(&self.as_any(), state, emitter, path) {
            return false;
        }

        let func = state
            .in_function_stack
            .pop()
            .expect("There must be a state");
        let returns = func.returns.read().unwrap().clone();

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
                return true;
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
            (*method_data).inferred_return_type = Some(ret_type);
        }
        true
    }
}
