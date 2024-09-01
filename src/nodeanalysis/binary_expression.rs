use crate::{
    analysis::state::AnalysisState,
    autonodes::{
        _expression::_ExpressionNode,
        _primary_expression::_PrimaryExpressionNode,
        any::AnyNodeRef,
        binary_expression::{
            BinaryExpressionNode, BinaryExpressionOperator, BinaryExpressionRight,
        },
    },
    issue::{Issue, IssueEmitter, VoidEmitter},
    missing,
    operators::{
        binary::{BinaryOperator, BinaryOperatorOperandAccess, InstanceOfSymbol},
        operator::{Operator, Operators},
    },
    types::union::PHPType,
    value::PHPValue,
};

use super::analysis::{FirstPassAnalyzeableNode, ThirdPassAnalyzeableNode};
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
    ) -> Option<PHPType> {
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
            // Mulig denne bør få Option<PHPType> her for å få
            // bedre sluttresultat
            op => op.get_operator_utype(self, state, emitter),
        }
    }
}

impl BinaryOperator for BinaryExpressionOperator {
    fn get_operator_utype(
        &self,
        operands: &impl BinaryOperatorOperandAccess,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPType> {
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
            BinaryExpressionOperator::Exponential(operator) => {
                operator.get_operator_utype(operands, state, emitter)
            }
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
            BinaryExpressionOperator::Exponential(operator) => {
                operator.get_operator_php_value(operands, state, emitter)
            }

            BinaryExpressionOperator::Extra(_) => None,
        }
    }
}

impl NodeAccess for BinaryExpressionOperator {
    fn brief_desc(&self) -> String {
        match self {
            BinaryExpressionOperator::NotEqual(op) => op.brief_desc(),
            BinaryExpressionOperator::NotIdentical(op) => op.brief_desc(),
            BinaryExpressionOperator::Mod(op) => op.brief_desc(),
            BinaryExpressionOperator::BinaryAnd(op) => op.brief_desc(),
            BinaryExpressionOperator::BooleanAnd(op) => op.brief_desc(),
            BinaryExpressionOperator::Mult(op) => op.brief_desc(),
            BinaryExpressionOperator::Add(op) => op.brief_desc(),
            BinaryExpressionOperator::Sub(op) => op.brief_desc(),
            BinaryExpressionOperator::Concat(op) => op.brief_desc(),
            BinaryExpressionOperator::Div(op) => op.brief_desc(),
            BinaryExpressionOperator::LessThan(op) => op.brief_desc(),
            BinaryExpressionOperator::LeftShift(op) => op.brief_desc(),
            BinaryExpressionOperator::LessThanOrEqual(op) => op.brief_desc(),
            BinaryExpressionOperator::Spaceship(op) => op.brief_desc(),
            BinaryExpressionOperator::Equal(op) => op.brief_desc(),
            BinaryExpressionOperator::Identical(op) => op.brief_desc(),
            BinaryExpressionOperator::GreaterThan(op) => op.brief_desc(),
            BinaryExpressionOperator::GreaterThanOrEqual(op) => op.brief_desc(),
            BinaryExpressionOperator::RightShift(op) => op.brief_desc(),
            BinaryExpressionOperator::NullCoalesce(op) => op.brief_desc(),
            BinaryExpressionOperator::BinaryXor(op) => op.brief_desc(),
            BinaryExpressionOperator::LogicalAnd(op) => op.brief_desc(),
            BinaryExpressionOperator::Instanceof(op) => op.brief_desc(),
            BinaryExpressionOperator::LogicalOr(op) => op.brief_desc(),
            BinaryExpressionOperator::LogicalXor(op) => op.brief_desc(),
            BinaryExpressionOperator::BinaryOr(op) => op.brief_desc(),
            BinaryExpressionOperator::BooleanOr(op) => op.brief_desc(),
            BinaryExpressionOperator::Exponential(ex) => ex.brief_desc(),
            BinaryExpressionOperator::Extra(ex) => ex.brief_desc(),
        }
    }

