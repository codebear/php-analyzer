use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    analysis::scope::BranchableScope, autonodes::any::AnyNodeRef, symboldata::FileLocation,
    symbols::FullyQualifiedName,
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
        read.get(&fname).cloned()
    }
}

impl FirstPassAnalyzeableNode for FunctionDefinitionNode {
    fn analyze_first_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        let fname = self.get_function_name(state, emitter);

        state
            .in_function_stack
            .push(FunctionState::new_function(fname.get_name().unwrap()));

        let mut is_dup = false;
        {
            let read = state.symbol_data.functions.read().unwrap();
            if let Some(_) = read.get(&fname.to_ascii_lowercase()) {
                emitter.emit(Issue::DuplicateFunction(self.pos(state), fname.clone()));
                is_dup = true;
            }
        }
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
                    php_return_type: None,
                    comment_return_type: None,
                    inferred_return_type: None,
                    arguments: vec![],
                    variadic: false,
                    pure: false,
                    deterministic: false,
                    return_value: None,
                    overload_map: HashMap::new(),
                }));

                write.insert(fname.to_ascii_lowercase(), fdata);
            }
        }
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
        let function = FunctionState::new_function(self.get_function_name(state, emitter).get_name().unwrap());
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
