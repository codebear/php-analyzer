use crate::Range;

use super::{
    binary::{BinaryAssignmentOperator, BinaryOperatorOperandAccess},
    operator::Operator,
};
#[derive(Clone, Debug)]

pub struct ModAssignOperator(pub Range);

impl Operator for ModAssignOperator {
    fn brief_desc(&self) -> String {
        todo!()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "%="
    }
}

impl BinaryAssignmentOperator for ModAssignOperator {
    fn get_operator_utype(
        &self,
        _operands: &impl BinaryOperatorOperandAccess,
        _state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::types::union::PHPType> {
        None
    }
}
