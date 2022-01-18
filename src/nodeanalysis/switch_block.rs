use crate::autonodes::any::AnyNodeRef;
use crate::autotree::NodeAccess;
use crate::{
    autonodes::switch_block::SwitchBlockNode, nodeanalysis::analysis::ThirdPassAnalyzeableNode,
};

impl ThirdPassAnalyzeableNode for SwitchBlockNode {
    fn analyze_third_pass(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn crate::issue::IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        crate::missing!();
        self.analyze_third_pass_children(&self.as_any(), state, emitter, path)
    }
}
