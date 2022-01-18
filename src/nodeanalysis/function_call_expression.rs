use std::{
    ffi::OsString,
    sync::{Arc, RwLock},
};

use crate::{
    analysis::state::{AnalysisState, ConstantData},
    autonodes::{
        any::AnyNodeRef,
        function_call_expression::{FunctionCallExpressionFunction, FunctionCallExpressionNode},
    },
    issue::{Issue, IssueEmitter},
    nodeanalysis::lang::AnalysisOfType,
    symboldata::FunctionData,
    symbols::{FullyQualifiedName, Name},
    types::union::{DiscreteType, UnionType},
    value::PHPValue,
};

use super::analysis::{FirstPassAnalyzeableNode, ThirdPassAnalyzeableNode};
use crate::autotree::NodeAccess;

impl FunctionCallExpressionNode {
    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        self.function.read_from(state, emitter);
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        if let Some(data_handle) = self.get_func_data(state, emitter) {
            let data = data_handle.read().unwrap();

            data.comment_return_type
                .as_ref()
                .or_else(|| data.php_return_type.as_ref())
                .or_else(|| data.inferred_return_type.as_ref())
                .cloned()
        } else {
            None
        }
    }

    pub fn get_func_data(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<Arc<RwLock<FunctionData>>> {
        let fq_name = self.get_fq_function_name(state, emitter)?;
        let func_handle = state.symbol_data.functions.read().unwrap();
        let func_data = func_handle.get(&fq_name.to_ascii_lowercase())?;
        Some(func_data.clone())
    }

    pub fn get_fq_function_name(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<FullyQualifiedName> {
        self.function.get_fq_function_name(state, emitter)
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        // A few deterministic function calls are possible to predict a return value from. i.e. `abs(-4)`
        // but for now, none
        None
    }
}

impl FunctionCallExpressionFunction {
    /// If the fully qualified function doesn't exist, check if a global function exists, if so, go for it
    /// else return the fully qualified name which doesn't exist
    fn fq_name_or_global_name(
        &self,
        state: &mut AnalysisState,
        fq_name: FullyQualifiedName,
    ) -> FullyQualifiedName {
        if state.symbol_data.get_function(&fq_name).is_some() {
            fq_name
        } else {
            let root_name =
                FullyQualifiedName::from(fq_name.get_name().unwrap_or_else(|| Name::new()));
            if state.symbol_data.get_function(&root_name).is_some() {
                root_name
            } else {
                fq_name
            }
        }
    }

    fn get_fq_function_name(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<FullyQualifiedName> {
        match self {
            FunctionCallExpressionFunction::Name(n) => {
                let fq_name = state.get_fq_symbol_name_from_local_name(&n.get_name());
                Some(self.fq_name_or_global_name(state, fq_name))
            }
            FunctionCallExpressionFunction::VariableName(vn) => {
                if state.pass == 1 {
                    return None;
                }
                let val = vn.get_php_value(state, emitter);
                let ftype = vn.get_declared_type(state, emitter);
                if let Some(PHPValue::String(fname)) = val {
                    // function names in variables need to always be fully qualified
                    let func_name = FullyQualifiedName::from(fname);
                    let funcs = state.symbol_data.functions.read().unwrap();
                    if let Some(_f) = funcs.get(&func_name) {
                        // SJEKK OM ARGUMENTER PASSER
                    } else {
                        emitter.emit(Issue::UnknownFunction(self.pos(state), func_name));
                    }
                }
                if let Some(f) = ftype {
                    if f.is_callable() {
                        // ok
                    } else {
                        match f.single_type() {
                            Some(DiscreteType::Int)
                            | Some(DiscreteType::Float)
                            | Some(DiscreteType::Resource)
                            | Some(DiscreteType::NULL) => {
                                emitter.emit(Issue::NotACallableVariable(
                                    self.pos(state),
                                    vn.get_variable_name(),
                                ))
                            }
                            _ => emitter.emit(Issue::NotAVerifiedCallableVariable(
                                self.pos(state),
                                vn.get_variable_name(),
                            )),
                        }
                    }
                } else {
                    emitter.emit(Issue::NotAVerifiedCallableVariable(
                        self.pos(state),
                        vn.get_variable_name(),
                    ));
                    /*                     emitter.emit(
                        state.filename.as_ref(),
                        self.range(),
                        Issue::new_hint(format!(
                            "VariableName function calling is not recommended, and unanalyzable"
                        )),
                    );*/
                }
                None
            }
            FunctionCallExpressionFunction::ParenthesizedExpression(_) => None, // Sannsynligvis anonym funksjon
            FunctionCallExpressionFunction::QualifiedName(qn) => {
                Some(self.fq_name_or_global_name(state, qn.get_fq_name()))
            }
            _ => crate::missing_none!("finne funksjonsnavn ut fra: {:?}", self.kind()),
        }
    }
}

fn analyze_define_call(
    call: &FunctionCallExpressionNode,
    state: &mut AnalysisState,
    emitter: &dyn IssueEmitter,
) {
    let arg_count = call.arguments.children.len();
    if arg_count != 2 {
        emitter.emit(Issue::WrongNumberOfArguments(
            call.pos(state),
            "define".into(),
            2,
            arg_count,
        ));
        return;
    }

    if state.in_function_stack.len() > 0 {
        emitter.emit(Issue::ConditionalConstantDeclaration(call.pos(state)));
    }

    if let Some(_) = state.namespace {
        crate::missing!("Constant in namespace?");
        return;
    }

    let mut args = call.arguments.children.iter();

    let name_value = if let Some(PHPValue::String(name)) =
        args.next().and_then(|n| n.get_php_value(state, emitter))
    {
        FullyQualifiedName::from(name)
    } else {
        eprintln!(
            "{} Her har vi et kall til define som ikke fikk ekstrahert navn, med argumenter: {:?}",
            state.pos_as_string(call.range()),
            call.arguments
        );
        return;
    };

    let value_node = args.next().unwrap();

    let value = value_node.get_php_value(state, emitter);
    let val_type = value_node
        .get_utype(state, emitter)
        .unwrap_or_else(|| DiscreteType::Unknown.into());

    let fname = state
        .filename
        .as_ref()
        .map(|x| x.as_os_str().to_os_string())
        .unwrap_or_else(|| OsString::new());

    //if let Some(val) = value {
    let mut mutable = state.global.constants.write().unwrap();
    if let Some(cdata) = (*mutable).get_mut(&name_value) {
        emitter.emit(Issue::DuplicateConstant(call.pos(state), name_value));
        cdata.add_value(fname, call.range(), val_type, value);
        // FIXME archive range for duplicate-analysis
    } else {
        //  eprintln!("Konstant {:?}: {:?} = {:?}", &name_value, &val_type, &value);
        let mut cdata = ConstantData::new(name_value.clone());

        cdata.add_value(fname, call.range(), val_type, value);

        (*mutable).insert(name_value, cdata); // (val_type, value));
    }

    //} else {
    //    emitter.emit(state.filename.as_ref(), call.range(), Issue::new_hint("define(..)-declaration with indeterminate value".into()));
    //eprintln!("{} Her har vi et kall til define for konstant {:?} som ikke fikk ekstrahert verdier, med argumenter: {:?}", state.pos_as_string(call.range()),  name_value, call.arguments);
    //}
}

impl FirstPassAnalyzeableNode for FunctionCallExpressionNode {
    fn analyze_first_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        if let Some(fname) = self.function.get_fq_function_name(state, emitter) {
            // FIXME check if newer rust does this cast automatically
            if fname.to_ascii_lowercase() == b"\\define" as &[u8] {
                analyze_define_call(self, state, emitter)
            }
        }
    }
}

impl ThirdPassAnalyzeableNode for FunctionCallExpressionNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        let data = self.get_func_data(state, emitter);

        let fq_name = self.get_fq_function_name(state, emitter);
        //         if data.is_none() && fq_name.is_some() {}
        match (data, fq_name) {
            // intrinsics needs special treatment
            (_, Some(fq_name)) if fq_name == b"\\isset" as &[u8] => {}
            (_, Some(fq_name)) if fq_name == b"\\empty" as &[u8] => {}
            (_, Some(fq_name)) if fq_name == b"\\exit" as &[u8] => {}
            (_, Some(fq_name)) if fq_name == b"\\die" as &[u8] => {}

            (None, Some(fq_name)) => {
                emitter.emit(Issue::UnknownFunction(self.pos(state), fq_name));
            }
            (Some(func_data_handle), Some(fq_name)) => {
                let func_data = func_data_handle.read().unwrap();
                if func_data.name != fq_name {
                    emitter.emit(Issue::WrongFunctionNameCasing(
                        self.pos(state),
                        func_data.name.clone(),
                        fq_name,
                    ));
                }
            }
            _ => (),
        }

        // FIXME analyze if arguments are correct

        self.analyze_third_pass_children(&self.as_any(), state, emitter, path)
    }
}
