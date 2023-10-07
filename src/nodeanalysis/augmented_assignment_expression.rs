use crate::{
    analysis::state::AnalysisState,
    autonodes::{
        any::AnyNodeRef,
        augmented_assignment_expression::{
            AugmentedAssignmentExpressionLeft, AugmentedAssignmentExpressionNode,
            AugmentedAssignmentExpressionOperator,
        },
    },
    issue::IssueEmitter,
    types::union::{DiscreteType, UnionType},
    value::PHPValue,
};

use super::analysis::ThirdPassAnalyzeableNode;
use crate::autotree::NodeAccess;

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
        let _left = if let Some(l) = self.left.get_php_value(state, emitter) {
            l
        } else {
            return None;
        };
        let _right = if let Some(r) = self.right.get_php_value(state, emitter) {
            r
        } else {
            return None;
        };
        let _noe: Option<u32> = match *self.operator {
            AugmentedAssignmentExpressionOperator::ModAssign(_, _) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::AndAssign(_, _) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::PowAssign(_, _) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::MultAssign(_, _) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::AddAssign(_, _) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::SubAssign(_, _) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::ConcatAssign(_, _) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::DivAssign(_, _) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::LeftShiftAssign(_, _) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::RightShiftAssign(_, _) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::NullsafeAssign(_, _) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::XorAssign(_, _) => crate::missing_none!(
                "{}.get_php_value(..) [op = {:?}]",
                self.kind(),
                self.operator
            ),
            AugmentedAssignmentExpressionOperator::OrAssign(_, _) => crate::missing_none!(
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
            AugmentedAssignmentExpressionOperator::ModAssign(_, _) => {
                crate::missing_none!("{}.get_utype(..)", self.kind())
            }
            AugmentedAssignmentExpressionOperator::AndAssign(_, _) => {
                crate::missing_none!("{}.get_utype(..)", self.kind())
            }
            AugmentedAssignmentExpressionOperator::PowAssign(_, _) => {
                crate::missing_none!("{}.get_utype(..)", self.kind())
            }
            AugmentedAssignmentExpressionOperator::MultAssign(_, _) => {
                crate::missing_none!("{}.get_utype(..)", self.kind())
            }
            AugmentedAssignmentExpressionOperator::AddAssign(_, _) => {
                if let Some(left_type) = self.left.get_utype(state, emitter) {
                    if left_type.is_float() {
                        return Some(DiscreteType::Float.into());
                    }
                }
                if let Some(right_utype) = self.right.get_utype(state, emitter) {
                    if right_utype.is_int() {
                        return Some(DiscreteType::Int.into());
                    }
                }
                let mut utype = UnionType::new();
                utype.push(DiscreteType::Int);
                utype.push(DiscreteType::Float);
                Some(utype)
            }
            AugmentedAssignmentExpressionOperator::SubAssign(_, _) => {
                crate::missing_none!("{}.get_utype(..)", self.kind())
            }
            AugmentedAssignmentExpressionOperator::ConcatAssign(_, _) => {
                Some(DiscreteType::String.into())
            }
            AugmentedAssignmentExpressionOperator::DivAssign(_, _) => {
                crate::missing_none!("{}.get_utype(..)", self.kind())
            }
            AugmentedAssignmentExpressionOperator::LeftShiftAssign(_, _) => {
                crate::missing_none!("{}.get_utype(..)", self.kind())
            }
            AugmentedAssignmentExpressionOperator::RightShiftAssign(_, _) => {
                crate::missing_none!("{}.get_utype(..)", self.kind())
            }
            AugmentedAssignmentExpressionOperator::NullsafeAssign(_, _) => {
                crate::missing_none!("{}.get_utype(..)", self.kind())
            }
            AugmentedAssignmentExpressionOperator::XorAssign(_, _) => {
                crate::missing_none!("{}.get_utype(..)", self.kind())
            }
            AugmentedAssignmentExpressionOperator::OrAssign(_, _) => {
                crate::missing_none!("{}.get_utype(..)", self.kind())
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
        let (val_type, value) = match *self.operator {
            AugmentedAssignmentExpressionOperator::ModAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),
            AugmentedAssignmentExpressionOperator::AndAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),
            AugmentedAssignmentExpressionOperator::PowAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),
            AugmentedAssignmentExpressionOperator::MultAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),
            AugmentedAssignmentExpressionOperator::AddAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),
            AugmentedAssignmentExpressionOperator::SubAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),
            AugmentedAssignmentExpressionOperator::ConcatAssign(op, _) => {
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
                    crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
                )
            }
            AugmentedAssignmentExpressionOperator::DivAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),

            AugmentedAssignmentExpressionOperator::LeftShiftAssign(_, _) => (
                Some(DiscreteType::Int.into()),
                crate::missing_none!("{}.analyze_round_two(..)", self.kind()),
            ),
            AugmentedAssignmentExpressionOperator::RightShiftAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),
            AugmentedAssignmentExpressionOperator::NullsafeAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),
            AugmentedAssignmentExpressionOperator::XorAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
            ),
            AugmentedAssignmentExpressionOperator::OrAssign(op, _) => (
                None,
                crate::missing_none!("{}[{}].analyze_round_two(..)", self.kind(), op),
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
