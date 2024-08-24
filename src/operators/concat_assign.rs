use crate::{types::union::DiscreteType, Range};

use super::{
    binary::{BinaryAssignmentOperator, BinaryOperatorOperandAccess},
    operator::Operator,
};
#[derive(Clone, Debug)]

pub struct ConcatAssignOperator(pub Range);

impl Operator for ConcatAssignOperator {
    fn brief_desc(&self) -> String {
        "ConcatAssignOperator".into()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        ".="
    }
}

impl BinaryAssignmentOperator for ConcatAssignOperator {
    fn get_operator_utype(
        &self,
        _operands: &impl BinaryOperatorOperandAccess,
        _state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::types::union::PHPType> {
        Some(DiscreteType::String.into())
    }
}
