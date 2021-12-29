use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, scoped_call_expression::ScopedCallExpressionNode},
};

use super::NodeDescription;

impl NodeDescription for ScopedCallExpressionNode {
    fn intersect_up_traversal(&self) -> bool {
        true
    }

    fn describe_node(
        &self,
        _path: Option<&[AnyNodeRef]>,
        _state: &mut AnalysisState,
    ) -> Option<String> {
        Some("ScopedCallExpressionNode call of sorts. Finn ut hvilke egenskaper den har...".into())
    }
}
