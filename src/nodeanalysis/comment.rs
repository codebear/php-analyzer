use crate::{
    analysis::state::AnalysisState, autonodes::comment::CommentNode, issue::IssueEmitter,
    types::union::UnionType,
};

impl CommentNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        ()
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        None
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        None
    }

    pub(crate) fn is_doc_comment(&self) -> bool {
        // The shortes possible doc-comment is /***/,
        self.raw.len() > 4 && &self.raw[0..3] == b"/**"
    }
}
