use crate::autonodes::any::AnyNodeRef;
use crate::autotree::NodeAccess;
use crate::{
    autonodes::switch_block::SwitchBlockNode, nodeanalysis::analysis::AnalyzeableRoundTwoNode,
};

impl AnalyzeableRoundTwoNode for SwitchBlockNode {
    fn analyze_round_two(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn crate::issue::IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        crate::missing!();
        self.analyze_round_two_children(&self.as_any(), state, emitter, path)
    }
}
