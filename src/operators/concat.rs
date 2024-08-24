use crate::{types::union::DiscreteType, value::PHPValue, Range};

use super::{
    binary::{BinaryOperator, BinaryOperatorOperandAccess},
    operator::Operator,
};
#[derive(Clone, Debug)]

pub struct ConcatOperator(pub Range);

impl Operator for ConcatOperator {
    fn brief_desc(&self) -> String {
        "ConcatOperator".into()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "."
    }
}

impl BinaryOperator for ConcatOperator {
    fn get_operator_utype(
        &self,
        _operands: &impl BinaryOperatorOperandAccess,
        _state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::types::union::PHPType> {
        Some(DiscreteType::String.into())
    }

    fn get_operator_php_value(
        &self,
        operands: &impl BinaryOperatorOperandAccess,
        state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        let mut left = (operands.get_left_value(state))?.as_os_string()?;
        let right = (operands.get_right_value(state))?.as_os_string()?;
        left.push(right);
        Some(PHPValue::String(left))
    }
}
