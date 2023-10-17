use crate::Range;

use super::{binary::BinaryAssignmentOperator, operator::Operator};
#[derive(Clone, Debug)]

pub struct DivAssignOperator(pub Range);

impl Operator for DivAssignOperator {
    fn brief_desc(&self) -> String {
        "DivAssignOperator".into()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "/="
    }
}

impl BinaryAssignmentOperator for DivAssignOperator {
    fn get_operator_utype(
        &self,
        _operands: &impl super::binary::BinaryOperatorOperandAccess,
        _state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::types::union::UnionType> {
        crate::missing_none!("{}.get_operator_utype(..)", self.brief_desc())
    }
}
