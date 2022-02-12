use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, simple_parameter::SimpleParameterNode},
    issue::IssueEmitter,
    symbols::Name,
    types::union::UnionType,
    value::PHPValue,
};

use super::analysis::{FirstPassAnalyzeableNode, ThirdPassAnalyzeableNode};
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
        if let Some(t) = &self.type_ {
            t.get_utype(state, emitter)
        } else {
            None
        }
    }

    pub fn get_variable_name(&self) -> Name {
        self.name.get_variable_name()
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
