use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, class_declaration::ClassDeclarationNode},
};

use super::NodeDescription;

impl NodeDescription for ClassDeclarationNode {
    fn describe_node(
        &self,
        _path: Option<&[AnyNodeRef]>,
        _state: &mut AnalysisState,
    ) -> Option<String> {
        Some("class of sorts. Finn ut hvilke egenskaper den har...".into())
    }

    fn intersect_up_traversal(&self) -> bool {
        true
    }
}
