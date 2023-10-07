use crate::analysis::scope::{BranchSide, BranchableScope};
use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::issue::Issue;
use crate::{
    autonodes::if_statement::{IfStatementAlternative, IfStatementNode},
    issue::IssueEmitter,
    types::union::UnionType,
    value::PHPValue,
};

use super::super::analysis::hardening::BranchTypeHardening;
use super::analysis::ThirdPassAnalyzeableNode;
use crate::autotree::NodeAccess;

impl IfStatementNode {
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

impl ThirdPassAnalyzeableNode for IfStatementNode {
    fn analyze_third_pass(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn crate::issue::IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        let scope = state.current_scope();
        self.condition.read_from(state, emitter);
        // Conditinal-statement is analyzed in current scope
        if !self
            .condition
            .as_any()
            .analyze_third_pass(state, emitter, path)
        {
            return false;
        }

        let cond_val = self.condition.get_php_value(state, emitter);
        let (true_branch, false_branch) = if let Some(PHPValue::Boolean(true_or_false)) =
            cond_val.and_then(|v| v.as_php_bool())
        {
            if true_or_false {
                (true, false)
            } else {
                (false, true)
            }
        } else {
            (true, true)
        };

        let mut scopes = vec![];
        if true_branch {
            let branch = self
                .condition
                .branch_with_hardened_types_base_on_conditional_node(
                    scope.clone(),
                    BranchSide::TrueBranch,
                    state,
                );
            // let branch = scope.branch_with_hardened_types_base_on_conditional_node(&*self.condition.child, BranchSide::True, state);
            state.push_scope(branch);

            let was_conditional = state.in_conditional_branch;
            if false_branch {
                state.in_conditional_branch = true;
            }

            let carry_on = self.body.as_any().analyze_third_pass(state, emitter, path);

            if false_branch {
                state.in_conditional_branch = was_conditional;
            }

            // self.analyze_round_two_children(&self.as_any(), state, emitter);
            scopes.push(state.pop_scope());
            if !carry_on {
                return false;
            }
        } else {
            // emit the other branch as unreachable
        }

        if false_branch {
            if let Some(alts) = &self.alternative {
                let was_conditional = state.in_conditional_branch;
                let mut carry_on = true;
                if true_branch || alts.len() > 1 {
                    state.in_conditional_branch = true;
                }
                let false_scope = self
                    .condition
                    .branch_with_hardened_types_base_on_conditional_node(
                        scope.clone(),
                        BranchSide::FalseBranch,
                        state,
                    );
                for a in alts {
                    let branch = false_scope.branch();
                    state.push_scope(branch);
                    carry_on = a.as_any().analyze_third_pass(state, emitter, path);
                    scopes.push(state.pop_scope());
                }
                state.in_conditional_branch = was_conditional;
                if !carry_on {
                    return false;
                }
            }
        } else {
            // emit the other branch as unreachable
        }

        let mut no_alt = false;
        if let Some(alt) = &self.alternative {
            if alt.len() == 0 {
                no_alt = true;
            }
        } else {
            no_alt = true;
        }

        if no_alt {
            let false_scope = self
                .condition
                .branch_with_hardened_types_base_on_conditional_node(
                    scope.clone(),
                    BranchSide::FalseBranch,
                    state,
                );
            scopes.push(false_scope);
        }
        //  println!("Fant scopes count={}: {:?}", scopes.len(), scopes);
        //eprintln!("SCOPES: {:#?}", scopes);
        //todo!();
        scope.join(scopes, emitter);

        true
        // todo!("BALLE@{} {:?}", state.pos_as_string(self.range), self);
    }
}

impl ThirdPassAnalyzeableNode for IfStatementAlternative {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        let else_if = match self {
            IfStatementAlternative::ElseClause(e) => {
                return e.as_any().analyze_third_pass(state, emitter, path)
            }
            IfStatementAlternative::ElseIfClause(else_if) => else_if,

            IfStatementAlternative::Extra(_) => return true,
        };

        if !else_if
            .condition
            .as_any()
            .analyze_third_pass(state, emitter, path)
        {
            return false;
        }

        let cond_val = else_if.condition.get_php_value(state, emitter);
        match cond_val.and_then(|v| v.as_php_bool()) {
            Some(PHPValue::Boolean(true_or_false)) if !true_or_false => {
                emitter.emit(Issue::UnreachableCode(else_if.body.pos(state)));
                return true;
            }
            _ => (),
        }

        let scope = state.current_scope();
        let branch = scope.branch();
        state.push_scope(branch);
        let carry_on = else_if
            .body
            .as_any()
            .analyze_third_pass(state, emitter, path);
        let scopes = vec![state.pop_scope()];
        scope.join(scopes, emitter);
        carry_on
    }
}
