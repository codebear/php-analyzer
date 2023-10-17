use crate::Range;

use super::{
    binary::{BinaryAssignmentOperator, BinaryOperatorOperandAccess},
    operator::Operator,
};
#[derive(Clone, Debug)]

pub struct LeftShiftAssignOperator(pub Range);

impl Operator for LeftShiftAssignOperator {
    fn brief_desc(&self) -> String {
        "LeftShiftAssignOperator".into()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "<<="
    }
}

impl BinaryAssignmentOperator for LeftShiftAssignOperator {
    fn get_operator_utype(
        &self,
        _operands: &impl BinaryOperatorOperandAccess,
        _state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::types::union::UnionType> {
        return None;
    }
}
