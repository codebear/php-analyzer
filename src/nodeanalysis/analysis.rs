use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autotree::NodeAccess;
use crate::issue::IssueEmitter;

pub trait FirstPassAnalyzeableNode {
    ///
    /// Simple inline analysis of basic symbols thats declared
    /// * class/interface/trait names and properties
    /// * methods/functions
    /// * declared native return types
    /// * capturing of doc-comment declarations
    ///
    fn analyze_first_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter);

    fn analyze_first_pass_children(
        &self,
        node_ref: &AnyNodeRef,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) {
        for child in node_ref.children_any() {
            child.analyze_first_pass(state, emitter);

            if let AnyNodeRef::Comment(c) = &child {
                state.last_doc_comment = if c.is_doc_comment() {
                    Some((c.get_raw(), c.range()))
                } else {
                    None
                }
            } else if state.last_doc_comment.is_some() {
                state.last_doc_comment = None;
            }
        }
    }
}

pub trait ErrorPassAnalyzableNode {
    fn analyze_errors(
        &self,
        node_ref: &AnyNodeRef,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) {
        for child in node_ref.children_any() {
            child.analyze_errors(&child, state, emitter);
        }
    }
}

pub trait SecondPassAnalyzeableNode {
    ///
    /// Second round, iterative
    ///
    fn analyze_second_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter);

    fn analyze_second_pass_children(
        &self,
        node_ref: &AnyNodeRef,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> bool {
        for child in node_ref.children_any() {
            child.analyze_second_pass(state, emitter);
        }
        true
    }
}

pub trait ThirdPassAnalyzeableNode {
    ///
    /// Second round, iterative
    ///
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &[AnyNodeRef],
    ) -> bool;

    fn analyze_third_pass_children(
        &self,
        node_ref: &AnyNodeRef,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &[AnyNodeRef],
    ) -> bool {
        let mut our_path = path.to_vec();
        our_path.push(node_ref.clone());

        for child in node_ref.children_any() {
            if !child.analyze_third_pass(state, emitter, &our_path) {
                return false;
            }
            let mut found = false;
            if let Some(looking_for) = &state.looking_for_node {
                if child.contains_pos(looking_for.pos) {
                    found = true;
                }
            }
            if found {
                if let Some(looking_for) = state.looking_for_node.take() {
                    looking_for.found(child, state, &our_path);
                    return false;
                }
            }

            if let AnyNodeRef::Comment(c) = &child {
                state.last_doc_comment = Some((c.get_raw(), c.range()));
            } else if state.last_doc_comment.is_some() {
                state.last_doc_comment = None;
            }
        }
        true
    }
}

pub trait IntoFirstPassAnalyzeable {
    fn with_first_pass_analyzeable<T, CB>(&self, cb: &mut CB) -> Option<T>
    where
        CB: FnMut(&dyn FirstPassAnalyzeableNode) -> T;
}

pub trait IntoSecondPassAnalyzeable {
    fn with_second_pass_analyzeable<T, CB>(&self, cb: &mut CB) -> Option<T>
    where
        CB: FnMut(&dyn SecondPassAnalyzeableNode) -> T;
}

pub trait IntoThirdPassAnalyzeable {
    fn with_third_pass_analyzeable<T, CB>(&self, cb: &mut CB) -> Option<T>
    where
        CB: FnMut(&dyn ThirdPassAnalyzeableNode) -> T;
}
