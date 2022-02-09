use crate::{
    analysis::state::AnalysisState,
    autonodes::{
        any::AnyNodeRef,
        update_expression::{
            UpdateExpressionExpr, UpdateExpressionNode, UpdateExpressionPostfix,
            UpdateExpressionPrefix,
        },
    },
    issue::{Issue, IssueEmitter},
    types::union::{DiscreteType, UnionType},
    value::{PHPFloat, PHPValue},
};

use super::analysis::ThirdPassAnalyzeableNode;

enum Operator {
    Increment,
    Decrement,
}

impl UpdateExpressionExpr {
    pub fn write_to(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn IssueEmitter,
        val_type: Option<UnionType>,
        value: Option<PHPValue>,
    ) {
        match self {
            UpdateExpressionExpr::CastExpression(_) => {
                crate::missing!("{}.write_to(..)", self.kind())
            }
            UpdateExpressionExpr::DynamicVariableName(_) => {
                crate::missing!("{}.write_to(..)", self.kind())
            }
            UpdateExpressionExpr::FunctionCallExpression(_) => {
                crate::missing!("{}.write_to(..)", self.kind())
            }
            UpdateExpressionExpr::MemberAccessExpression(_) => {
                crate::missing!("{}.write_to(..)", self.kind())
            }
            UpdateExpressionExpr::MemberCallExpression(_) => {
                crate::missing!("{}.write_to(..)", self.kind())
            }
            UpdateExpressionExpr::NullsafeMemberAccessExpression(_) => {
                crate::missing!("{}.write_to(..)", self.kind())
            }
            UpdateExpressionExpr::NullsafeMemberCallExpression(_) => {
                crate::missing!("{}.write_to(..)", self.kind())
            }
            UpdateExpressionExpr::ScopedCallExpression(_) => {
                crate::missing!("{}.write_to(..)", self.kind())
            }
            UpdateExpressionExpr::ScopedPropertyAccessExpression(_) => {
                crate::missing!("{}.write_to(..)", self.kind())
            }
            UpdateExpressionExpr::SubscriptExpression(_) => {
                crate::missing!("{}.write_to(..)", self.kind())
            }
            UpdateExpressionExpr::VariableName(vn) => vn.write_to(state, emitter, val_type, value),

            UpdateExpressionExpr::Comment(_)
            | UpdateExpressionExpr::TextInterpolation(_)
            | UpdateExpressionExpr::Error(_) => (),
        }
    }
}

impl UpdateExpressionNode {
    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        self.expr.read_from(state, emitter)
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        let val = self.expr.get_php_value(state, emitter)?;

        if let Some(prefix) = &self.prefix {
            match (&**prefix, &val) {
                (UpdateExpressionPrefix::Increment(_, _), PHPValue::NULL) => Some(PHPValue::Int(1)),
                (UpdateExpressionPrefix::Increment(_, _), PHPValue::Boolean(_)) => Some(val),
                (UpdateExpressionPrefix::Increment(_, _), PHPValue::Int(i)) => {
                    Some(PHPValue::Int(i + 1))
                }
                (UpdateExpressionPrefix::Increment(_, _), PHPValue::Float(PHPFloat::Real(f))) => {
                    Some(PHPValue::Float(PHPFloat::new(f + 1.0)))
                }

                (UpdateExpressionPrefix::Decrement(_, _), PHPValue::NULL) => Some(PHPValue::NULL),
                (UpdateExpressionPrefix::Decrement(_, _), PHPValue::Boolean(_)) => Some(val),
                (UpdateExpressionPrefix::Decrement(_, _), PHPValue::Int(i)) => {
                    Some(PHPValue::Int(i - 1))
                }
                (UpdateExpressionPrefix::Decrement(_, _), PHPValue::Float(PHPFloat::Real(f))) => {
                    Some(PHPValue::Float(PHPFloat::new(f - 1.0)))
                }
                (_, PHPValue::Float(_)) => crate::missing_none!(
                    "++$none_finite_float/--$none_finite_float is not implemented"
                ),
                (_, PHPValue::String(_)) => crate::missing_none!("++$str/--$str does funky things"),
                (_, PHPValue::Array(_)) => None, // this emits in analysis round two
                (_, PHPValue::ObjectInstance(_)) => None, // this emits in analysis round two,

                (UpdateExpressionPrefix::Comment(_), _)
                | (UpdateExpressionPrefix::TextInterpolation(_), _)
                | (UpdateExpressionPrefix::Error(_), _) => None,
            }
        } else if let Some(_) = &self.postfix {
            Some(val)
        } else {
            None
        }
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        if let Some(prefix) = &self.prefix {
            let expr_type = self.expr.get_utype(state, emitter)?.single_type()?;

            match (&**prefix, expr_type) {
                (UpdateExpressionPrefix::Increment(_, _), DiscreteType::Int)
                | (UpdateExpressionPrefix::Decrement(_, _), DiscreteType::Int) => {
                    Some(DiscreteType::Int.into())
                }

                (UpdateExpressionPrefix::Increment(_, _), DiscreteType::Float)
                | (UpdateExpressionPrefix::Decrement(_, _), DiscreteType::Float) => {
                    Some(DiscreteType::Float.into())
                }

                (UpdateExpressionPrefix::Increment(_, _), DiscreteType::Bool)
                | (UpdateExpressionPrefix::Decrement(_, _), DiscreteType::Bool) => {
                    Some(DiscreteType::Bool.into())
                }

                (UpdateExpressionPrefix::Increment(_, _), DiscreteType::String)
                | (UpdateExpressionPrefix::Decrement(_, _), DiscreteType::String) => {
                    Some(DiscreteType::String.into())
                }

                (UpdateExpressionPrefix::Increment(_, _), DiscreteType::NULL) => {
                    Some(DiscreteType::Int.into())
                }

                (UpdateExpressionPrefix::Decrement(_, _), DiscreteType::NULL) => {
                    Some(DiscreteType::NULL.into())
                }

                _ => None,
            }
        } else {
            // we're a postfix-operator, we'll return the type of the expr
            self.expr.get_utype(state, emitter)
        }
    }

    fn prefix_op(&self) -> Option<Operator> {
        self.prefix.as_ref().and_then(|op_ref| match &**op_ref {
            UpdateExpressionPrefix::Increment(_, _) => Some(Operator::Increment),
            UpdateExpressionPrefix::Decrement(_, _) => Some(Operator::Decrement),

            UpdateExpressionPrefix::Comment(_)
            | UpdateExpressionPrefix::TextInterpolation(_)
            | UpdateExpressionPrefix::Error(_) => None,
        })
    }

    fn postfix_op(&self) -> Option<Operator> {
        self.postfix.as_ref().and_then(|op_ref| match &**op_ref {
            UpdateExpressionPostfix::Increment(_, _) => Some(Operator::Increment),
            UpdateExpressionPostfix::Decrement(_, _) => Some(Operator::Decrement),

            UpdateExpressionPostfix::Comment(_)
            | UpdateExpressionPostfix::TextInterpolation(_)
            | UpdateExpressionPostfix::Error(_) => None,
        })
    }

    fn op(&self) -> Option<Operator> {
        self.prefix_op().or_else(|| self.postfix_op())
    }
}

