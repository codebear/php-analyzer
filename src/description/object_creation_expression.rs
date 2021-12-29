use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, object_creation_expression::ObjectCreationExpressionNode},
    issue::VoidEmitter,
};

use super::NodeDescription;

impl NodeDescription for ObjectCreationExpressionNode {
    fn intersect_up_traversal(&self) -> bool {
        true
    }

    fn describe_node(
        &self,
        _path: Option<&[AnyNodeRef]>,
        state: &mut AnalysisState,
    ) -> Option<String> {
        if let Some(types) = self.get_utype(state, &VoidEmitter::new()) {
            Some(format!("Create object of type {:?}", types))
        } else {
            Some("get_utype() returned None...!?!?".into())
        }
    }
}
