use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, member_call_expression::MemberCallExpressionNode},
};

use super::{scoped_call_expression::describe_method, NodeDescription};

impl NodeDescription for MemberCallExpressionNode {
    fn describe_node(
        &self,
        _path: Option<&[AnyNodeRef]>,
        state: &mut AnalysisState,
    ) -> Option<String> {
        let methods = self.get_methods_data(state);
        if methods.is_empty() {
            return if let Some(mname) = self.name.get_method_name(state) {
                Some(format!(
                    "Dynamic method call to method {} with no known data",
                    mname
                ))
            } else {
                Some("Dynamic method call with no known data".to_string())
            };
        }
        let mut buffer = String::new();
        if methods.len() > 1 {
            buffer.push_str(&format!(
                "Call to {} possible different methods<br />\n",
                methods.len()
            ));
        }
        for maybe in methods {
            if let Some((class_name, method_data)) = maybe {
                buffer.push_str("----  <br />\n ");
                buffer.push_str(&describe_method(&class_name, &method_data));
            } else {
                // FIXME this could probably supply partial info with class or method name, indicating
                // more precisely WHY this is missing. This will probably allready have emitted some issue.
                buffer.push_str("Unknown method<br />\n ");
            }
        }
        Some(buffer)
    }
}
