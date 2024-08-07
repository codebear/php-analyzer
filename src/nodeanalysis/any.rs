use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::issue::IssueEmitter;
use crate::nodeanalysis::analysis::FirstPassAnalyzeableNode;
use crate::nodeanalysis::analysis::IntoFirstPassAnalyzeable;
use crate::types::union::UnionType;
use crate::value::PHPValue;

use super::analysis::ErrorPassAnalyzableNode;
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

            // Binary expression
            AnyNodeRef::BinaryExpression(be) => cb(*be),

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

impl ErrorPassAnalyzableNode for AnyNodeRef<'_> {
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

impl FirstPassAnalyzeableNode for AnyNodeRef<'_> {
    fn analyze_first_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        if self
            .with_first_pass_analyzeable(&mut |x: &dyn FirstPassAnalyzeableNode| {
                x.analyze_first_pass(state, emitter)
            })
            .is_some()
        {
            // good
        } else {
            self.analyze_first_pass_children(self, state, emitter);
        }
    }
}

impl SecondPassAnalyzeableNode for AnyNodeRef<'_> {
    fn analyze_second_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        if self
            .with_second_pass_analyzeable(&mut |x: &dyn SecondPassAnalyzeableNode| {
                x.analyze_second_pass(state, emitter)
            })
            .is_some()
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

