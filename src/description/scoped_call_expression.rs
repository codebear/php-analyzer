use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, scoped_call_expression::ScopedCallExpressionNode}, symboldata::class::{ClassName, MethodData},
};

use super::NodeDescription;

impl NodeDescription for ScopedCallExpressionNode {
    fn intersect_up_traversal(&self) -> bool {
        true
    }

    fn describe_node(
        &self,
        _path: Option<&[AnyNodeRef]>,
        state: &mut AnalysisState,
    ) -> Option<String> {
        if let Some((class_name, data)) = self.get_method_data(state) {
            Some(describe_method(&class_name, &data))
        } else {
            Some("Static method call with no known info".into())
        }
    }
}

pub fn describe_method(class_name: &ClassName, data: &MethodData) -> String {
    let mut buffer = String::new();
    buffer.push_str(&format!("### `{}::{}(..)`\n", class_name.get_fq_name(), data.name));
    buffer.push_str(&format!("{}  \n", data.description));
    buffer.push_str("|   |   |\n| --- | --- |\n");
    buffer.push_str(&format!("| Declared in | {:?}:{} |\n", data.position.uri, data.position.start.line));
    let mut any_known_type = false;
    if let Some(ptype) = &data.php_return_type {
        buffer.push_str(&format!("| Declared return | {} |\n", ptype));
        any_known_type = true;
    }
    if let Some(itype) = &data.inferred_return_type {
        buffer.push_str(&format!("| Inferred return type | {} |\n", itype));
        any_known_type = true;
    }
    if let Some(ctype) = &data.comment_return_type {
        buffer.push_str(&format!("| Doc-comment return type | {} |\n", ctype));
        any_known_type = true;
    }
    if !any_known_type {
        buffer.push_str(&format!("| No known return-type | |\n"));

    }
    buffer
}