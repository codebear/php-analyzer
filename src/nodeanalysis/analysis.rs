use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autotree::NodeAccess;
use crate::issue::IssueEmitter;

pub trait AnalyzeableNode {
    ///
    /// Simple inline analysis of basic symbols thats declared
    /// * class/interface/trait names and properties
    /// * methods/functions
    /// * declared native return types
    /// * capturing of doc-comment declarations
    ///
    fn analyze_round_one(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter);

    fn analyze_round_one_children(
        &self,
        node_ref: &AnyNodeRef,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) {
        for child in node_ref.children_any() {
            child.analyze_round_one(state, emitter);
            if let AnyNodeRef::Comment(c) = &child {
                state.last_doc_comment = Some((c.get_raw(), c.range()));
            } else if let Some(_) = state.last_doc_comment {
                state.last_doc_comment = None;
            }
        }
    }
}

pub trait AnalyzeableRoundTwoNode {
    ///
    /// Second round, iterative
    ///
    fn analyze_round_two(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool;

    fn analyze_round_two_children(
        &self,
        node_ref: &AnyNodeRef,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        let mut our_path = path.clone();
        our_path.push(node_ref.clone());

        for child in node_ref.children_any() {
            if !child.analyze_round_two(state, emitter, &our_path) {
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
            } else if let Some(_) = state.last_doc_comment {
                state.last_doc_comment = None;
            }
        }
        return true;
    }
}

pub trait IntoAnalyzeable {
    fn with_analyzeable<T, CB>(&self, cb: &mut CB) -> Option<T>
    where
        CB: FnMut(&dyn AnalyzeableNode) -> T;
}

pub trait IntoAnalyzeableRoundTwo {
    fn with_analyzeable_round_two<T, CB>(&self, cb: &mut CB) -> Option<T>
    where
        CB: FnMut(&dyn AnalyzeableRoundTwoNode) -> T;
}
