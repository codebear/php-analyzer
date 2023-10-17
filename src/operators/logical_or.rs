use crate::{types::union::DiscreteType, value::PHPValue, Range};

use super::{
    binary::{BinaryOperator, BinaryOperatorOperandAccess},
    operator::Operator,
};
#[derive(Clone, Debug)]

pub struct LogicalOrOperator(pub Range);

impl Operator for LogicalOrOperator {
    fn brief_desc(&self) -> String {
        "LogicalOrOperator".into()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "or"
    }
}

impl BinaryOperator for LogicalOrOperator {
    fn get_operator_utype(
        &self,
        _operands: &impl BinaryOperatorOperandAccess,
        _state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::types::union::UnionType> {
        Some(DiscreteType::Bool.into())
    }

    fn get_operator_php_value(
        &self,
        operands: &impl BinaryOperatorOperandAccess,
        state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        let left = operands.get_left_value(state)?.as_bool()?;
        let right = operands.get_right_value(state)?.as_bool()?;

        // FIXME overflow og sånt her bør trigg emitting
        Some(PHPValue::Boolean(left || right))
    }
}
