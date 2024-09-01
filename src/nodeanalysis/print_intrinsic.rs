use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, print_intrinsic::PrintIntrinsicNode},
    issue::IssueEmitter,
    types::union::{DiscreteType, PHPType},
    value::PHPValue,
};

use super::analysis::ThirdPassAnalyzeableNode;
use crate::autotree::NodeAccess;

impl PrintIntrinsicNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        // void
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        // https://www.php.net/print
        Some(PHPValue::Int(1))
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<PHPType> {
        Some(DiscreteType::Int.into())
    }
}

impl ThirdPassAnalyzeableNode for PrintIntrinsicNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &[AnyNodeRef],
    ) -> bool {
        self.child.read_from(state, emitter);
        // todo
        self.analyze_third_pass_children(&self.as_any(), state, emitter, path)
    }
}
