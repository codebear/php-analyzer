use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, member_call_expression::MemberCallExpressionNode},
};

use super::NodeDescription;

impl NodeDescription for MemberCallExpressionNode {
    fn describe_node(
        &self,
        _path: Option<&[AnyNodeRef]>,
        _state: &mut AnalysisState,
    ) -> Option<String> {
        Some("MemberCallExpressionNode call of sorts. Finn ut hvilke egenskaper den har...".into())
    }
}
