use crate::{
    analysis::state::{AnalysisState, FunctionDataPointer},
    autonodes::{any::AnyNodeRef, simple_parameter::SimpleParameterNode},
    issue::IssueEmitter,
    phpdoc::types::PHPDocEntry,
    symboldata::class::FunctionArgumentData,
    symbols::Name,
    types::union::{DiscreteType, UnionType},
    value::PHPValue,
};

use super::analysis::ThirdPassAnalyzeableNode;
use crate::autotree::NodeAccess;

impl SimpleParameterNode {
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
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        let comment_type = self.get_declared_comment_type(state, emitter);
        let mut utype = comment_type.or_else(|| self.get_declared_native_type(state, emitter))?;
        if let Some(x) = &self.default_value {
            if let Some(PHPValue::NULL) = x.get_php_value(state, emitter) {
                utype.types.insert(DiscreteType::NULL);
            }
        }
        Some(utype)
    }

    pub fn get_variable_name(&self) -> Name {
        self.name.get_variable_name()
    }

    pub fn get_declared_comment_type(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        let param_data = self.get_parameter_data(state)?;
        if let Some(PHPDocEntry::Param(_range, union_of_types, _name, _desc)) =
            param_data.phpdoc_entry
        {
            UnionType::from_parsed_type(union_of_types, state, emitter)
        } else {
            param_data.inline_phpdoc_type.map(|x| x.1)
        }
    }

    fn get_parameter_data(&self, state: &mut AnalysisState) -> Option<FunctionArgumentData> {
        let func_state_ref = state.in_function_stack.last()?;
        let data = &func_state_ref.data.as_ref()?;
        let name = self.get_variable_name();
        let args = match data {
            FunctionDataPointer::Method(m) => m.read().unwrap().arguments.clone(),
            FunctionDataPointer::Function(f) => f.read().unwrap().arguments.clone(),
        };

        for arg in args {
            if arg.name == name {
                return Some(arg);
            }
        }
        None
    }

    pub fn get_declared_native_type(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        if let Some(t) = &self.type_ {
            t.get_utype(state, emitter)
        } else {
            None
        }
    }

    pub fn get_default_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        if let Some(val) = &self.default_value {
            val.get_php_value(state, emitter)
        } else {
            None
        }
    }
}

impl ThirdPassAnalyzeableNode for SimpleParameterNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        _path: &Vec<AnyNodeRef>,
    ) -> bool {
        let scope = state.current_scope();

        let var_name = self.get_variable_name();
        let declared_type = self.get_utype(state, emitter);
        let default_value = self.get_default_value(state, emitter);

        let mut write_scope = scope.write().expect("Sucess");
        let write_ax = write_scope.get_or_create_var(var_name);
        let mut var_data = write_ax.write().unwrap();
        var_data.referenced_ranges.push(self.range.clone());
        var_data.php_declared_type = declared_type;
        var_data.is_argument = true;
        // FIXME extract comment types
        var_data.default_value = default_value;
        var_data.written_to += 1;
        //        var_data.written_data()
        true
    }
}
