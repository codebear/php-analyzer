use super::analysis::{
    FirstPassAnalyzeableNode, SecondPassAnalyzeableNode, ThirdPassAnalyzeableNode,
};
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::property_declaration::PropertyDeclarationProperties;
use crate::autotree::NodeAccess;
use crate::extra::ExtraChild;
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

impl FirstPassAnalyzeableNode for PropertyDeclarationNode {
    fn analyze_first_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        let mut inline_doc_comment = false;
        let mut extra_iter = self.extras.iter().peekable();
        for prop in &self.properties {
            let peeked = extra_iter.peek().cloned();
            // FIXME This algorithm is probably broken
            let extra_comment = if let Some(extra) = peeked {
                let start_byte = prop.range().start_byte;

                // Hvis extra e før noden
                if extra.range().start_byte < start_byte {
                    extra_iter.next();
                    match &**extra {
                        ExtraChild::Comment(c) => Some(c),

                        ExtraChild::TextInterpolation(_) | ExtraChild::Error(_) => None,
                    }
                } else {
                    None
                }
            } else {
                None
            };

            match extra_comment {
                Some(c) => {
                    if let None = state.last_doc_comment {
                        state.last_doc_comment = Some((c.get_raw(), c.range()));
                        inline_doc_comment = true;
                    }
                }
                None => (),
            }

            match &**prop {
                PropertyDeclarationProperties::Extra(ExtraChild::Comment(c)) => {
                    if let None = state.last_doc_comment {
                        state.last_doc_comment = Some((c.get_raw(), c.range()));
                        inline_doc_comment = true;
                    }
                }
                PropertyDeclarationProperties::PropertyElement(p) => {
                    p.analyze_round_one_with_declaration(state, emitter, self);
                    if inline_doc_comment {
                        state.last_doc_comment = None;
                    }
                }
                _ => (),
            }
        }
    }
}

impl SecondPassAnalyzeableNode for PropertyDeclarationNode {
    fn analyze_second_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        crate::missing!("PropertyDeclarationNode.analyze_second_pass()");
        self.analyze_second_pass_children(&self.as_any(), state, emitter);
    }
}

impl ThirdPassAnalyzeableNode for PropertyDeclarationNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        _path: &Vec<AnyNodeRef>,
    ) -> bool {
        for prop in &self.properties {
            match &**prop {
                PropertyDeclarationProperties::PropertyElement(p) => {
                    p.analyze_third_pass_with_declaration(state, emitter, self)
                }
                _ => (),
            }
        }
        true
    }
}
