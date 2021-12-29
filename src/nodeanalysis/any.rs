use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::issue::IssueEmitter;
use crate::nodeanalysis::analysis::AnalyzeableNode;
use crate::nodeanalysis::analysis::IntoAnalyzeable;

use super::analysis::{AnalyzeableRoundTwoNode, IntoAnalyzeableRoundTwo};

impl IntoAnalyzeable for AnyNodeRef<'_> {
    fn with_analyzeable<T, CB>(&self, cb: &mut CB) -> std::option::Option<T>
    where
        CB: FnMut(&dyn AnalyzeableNode) -> T,
    {
        Some(match self {
            // Class like definitions
            AnyNodeRef::ClassDeclaration(x) => cb(*x),
            AnyNodeRef::InterfaceDeclaration(x) => cb(*x),
            AnyNodeRef::TraitDeclaration(x) => cb(*x),

            // Class methods
            AnyNodeRef::MethodDeclaration(x) => cb(*x),

            // Class Constants
            AnyNodeRef::ConstDeclaration(c) => cb(*c),

            // Class properties
            AnyNodeRef::PropertyDeclaration(p) => cb(*p),

            // Defining which namespace we're currently in
            AnyNodeRef::NamespaceDefinition(n) => cb(*n),

            // Different modes of namespace usage
            AnyNodeRef::NamespaceUseClause(n) => cb(*n),
            AnyNodeRef::NamespaceUseGroup(n) => cb(*n),

            // Function like declarations
            AnyNodeRef::FunctionDefinition(f) => cb(*f),
            AnyNodeRef::SimpleParameter(sp) => cb(*sp),

            // Function call
            AnyNodeRef::FunctionCallExpression(fce) => cb(*fce),

            _ => return None,
        })
    }
}

impl IntoAnalyzeableRoundTwo for AnyNodeRef<'_> {
    fn with_analyzeable_round_two<T, CB>(&self, cb: &mut CB) -> Option<T>
    where
        CB: FnMut(&dyn AnalyzeableRoundTwoNode) -> T,
    {
        Some(match self {
            AnyNodeRef::ClassDeclaration(c) => cb(*c),
            AnyNodeRef::MethodDeclaration(md) => cb(*md),

            AnyNodeRef::EchoStatement(e) => cb(*e),
            AnyNodeRef::PrintIntrinsic(p) => cb(*p),
            // AnyNodeRef::FormalParameters(fp) => cb(*fp),
            AnyNodeRef::SimpleParameter(sp) => cb(*sp),
            AnyNodeRef::FunctionDefinition(fd) => cb(*fd),
            AnyNodeRef::AnonymousFunctionCreationExpression(e) => cb(*e),
            AnyNodeRef::BinaryExpression(x) => cb(*x),
            AnyNodeRef::AssignmentExpression(n) => cb(*n),
            AnyNodeRef::AugmentedAssignmentExpression(e) => cb(*e),
            AnyNodeRef::UpdateExpression(ue) => cb(*ue),

            AnyNodeRef::NamespaceUseClause(n) => cb(*n),

            AnyNodeRef::PropertyDeclaration(x) => cb(*x),
            AnyNodeRef::IfStatement(x) => cb(*x),
            // AnyNodeRef::ElseIfClause(x) => cb(*x),
            // AnyNodeRef::ElseClause(x) => cb(*x),
            AnyNodeRef::ForeachStatement(fs) => cb(*fs),
            AnyNodeRef::ForStatement(fs) => cb(*fs),

            AnyNodeRef::TryStatement(ts) => cb(*ts),
            AnyNodeRef::CatchClause(cc) => cb(*cc),

            AnyNodeRef::SwitchStatement(x) => cb(*x),
            AnyNodeRef::SwitchBlock(x) => cb(*x),

            AnyNodeRef::Argument(a) => cb(*a),
            AnyNodeRef::VariableName(vn) => cb(*vn),

            AnyNodeRef::MemberCallExpression(mce) => cb(*mce),
            AnyNodeRef::FunctionCallExpression(fc) => cb(*fc),
            AnyNodeRef::NamespaceDefinition(ns) => cb(*ns),
            AnyNodeRef::MemberAccessExpression(ma) => cb(*ma),
            AnyNodeRef::SubscriptExpression(se) => cb(*se),

            AnyNodeRef::ConditionalExpression(ce) => cb(*ce),
            AnyNodeRef::ReturnStatement(ret) => cb(*ret),
            AnyNodeRef::TraitDeclaration(t) => cb(*t),
            AnyNodeRef::InterfaceDeclaration(i) => cb(*i),
            AnyNodeRef::ScopedPropertyAccessExpression(ret) => cb(*ret),
            _ => return None,
        })
    }
}

impl AnalyzeableNode for AnyNodeRef<'_> {
    fn analyze_round_one(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        if let Some(_) = self
            .with_analyzeable(&mut |x: &dyn AnalyzeableNode| x.analyze_round_one(state, emitter))
        {
            // good
        } else {
            self.analyze_round_one_children(self, state, emitter);
        }
    }
}

impl AnalyzeableRoundTwoNode for AnyNodeRef<'_> {
    fn analyze_round_two(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        if let Some(carry_on) =
            self.with_analyzeable_round_two(&mut |x: &dyn AnalyzeableRoundTwoNode| {
                x.analyze_round_two(state, emitter, path)
            })
        {
            /*if let Some(looking_for) = state.looking_for_node {

            }*/
            carry_on
        } else {
            self.analyze_round_two_children(self, state, emitter, path)
        }
    }
}
