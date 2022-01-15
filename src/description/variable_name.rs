use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, variable_name::VariableNameNode},
};

use super::NodeDescription;

impl NodeDescription for VariableNameNode {
    fn describe_node(
        &self,
        _path: Option<&[AnyNodeRef]>,
        state: &mut AnalysisState,
    ) -> Option<String> {
        let var_name = self.get_variable_name();
        let scope = state.current_scope();
        let scope_handle = scope.read().unwrap();
        let var_handle = scope_handle.get_var(&var_name)?;
        let var_data = var_handle.read().unwrap();
        let mut buf = String::new();
        buf.push_str(&format!("|  |  |\n"));
        buf.push_str(&format!("| --- | --- |\n"));
        buf.push_str(&format!("| Var-name |  `${}` |\n", var_data.name));
        if let Some(dtype) = var_data.get_declared_type() {
            buf.push_str(&format!("| Declared-type: |  {} |\n", dtype));
        }
        if let Some(itype) = var_data.get_inferred_type() {
            buf.push_str(&format!("| Inferred-type: | {} |\n", itype));
        }
        if let Some(ctype) = &var_data.comment_declared_type {
            buf.push_str(&format!("| Comment-type: | {} |\n", ctype));
        }
        if let Some(dval) = &var_data.default_value {
            buf.push_str(&format!("| Default value | {:?} |\n", dval));
        }

        Some(buf)
    }
}
