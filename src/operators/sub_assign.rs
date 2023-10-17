use crate::Range;

use super::{binary::BinaryAssignmentOperator, operator::Operator};
#[derive(Clone, Debug)]

pub struct SubAssignOperator(pub Range);

impl Operator for SubAssignOperator {
    fn brief_desc(&self) -> String {
        "SubAssignOperator".into()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "-="
    }
}

impl BinaryAssignmentOperator for SubAssignOperator {
    fn get_operator_utype(
        &self,
        _operands: &impl super::binary::BinaryOperatorOperandAccess,
        _state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::types::union::UnionType> {
        crate::missing_none!("{}.get_operator_utype(..)", self.brief_desc())
    }
}
