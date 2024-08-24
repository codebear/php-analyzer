use crate::Range;

use super::{binary::BinaryAssignmentOperator, operator::Operator};
#[derive(Clone, Debug)]

pub struct PowAssignOperator(pub Range);

impl Operator for PowAssignOperator {
    fn brief_desc(&self) -> String {
        "PowAssignOperator".into()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "**="
    }
}

impl BinaryAssignmentOperator for PowAssignOperator {
    fn get_operator_utype(
        &self,
        _node: &impl super::binary::BinaryOperatorOperandAccess,
        _state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::types::union::PHPType> {
        crate::missing_none!("{}.get_operator_utype(..)", self.brief_desc())
    }
}
