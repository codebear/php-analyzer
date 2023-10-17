use crate::Range;

use super::{binary::BinaryAssignmentOperator, operator::Operator};
#[derive(Clone, Debug)]

pub struct MultAssignOperator(pub Range);

impl Operator for MultAssignOperator {
    fn brief_desc(&self) -> String {
        "MultAssignOperator".into()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "*="
    }
}

impl BinaryAssignmentOperator for MultAssignOperator {
    fn get_operator_utype(
        &self,
        _operands: &impl super::binary::BinaryOperatorOperandAccess,
        _state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::types::union::UnionType> {
        crate::missing_none!("{}.get_operator_utype(..)", self.brief_desc())
    }
}
