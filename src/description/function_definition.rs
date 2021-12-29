use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, function_definition::FunctionDefinitionNode},
};

use super::NodeDescription;

impl NodeDescription for FunctionDefinitionNode {
    fn intersect_up_traversal(&self) -> bool {
        true
    }

    fn describe_node(
        &self,
        _path: Option<&[AnyNodeRef]>,
        _state: &mut AnalysisState,
    ) -> Option<String> {
        Some("function of sorts. Finn ut hvilke egenskaper den har...".into())
    }
}