    fn range(&self) -> crate::parser::Range {
        match self {
            BinaryExpressionOperator::NotEqual(op) => op.range(),
            BinaryExpressionOperator::NotIdentical(op) => op.range(),
            BinaryExpressionOperator::Mod(op) => op.range(),
            BinaryExpressionOperator::BinaryAnd(op) => op.range(),
            BinaryExpressionOperator::BooleanAnd(op) => op.range(),
            BinaryExpressionOperator::Mult(op) => op.range(),
            BinaryExpressionOperator::Add(op) => op.range(),
            BinaryExpressionOperator::Sub(op) => op.range(),
            BinaryExpressionOperator::Concat(op) => op.range(),
            BinaryExpressionOperator::Div(op) => op.range(),
            BinaryExpressionOperator::LessThan(op) => op.range(),
            BinaryExpressionOperator::LeftShift(op) => op.range(),
            BinaryExpressionOperator::LessThanOrEqual(op) => op.range(),
            BinaryExpressionOperator::Spaceship(op) => op.range(),
            BinaryExpressionOperator::Equal(op) => op.range(),
            BinaryExpressionOperator::Identical(op) => op.range(),
            BinaryExpressionOperator::GreaterThan(op) => op.range(),
            BinaryExpressionOperator::GreaterThanOrEqual(op) => op.range(),
            BinaryExpressionOperator::RightShift(op) => op.range(),
            BinaryExpressionOperator::NullCoalesce(op) => op.range(),
            BinaryExpressionOperator::BinaryXor(op) => op.range(),
            BinaryExpressionOperator::LogicalAnd(op) => op.range(),
            BinaryExpressionOperator::Instanceof(op) => op.range(),
            BinaryExpressionOperator::LogicalOr(op) => op.range(),
            BinaryExpressionOperator::LogicalXor(op) => op.range(),
            BinaryExpressionOperator::BinaryOr(op) => op.range(),
            BinaryExpressionOperator::BooleanOr(op) => op.range(),
            BinaryExpressionOperator::Exponential(ex) => ex.range(),
            BinaryExpressionOperator::Extra(ex) => ex.range(),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::Operator(Operators::Binary(self))
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        vec![]
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

impl BinaryExpressionNode {
    pub fn first_pass_check_instanceof(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) {
        // thing to check:
        //
        // Dependening on versions, a left-hand side of instanceof can not be a constant
        // > As of PHP 7.3.0, constants are allowed on the left-hand-side of the instanceof operator.
        //
        // The right hand side can also be verified to some extent
        // > As of PHP 8.0.0, instanceof can now be used with arbitrary expressions.
        // > The expression must be wrapped in parentheses and produce a string.
        //

        if !state.config.php_version.is_less_than(7, 3, 0) {
            return;
        };

        let _ExpressionNode::_PrimaryExpression(p) = &self.left else {
            return;
        };

        let (_PrimaryExpressionNode::ClassConstantAccessExpression(_)
        | _PrimaryExpressionNode::Name(_)
        | _PrimaryExpressionNode::QualifiedName(_)) = &**p
        else {
            return;
        };

        emitter.emit(Issue::IllegalTypeInInstanceof(
            self.pos(state),
            "In PHP less than 7.3.0 constants can not be on the left side of instanceof"
                .to_string(),
        ));
    }
}

impl FirstPassAnalyzeableNode for BinaryExpressionNode {
    fn analyze_first_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match *self.operator {
            BinaryExpressionOperator::NotEqual(_) => crate::missing!(),
            BinaryExpressionOperator::NotIdentical(_) => crate::missing!(),
            BinaryExpressionOperator::Mod(_) => crate::missing!(),
            BinaryExpressionOperator::BinaryAnd(_) => crate::missing!(),
            BinaryExpressionOperator::BooleanAnd(_) => crate::missing!(),
            BinaryExpressionOperator::Mult(_) => crate::missing!(),
            BinaryExpressionOperator::Exponential(_) => crate::missing!(),
            BinaryExpressionOperator::Add(_) => crate::missing!(),
            BinaryExpressionOperator::Sub(_) => crate::missing!(),
            BinaryExpressionOperator::Concat(_) => crate::missing!(),
            BinaryExpressionOperator::Div(_) => crate::missing!(),
            BinaryExpressionOperator::LessThan(_) => crate::missing!(),
            BinaryExpressionOperator::LeftShift(_) => crate::missing!(),
            BinaryExpressionOperator::LessThanOrEqual(_) => crate::missing!(),
            BinaryExpressionOperator::Spaceship(_) => crate::missing!(),
            BinaryExpressionOperator::Equal(_) => crate::missing!(),
            BinaryExpressionOperator::Identical(_) => crate::missing!(),
            BinaryExpressionOperator::GreaterThan(_) => crate::missing!(),
            BinaryExpressionOperator::GreaterThanOrEqual(_) => crate::missing!(),
            BinaryExpressionOperator::RightShift(_) => crate::missing!(),
            BinaryExpressionOperator::NullCoalesce(_) => crate::missing!(),
            BinaryExpressionOperator::BinaryXor(_) => crate::missing!(),
            BinaryExpressionOperator::LogicalAnd(_) => crate::missing!(),
            BinaryExpressionOperator::Instanceof(_) => {
                self.first_pass_check_instanceof(state, emitter)
            }
            BinaryExpressionOperator::LogicalOr(_) => crate::missing!(),
            BinaryExpressionOperator::LogicalXor(_) => crate::missing!(),
            BinaryExpressionOperator::BinaryOr(_) => crate::missing!(),
            BinaryExpressionOperator::BooleanOr(_) => crate::missing!(),
            BinaryExpressionOperator::Extra(_) => crate::missing!(),
        }
    }
}

impl ThirdPassAnalyzeableNode for BinaryExpressionNode {
    fn analyze_third_pass(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn crate::issue::IssueEmitter,
        path: &[AnyNodeRef],
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

    fn get_left_type(&self, state: &mut AnalysisState) -> Option<PHPType> {
        self.left.get_utype(state, &VoidEmitter::new())
    }

    fn get_right_value(&self, state: &mut AnalysisState) -> Option<PHPValue> {
        self.right.get_php_value(state, &VoidEmitter::new())
    }

    fn get_right_type(&self, state: &mut AnalysisState) -> Option<PHPType> {
        self.right.get_utype(state, &VoidEmitter::new())
    }

    fn get_right_symbol(&self, state: &mut AnalysisState) -> Option<InstanceOfSymbol> {
        match &*self.right {
            BinaryExpressionRight::_Expression(_) => {
                crate::missing_none!("binary get_right_symbol with _Expression")
            }
            BinaryExpressionRight::DynamicVariableName(_) => {
                crate::missing_none!("binary get_right_symbol with DynamicVariableName")
            }
            BinaryExpressionRight::MemberAccessExpression(_) => {
                crate::missing_none!("binary get_right_symbol with MemberAccessExpression")
            }
            BinaryExpressionRight::Name(n) => Some(InstanceOfSymbol::FullyQualifiedName(
                state.get_fq_symbol_name_from_local_name(&n.get_name()),
            )),
            BinaryExpressionRight::NullsafeMemberAccessExpression(_) => {
                crate::missing_none!("binary get_right_symbol with NullsafeMemberAccessExpression")
            }
            BinaryExpressionRight::QualifiedName(q) => {
                Some(InstanceOfSymbol::FullyQualifiedName(q.get_fq_name(state)))
            }
            BinaryExpressionRight::ScopedPropertyAccessExpression(_) => {
                crate::missing_none!("binary get_right_symbol with ScopedPropertyAccessExpression")
            }
            BinaryExpressionRight::SubscriptExpression(_) => {
                crate::missing_none!("binary get_right_symbol with SubscriptExpression")
            }
            BinaryExpressionRight::VariableName(_) => {
                crate::missing_none!("binary get_right_symbol with VariableName")
            }
            BinaryExpressionRight::Extra(_) => {
                crate::missing_none!("binary get_right_symbol with Extra")
            }
        }
    }
}
