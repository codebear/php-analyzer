use crate::analysis::scope::BranchableScope;
use crate::autonodes::any::AnyNodeRef;
use crate::issue::Issue;
use crate::{
    analysis::state::AnalysisState, autonodes::conditional_expression::ConditionalExpressionNode,
    issue::IssueEmitter, types::union::UnionType, value::PHPValue,
};

use super::analysis::AnalyzeableRoundTwoNode;
use crate::autotree::NodeAccess;
///
/// Ternary: $a ? $b : $c
///
impl ConditionalExpressionNode {
    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        self.condition.read_from(state, emitter);
        let cond_val = self.condition.get_php_value(state, emitter);
        if let Some(PHPValue::Boolean(true_or_false)) = cond_val.and_then(|v| v.as_php_bool()) {
            // We can safely assume a correct boolean value, so one of the branches will
            // never be visited
            if true_or_false {
                if let Some(b) = &self.body {
                    b.read_from(state, emitter);
                }
            } else {
                self.alternative.read_from(state, emitter);
            }
        } else {
            // Both branches are valid
            if let Some(b) = &self.body {
                b.read_from(state, emitter);
            }
            self.alternative.read_from(state, emitter);
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        let cond_value = self.condition.get_php_value(state, emitter);
        if let Some(cond) = &cond_value {
            if let Some(cond_bool) = cond.as_bool() {
                // We have a known value, safely castable to boolean, we can
                // extract the correct value
                return if cond_bool {
                    if let Some(true_branch) = &self.body {
                        true_branch.get_php_value(state, emitter)
                    } else {
                        Some(cond.clone())
                    }
                } else {
                    self.alternative.get_php_value(state, emitter)
                };
            }
        }
        let alt_1_val = if let Some(true_branch) = &self.body {
            true_branch.get_php_value(state, emitter)
        } else {
            cond_value
        }?;

        let alt_2_val = self.alternative.get_php_value(state, emitter)?;

        PHPValue::common_value(&[alt_1_val, alt_2_val])
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        // If the conditional term is of known value it gives us 2 possible different routes to analyze this from
        // one the one side we can asure that both the true and false branches yields the same compatible type
        // or we can just ignore the branch that is guaranteed not to be call.

        let cond_value = self.condition.get_php_value(state, emitter);
        if let Some(cond) = &cond_value {
            if let Some(cond_bool) = cond.as_bool() {
                // We have a known value, safely castable to boolean, we can
                // extract the correct value
                return if cond_bool {
                    if let Some(true_branch) = &self.body {
                        true_branch.get_utype(state, emitter)
                    } else {
                        cond.get_utype()
                    }
                } else {
                    self.alternative.get_utype(state, emitter)
                };
            }
        }
        let alt_1_type = if let Some(true_branch) = &self.body {
            true_branch.get_utype(state, emitter)
        } else {
            self.condition.get_utype(state, emitter)
        }?;

        let alt_2_type = self.alternative.get_utype(state, emitter)?;
        Some(UnionType::from(&[&alt_1_type, &alt_2_type] as &[&UnionType]))
    }
}

impl AnalyzeableRoundTwoNode for ConditionalExpressionNode {
    fn analyze_round_two(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        if !self
            .condition
            .as_any()
            .analyze_round_two(state, emitter, path)
        {
            return false;
        }
        self.condition.read_from(state, emitter);
        let cond_val = self.condition.get_php_value(state, emitter);
        let (true_branch, false_branch) = if let Some(PHPValue::Boolean(true_or_false)) =
            cond_val.and_then(|v| v.as_php_bool())
        {
            // We can safely assume a correct boolean value, so one of the branches will
            // never be visited
            if true_or_false {
                (true, false)
            } else {
                (false, true)
            }
        } else {
            // Both branches are valid
            (true, true)
        };

        let scope = state.current_scope();

        let mut scopes = vec![];

        if let Some(b) = &self.body {
            if true_branch {
                let branch = scope.branch();
                state.push_scope(branch);
                let carry_on = b.as_any().analyze_round_two(state, emitter, path);
                scopes.push(state.pop_scope());
                if !carry_on {
                    return false;
                }
            } else {
                emitter.emit(Issue::UnreachableCode(b.pos(state)));
            }
        }
        if false_branch {
            let branch = scope.branch();
            state.push_scope(branch);
            let carry_on = self
                .alternative
                .as_any()
                .analyze_round_two(state, emitter, path);
            scopes.push(state.pop_scope());
            if !carry_on {
                return false;
            }
        } else {
            emitter.emit(Issue::UnreachableCode(self.alternative.pos(state)));
        }
        scope.join(scopes, emitter);

        true
    }
}
