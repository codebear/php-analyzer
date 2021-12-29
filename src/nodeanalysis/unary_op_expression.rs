use crate::{
    analysis::state::AnalysisState,
    autonodes::unary_op_expression::{UnaryOpExpressionNode, UnaryOpExpressionOperator},
    issue::IssueEmitter,
    types::union::{DiscreteType, UnionType},
    value::PHPValue,
};

impl UnaryOpExpressionNode {
    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        if let Some(e) = &self.expr {
            e.read_from(state, emitter);
        }
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        match &**self.operator.as_ref()? {
            UnaryOpExpressionOperator::Not(_, _) => Some(DiscreteType::Bool.into()),
            UnaryOpExpressionOperator::Add(_, _) => self
                .expr
                .as_ref()?
                .get_php_value(state, emitter)?
                .as_php_num()?
                .get_utype(),
            UnaryOpExpressionOperator::Sub(_, _) => self
                .expr
                .as_ref()?
                .get_php_value(state, emitter)?
                .as_php_num()?
                .get_utype(),
            UnaryOpExpressionOperator::BinaryNot(_, _) => Some(DiscreteType::Int.into()),

            UnaryOpExpressionOperator::Comment(_)
            | UnaryOpExpressionOperator::TextInterpolation(_)
            | UnaryOpExpressionOperator::Error(_) => None,
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        let operator = if let Some(oper) = &self.operator {
            oper
        } else {
            return None;
        };

        let value = if let Some(v) = self
            .expr
            .as_ref()
            .and_then(|x| x.get_php_value(state, emitter))
        {
            v
        } else {
            return None;
        };

        match (&**operator, value) {
            (UnaryOpExpressionOperator::Not(_, _), _) => crate::missing_none!("unary not"),
            (UnaryOpExpressionOperator::Add(_, _), _) => crate::missing_none!("unary add"),
            (UnaryOpExpressionOperator::Sub(_, _), PHPValue::Int(i)) => Some(PHPValue::Int(-i)),
            (UnaryOpExpressionOperator::Sub(_, _), PHPValue::Float(f)) => Some(PHPValue::Float(-f)),
            // (UnaryOpExpressionOperator::Squelch(_, _), _) => todo!(),
            (UnaryOpExpressionOperator::BinaryNot(_, _), _) => {
                crate::missing_none!("unary binary not")
            }

            (
                UnaryOpExpressionOperator::Comment(_)
                | UnaryOpExpressionOperator::TextInterpolation(_)
                | UnaryOpExpressionOperator::Error(_),
                _,
            ) => return None,
            _ => crate::missing_none!("get_php_value: {:?}", self),
        }
    }
}
