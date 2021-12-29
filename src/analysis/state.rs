use std::ffi::OsStr;
use std::ffi::OsString;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use tree_sitter::Point;
use tree_sitter::Range;

use crate::autonodes::any::AnyNodeRef;
use crate::symboldata::class::ClassName;
use crate::symboldata::SymbolData;
use crate::symbols::FullyQualifiedName;
use crate::symbols::Name;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use std::collections::HashMap;

use super::scope::{Scope, ScopeStack};

#[derive(Debug)]
pub struct ConstantData {
    pub fq_name: FullyQualifiedName,
    pub values: HashMap<(OsString, Range), (UnionType, Option<PHPValue>)>,
}

impl ConstantData {
    pub fn new(fq_name: FullyQualifiedName) -> Self {
        Self {
            fq_name,
            values: HashMap::new(),
        }
    }

    pub fn add_value(
        &mut self,
        filename: OsString,
        range: Range,
        constant_type: UnionType,
        val: Option<PHPValue>,
    ) {
        //
        let key = (filename, range);
        let value = (constant_type, val);
        self.values.insert(key, value);
    }

    ///
    /// Returns a value if there is only one known definition of the constant
    pub fn get_value(&self) -> Option<PHPValue> {
        if self.values.len() == 1 {
            for (_, (_, val)) in &self.values {
                return val.clone();
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct GlobalState {
    pub scope_stack: RwLock<ScopeStack>,
    pub constants: RwLock<HashMap<FullyQualifiedName, ConstantData>>,
}

impl GlobalState {
    pub fn new() -> Self {
        GlobalState {
            scope_stack: RwLock::new(ScopeStack::new()),
            constants: RwLock::new(HashMap::new()),
        }
    }
}

#[derive(Debug)]
pub enum ClassState {
    Interface(ClassName),
    Class(ClassName),
    Trait(ClassName),
}

impl ClassState {
    pub fn get_name(&self) -> ClassName {
        match self {
            Self::Class(c) => c.clone(),
            Self::Interface(i) => i.clone(),
            Self::Trait(t) => t.clone(),
        }
    }
}

#[derive(Debug)]
pub struct FunctionState {
    pub name: Option<Name>,
    pub is_method: bool,
    pub scope_stack: RwLock<ScopeStack>,
    pub returns: RwLock<Vec<(Option<UnionType>, Option<PHPValue>)>>,
}

impl FunctionState {
    pub fn new(name: Option<Name>, is_method: bool) -> Self {
        Self {
            scope_stack: RwLock::new(ScopeStack::new()),
            returns: RwLock::new(Vec::new()),
            name,
            is_method,
        }
    }

    pub fn add_return(&self, ret_type: Option<UnionType>, ret_value: Option<PHPValue>) {
        let mut rets = self.returns.write().expect("Noe");
        rets.push((ret_type, ret_value));
    }

    pub(crate) fn new_method(method_name: Name) -> FunctionState {
        Self::new(Some(method_name), true)
    }

    pub(crate) fn new_function(name: FullyQualifiedName) -> FunctionState {
        if name.level() > 0 {
            crate::missing!(
                "FunctionState::new_function({:?}) drops namespace info... verify correctness",
                name
            );
        }
        Self::new(name.get_name(), false)
    }

    pub(crate) fn new_anonymous() -> FunctionState {
        Self::new(None, false)
    }
}

#[derive(Clone)]
pub struct LookingForNode {
    pub pos: Point,
    pub callback: Arc<
        RwLock<
            Option<
                Box<
                    dyn FnOnce(AnyNodeRef, &mut AnalysisState, &Vec<AnyNodeRef>) -> ()
                        + Send
                        + Sync,
                >,
            >,
        >,
    >,
}

impl std::fmt::Debug for LookingForNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LookingForNode")
            .field("pos", &self.pos)
            // .field("callback", &self.callback)
            .finish()
    }
    // void
}

#[derive(Debug)]
pub struct AnalysisState {
    pub pass: usize,
    pub filename: Option<PathBuf>,
    pub global: Arc<GlobalState>,
    pub in_class: Option<ClassState>,
    pub in_function_stack: Vec<FunctionState>,
    pub use_map: HashMap<Name, FullyQualifiedName>,
    pub namespace: Option<FullyQualifiedName>,
    pub symbol_data: Arc<SymbolData>,
    pub last_doc_comment: Option<(OsString, Range)>,
    pub in_conditional_branch: bool,
    pub looking_for_node: Option<LookingForNode>,
}

impl AnalysisState {
    pub fn new() -> Self {
        Self::new_with_symbols(Arc::new(SymbolData::new()))
    }

    pub fn new_with_symbols(symbol_data: Arc<SymbolData>) -> Self {
        AnalysisState {
            pass: 0,
            filename: None,
            global: Arc::new(GlobalState::new()),
            in_class: None,
            in_function_stack: vec![],
            use_map: HashMap::new(),
            namespace: None,
            symbol_data,
            last_doc_comment: None,
            in_conditional_branch: false,
            looking_for_node: None,
        }
    }

    pub fn pos_as_string(&self, range: Range) -> String {
        let fname = if let Some(fname) = &self.filename {
            String::from_utf8_lossy(fname.as_os_str().as_bytes()).to_string()
        } else {
            String::from("*unknown*")
        };
        format!(
            "{}:{}:{}",
            fname,
            range.start_point.row + 1,
            range.start_point.column + 1,
        )
    }

    /// Appends namespace to local names. Does no lookup in use-map
    pub fn get_fq_symbol_name_without_aliasing(&self, symbol_name: &Name) -> FullyQualifiedName {
        let mut fq_name = if let Some(ns) = &self.namespace {
            ns.clone()
        } else {
            FullyQualifiedName::new()
        };
        fq_name.push(symbol_name);
        fq_name
    }

    ///
    pub fn get_fq_symbol_name_from_local_name(&self, symbol_name: &Name) -> FullyQualifiedName {
        // eprintln!("AnalysisState.get_fq_symbol_name({:?}). use_map: {:?}", symbol_name, &self.use_map);
        if let Some(fq_name) = self.use_map.get(symbol_name) {
            // eprintln!("USEMAP: {:?}", &self.use_map);
            //   eprintln!("fra use_map: {:?}", &fq_name);
            return fq_name.clone();
        };
        if let Some(ns) = &self.namespace {
            let mut fq = ns.clone();
            fq.push(symbol_name);
            fq
        } else {
            self.get_fq_symbol_name_without_aliasing(symbol_name)
        }
    }

    pub fn get_fq_function_name(&self, local_name: Name) -> FullyQualifiedName {
        self.get_fq_symbol_name_from_local_name(&local_name)
    }

    pub fn current_scope_stack(&self) -> &RwLock<ScopeStack> {
        if let Some(current_func) = self.in_function_stack.last() {
            &current_func.scope_stack
        } else {
            &self.global.scope_stack
        }
    }

    pub fn current_scope(&self) -> Arc<RwLock<Scope>> {
        self.current_scope_stack().read().unwrap().top()
    }

    pub(crate) fn push_scope(&self, scope: Arc<RwLock<Scope>>) {
        self.current_scope_stack().write().unwrap().push(scope)
    }

    pub(crate) fn pop_scope(&self) -> Arc<RwLock<Scope>> {
        self.current_scope_stack().write().unwrap().pop()
    }

    pub fn in_method<S>(&self, method_name: S) -> bool
    where
        S: AsRef<OsStr>,
    {
        let in_function = if let Some(in_function) = self.in_function_stack.last() {
            in_function
        } else {
            return false;
        };

        if !in_function.is_method {
            return false;
        }

        let name = if let Some(name) = &in_function.name {
            name
        } else {
            return false;
        };

        name.eq_ignore_ascii_case(method_name)
    }

    pub(crate) fn in_constructor(&self) -> bool {
        self.in_method("__construct")
    }
}
impl LookingForNode {
    pub fn found(&self, child: AnyNodeRef, state: &mut AnalysisState, path: &Vec<AnyNodeRef>) {
        let mut handle = self.callback.write().unwrap();
        let cb = handle.take().expect("Already consumed the callback?");
        eprintln!(
            "FANT EN NODE: {:?}, path:len() = {}",
            child.kind(),
            path.len()
        );
        cb(child, state, path);
    }
}
