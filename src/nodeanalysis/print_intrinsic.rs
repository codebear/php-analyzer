use crate::{
    analysis::state::AnalysisState,
    autonodes::{any::AnyNodeRef, print_intrinsic::PrintIntrinsicNode},
    issue::IssueEmitter,
    types::union::{DiscreteType, UnionType},
    value::PHPValue,
};

use super::analysis::AnalyzeableRoundTwoNode;
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
    ) -> Option<UnionType> {
        Some(DiscreteType::Int.into())
    }
}

impl AnalyzeableRoundTwoNode for PrintIntrinsicNode {
    fn analyze_round_two(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        self.child.read_from(state, emitter);
        // todo
        self.analyze_round_two_children(&self.as_any(), state, emitter, path)
    }
}
