use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

//use tree_sitter::Range;
use crate::parser::Range;
use crate::{
    analysis::scope::BranchableScope,
    autonodes::any::AnyNodeRef,
    extra::ExtraChild,
    issue::VoidEmitter,
    phpdoc::types::{PHPDocComment, PHPDocEntry},
    symboldata::FileLocation,
    symbols::{FullyQualifiedName, Name},
};
use crate::{
    analysis::state::{AnalysisState, FunctionState},
    autonodes::function_definition::FunctionDefinitionNode,
    issue::{Issue, IssueEmitter},
    symboldata::FunctionData,
    types::union::UnionType,
    value::PHPValue,
};

use super::analysis::{FirstPassAnalyzeableNode, ThirdPassAnalyzeableNode};

use crate::autotree::NodeAccess;

impl FunctionDefinitionNode {
    fn get_function_name(
        &self,
        state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> FullyQualifiedName {
        state.get_fq_function_name(self.name.get_name())
    }

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

    pub fn get_function_data(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<Arc<RwLock<FunctionData>>> {
        let fname = self.get_function_name(state, emitter);
        let read = state.symbol_data.functions.read().unwrap();
        read.get(&fname.to_ascii_lowercase()).cloned()
    }

    fn get_php_declared_return_type(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        let ret = &self.return_type.as_ref()?;
        ret.get_utype(state, emitter)
    }

    fn get_inline_phpdoc_return_type(
        &self,
        state: &mut AnalysisState,
    ) -> Option<(UnionType, Range)> {
        let arg_range = self.parameters.range;
        let statement_range = self.body.range;

        let comments: Vec<_> = self
            .extras
            .iter()
            .filter_map(|x| {
                if let ExtraChild::Comment(c) = &**x {
                    Some(c.clone())
                } else {
                    None
                }
            })
            .collect();

        let comment = comments.first()?;

        if comment.range.start_byte <= arg_range.end_byte {
            return None;
        }
        if comment.range.end_byte >= statement_range.start_byte {
            return None;
        }

        let emitter = VoidEmitter::new();

        PHPDocComment::parse_inline_return_type(&comment.get_raw(), &comment.range, state, &emitter)
    }
}

impl FirstPassAnalyzeableNode for FunctionDefinitionNode {
    fn analyze_first_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        let fname = self.get_function_name(state, emitter);

        let mut is_dup = false;
        {
            let read = state.symbol_data.functions.read().unwrap();
            if let Some(_) = read.get(&fname.to_ascii_lowercase()) {
                emitter.emit(Issue::DuplicateFunction(self.pos(state), fname.clone()));
                is_dup = true;
            }
        }

        let mut comment_return_type = None;
        let mut param_map = HashMap::new();
        //let mut phpdoc = None;
        let mut function_template_params = vec![];
        if let Some((doc_comment, range)) = &state.last_doc_comment {
            match PHPDocComment::parse(doc_comment, range) {
                Ok(doc_comment) => {
                    for entry in &doc_comment.entries {
                        match entry {
                            PHPDocEntry::Return(range, ptype, _desc) => {
                                comment_return_type = UnionType::from_parsed_type(
                                    ptype.clone(),
                                    state,
                                    emitter,
                                    Some(&function_template_params),
                                )
                                .map(|x| (x, range.clone()));
                            }
                            PHPDocEntry::Param(_, _, osstr_name, _) => {
                                if let Some(osstr_name) = osstr_name {
                                    let name = osstr_name.into();
                                    if param_map.contains_key(&name) {
                                        crate::missing!("Emit duplicate phpdoc param name");
                                    } else {
                                        param_map.insert(name, entry.clone());
                                    }
                                } else {
                                    crate::missing!("Emit phpdoc param without name");
                                }
                            }
                            PHPDocEntry::Var(range, _, _, _) => {
                                emitter.emit(Issue::MisplacedPHPDocEntry(
                                    state.pos_from_range(range.clone()),
                                    "@var can't be used on a function-declaration".into(),
                                ));
                            }
                            PHPDocEntry::Template(_range, t, _desc) => {
                                let temp_name: Name = t.into();
                                function_template_params.push(temp_name);
                            }
                            _ => (),
                        }
                    }
                    //phpdoc = Some(doc_comment);
                }
                Err(_) => {
                    emitter.emit(Issue::PHPDocParseError(state.pos_from_range(range.clone())))
                }
            }
        }

        if let None = comment_return_type {
            comment_return_type = self.get_inline_phpdoc_return_type(state);
        }

        let php_return_type = self.get_php_declared_return_type(state, emitter);

        let arguments = self.parameters.analyze_first_pass_parameters(
            state,
            emitter,
            &param_map,
            Some(&function_template_params),
        );

        let mut maybe_fdata = None;
        if !is_dup {
            let mut write = state.symbol_data.functions.write().unwrap();
            if let Some(_) = write.get(&fname.to_ascii_lowercase()) {
                // Someone beat us to it
                emitter.emit(Issue::DuplicateFunction(self.pos(state), fname.clone()));
            } else {
                let position = FileLocation::new(self.name.pos(state));
                let fdata = Arc::new(RwLock::new(FunctionData {
                    name: fname.clone(),
                    position,
                    php_return_type,
                    comment_return_type,
                    inferred_return_type: None,
                    arguments,
                    variadic: false,
                    pure: false,
                    deterministic: false,
                    return_value: None,
                    overload_map: HashMap::new(),
                    generic_templates: if function_template_params.len() > 0 {
                        Some(function_template_params)
                    } else {
                        None
                    },
                }));
                maybe_fdata = Some(fdata.clone());
                write.insert(fname.to_ascii_lowercase(), fdata);
            }
        }
        state.in_function_stack.push(FunctionState::new_function(
            fname.get_name().unwrap(),
            maybe_fdata,
        ));

        self.analyze_first_pass_children(&self.as_any(), state, emitter);

        state.in_function_stack.pop();
    }
}

impl ThirdPassAnalyzeableNode for FunctionDefinitionNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        let data = if let Some(dt) = self.get_function_data(state, emitter) {
            dt
        } else {
            eprintln!(
                "missing function_data for {}",
                self.get_function_name(state, emitter)
            );
            return true;
        };
        let function = FunctionState::new_function(
            self.get_function_name(state, emitter).get_name().unwrap(),
            Some(data),
        );
        state.in_function_stack.push(function);
        if !self.analyze_third_pass_children(&self.as_any(), state, emitter, path) {
            return false;
        }
        let func = state
            .in_function_stack
            .pop()
            .expect("There must be a state");
        let returns = func.returns.read().unwrap().clone();

        let mut values: Vec<PHPValue> = vec![];
        for ret in &returns {
            if let Some(val) = &ret.1 {
                values.push(val.clone());
            } else {
                values.truncate(0);
                break;
            }
        }
        let return_value = if values.len() > 0 {
            PHPValue::common_value(values.iter().collect::<Vec<&PHPValue>>())
        } else {
            None
        };

        // eprintln!("returns: {:?}", returns);
        let scope_handle = func.scope_stack.read().unwrap().top();
        scope_handle.analyze_for_unused_vars(state, emitter);

        let mut ret_type = UnionType::new();

        for (r_type, _val) in returns {
            if let Some(t) = r_type {
                // t;
                ret_type.merge_into(t);
            } else {
                return true;
            }
        }

        if let Some(function) = self.get_function_data(state, emitter) {
            let mut function_data = function.write().unwrap();
            (*function_data).inferred_return_type = Some(ret_type);
            (*function_data).return_value = return_value;
        }
        true
    }
}
