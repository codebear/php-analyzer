use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, member_access_expression::MemberAccessExpressionNode},
    issue::VoidEmitter,
};

use super::NodeDescription;

impl NodeDescription for MemberAccessExpressionNode {
    fn describe_node(
        &self,
        _path: Option<&[AnyNodeRef]>,
        state: &mut AnalysisState,
    ) -> Option<String> {
        // FIXME sørg for at vi ikke trenger en emitter for å kalle disse funksjonene
        let emitter = VoidEmitter::new();
        Some(format!(
            "{:?}->{:?}: {:?}",
            self.get_class_name(state).map(|x| x.get_fq_name().clone()),
            self.get_property_name(state, &emitter),
            self.get_utype(state, &emitter)
        ))
    }

    fn intersect_up_traversal(&self) -> bool {
        true
    }
}