impl AnyNodeRef<'_> {
    pub fn write_to(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn IssueEmitter,
        val_type: Option<UnionType>,
        value: Option<PHPValue>,
    ) {
        match self {
            AnyNodeRef::StaticExpr(_, _) => crate::missing!(),
            AnyNodeRef::Error(_) => crate::missing!(),
            AnyNodeRef::Operator(_) => crate::missing!(),
            AnyNodeRef::_Expression(_) => crate::missing!(),
            AnyNodeRef::_Literal(_) => crate::missing!(),
            AnyNodeRef::_PrimaryExpression(_) => crate::missing!(),
            AnyNodeRef::_Statement(_) => crate::missing!(),
            AnyNodeRef::_Type(_) => crate::missing!(),
            AnyNodeRef::AbstractModifier(_) => crate::missing!(),
            AnyNodeRef::AnonymousFunctionCreationExpression(_) => crate::missing!(),
            AnyNodeRef::AnonymousFunctionUseClause(_) => crate::missing!(),
            AnyNodeRef::Argument(_) => crate::missing!(),
            AnyNodeRef::Arguments(_) => crate::missing!(),
            AnyNodeRef::ArrayCreationExpression(_) => crate::missing!(),
            AnyNodeRef::ArrayElementInitializer(_) => crate::missing!(),
            AnyNodeRef::ArrowFunction(_) => crate::missing!(),
            AnyNodeRef::AssignmentExpression(_) => crate::missing!(),
            AnyNodeRef::Attribute(_) => crate::missing!(),
            AnyNodeRef::AttributeGroup(_) => crate::missing!(),
            AnyNodeRef::AttributeList(_) => crate::missing!(),
            AnyNodeRef::AugmentedAssignmentExpression(_) => crate::missing!(),
            AnyNodeRef::BaseClause(_) => crate::missing!(),
            AnyNodeRef::BinaryExpression(_) => crate::missing!(),
            AnyNodeRef::Boolean(_) => crate::missing!(),
            AnyNodeRef::BreakStatement(_) => crate::missing!(),
            AnyNodeRef::ByRef(_) => crate::missing!(),
            AnyNodeRef::CaseStatement(_) => crate::missing!(),
            AnyNodeRef::CastExpression(_) => crate::missing!(),
            AnyNodeRef::CastType(_) => crate::missing!(),
            AnyNodeRef::CatchClause(_) => crate::missing!(),
            AnyNodeRef::ClassConstantAccessExpression(_) => crate::missing!(),
            AnyNodeRef::ClassConstantAccessIdentifier(_) => crate::missing!(),
            AnyNodeRef::ClassDeclaration(_) => crate::missing!(),
            AnyNodeRef::ClassInterfaceClause(_) => crate::missing!(),
            AnyNodeRef::CloneExpression(_) => crate::missing!(),
            AnyNodeRef::ColonBlock(_) => crate::missing!(),
            AnyNodeRef::CompoundStatement(_) => crate::missing!(),
            AnyNodeRef::ConditionalExpression(_) => crate::missing!(),
            AnyNodeRef::ConstDeclaration(_) => crate::missing!(),
            AnyNodeRef::ConstElement(_) => crate::missing!(),
            AnyNodeRef::ContinueStatement(_) => crate::missing!(),
            AnyNodeRef::DeclarationList(_) => crate::missing!(),
            AnyNodeRef::DeclareDirective(_) => crate::missing!(),
            AnyNodeRef::DeclareStatement(_) => crate::missing!(),
            AnyNodeRef::DefaultStatement(_) => crate::missing!(),
            AnyNodeRef::DisjunctiveNormalFormType(_) => crate::missing!(),
            AnyNodeRef::DoStatement(_) => crate::missing!(),
            AnyNodeRef::DynamicVariableName(_) => crate::missing!(),
            AnyNodeRef::EchoStatement(_) => crate::missing!(),
            AnyNodeRef::ElseClause(_) => crate::missing!(),
            AnyNodeRef::ElseIfClause(_) => crate::missing!(),
            AnyNodeRef::EmptyStatement(_) => crate::missing!(),
            AnyNodeRef::EncapsedString(_) => crate::missing!(),
            AnyNodeRef::EnumCase(_) => crate::missing!(),
            AnyNodeRef::EnumDeclaration(_) => crate::missing!(),
            AnyNodeRef::EnumDeclarationList(_) => crate::missing!(),
            AnyNodeRef::ErrorSuppressionExpression(_) => crate::missing!(),
            AnyNodeRef::ExpressionStatement(_) => crate::missing!(),
            AnyNodeRef::FinalModifier(_) => crate::missing!(),
            AnyNodeRef::FinallyClause(_) => crate::missing!(),
            AnyNodeRef::ForStatement(_) => crate::missing!(),
            AnyNodeRef::ForeachStatement(_) => crate::missing!(),
            AnyNodeRef::FormalParameters(_) => crate::missing!(),
            AnyNodeRef::FunctionCallExpression(_) => crate::missing!(),
            AnyNodeRef::FunctionDefinition(_) => crate::missing!(),
            AnyNodeRef::FunctionStaticDeclaration(_) => crate::missing!(),
            AnyNodeRef::GlobalDeclaration(_) => crate::missing!(),
            AnyNodeRef::GotoStatement(_) => crate::missing!(),
            AnyNodeRef::Heredoc(_) => crate::missing!(),
            AnyNodeRef::HeredocBody(_) => crate::missing!(),
            AnyNodeRef::IfStatement(_) => crate::missing!(),
            AnyNodeRef::IncludeExpression(_) => crate::missing!(),
            AnyNodeRef::IncludeOnceExpression(_) => crate::missing!(),
            AnyNodeRef::InterfaceDeclaration(_) => crate::missing!(),
            AnyNodeRef::IntersectionType(_) => crate::missing!(),
            AnyNodeRef::ListLiteral(_) => crate::missing!(),
            AnyNodeRef::MatchBlock(_) => crate::missing!(),
            AnyNodeRef::MatchConditionList(_) => crate::missing!(),
            AnyNodeRef::MatchConditionalExpression(_) => crate::missing!(),
            AnyNodeRef::MatchDefaultExpression(_) => crate::missing!(),
            AnyNodeRef::MatchExpression(_) => crate::missing!(),
            AnyNodeRef::MemberAccessExpression(_) => crate::missing!(),
            AnyNodeRef::MemberCallExpression(_) => crate::missing!(),
            AnyNodeRef::MethodDeclaration(_) => crate::missing!(),
            AnyNodeRef::Name(_) => crate::missing!(),
            AnyNodeRef::NamedLabelStatement(_) => crate::missing!(),
            AnyNodeRef::NamedType(_) => crate::missing!(),
            AnyNodeRef::NamespaceAliasingClause(_) => crate::missing!(),
            AnyNodeRef::NamespaceDefinition(_) => crate::missing!(),
            AnyNodeRef::NamespaceName(_) => crate::missing!(),
            AnyNodeRef::NamespaceNameAsPrefix(_) => crate::missing!(),
            AnyNodeRef::NamespaceUseClause(_) => crate::missing!(),
            AnyNodeRef::NamespaceUseDeclaration(_) => crate::missing!(),
            AnyNodeRef::NamespaceUseGroup(_) => crate::missing!(),
            AnyNodeRef::NamespaceUseGroupClause(_) => crate::missing!(),
            AnyNodeRef::Nowdoc(_) => crate::missing!(),
            AnyNodeRef::NowdocBody(_) => crate::missing!(),
            AnyNodeRef::Null(_) => crate::missing!(),
            AnyNodeRef::NullsafeMemberAccessExpression(_) => crate::missing!(),
            AnyNodeRef::NullsafeMemberCallExpression(_) => crate::missing!(),
            AnyNodeRef::ObjectCreationExpression(_) => crate::missing!(),
            AnyNodeRef::OptionalType(_) => crate::missing!(),
            AnyNodeRef::Pair(_) => crate::missing!(),
            AnyNodeRef::ParenthesizedExpression(_) => crate::missing!(),
            AnyNodeRef::PrimitiveType(_) => crate::missing!(),
            AnyNodeRef::PrintIntrinsic(_) => crate::missing!(),
            AnyNodeRef::Program(_) => crate::missing!(),
            AnyNodeRef::PropertyDeclaration(_) => crate::missing!(),
            AnyNodeRef::PropertyElement(_) => crate::missing!(),
            AnyNodeRef::PropertyInitializer(_) => crate::missing!(),
            AnyNodeRef::PropertyPromotionParameter(_) => crate::missing!(),
            AnyNodeRef::QualifiedName(_) => crate::missing!(),
            AnyNodeRef::ReadonlyModifier(_) => crate::missing!(),
            AnyNodeRef::ReferenceAssignmentExpression(_) => crate::missing!(),
            AnyNodeRef::ReferenceModifier(_) => crate::missing!(),
            AnyNodeRef::RelativeScope(_) => crate::missing!(),
            AnyNodeRef::RequireExpression(_) => crate::missing!(),
            AnyNodeRef::RequireOnceExpression(_) => crate::missing!(),
            AnyNodeRef::ReturnStatement(_) => crate::missing!(),
            AnyNodeRef::ScopedCallExpression(_) => crate::missing!(),
            AnyNodeRef::ScopedPropertyAccessExpression(_) => crate::missing!(),
            AnyNodeRef::SequenceExpression(_) => crate::missing!(),
            AnyNodeRef::ShellCommandExpression(_) => crate::missing!(),
            AnyNodeRef::SimpleParameter(_) => crate::missing!(),
            AnyNodeRef::StaticModifier(_) => crate::missing!(),
            AnyNodeRef::StaticVariableDeclaration(_) => crate::missing!(),
            AnyNodeRef::String(_) => crate::missing!(),
            AnyNodeRef::SubscriptExpression(_) => crate::missing!(),
            AnyNodeRef::SwitchBlock(_) => crate::missing!(),
            AnyNodeRef::SwitchStatement(_) => crate::missing!(),
            AnyNodeRef::Text(_) => crate::missing!(),
            AnyNodeRef::TextInterpolation(_) => crate::missing!(),
            AnyNodeRef::ThrowExpression(_) => crate::missing!(),
            AnyNodeRef::TraitDeclaration(_) => crate::missing!(),
            AnyNodeRef::TryStatement(_) => crate::missing!(),
            AnyNodeRef::TypeList(_) => crate::missing!(),
            AnyNodeRef::UnaryOpExpression(_) => crate::missing!(),
            AnyNodeRef::UnionType(_) => crate::missing!(),
            AnyNodeRef::UnsetStatement(_) => crate::missing!(),
            AnyNodeRef::UpdateExpression(_) => crate::missing!(),
            AnyNodeRef::UseAsClause(_) => crate::missing!(),
            AnyNodeRef::UseDeclaration(_) => crate::missing!(),
            AnyNodeRef::UseInsteadOfClause(_) => crate::missing!(),
            AnyNodeRef::UseList(_) => crate::missing!(),
            AnyNodeRef::VariableName(v) => v.write_to(state, emitter, val_type, value),
            AnyNodeRef::VariadicParameter(_) => crate::missing!(),
            AnyNodeRef::VariadicPlaceholder(_) => crate::missing!(),
            AnyNodeRef::VariadicUnpacking(_) => crate::missing!(),
            AnyNodeRef::VisibilityModifier(_) => crate::missing!(),
            AnyNodeRef::WhileStatement(_) => crate::missing!(),
            AnyNodeRef::YieldExpression(_) => crate::missing!(),
            AnyNodeRef::BottomType(_) => crate::missing!(),
            AnyNodeRef::Comment(_) => crate::missing!(),
            AnyNodeRef::EscapeSequence(_) => crate::missing!(),
            AnyNodeRef::Float(_) => crate::missing!(),
            AnyNodeRef::HeredocEnd(_) => crate::missing!(),
            AnyNodeRef::HeredocStart(_) => crate::missing!(),
            AnyNodeRef::Integer(_) => crate::missing!(),
            AnyNodeRef::NowdocString(_) => crate::missing!(),
            AnyNodeRef::PhpTag(_) => crate::missing!(),
            AnyNodeRef::StringValue(_) => crate::missing!(),
            AnyNodeRef::VarModifier(_) => crate::missing!(),
        }
    }
}
