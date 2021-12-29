use super::analysis::{AnalyzeableNode, AnalyzeableRoundTwoNode};
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::property_declaration::PropertyDeclarationProperties;
use crate::{
    analysis::state::AnalysisState, autonodes::property_declaration::PropertyDeclarationNode,
    issue::IssueEmitter, types::union::UnionType,
};

impl PropertyDeclarationNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        // void
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

impl AnalyzeableNode for PropertyDeclarationNode {
    fn analyze_round_one(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        for prop in &self.properties {
            match &**prop {
                PropertyDeclarationProperties::PropertyElement(p) => {
                    p.analyze_round_one_with_declaration(state, emitter, self)
                }
                _ => (),
            }
        }
    }
}

impl AnalyzeableRoundTwoNode for PropertyDeclarationNode {
    fn analyze_round_two(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        _path: &Vec<AnyNodeRef>,
    ) -> bool {
        for prop in &self.properties {
            match &**prop {
                PropertyDeclarationProperties::PropertyElement(p) => {
                    p.analyze_round_two_with_declaration(state, emitter, self)
                }
                _ => (),
            }
        }
        true
    }
}
