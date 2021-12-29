use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, method_declaration::MethodDeclarationNode},
};

use super::NodeDescription;

impl NodeDescription for MethodDeclarationNode {
    fn intersect_up_traversal(&self) -> bool {
        true
    }

    fn describe_node(
        &self,
        _path: Option<&[AnyNodeRef]>,
        state: &mut AnalysisState,
    ) -> Option<String> {
        let locked_data = self.get_method_data(state)?;

        let data = locked_data.read().ok()?;

        Some(format!(
            "
|   |   |
|---|---|
| Method | {:?} |
| Comment-type | {:?} |
| Declared-type | {:?} |
| Inferred | {:?} |
        ",
            data.name, data.comment_return_type, data.php_return_type, data.inferred_return_type
        ))
    }
}
