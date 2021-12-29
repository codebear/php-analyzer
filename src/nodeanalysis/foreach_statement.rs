use crate::{
    analysis::state::AnalysisState,
    autonodes::{
        any::AnyNodeRef,
        foreach_statement::{ForeachStatementNode, ForeachStatementValue},
    },
    issue::{Issue, IssueEmitter},
    types::union::{DiscreteType, UnionType},
    value::PHPValue,
};

use super::analysis::AnalyzeableRoundTwoNode;
use crate::autotree::NodeAccess;

impl ForeachStatementNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        crate::missing!("{}.read_from(..)", self.kind());
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        crate::missing_none!("{}.get_php_value(..)", self.kind())
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        crate::missing_none!("{}.get_utype(..)", self.kind())
    }
}

impl AnalyzeableRoundTwoNode for ForeachStatementNode {
    fn analyze_round_two(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn crate::issue::IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        self.traversable.read_from(state, emitter);

        let traversable_type = if let Some(t) = self.traversable.get_utype(state, emitter) {
            Some(t)
        } else {
            emitter.emit(Issue::TraversalOfUnknownType(self.traversable.pos(state)));

            None
        };

        if let Some(key) = &self.key {
            let key_type = if let Some(tt) = &traversable_type {
                match tt.single_type() {
                    // Vectors are indiced by int
                    Some(DiscreteType::Vector(_)) => Some(DiscreteType::Int.into()),
                    Some(DiscreteType::HashMap(k, _)) => Some(k),
                    Some(d) => crate::missing_none!(
                        "Extracting key_type from traversable-type gave {:?}",
                        d
                    ),
                    None => crate::missing_none!(
                        "Failed to extract single key_type from traversable-type"
                    ),
                }
            } else {
                None
            };

            key.write_to(state, emitter, key_type, None);
        }

        let value_type = if let Some(tt) = &traversable_type {
            match tt.single_type() {
                // Vectors are indiced by int
                Some(DiscreteType::Vector(v)) => Some(v),
                Some(DiscreteType::HashMap(_, v)) => Some(v),
                Some(DiscreteType::Named(_, fq_name)) => {
                    crate::missing_none!("Need to extract a value_type from a class-type of {:?}, Perhaps it's traversable or similar", fq_name)
                }
                Some(d) => {
                    crate::missing_none!("Extracting value_type from traversable-type of {:?}", d)
                }
                None => crate::missing_none!("Failed to extract single from traversable-type"),
            }
        } else {
            None
        };

        self.value.write_to(state, emitter, value_type, None);

        self.analyze_round_two_children(&self.as_any(), state, emitter, path)
    }
}

impl ForeachStatementValue {
    pub fn write_to(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn IssueEmitter,
        val_type: Option<UnionType>,
        value: Option<PHPValue>,
    ) {
        match self {
            ForeachStatementValue::_Expression(e) => e.write_to(state, emitter, val_type, value),
            ForeachStatementValue::ByRef(br) => br.write_to(state, emitter, val_type, value),
            ForeachStatementValue::ListLiteral(ll) => ll.write_to(state, emitter, val_type, value),

            ForeachStatementValue::Comment(_)
            | ForeachStatementValue::TextInterpolation(_)
            | ForeachStatementValue::Error(_) => crate::missing!(),
        }
    }
}
