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

        let class_name = self.get_class_name(state)?;

        Some(format!(
            "
### `{}::{}(..)`
|     |     |
| --- | --- |
| Method | {} |
| Comment-type | {} |
| Declared-type | {} |
| Inferred | {} |
        ",
            class_name.get_fq_name(),
            data.name,
            data.name,
            data.comment_return_type
                .as_ref()
                .map(|x| x.0.to_markdown())
                .unwrap_or_else(|| " _none found_".to_string()),
            data.php_return_type
                .as_ref()
                .map(|x| x.to_markdown())
                .unwrap_or_else(|| " _none found_".to_string()),
            data.inferred_return_type
                .as_ref()
                .map(|x| x.to_markdown())
                .unwrap_or_else(|| " _none found_".to_string())
        ))
    }
}