// FIXME here are plenty to analyze
// --NULL == NULL
// ++NULL == 1
// ++true == true
// ++false == false
//

use crate::autotree::NodeAccess;
impl ThirdPassAnalyzeableNode for UpdateExpressionNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        if !self.analyze_third_pass_children(&self.as_any(), state, emitter, path) {
            return false;
        }

        if let Some(val) = self.expr.get_php_value(state, emitter) {
            let new_value = match self.op() {
                Some(Operator::Increment) => match val {
                    PHPValue::NULL => Some(PHPValue::Int(1)),
                    PHPValue::Boolean(_) => None,
                    PHPValue::Int(i) => Some(PHPValue::Int(i + 1)),
                    PHPValue::Float(PHPFloat::Real(f)) => {
                        Some(PHPValue::Float(PHPFloat::new(f + 1.0)))
                    }
                    PHPValue::Float(_) => crate::missing_none!("None-Real float increment"),
                    PHPValue::String(_) => crate::missing_none!("String increment"),
                    PHPValue::Array(_) => {
                        let atype = val
                            .get_utype()
                            .unwrap_or_else(|| DiscreteType::Unknown.into());
                        emitter.emit(Issue::IncrementIsIllegalOnType(self.pos(state), atype));
                        None
                    }
                    PHPValue::ObjectInstance(oi) => {
                        emitter.emit(Issue::IncrementIsIllegalOnType(
                            self.pos(state),
                            oi.get_utype(),
                        ));
                        None
                    }
                },
                Some(Operator::Decrement) => match val {
                    PHPValue::NULL => None,
                    PHPValue::Boolean(_) => None,
                    PHPValue::Int(i) => Some(PHPValue::Int(i - 1)),
                    PHPValue::Float(PHPFloat::Real(f)) => {
                        Some(PHPValue::Float(PHPFloat::new(f - 1.0)))
                    }
                    PHPValue::Float(_) => crate::missing_none!("None-Real float decrement"),
                    PHPValue::String(_) => crate::missing_none!("String decrement"),
                    PHPValue::Array(_) => {
                        let atype = val
                            .get_utype()
                            .unwrap_or_else(|| DiscreteType::Unknown.into());
                        emitter.emit(Issue::DecrementIsIllegalOnType(self.pos(state), atype));

                        None
                    }
                    PHPValue::ObjectInstance(oi) => {
                        emitter.emit(Issue::DecrementIsIllegalOnType(
                            self.pos(state),
                            oi.get_utype(),
                        ));
                        //                        emitter.emit(state.filename.as_ref(), self.range, "-- is illegal on type".into());
                        None
                    }
                },
                _ => None,
            };
            if let Some(value) = new_value {
                self.expr
                    .write_to(state, emitter, value.get_utype(), Some(value));
            }

            // let new_val = match self.
        }
        true
    }
}
