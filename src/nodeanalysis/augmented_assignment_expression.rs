use crate::{
    analysis::state::AnalysisState,
    autonodes::{
        any::AnyNodeRef,
        augmented_assignment_expression::{
            AugmentedAssignmentExpressionLeft, AugmentedAssignmentExpressionNode,
            AugmentedAssignmentExpressionOperator,
        },
    },
    issue::{IssueEmitter, VoidEmitter},
    operators::{
        binary::{BinaryAssignmentOperator, BinaryOperatorOperandAccess},
        operator::{Operator, Operators},
    },
    types::union::{DiscreteType, UnionType},
    value::PHPValue,
};

use super::analysis::ThirdPassAnalyzeableNode;
use crate::autotree::NodeAccess;

impl NodeAccess for AugmentedAssignmentExpressionOperator {
    fn brief_desc(&self) -> String {
        match self {
            AugmentedAssignmentExpressionOperator::ModAssign(op) => op.brief_desc(),
            AugmentedAssignmentExpressionOperator::AndAssign(op) => op.brief_desc(),
            AugmentedAssignmentExpressionOperator::PowAssign(op) => op.brief_desc(),
            AugmentedAssignmentExpressionOperator::MultAssign(op) => op.brief_desc(),
            AugmentedAssignmentExpressionOperator::AddAssign(op) => op.brief_desc(),
            AugmentedAssignmentExpressionOperator::SubAssign(op) => op.brief_desc(),
            AugmentedAssignmentExpressionOperator::ConcatAssign(op) => op.brief_desc(),
            AugmentedAssignmentExpressionOperator::DivAssign(op) => op.brief_desc(),
            AugmentedAssignmentExpressionOperator::LeftShiftAssign(op) => op.brief_desc(),
            AugmentedAssignmentExpressionOperator::RightShiftAssign(op) => op.brief_desc(),
            AugmentedAssignmentExpressionOperator::NullsafeAssign(op) => op.brief_desc(),
            AugmentedAssignmentExpressionOperator::XorAssign(op) => op.brief_desc(),
            AugmentedAssignmentExpressionOperator::OrAssign(op) => op.brief_desc(),
            AugmentedAssignmentExpressionOperator::Extra(op) => op.brief_desc(),
        }
    }

    fn range(&self) -> crate::parser::Range {
        match self {
            AugmentedAssignmentExpressionOperator::ModAssign(op) => op.range(),
            AugmentedAssignmentExpressionOperator::AndAssign(op) => op.range(),
            AugmentedAssignmentExpressionOperator::PowAssign(op) => op.range(),
            AugmentedAssignmentExpressionOperator::MultAssign(op) => op.range(),
            AugmentedAssignmentExpressionOperator::AddAssign(op) => op.range(),
            AugmentedAssignmentExpressionOperator::SubAssign(op) => op.range(),
            AugmentedAssignmentExpressionOperator::ConcatAssign(op) => op.range(),
            AugmentedAssignmentExpressionOperator::DivAssign(op) => op.range(),
            AugmentedAssignmentExpressionOperator::LeftShiftAssign(op) => op.range(),
            AugmentedAssignmentExpressionOperator::RightShiftAssign(op) => op.range(),
            AugmentedAssignmentExpressionOperator::NullsafeAssign(op) => op.range(),
            AugmentedAssignmentExpressionOperator::XorAssign(op) => op.range(),
            AugmentedAssignmentExpressionOperator::OrAssign(op) => op.range(),
            AugmentedAssignmentExpressionOperator::Extra(op) => op.range(),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::Operator(Operators::AugmentedAssignment(self))
    }
}

impl AugmentedAssignmentExpressionNode {
    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        self.left.read_from(state, emitter);
        self.right.read_from(state, emitter);
        /*         match &*self.operator {
            AugmentedAssignmentExpressionOperator::ModAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::AndAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::PowAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::MultAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::AddAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::SubAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::ConcatAssign(_, _) => self.right.read_from(state, emitter),
            AugmentedAssignmentExpressionOperator::DivAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::LeftShiftAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::RightShiftAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::NullsafeAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::XorAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),
            AugmentedAssignmentExpressionOperator::OrAssign(op, _) => crate::missing!("{}[{}].read_from(..)", self.kind(), op),

            AugmentedAssignmentExpressionOperator::Comment(_) |
            AugmentedAssignmentExpressionOperator::TextInterpolation(_) |
            AugmentedAssignmentExpressionOperator::Error(_) => (),
        }*/
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        let _left = self.left.get_php_value(state, emitter)?;
        let _right = self.right.get_php_value(state, emitter)?;
        let _noe: Option<u32> = match *self.operator {
            AugmentedAssignmentExpressionOperator::ModAssign(_) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::AndAssign(_) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::PowAssign(_) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::MultAssign(_) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::AddAssign(_) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::SubAssign(_) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::ConcatAssign(_) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::DivAssign(_) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::LeftShiftAssign(_) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::RightShiftAssign(_) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::NullsafeAssign(_) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::XorAssign(_) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::OrAssign(_) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::Extra(_) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
        };
        crate::missing_none!("{}.get_php_value(..)", self.kind())
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        match *self.operator {
            AugmentedAssignmentExpressionOperator::Extra(_) => None,
            _ => self.operator.get_operator_utype(self, state, emitter),
        }
    }
}

