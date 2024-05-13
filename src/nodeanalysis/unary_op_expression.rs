use crate::{
    analysis::state::AnalysisState,
    autonodes::{
        any::AnyNodeRef,
        unary_op_expression::{UnaryOpExpressionNode, UnaryOpExpressionOperator},
    },
    autotree::NodeAccess,
    issue::IssueEmitter,
    operators::operator::{Operator, Operators},
    types::union::{DiscreteType, UnionType},
    value::{PHPFloat, PHPValue},
};

impl UnaryOpExpressionNode {
    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        self.argument.read_from(state, emitter);
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        match &*self.operator {
            UnaryOpExpressionOperator::Not(_) => Some(DiscreteType::Bool.into()),
            UnaryOpExpressionOperator::Add(_) => self
                .argument
                .get_php_value(state, emitter)?
                .as_php_num()?
                .get_utype(),
            UnaryOpExpressionOperator::Sub(_) => self
                .argument
                .get_php_value(state, emitter)?
                .as_php_num()?
                .get_utype(),
            UnaryOpExpressionOperator::BinaryNot(_) => Some(DiscreteType::Int.into()),

            UnaryOpExpressionOperator::Extra(_) => None,
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        let operator = &self.operator;

        let value = self.argument.get_php_value(state, emitter)?;

        match (&**operator, value) {
            (UnaryOpExpressionOperator::Not(_), v) => v.as_bool().map(|x| PHPValue::Boolean(!x)),
            (UnaryOpExpressionOperator::Add(op), _) => {
                crate::missing_none!("unary add [{}]", op.operator())
            }
            (UnaryOpExpressionOperator::Sub(_), PHPValue::Int(i)) => Some(PHPValue::Int(-i)),
            (UnaryOpExpressionOperator::Sub(_), PHPValue::Float(PHPFloat::Real(f))) => {
                Some(PHPValue::Float(PHPFloat::new(-f)))
            }
            // (UnaryOpExpressionOperator::Squelch(_, _), _) => todo!(),
            (UnaryOpExpressionOperator::BinaryNot(_), _) => {
                crate::missing_none!("unary binary not")
            }

            (UnaryOpExpressionOperator::Extra(_), _) => None,
            _ => crate::missing_none!("get_php_value: {:?}", self),
        }
    }
}

impl NodeAccess for UnaryOpExpressionOperator {
    fn brief_desc(&self) -> String {
        match self {
            UnaryOpExpressionOperator::Not(op) => op.brief_desc(),
            UnaryOpExpressionOperator::Add(op) => op.brief_desc(),
            UnaryOpExpressionOperator::Sub(op) => op.brief_desc(),
            UnaryOpExpressionOperator::BinaryNot(op) => op.brief_desc(),
            UnaryOpExpressionOperator::Extra(op) => op.brief_desc(),
        }
    }

    fn range(&self) -> crate::parser::Range {
        match self {
            UnaryOpExpressionOperator::Not(op) => op.range(),
            UnaryOpExpressionOperator::Add(op) => op.range(),
            UnaryOpExpressionOperator::Sub(op) => op.range(),
            UnaryOpExpressionOperator::BinaryNot(op) => op.range(),
            UnaryOpExpressionOperator::Extra(op) => op.range(),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::Operator(Operators::Unary(self))
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        vec![]
    }
}
