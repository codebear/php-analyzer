mod array;
mod basic;
pub mod interface;
pub mod namespace;
mod objects;
pub mod phpdocs;
pub mod try_catch;
mod values;

use std::{
    ffi::OsString,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use crate::{
    analysis::analyzer::Analyzer,
    analysis::state::AnalysisState,
    issue::{Issue, IssueEmitter},
    symboldata::{FunctionData, SymbolData},
    symbols::FullyQualifiedName,
    types::union::UnionType,
    value::PHPValue,
};

pub struct TestEmitter {
    pub file_name: RwLock<Option<PathBuf>>,
    issues: Arc<RwLock<Vec<Issue>>>,
}

impl TestEmitter {
    pub fn new() -> Self {
        TestEmitter {
            file_name: RwLock::new(None),
            issues: Arc::new(RwLock::new(vec![])),
        }
    }
}

impl IssueEmitter for TestEmitter {
    fn emit(&self, issue: Issue) {
        let start = issue.range().start_point;
        let err = issue.as_string();
        if let Some(f) = issue.filename() {
            let fname = f.to_string_lossy();
            eprintln!(
                "Issue: {:?} {}:{}:{}: {}",
                issue.severity(),
                fname,
                start.row + 1,
                start.column + 1,
                err
            );
        } else {
            eprintln!(
                "Issue: {:?} *Unknown buffer*:{}:{}: {}",
                issue.severity(),
                start.row + 1,
                start.column + 1,
                err
            );
        }
        {
            self.issues.write().unwrap().push(issue);
        }
    }
}

#[derive(Debug)]
pub struct EvaluationResult {
    pub function_data: Option<FunctionData>,
    pub return_type: Option<UnionType>,
    pub return_value: Option<PHPValue>,
    pub symbol_data: Option<Arc<SymbolData>>,
    pub issues: Vec<Issue>,
}

impl EvaluationResult {
    pub fn new() -> Self {
        Self {
            function_data: None,
            return_type: None,
            return_value: None,
            symbol_data: None,
            issues: vec![],
        }
    }
}

fn evaluate_php_buffers<T>(buffers: T, load_native: bool) -> EvaluationResult
where
    T: IntoIterator<Item = (OsString, OsString)>,
{
    let emitter = TestEmitter::new();

    let symbols = Arc::new(SymbolData::new());
    if load_native {
        let mut state = AnalysisState::new_with_symbols(symbols.clone());
        crate::native::register(&mut state);
    }

    let buffers: Vec<_> = buffers.into_iter().collect();
    for (buffer_name, outer_buffer) in &buffers {
        let mut state = AnalysisState::new_with_symbols(symbols.clone());
        state.pass = 1;
        let mut analyzer =
            Analyzer::new_from_buffer(outer_buffer.clone(), Some(buffer_name.clone()));
        assert!(analyzer.parse().is_ok());

        // analyzer.dump();
        analyzer.first_pass(&mut state, &emitter);
    }

    for (buffer_name, outer_buffer) in &buffers {
        let mut state = AnalysisState::new_with_symbols(symbols.clone());
        state.pass = 2;
        let mut analyzer =
            Analyzer::new_from_buffer(outer_buffer.clone(), Some(buffer_name.clone()));
        assert!(analyzer.parse().is_ok());

        // analyzer.dump();
        analyzer.second_pass(&mut state, &emitter);
    }


    for idx in 0..1 {
        for (buffer_name, outer_buffer) in &buffers {
            let mut state = AnalysisState::new_with_symbols(symbols.clone());
            state.pass = 3 + idx;
            let mut analyzer =
                Analyzer::new_from_buffer(outer_buffer.clone(), Some(buffer_name.clone()));
            assert!(analyzer.parse().is_ok());
            analyzer.third_pass(&mut state, &emitter);
        }
    }

    let mut result = EvaluationResult::new();
    result.symbol_data = Some(symbols);
    result.issues = emitter.issues.read().unwrap().clone();
    crate::dump_missing_stats();

    result
}

fn evaluate_php_code_in_function<T: Into<OsString>>(buffer: T) -> EvaluationResult {
    let mut outer_buffer = OsString::from("<?php ");
    outer_buffer.push("function test_output() { ");
    outer_buffer.push(buffer.into());
    outer_buffer.push("}");

    let mut analyzer = Analyzer::new_from_buffer(outer_buffer, Some("test_buffer".into()));
    assert!(analyzer.parse().is_ok());

    let mut state = AnalysisState::new();
    crate::native::register(&mut state);
    let emitter = TestEmitter::new();
    // analyzer.dump();
    analyzer.first_pass(&mut state, &emitter);
    analyzer.second_pass(&mut state, &emitter);
    analyzer.third_pass(&mut state, &emitter);
    analyzer.third_pass(&mut state, &emitter);

    let mut result = EvaluationResult::new();
    let func_name = FullyQualifiedName::from("\\test_output");
    if let Some(functions_handle) = state.symbol_data.functions.read().ok() {
        if let Some(func_data_handle) = functions_handle.get(&func_name).cloned() {
            let func_data = func_data_handle.read().unwrap();
            result.function_data = Some(func_data.clone());
            result.return_type = func_data.inferred_return_type.clone();
            result.return_value = func_data.return_value.clone();
        } else {
            eprintln!("Mangler data om funksjonen test_output");
        }
    } else {
        eprintln!("Failed reading function data");
    }
    result.symbol_data = Some(state.symbol_data);
    result.issues = emitter.issues.read().unwrap().clone();
    crate::dump_missing_stats();

    result
}

fn get_inferred_return_type<T: Into<OsString>>(buffer: T) -> Option<UnionType> {
    evaluate_php_code_in_function(buffer).return_type
}

fn get_inferred_return_value<T: Into<OsString>>(buffer: T) -> Option<PHPValue> {
    evaluate_php_code_in_function(buffer).return_value
}