impl BinaryOperatorOperandAccess for AugmentedAssignmentExpressionNode {
    fn get_left_value(&self, state: &mut AnalysisState) -> Option<PHPValue> {
        self.left.get_php_value(state, &VoidEmitter::new())
    }

    fn get_right_value(&self, state: &mut AnalysisState) -> Option<PHPValue> {
        self.right.get_php_value(state, &VoidEmitter::new())
    }

    fn get_left_type(&self, state: &mut AnalysisState) -> Option<UnionType> {
        self.left.get_utype(state, &VoidEmitter::new())
    }

    fn get_right_type(&self, state: &mut AnalysisState) -> Option<UnionType> {
        self.right.get_utype(state, &VoidEmitter::new())
    }
}

impl BinaryAssignmentOperator for AugmentedAssignmentExpressionOperator {
    fn get_operator_utype(
        &self,
        operands: &impl BinaryOperatorOperandAccess,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        match self {
            AugmentedAssignmentExpressionOperator::ModAssign(oper) => {
                oper.get_operator_utype(operands, state, emitter)
            }
            AugmentedAssignmentExpressionOperator::AndAssign(oper) => {
                oper.get_operator_utype(operands, state, emitter)
            }
            AugmentedAssignmentExpressionOperator::PowAssign(oper) => {
                oper.get_operator_utype(operands, state, emitter)
            }
            AugmentedAssignmentExpressionOperator::MultAssign(oper) => {
                oper.get_operator_utype(operands, state, emitter)
            }
            AugmentedAssignmentExpressionOperator::AddAssign(oper) => {
                oper.get_operator_utype(operands, state, emitter)
            }
            AugmentedAssignmentExpressionOperator::SubAssign(oper) => {
                oper.get_operator_utype(operands, state, emitter)
            }
            AugmentedAssignmentExpressionOperator::ConcatAssign(oper) => {
                oper.get_operator_utype(operands, state, emitter)
            }
            AugmentedAssignmentExpressionOperator::DivAssign(oper) => {
                oper.get_operator_utype(operands, state, emitter)
            }
            AugmentedAssignmentExpressionOperator::LeftShiftAssign(oper) => {
                oper.get_operator_utype(operands, state, emitter)
            }
            AugmentedAssignmentExpressionOperator::RightShiftAssign(oper) => {
                oper.get_operator_utype(operands, state, emitter)
            }
            AugmentedAssignmentExpressionOperator::NullsafeAssign(oper) => {
                oper.get_operator_utype(operands, state, emitter)
            }
            AugmentedAssignmentExpressionOperator::XorAssign(oper) => {
                oper.get_operator_utype(operands, state, emitter)
            }
            AugmentedAssignmentExpressionOperator::OrAssign(oper) => {
                oper.get_operator_utype(operands, state, emitter)
            }
            AugmentedAssignmentExpressionOperator::Extra(_) => None,
        }
    }
}

// FIXME den her må sjekk at right-operanden har rett type

