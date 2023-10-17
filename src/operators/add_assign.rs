use crate::{
    types::union::{DiscreteType, UnionType},
    Range,
};

use super::{
    binary::{BinaryAssignmentOperator, BinaryOperatorOperandAccess},
    operator::Operator,
};
#[derive(Clone, Debug)]

pub struct AddAssignOperator(pub Range);

impl Operator for AddAssignOperator {
    fn brief_desc(&self) -> String {
        "AddAssignOperator".into()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "+="
    }
}

impl BinaryAssignmentOperator for AddAssignOperator {
    fn get_operator_utype(
        &self,
        operands: &impl BinaryOperatorOperandAccess,
        state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::types::union::UnionType> {
        if let Some(left_type) = operands.get_left_type(state) {
            if left_type.is_float() {
                return Some(DiscreteType::Float.into());
            }
        }
        if let Some(right_utype) = operands.get_right_type(state) {
            if right_utype.is_int() {
                return Some(DiscreteType::Int.into());
            }
        }
        let mut utype = UnionType::new();
        utype.push(DiscreteType::Int);
        utype.push(DiscreteType::Float);
        Some(utype)
    }
}
