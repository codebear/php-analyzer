use crate::{
    analysis::state::AnalysisState,
    autonodes::{
        any::AnyNodeRef,
        foreach_statement::{ForeachStatementNode, ForeachStatementValue},
    },
    issue::{Issue, IssueEmitter},
    types::{
        traversable::{get_key_type, get_value_type},
        union::UnionType,
    },
    value::PHPValue,
};

use super::analysis::ThirdPassAnalyzeableNode;
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

impl ThirdPassAnalyzeableNode for ForeachStatementNode {
    fn analyze_third_pass(
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
        let value_type = if let Some(trav) = traversable_type {
            if let Some(key) = &self.key {
                let key_type = get_key_type(&trav, state.symbol_data.clone());

                key.write_to(state, emitter, key_type, None);
            }

            get_value_type(&trav, state.symbol_data.clone())
        } else {
            None
        };

        self.value.write_to(state, emitter, value_type, None);

        self.analyze_third_pass_children(&self.as_any(), state, emitter, path)
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