impl ThirdPassAnalyzeableNode for AugmentedAssignmentExpressionNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        self.right.read_from(state, emitter);
        let maybe_right_utype = self.right.get_utype(state, emitter);
        let maybe_left_utype = self.left.get_utype(state, emitter);

        let right_val = self.right.get_php_value(state, emitter);
        let left_val = self.left.get_php_value(state, emitter);
        let (val_type, value) = match &*self.operator {
            AugmentedAssignmentExpressionOperator::ModAssign(op) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op.operator()),
            ),
            AugmentedAssignmentExpressionOperator::AndAssign(op) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op.operator()),
            ),
            AugmentedAssignmentExpressionOperator::PowAssign(op) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op.operator()),
            ),
            AugmentedAssignmentExpressionOperator::MultAssign(op) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op.operator()),
            ),
            AugmentedAssignmentExpressionOperator::AddAssign(op) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op.operator()),
            ),
            AugmentedAssignmentExpressionOperator::SubAssign(op) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op.operator()),
            ),
            AugmentedAssignmentExpressionOperator::ConcatAssign(op) => {
                if let Some(_utype) = &maybe_left_utype {
                    /*if !utype.can_be_cast_to_string() {
                        emitter.emit();
                    }*/
                } else {
                    crate::missing!("String concat with unknown type. Perhaps emit notice?");
                }

                if let Some(_utype) = &maybe_right_utype {
                    /*if !utype.can_be_cast_to_string() {
                        emitter.emit();
                    }*/
                } else {
                    crate::missing!("String concat with unknown type. Perhaps emit notice?");
                }

                match (left_val, right_val) {
                    (Some(lval), Some(rval)) => {
                        if let (Some(_left_string), Some(_right_string)) =
                            (lval.as_os_string(), rval.as_os_string())
                        {
                            // void
                        }
                    }
                    _ => (),
                }

                (
                    Some(DiscreteType::String.into()),
                    crate::missing_none!(
                        "{}[{}].analyze_round_two(..)",
                        self.kind(),
                        op.operator()
                    ),
                )
            }
            AugmentedAssignmentExpressionOperator::DivAssign(op) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op.operator()),
            ),

            AugmentedAssignmentExpressionOperator::LeftShiftAssign(_) => (
                Some(DiscreteType::Int.into()),
                crate::missing_none!("{}.analyze_round_two(..)", self.kind()),
            ),
            AugmentedAssignmentExpressionOperator::RightShiftAssign(op) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op.operator()),
            ),
            AugmentedAssignmentExpressionOperator::NullsafeAssign(op) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op.operator()),
            ),
            AugmentedAssignmentExpressionOperator::XorAssign(op) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op.operator()),
            ),
            AugmentedAssignmentExpressionOperator::OrAssign(op) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op.operator()),
            ),

            AugmentedAssignmentExpressionOperator::Extra(_) => (None, None),
        };
        // let val_type = self.right.get_utype(state, emitter);
        self.left.write_to(state, emitter, val_type, value);
        self.analyze_third_pass_children(&self.as_any(), state, emitter, path)
    }
}

impl AugmentedAssignmentExpressionLeft {
    pub fn write_to(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn IssueEmitter,
        val_type: Option<UnionType>,
        value: Option<PHPValue>,
    ) {
        match self {
            AugmentedAssignmentExpressionLeft::CastExpression(_) => {
                crate::missing!("{}.write_to(..)", self.brief_desc())
            }
            AugmentedAssignmentExpressionLeft::DynamicVariableName(_) => {
                crate::missing!("{}.write_to(..)", self.brief_desc())
            }
            AugmentedAssignmentExpressionLeft::FunctionCallExpression(_) => {
                crate::missing!("{}.write_to(..)", self.brief_desc())
            }
            AugmentedAssignmentExpressionLeft::MemberAccessExpression(ma) => {
                ma.write_to(state, emitter, val_type, value)
            }
            AugmentedAssignmentExpressionLeft::MemberCallExpression(_) => {
                crate::missing!("{}.write_to(..)", self.brief_desc())
            }
            AugmentedAssignmentExpressionLeft::NullsafeMemberAccessExpression(_) => {
                crate::missing!("{}.write_to(..)", self.brief_desc())
            }
            AugmentedAssignmentExpressionLeft::NullsafeMemberCallExpression(_) => {
                crate::missing!("{}.write_to(..)", self.brief_desc())
            }
            AugmentedAssignmentExpressionLeft::ScopedCallExpression(_) => {
                crate::missing!("{}.write_to(..)", self.brief_desc())
            }
            AugmentedAssignmentExpressionLeft::ScopedPropertyAccessExpression(sp) => {
                sp.write_to(state, emitter, val_type, value)
            }
            AugmentedAssignmentExpressionLeft::SubscriptExpression(se) => {
                se.write_to(state, emitter, val_type, value)
            }
            AugmentedAssignmentExpressionLeft::VariableName(vn) => {
                vn.write_to(state, emitter, val_type, value)
            }

            AugmentedAssignmentExpressionLeft::Extra(_) => (),
        }
    }
}
