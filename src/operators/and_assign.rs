use crate::Range;

use super::{
    binary::{BinaryAssignmentOperator, BinaryOperatorOperandAccess},
    operator::Operator,
};
#[derive(Clone, Debug)]

pub struct AndAssignOperator(pub Range);

impl Operator for AndAssignOperator {
    fn brief_desc(&self) -> String {
        "AndAssignOperator".into()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "&="
    }
}

impl BinaryAssignmentOperator for AndAssignOperator {
    fn get_operator_utype(
        &self,
        _operands: &impl BinaryOperatorOperandAccess,
        _state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::types::union::PHPType> {
        crate::missing_none!("{}.get_operator_utype(..)", self.brief_desc())
    }
}
