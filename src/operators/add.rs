use crate::{
    types::union::DiscreteType,
    value::{PHPFloat, PHPValue},
    Range,
};

use super::{
    binary::{BinaryOperator, BinaryOperatorOperandAccess},
    operator::Operator,
};

#[derive(Clone, Debug)]
pub struct AddOperator(pub Range);

impl Operator for AddOperator {
    fn brief_desc(&self) -> String {
        "AddOperator".into()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "+"
    }
}

impl BinaryOperator for AddOperator {
    fn get_operator_utype(
        &self,
        operands: &impl BinaryOperatorOperandAccess,
        state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::types::union::UnionType> {
        let ltype = operands.get_left_type(state)?.single_type()?;
        let rtype = operands.get_right_type(state)?.single_type()?;
        match (&ltype, &rtype) {
            (DiscreteType::Int, DiscreteType::Int) => Some(DiscreteType::Int.into()),

            (DiscreteType::Float, DiscreteType::Int)
            | (DiscreteType::Int, DiscreteType::Float)
            | (DiscreteType::Float, DiscreteType::Float) => Some(DiscreteType::Float.into()),

            (DiscreteType::Int, DiscreteType::Bool)
            | (DiscreteType::Bool, DiscreteType::Bool)
            | (DiscreteType::Bool, DiscreteType::Int) => Some(DiscreteType::Int.into()),

            (DiscreteType::Unknown, _) | (_, DiscreteType::Unknown) => None,

            _ => crate::missing_none!("{:?} {} {:?}", ltype, self.operator(), rtype),
        }
    }

    fn get_operator_php_value(
        &self,
        operands: &impl BinaryOperatorOperandAccess,
        state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        let left = operands.get_left_value(state)?.as_php_num()?;
        let right = operands.get_right_value(state)?.as_php_num()?;
        match (left, right) {
            (PHPValue::Int(a), PHPValue::Int(b)) => Some(PHPValue::Int(a + b)),
            (PHPValue::Int(_a), PHPValue::Float(_b)) => {
                crate::missing_none!("i64 to f64 conversion")
            } // Some(PHPValue::Float(b+a.into())),
            (PHPValue::Float(_a), PHPValue::Int(_b)) => {
                crate::missing_none!("i64 to f64 conversion")
            } // Some(PHPValue::Float(a+b.into())),
            (PHPValue::Float(PHPFloat::Real(a)), PHPValue::Float(PHPFloat::Real(b))) => {
                Some(PHPValue::Float(PHPFloat::new(a + b)))
            }
            _ => None,
        }
    }
}
