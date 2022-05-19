use crate::{
    analysis::state::AnalysisState,
    autonodes::{
        any::AnyNodeRef,
        binary_expression::{
            BinaryExpressionNode, BinaryExpressionOperator, BinaryExpressionRight,
        },
    },
    issue::IssueEmitter,
    missing,
    types::union::{DiscreteType, UnionType},
    value::{PHPFloat, PHPValue},
};

use super::analysis::ThirdPassAnalyzeableNode;
use crate::autotree::NodeAccess;

impl BinaryExpressionNode {
    pub fn read_from(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn crate::issue::IssueEmitter,
    ) {
        // FIXME might be able to determine more precisely if both left and right are viabla paths
        // i.e.: `false && $a` should probably not mark $a as read from...

        self.left.as_ref().map(|x| x.read_from(state, emitter));
        self.right.as_ref().map(|y| y.read_from(state, emitter));
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        let lval = self
            .left
            .as_ref()
            .and_then(|x| x.get_php_value(state, emitter))?;
        let rval = self
            .right
            .as_ref()
            .and_then(|x| x.get_php_value(state, emitter))?;

        let ltype = lval.get_utype();
        let rtype = rval.get_utype();

        let op = self.operator.as_ref()?;
        match &**op {
            // comparison
            BinaryExpressionOperator::NotEqual(_, _) => {
                Some(PHPValue::Boolean(!lval.equal_to(&rval)?))
            }
            BinaryExpressionOperator::NotIdentical(_, _) => {
                Some(PHPValue::Boolean(!lval.identical_to(&rval)?))
            }
            BinaryExpressionOperator::LessThan(be, _) => match (&lval, &rval) {
                (PHPValue::Int(lint), PHPValue::Int(rint)) => Some(PHPValue::Boolean(lint < rint)),
                (PHPValue::Float(lint), PHPValue::Float(rint)) => {
                    Some(PHPValue::Boolean(lint < rint))
                }
                _ => crate::missing_none!(
                    "{}[{:?} {} {:?}].get_php_value(..)",
                    self.brief_desc(),
                    ltype,
                    be,
                    rtype
                ),
            },
            BinaryExpressionOperator::LessThanOrEqual(be, _) => match (&lval, &rval) {
                (PHPValue::Int(lint), PHPValue::Int(rint)) => Some(PHPValue::Boolean(lint <= rint)),
                (PHPValue::Float(lint), PHPValue::Float(rint)) => {
                    Some(PHPValue::Boolean(lint <= rint))
                }
                _ => crate::missing_none!(
                    "{}[{:?} {} {:?}].get_php_value(..)",
                    self.brief_desc(),
                    ltype,
                    be,
                    rtype
                ),
            },
            BinaryExpressionOperator::GreaterThan(be, _) => match (&lval, &rval) {
                (PHPValue::Int(lint), PHPValue::Int(rint)) => Some(PHPValue::Boolean(lint > rint)),
                (PHPValue::Float(lint), PHPValue::Float(rint)) => {
                    Some(PHPValue::Boolean(lint > rint))
                }
                _ => crate::missing_none!(
                    "{}[{:?} {} {:?}].get_php_value(..)",
                    self.brief_desc(),
                    ltype,
                    be,
                    rtype
                ),
            },
            BinaryExpressionOperator::GreaterThanOrEqual(be, _) => match (&lval, &rval) {
                (PHPValue::Int(lint), PHPValue::Int(rint)) => Some(PHPValue::Boolean(lint >= rint)),
                (PHPValue::Float(lint), PHPValue::Float(rint)) => {
                    Some(PHPValue::Boolean(lint >= rint))
                }
                _ => crate::missing_none!(
                    "{}[{:?} {} {:?}].get_php_value(..)",
                    self.brief_desc(),
                    ltype,
                    be,
                    rtype
                ),
            },
            BinaryExpressionOperator::Spaceship(be, _) => {
                crate::missing_none!("{}[{}].get_php_value(..)", self.brief_desc(), be)
            }
            BinaryExpressionOperator::Equal(_, _) => Some(PHPValue::Boolean(lval.equal_to(&rval)?)),
            BinaryExpressionOperator::Identical(_, _) => {
                Some(PHPValue::Boolean(lval.identical_to(&rval)?))
            }

            // class
            BinaryExpressionOperator::Instanceof(be, _) => {
                crate::missing_none!("{}[{}].get_php_value(..)", self.brief_desc(), be)
            }

            // boolean
            BinaryExpressionOperator::And(_, _) | BinaryExpressionOperator::BooleanAnd(_, _) => {
                let left = lval.as_bool()?;
                let right = rval.as_bool()?;
                // FIXME overflow og sånt her bør trigg emitting
                Some(PHPValue::Boolean(left && right))
            }

            BinaryExpressionOperator::Or(_, _) | BinaryExpressionOperator::BooleanOr(_, _) => {
                let left = lval.as_bool()?;
                let right = rval.as_bool()?;

                Some(PHPValue::Boolean(left || right))
            }

            // numerical
            BinaryExpressionOperator::Mod(_, _) => {
                let left = lval.as_i64()?;
                let right = rval.as_i64()?;
                // FIXME overflow og sånt her bør trigg emitting
                Some(PHPValue::Int(left % right))
            }
            BinaryExpressionOperator::BinaryAnd(_, _) => {
                let left = lval.as_i64()?;
                let right = rval.as_i64()?;

                Some(PHPValue::Int(left & right))
            }
            BinaryExpressionOperator::BinaryXor(_, _) => {
                let left = lval.as_i64()?;
                let right = rval.as_i64()?;

                Some(PHPValue::Int(left ^ right))
            }
            BinaryExpressionOperator::Xor(_, _) => {
                let left = lval.as_bool()?;
                let right = rval.as_bool()?;

                Some(PHPValue::Boolean(left ^ right))
            }

            BinaryExpressionOperator::BinaryOr(_, _) => {
                let left = lval.as_i64()?;
                let right = rval.as_i64()?;

                Some(PHPValue::Int(left | right))
            }

            // operator
            BinaryExpressionOperator::Div(_, _) => {
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
                            let rfloat = lval.as_php_float()?.as_f64()?;

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
            BinaryExpressionOperator::Add(_, _) => {
                let left = lval.as_php_num()?;
                let right = rval.as_php_num()?;
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
            BinaryExpressionOperator::Sub(_, _) => {
                let left = lval.as_php_num()?;
                let right = rval.as_php_num()?;
                match (left, right) {
                    (PHPValue::Int(a), PHPValue::Int(b)) => Some(PHPValue::Int(a - b)),
                    (PHPValue::Int(_a), PHPValue::Float(_b)) => {
                        crate::missing_none!("i64 to f64 conversion")
                    } // Some(PHPValue::Float(a.into()-b)),
                    (PHPValue::Float(_a), PHPValue::Int(_b)) => {
                        crate::missing_none!("i64 to f64 conversion")
                    } // Some(PHPValue::Float(a-b.into())),
                    (PHPValue::Float(PHPFloat::Real(a)), PHPValue::Float(PHPFloat::Real(b))) => {
                        Some(PHPValue::Float(PHPFloat::new(a - b)))
                    }
                    _ => None,
                }
            }
            BinaryExpressionOperator::Mult(_, _) => {
                let left = lval.as_php_num()?;
                let right = rval.as_php_num()?;
                match (left, right) {
                    (PHPValue::Int(a), PHPValue::Int(b)) => Some(PHPValue::Int(a * b)),
                    (PHPValue::Int(_a), PHPValue::Float(_b)) => {
                        crate::missing_none!("i64 to f64 conversion")
                    } // Some(PHPValue::Float(b*a.into())),
                    (PHPValue::Float(_a), PHPValue::Int(_b)) => {
                        crate::missing_none!("i64 to f64 conversion")
                    } // Some(PHPValue::Float(a*b.into())),
                    (PHPValue::Float(PHPFloat::Real(a)), PHPValue::Float(PHPFloat::Real(b))) => {
                        Some(PHPValue::Float(PHPFloat::new(a * b)))
                    }
                    _ => None,
                }
            }

            BinaryExpressionOperator::RightShift(_, _) => {
                let left = lval.as_i64()?;
                let right = rval.as_i64()?;
                // FIXME overflow og sånt her bør trigg emitting
                Some(PHPValue::Int(left >> right))
            }
            BinaryExpressionOperator::LeftShift(_, _) => {
                let left = lval.as_i64()?;
                let right = rval.as_i64()?;
                // FIXME overflow og sånt her bør trigg emitting

                /*eprintln!(
                    "Attempting left-shift: {} << {} fra {}",
                    left,
                    right,
                    state.pos_as_string(self.range)
                );*/
                Some(PHPValue::Int(left << right))
            }
            BinaryExpressionOperator::Concat(_, _) => {
                let mut left = lval.as_os_string()?;
                let right = rval.as_os_string()?;
                left.push(right);
                Some(PHPValue::String(left))
            }

            // void
            BinaryExpressionOperator::Comment(_)
            | BinaryExpressionOperator::TextInterpolation(_)
            | BinaryExpressionOperator::Error(_) => None,
        }
        // crate::missing_none!("{}.get_php_value(..)", self.kind())
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        /*
         * There is a challenge here in that the matematical operators for the most part return int or float, regardless
         * of input, so you could argue that `anything + anything` should be a union(int|float) or similar, however
         * if we can't make sure that anything is not i.e. an array, this might crash, as `[] + 0` is fatal in newer PHP
         *
         * Therefor the strategy here is to only make promises we can keep. Only return the correct type if we can
         * be certain of the type of the arguments
         */
        let operator = self.operator.as_ref()?;
        if let BinaryExpressionOperator::Instanceof(_, _) = &**operator {
            // FIXME verify that instanceof is valid for all types
            return Some(DiscreteType::Bool.into());
        }

        // For all other operators, ensure that we have known types on both sides
        let ltype = self
            .left
            .as_ref()
            .and_then(|x| x.get_utype(state, emitter))
            .and_then(|x| x.single_type())?;
        let rtype = self
            .right
            .as_ref()
            .and_then(|x| x.get_utype(state, emitter))
            .and_then(|x| x.single_type())?;
        match &**operator {
            BinaryExpressionOperator::NotEqual(_, _)
            | BinaryExpressionOperator::NotIdentical(_, _)
            | BinaryExpressionOperator::LessThan(_, _)
            | BinaryExpressionOperator::LessThanOrEqual(_, _)
            | BinaryExpressionOperator::Identical(_, _)
            | BinaryExpressionOperator::GreaterThan(_, _)
            | BinaryExpressionOperator::GreaterThanOrEqual(_, _)
            | BinaryExpressionOperator::BooleanAnd(_, _)
            | BinaryExpressionOperator::Instanceof(_, _)
            | BinaryExpressionOperator::BooleanOr(_, _)
            | BinaryExpressionOperator::Equal(_, _)
            | BinaryExpressionOperator::And(_, _)
            | BinaryExpressionOperator::Or(_, _)
            | BinaryExpressionOperator::Xor(_, _) => Some(DiscreteType::Bool.into()), // FIXME ensure that this is valid for all types

            // Int
            BinaryExpressionOperator::Mod(_, _)
            | BinaryExpressionOperator::BinaryAnd(_, _)
            | BinaryExpressionOperator::LeftShift(_, _)
            | BinaryExpressionOperator::Spaceship(_, _)
            | BinaryExpressionOperator::RightShift(_, _)
            | BinaryExpressionOperator::BinaryXor(_, _)
            | BinaryExpressionOperator::BinaryOr(_, _) => Some(DiscreteType::Int.into()), // FIXME ensure that this is valid for all types

            // Num
            BinaryExpressionOperator::Mult(op, _) => match (&ltype, &rtype) {
                (DiscreteType::Int, DiscreteType::Int) => Some(DiscreteType::Int.into()),
                (DiscreteType::Float, DiscreteType::Int) => Some(DiscreteType::Float.into()),
                (DiscreteType::Int, DiscreteType::Float) => Some(DiscreteType::Float.into()),
                (DiscreteType::Float, DiscreteType::Float) => Some(DiscreteType::Float.into()),
                (DiscreteType::Unknown, _) | (_, DiscreteType::Unknown) => None,

                _ => crate::missing_none!("{:?} {} {:?}", ltype, op, rtype),
            },
            BinaryExpressionOperator::Add(op, _) => match (&ltype, &rtype) {
                (DiscreteType::Int, DiscreteType::Int) => Some(DiscreteType::Int.into()),
                (DiscreteType::Float, DiscreteType::Int)
                | (DiscreteType::Int, DiscreteType::Float)
                | (DiscreteType::Float, DiscreteType::Float) => Some(DiscreteType::Float.into()),
                (DiscreteType::Int, DiscreteType::Bool)
                | (DiscreteType::Bool, DiscreteType::Bool)
                | (DiscreteType::Bool, DiscreteType::Int) => Some(DiscreteType::Int.into()),
                (DiscreteType::Unknown, _) | (_, DiscreteType::Unknown) => None,
                _ => crate::missing_none!("{:?} {} {:?}", ltype, op, rtype),
            },
            BinaryExpressionOperator::Sub(op, _) => match (&ltype, &rtype) {
                (DiscreteType::Int, DiscreteType::Int) => Some(DiscreteType::Int.into()),
                (DiscreteType::Float, DiscreteType::Int) => Some(DiscreteType::Float.into()),
                (DiscreteType::Int, DiscreteType::Float) => Some(DiscreteType::Float.into()),
                (DiscreteType::Float, DiscreteType::Float) => Some(DiscreteType::Float.into()),
                (DiscreteType::Unknown, _) | (_, DiscreteType::Unknown) => None,

                // These are failures in PHP8
                (DiscreteType::String, DiscreteType::Int)
                | (DiscreteType::Int, DiscreteType::String) => Some(UnionType::from(vec![
                    DiscreteType::Int,
                    DiscreteType::Float,
                ])),
                (DiscreteType::String, DiscreteType::Float)
                | (DiscreteType::Float, DiscreteType::String) => Some(DiscreteType::Float.into()),

                _ => crate::missing_none!("{:?} {} {:?}", ltype, op, rtype),
            },
            BinaryExpressionOperator::Div(op, _) => match (&ltype, &rtype) {
                (DiscreteType::Int, DiscreteType::Int) => {
                    let lval = self
                        .left
                        .as_ref()
                        .and_then(|x| x.get_php_value(state, emitter));
                    let rval = self
                        .right
                        .as_ref()
                        .and_then(|x| x.get_php_value(state, emitter));
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
                _ => crate::missing_none!("{:?} {} {:?}", ltype, op, rtype),
            },

            // String
            BinaryExpressionOperator::Concat(_, _) => Some(DiscreteType::String.into()),

            BinaryExpressionOperator::Comment(_)
            | BinaryExpressionOperator::TextInterpolation(_)
            | BinaryExpressionOperator::Error(_) => None,
        }
    }
}

impl BinaryExpressionRight {
    pub fn xx_read_from(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn crate::issue::IssueEmitter,
    ) {
        match self {
            BinaryExpressionRight::_Expression(e) => e.read_from(state, emitter),
            // BinaryExpressionRight::DynamicVariableName(_) => todo!(),
            // BinaryExpressionRight::MemberAccessExpression(_) => todo!(),
            // BinaryExpressionRight::Name(_) => todo!(),
            // BinaryExpressionRight::NullsafeMemberAccessExpression(_) => todo!(),
            // BinaryExpressionRight::QualifiedName(_) => todo!(),
            BinaryExpressionRight::ScopedPropertyAccessExpression(se) => {
                se.read_from(state, emitter)
            }
            // BinaryExpressionRight::SubscriptExpression(_) => todo!(),
            // BinaryExpressionRight::VariableName(_) => todo!(),
            BinaryExpressionRight::Comment(_)
            | BinaryExpressionRight::TextInterpolation(_)
            | BinaryExpressionRight::Error(_) => (),

            _ => missing!("BinaryExpressionRight.read_from({})", self.brief_desc()),
        }
    }
}

impl ThirdPassAnalyzeableNode for BinaryExpressionNode {
    fn analyze_third_pass(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn crate::issue::IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        /* if let Some(operator) = &self.operator  {
            match &**operator {
                BinaryExpressionOperator::NotEqual(_, _) => todo!(),
                BinaryExpressionOperator::NotIdentical(_, _) => todo!(),
                BinaryExpressionOperator::Mod(_, _) => todo!(),
                BinaryExpressionOperator::BinaryAnd(_, _) => todo!(),
                BinaryExpressionOperator::BooleanAnd(_, _) => todo!(),
                BinaryExpressionOperator::Mult(_, _) => todo!(),
                BinaryExpressionOperator::Add(_, _) => todo!(),
                BinaryExpressionOperator::Sub(_, _) => todo!(),
                BinaryExpressionOperator::Concat(_, _) => todo!(),
                BinaryExpressionOperator::Div(_, _) => todo!(),
                BinaryExpressionOperator::LessThan(_, _) => todo!(),
                BinaryExpressionOperator::LeftShift(_, _) => todo!(),
                BinaryExpressionOperator::LessThanOrEqual(_, _) => todo!(),
                BinaryExpressionOperator::Spaceship(_, _) => todo!(),
                BinaryExpressionOperator::Equal(_, _) => todo!(),
                BinaryExpressionOperator::Identical(_, _) => todo!(),
                BinaryExpressionOperator::GreaterThan(_, _) => todo!(),
                BinaryExpressionOperator::GreaterThanOrEqual(_, _) => todo!(),
                BinaryExpressionOperator::RightShift(_, _) => todo!(),
                BinaryExpressionOperator::BinaryXor(_, _) => todo!(),
                BinaryExpressionOperator::And(_, _) => todo!(),
                BinaryExpressionOperator::Instanceof(_, _) => todo!(),
                BinaryExpressionOperator::Or(_, _) => todo!(),
                BinaryExpressionOperator::Xor(_, _) => todo!(),
                BinaryExpressionOperator::BinaryOr(_, _) => todo!(),
                BinaryExpressionOperator::BooleanOr(_, _) => todo!(),

                BinaryExpressionOperator::Comment(_) => todo!(),
                BinaryExpressionOperator::TextInterpolation(_) => todo!(),
                BinaryExpressionOperator::Error(_) => todo!(),
            }
        }*/
        // eprintln!("TODO: {}", state.pos_as_string(self.range));
        self.analyze_third_pass_children(&self.as_any(), state, emitter, path)
    }
}
