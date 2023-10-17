use crate::{types::union::DiscreteType, Range};

use super::{
    binary::{BinaryOperator, BinaryOperatorOperandAccess},
    operator::Operator,
};
#[derive(Clone, Debug)]

pub struct SpaceshipOperator(pub Range);

impl Operator for SpaceshipOperator {
    fn brief_desc(&self) -> String {
        "SpaceshipOperator".into()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "<=>"
    }
}

impl BinaryOperator for SpaceshipOperator {
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
        _operands: &impl BinaryOperatorOperandAccess,
        _state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        crate::missing_none!(
            "{}[{}].get_operator_php_value(..)",
            self.brief_desc(),
            self.operator()
        )
    }
}
