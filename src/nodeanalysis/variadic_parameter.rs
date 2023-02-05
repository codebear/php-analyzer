use crate::{
    analysis::state::{AnalysisState, FunctionDataPointer},
    autonodes::variadic_parameter::VariadicParameterNode,
    issue::IssueEmitter,
    phpdoc::types::PHPDocEntry,
    symboldata::class::FunctionArgumentData,
    symbols::Name,
    types::union::{DiscreteType, UnionType},
};

impl VariadicParameterNode {
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

    pub fn get_variable_name(&self) -> Name {
        self.name.get_variable_name()
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        let comment_type = self.get_declared_comment_type(state, emitter);
        let utype = comment_type.or_else(|| self.get_declared_native_type(state, emitter))?;
        Some(utype)
    }

    pub fn get_declared_native_type(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        let vtype = self.type_.as_ref()?.get_utype(state, emitter)?;
        Some(DiscreteType::Vector(vtype).into())
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

    pub fn get_declared_comment_type(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        let param_data = self.get_parameter_data(state)?;
        if let Some(PHPDocEntry::Param(_range, union_of_types, _name, _desc)) =
            param_data.phpdoc_entry
        {
            UnionType::from_parsed_type(union_of_types, state, emitter, None)
        } else {
            param_data.inline_phpdoc_type.map(|x| x.1)
        }
    }
}
