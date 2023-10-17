use crate::{
    analysis::state::AnalysisState,
    autonodes::{
        any::AnyNodeRef,
        binary_expression::{
            BinaryExpressionNode, BinaryExpressionOperator, BinaryExpressionRight,
        },
    },
    issue::{IssueEmitter, VoidEmitter},
    missing,
    operators::binary::{BinaryOperator, BinaryOperatorOperandAccess},
    types::union::UnionType,
    value::PHPValue,
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

        self.left.read_from(state, emitter);
        self.right.read_from(state, emitter);
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        match &*self.operator {
            // numerical

            // operator

            // void
            BinaryExpressionOperator::Extra(_) => None,
            _ => self.operator.get_operator_php_value(self, state, emitter),
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

        match &*self.operator {
            // String
            BinaryExpressionOperator::Extra(_) => None,
            // Mulig denne bør få Option<UnionType> her for å få
            // bedre sluttresultat
            op @ _ => op.get_operator_utype(self, state, emitter),
        }
    }
}

impl BinaryOperator for BinaryExpressionOperator {
    fn get_operator_utype(
        &self,
        operands: &impl BinaryOperatorOperandAccess,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        match self {
            BinaryExpressionOperator::NotEqual(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::NotIdentical(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::Mod(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::BinaryAnd(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::BooleanAnd(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::Mult(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::Add(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::Sub(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::Concat(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::Div(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::LessThan(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::LeftShift(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::LessThanOrEqual(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::Spaceship(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::Equal(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::Identical(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::GreaterThan(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::GreaterThanOrEqual(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::RightShift(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::NullCoalesce(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::BinaryXor(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::LogicalAnd(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::Instanceof(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::LogicalOr(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::LogicalXor(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::BinaryOr(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::BooleanOr(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
            BinaryExpressionOperator::Extra(_) => None,
        }
    }

    fn get_operator_php_value(
        &self,
        operands: &impl BinaryOperatorOperandAccess,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            BinaryExpressionOperator::NotEqual(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::NotIdentical(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::Mod(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::BinaryAnd(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::BooleanAnd(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::Mult(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::Add(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::Sub(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::Concat(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::Div(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::LessThan(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::LeftShift(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::LessThanOrEqual(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::Spaceship(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::Equal(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::Identical(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::GreaterThan(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::GreaterThanOrEqual(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::RightShift(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::NullCoalesce(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::BinaryXor(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::LogicalAnd(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::Instanceof(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::LogicalOr(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::LogicalXor(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::BinaryOr(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::BooleanOr(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }
            BinaryExpressionOperator::Extra(_) => None,
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
            BinaryExpressionRight::Extra(_) => (),

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

impl BinaryOperatorOperandAccess for BinaryExpressionNode {
    fn get_left_value(&self, state: &mut AnalysisState) -> Option<PHPValue> {
        self.left.get_php_value(state, &VoidEmitter::new())
    }

    fn get_left_type(&self, state: &mut AnalysisState) -> Option<UnionType> {
        self.left.get_utype(state, &VoidEmitter::new())
    }

    fn get_right_value(&self, state: &mut AnalysisState) -> Option<PHPValue> {
        self.right.get_php_value(state, &VoidEmitter::new())
    }

    fn get_right_type(&self, state: &mut AnalysisState) -> Option<UnionType> {
        self.right.get_utype(state, &VoidEmitter::new())
    }
}
