use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::issue::IssueEmitter;
use crate::nodeanalysis::analysis::FirstPassAnalyzeableNode;
use crate::nodeanalysis::analysis::IntoFirstPassAnalyzeable;

use super::analysis::IntoSecondPassAnalyzeable;
use super::analysis::SecondPassAnalyzeableNode;
use super::analysis::{IntoThirdPassAnalyzeable, ThirdPassAnalyzeableNode};

impl IntoFirstPassAnalyzeable for AnyNodeRef<'_> {
    fn with_first_pass_analyzeable<T, CB>(&self, cb: &mut CB) -> std::option::Option<T>
    where
        CB: FnMut(&dyn FirstPassAnalyzeableNode) -> T,
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

            // Function call
            AnyNodeRef::FunctionCallExpression(fce) => cb(*fce),

            _ => return None,
        })
    }
}

impl IntoSecondPassAnalyzeable for AnyNodeRef<'_> {
    fn with_second_pass_analyzeable<T, CB>(&self, cb: &mut CB) -> Option<T>
    where
        CB: FnMut(&dyn SecondPassAnalyzeableNode) -> T,
    {
        Some(match self {
            AnyNodeRef::NamespaceDefinition(ns) => cb(*ns),
            AnyNodeRef::NamespaceUseClause(n) => cb(*n),
            AnyNodeRef::NamespaceUseGroup(n) => cb(*n),

            AnyNodeRef::ClassDeclaration(cd) => cb(*cd),
            AnyNodeRef::InterfaceDeclaration(id) => cb(*id),
            AnyNodeRef::TraitDeclaration(td) => cb(*td),
            AnyNodeRef::MethodDeclaration(md) => cb(*md),
            AnyNodeRef::PropertyDeclaration(pd) => cb(*pd),
            AnyNodeRef::ClassConstantAccessExpression(cc) => cb(*cc),
            _ => return None,
        })
    }
}

impl IntoThirdPassAnalyzeable for AnyNodeRef<'_> {
    fn with_third_pass_analyzeable<T, CB>(&self, cb: &mut CB) -> Option<T>
    where
        CB: FnMut(&dyn ThirdPassAnalyzeableNode) -> T,
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

            AnyNodeRef::NamespaceDefinition(ns) => cb(*ns),
            AnyNodeRef::NamespaceUseClause(n) => cb(*n),
            AnyNodeRef::NamespaceUseGroup(n) => cb(*n),

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

impl FirstPassAnalyzeableNode for AnyNodeRef<'_> {
    fn analyze_first_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        if let Some(_) =
            self.with_first_pass_analyzeable(&mut |x: &dyn FirstPassAnalyzeableNode| {
                x.analyze_first_pass(state, emitter)
            })
        {
            // good
        } else {
            self.analyze_first_pass_children(self, state, emitter);
        }
    }
}

impl SecondPassAnalyzeableNode for AnyNodeRef<'_> {
    fn analyze_second_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        if let Some(_) =
            self.with_second_pass_analyzeable(&mut |x: &dyn SecondPassAnalyzeableNode| {
                x.analyze_second_pass(state, emitter)
            })
        {
            // good
        } else {
            self.analyze_second_pass_children(self, state, emitter);
        }
    }
}

impl ThirdPassAnalyzeableNode for AnyNodeRef<'_> {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        if let Some(carry_on) =
            self.with_third_pass_analyzeable(&mut |x: &dyn ThirdPassAnalyzeableNode| {
                x.analyze_third_pass(state, emitter, path)
            })
        {
            /*if let Some(looking_for) = state.looking_for_node {

            }*/
            carry_on
        } else {
            self.analyze_third_pass_children(self, state, emitter, path)
        }
    }
}
