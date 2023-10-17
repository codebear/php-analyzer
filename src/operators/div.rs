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

pub struct DivOperator(pub Range);

impl Operator for DivOperator {
    fn brief_desc(&self) -> String {
        "DivOperator".into()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "/"
    }
}

impl BinaryOperator for DivOperator {
    fn get_operator_utype(
        &self,
        operands: &impl BinaryOperatorOperandAccess,
        state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::types::union::UnionType> {
        let ltype = operands.get_left_type(state)?.single_type()?;
        let rtype = operands.get_right_type(state)?.single_type()?;
        crate::missing!("Fix saa denne er mer union-type-basert");
        match (&ltype, &rtype) {
            (DiscreteType::Int, DiscreteType::Int) => {
                let lval = operands.get_left_value(state);
                let rval = operands.get_right_value(state);
                if let (Some(PHPValue::Int(lint)), Some(PHPValue::Int(rint))) = (lval, rval) {
                    if (lint % rint) == 0 {
                        return Some(DiscreteType::Int.into());
                    }
                }
                Some(DiscreteType::Float.into())
            }
            (DiscreteType::Float, DiscreteType::Int) => Some(DiscreteType::Float.into()),
            (DiscreteType::Int, DiscreteType::Float) => Some(DiscreteType::Float.into()),
            (DiscreteType::Float, DiscreteType::Float) => Some(DiscreteType::Float.into()),
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
        let lval = operands.get_left_value(state)?;
        let rval = operands.get_right_value(state)?;
        let left = lval.as_php_num()?;
        let right = rval.as_php_num()?;
        match (left, right) {
            (PHPValue::Int(a), PHPValue::Int(b)) => {
                if b == 0 {
                    // FIXME Emit div by zero, men i analyze-pass?
                    return None;
                }
                if a % b == 0 && b != 0 {
                    // int result
                    Some(PHPValue::Int(a / b))
                } else {
                    let lfloat = lval.as_php_float()?.as_f64()?;
                    let rfloat = rval.as_php_float()?.as_f64()?;

                    if rfloat == 0.0 {
                        // FIXME Emit div by zero, men i analyze-pass?
                        // Eventuelt return en Some(...::NaN)??
                        return None;
                    }
                    Some(PHPValue::Float(PHPFloat::new(lfloat / rfloat)))
                }
            }
            (PHPValue::Int(_), PHPValue::Float(b)) => {
                let fval = match b {
                    PHPFloat::Real(f) => f,
                    PHPFloat::NaN | PHPFloat::Infinite => return None,
                };
                if fval == 0.0 {
                    // FIXME Emit div by zero, men i analyze-pass?
                    return None;
                }
                let lfloat = lval.as_php_float()?.as_f64()?;
                Some(PHPValue::Float(PHPFloat::new(lfloat / fval)))
            }
            (PHPValue::Float(a), PHPValue::Int(b)) => {
                let a = match a {
                    PHPFloat::Real(f) => f,
                    PHPFloat::NaN | PHPFloat::Infinite => return None,
                };
                if b == 0 {
                    // FIXME Emit div by zero, men i analyze-pass?
                    return None;
                }
                let rfloat = rval.as_php_float()?.as_f64()?;
                if rfloat == 0.0 {
                    // FIXME Emit div by zero, men i analyze-pass?
                    return None;
                }
                Some(PHPValue::Float(PHPFloat::Real(a / rfloat)))
            }
            (PHPValue::Float(PHPFloat::NaN), PHPValue::Float(_))
            | (PHPValue::Float(_), PHPValue::Float(PHPFloat::NaN)) => None,

            (PHPValue::Float(PHPFloat::Real(a)), PHPValue::Float(PHPFloat::Real(b))) => {
                if b == 0.0 {
                    // FIXME emit div by zero, men gjør det i analyze pass
                    None
                } else {
                    Some(PHPValue::Float(PHPFloat::new(a / b)))
                }
            }
            _ => None,
        }
    }
}
