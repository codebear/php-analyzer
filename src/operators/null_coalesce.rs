use crate::{types::union::PHPType, Range};

use super::{
    binary::{BinaryOperator, BinaryOperatorOperandAccess},
    operator::Operator,
};
#[derive(Clone, Debug)]

pub struct NullCoalesceOperator(pub Range);

impl Operator for NullCoalesceOperator {
    fn brief_desc(&self) -> String {
        "NullCoalesceOperator".into()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "??"
    }
}

impl BinaryOperator for NullCoalesceOperator {
    fn get_operator_utype(
        &self,
        operands: &impl BinaryOperatorOperandAccess,
        state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<PHPType> {
        //if let Some(val) = self.get_php_value(state, emitter) {
        //    return val.get_utype();
        //}
        crate::missing_none!(
            "Handle {:?} ?? {:?}",
            operands.get_left_type(state),
            operands.get_right_type(state)
        )
    }

    fn get_operator_php_value(
        &self,
        operands: &impl BinaryOperatorOperandAccess,
        state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        let lval = operands.get_left_value(state)?;
        if lval.is_null() {
            operands.get_right_value(state)
        } else {
            Some(lval)
        }
    }
}
