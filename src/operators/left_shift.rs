use std::convert::TryInto;

use crate::{types::union::DiscreteType, value::PHPValue, Range};

use super::{
    binary::{BinaryOperator, BinaryOperatorOperandAccess},
    operator::Operator,
};
#[derive(Clone, Debug)]

pub struct LeftShiftOperator(pub Range);

impl Operator for LeftShiftOperator {
    fn brief_desc(&self) -> String {
        "LeftShiftOperator".into()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "<<"
    }
}

impl BinaryOperator for LeftShiftOperator {
    fn get_operator_utype(
        &self,
        _operands: &impl BinaryOperatorOperandAccess,
        _state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::types::union::UnionType> {
        Some(DiscreteType::Int.into())
    }

    fn get_operator_php_value(
        &self,
        operands: &impl BinaryOperatorOperandAccess,
        state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        let left_value = operands.get_left_value(state)?;
        let right_value = operands.get_right_value(state)?;
        let left = (left_value).as_i64()?;
        let right = (right_value).as_i64()?;
        // FIXME overflow og sånt her bør trigg emitting

        /*eprintln!(
            "Attempting left-shift: {} << {} fra {}",
            left,
            right,
            state.pos_as_string(self.range)
        );*/
        let r32: u32 = right.try_into().ok()?;

        let shifted = left.checked_shl(r32)?;
        Some(PHPValue::Int(shifted))
    }
}
