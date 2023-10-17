use crate::{types::union::DiscreteType, value::PHPValue, Range};

use super::{
    binary::{BinaryOperator, BinaryOperatorOperandAccess},
    operator::Operator,
};
#[derive(Clone, Debug)]

pub struct BinaryOrOperator(pub Range);

impl Operator for BinaryOrOperator {
    fn brief_desc(&self) -> String {
        "BinaryOrOperator".into()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "|"
    }
}

impl BinaryOperator for BinaryOrOperator {
    fn get_operator_utype(
        &self,
        _operands: &impl BinaryOperatorOperandAccess,
        _state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::types::union::UnionType> {
        return Some(DiscreteType::Int.into());
    }

    fn get_operator_php_value(
        &self,
        operands: &impl BinaryOperatorOperandAccess,
        state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        let left = (operands.get_left_value(state))?.as_i64()?;
        let right = (operands.get_right_value(state))?.as_i64()?;

        Some(PHPValue::Int(left | right))
    }
}
