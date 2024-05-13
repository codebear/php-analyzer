use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::_literal::_LiteralNode;
use crate::autonodes::_primary_expression::_PrimaryExpressionNode;
use crate::autonodes::_statement::_StatementNode;
use crate::autonodes::_type::_TypeNode;
use crate::autonodes::abstract_modifier::AbstractModifierNode;
use crate::autonodes::anonymous_function_creation_expression::AnonymousFunctionCreationExpressionNode;
use crate::autonodes::anonymous_function_use_clause::AnonymousFunctionUseClauseNode;
use crate::autonodes::argument::ArgumentNode;
use crate::autonodes::arguments::ArgumentsNode;
use crate::autonodes::array_creation_expression::ArrayCreationExpressionNode;
use crate::autonodes::array_element_initializer::ArrayElementInitializerNode;
use crate::autonodes::arrow_function::ArrowFunctionNode;
use crate::autonodes::assignment_expression::AssignmentExpressionNode;
use crate::autonodes::attribute::AttributeNode;
use crate::autonodes::attribute_group::AttributeGroupNode;
use crate::autonodes::attribute_list::AttributeListNode;
use crate::autonodes::augmented_assignment_expression::AugmentedAssignmentExpressionNode;
use crate::autonodes::base_clause::BaseClauseNode;
use crate::autonodes::binary_expression::BinaryExpressionNode;
use crate::autonodes::boolean::BooleanNode;
use crate::autonodes::bottom_type::BottomTypeNode;
use crate::autonodes::break_statement::BreakStatementNode;
use crate::autonodes::by_ref::ByRefNode;
use crate::autonodes::case_statement::CaseStatementNode;
use crate::autonodes::cast_expression::CastExpressionNode;
use crate::autonodes::cast_type::CastTypeNode;
use crate::autonodes::catch_clause::CatchClauseNode;
use crate::autonodes::class_constant_access_expression::ClassConstantAccessExpressionNode;
use crate::autonodes::class_constant_access_identifier::ClassConstantAccessIdentifierNode;
use crate::autonodes::class_declaration::ClassDeclarationNode;
use crate::autonodes::class_interface_clause::ClassInterfaceClauseNode;
use crate::autonodes::clone_expression::CloneExpressionNode;
use crate::autonodes::colon_block::ColonBlockNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::compound_statement::CompoundStatementNode;
use crate::autonodes::conditional_expression::ConditionalExpressionNode;
use crate::autonodes::const_declaration::ConstDeclarationNode;
use crate::autonodes::const_element::ConstElementNode;
use crate::autonodes::continue_statement::ContinueStatementNode;
use crate::autonodes::declaration_list::DeclarationListNode;
use crate::autonodes::declare_directive::DeclareDirectiveNode;
use crate::autonodes::declare_statement::DeclareStatementNode;
use crate::autonodes::default_statement::DefaultStatementNode;
use crate::autonodes::disjunctive_normal_form_type::DisjunctiveNormalFormTypeNode;
use crate::autonodes::do_statement::DoStatementNode;
use crate::autonodes::dynamic_variable_name::DynamicVariableNameNode;
use crate::autonodes::echo_statement::EchoStatementNode;
use crate::autonodes::else_clause::ElseClauseNode;
use crate::autonodes::else_if_clause::ElseIfClauseNode;
use crate::autonodes::empty_statement::EmptyStatementNode;
use crate::autonodes::encapsed_string::EncapsedStringNode;
use crate::autonodes::enum_case::EnumCaseNode;
use crate::autonodes::enum_declaration::EnumDeclarationNode;
use crate::autonodes::enum_declaration_list::EnumDeclarationListNode;
use crate::autonodes::error_suppression_expression::ErrorSuppressionExpressionNode;
use crate::autonodes::escape_sequence::EscapeSequenceNode;
use crate::autonodes::expression_statement::ExpressionStatementNode;
use crate::autonodes::final_modifier::FinalModifierNode;
use crate::autonodes::finally_clause::FinallyClauseNode;
use crate::autonodes::float::FloatNode;
use crate::autonodes::for_statement::ForStatementNode;
use crate::autonodes::foreach_statement::ForeachStatementNode;
use crate::autonodes::formal_parameters::FormalParametersNode;
use crate::autonodes::function_call_expression::FunctionCallExpressionNode;
use crate::autonodes::function_definition::FunctionDefinitionNode;
use crate::autonodes::function_static_declaration::FunctionStaticDeclarationNode;
use crate::autonodes::global_declaration::GlobalDeclarationNode;
use crate::autonodes::goto_statement::GotoStatementNode;
use crate::autonodes::heredoc::HeredocNode;
use crate::autonodes::heredoc_body::HeredocBodyNode;
use crate::autonodes::heredoc_end::HeredocEndNode;
use crate::autonodes::heredoc_start::HeredocStartNode;
use crate::autonodes::if_statement::IfStatementNode;
use crate::autonodes::include_expression::IncludeExpressionNode;
use crate::autonodes::include_once_expression::IncludeOnceExpressionNode;
use crate::autonodes::integer::IntegerNode;
use crate::autonodes::interface_declaration::InterfaceDeclarationNode;
use crate::autonodes::intersection_type::IntersectionTypeNode;
use crate::autonodes::list_literal::ListLiteralNode;
use crate::autonodes::match_block::MatchBlockNode;
use crate::autonodes::match_condition_list::MatchConditionListNode;
use crate::autonodes::match_conditional_expression::MatchConditionalExpressionNode;
use crate::autonodes::match_default_expression::MatchDefaultExpressionNode;
use crate::autonodes::match_expression::MatchExpressionNode;
use crate::autonodes::member_access_expression::MemberAccessExpressionNode;
use crate::autonodes::member_call_expression::MemberCallExpressionNode;
use crate::autonodes::method_declaration::MethodDeclarationNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::named_label_statement::NamedLabelStatementNode;
use crate::autonodes::named_type::NamedTypeNode;
use crate::autonodes::namespace_aliasing_clause::NamespaceAliasingClauseNode;
use crate::autonodes::namespace_definition::NamespaceDefinitionNode;
use crate::autonodes::namespace_name::NamespaceNameNode;
use crate::autonodes::namespace_name_as_prefix::NamespaceNameAsPrefixNode;
use crate::autonodes::namespace_use_clause::NamespaceUseClauseNode;
use crate::autonodes::namespace_use_declaration::NamespaceUseDeclarationNode;
use crate::autonodes::namespace_use_group::NamespaceUseGroupNode;
use crate::autonodes::namespace_use_group_clause::NamespaceUseGroupClauseNode;
use crate::autonodes::nowdoc::NowdocNode;
use crate::autonodes::nowdoc_body::NowdocBodyNode;
use crate::autonodes::nowdoc_string::NowdocStringNode;
use crate::autonodes::null::NullNode;
use crate::autonodes::nullsafe_member_access_expression::NullsafeMemberAccessExpressionNode;
use crate::autonodes::nullsafe_member_call_expression::NullsafeMemberCallExpressionNode;
use crate::autonodes::object_creation_expression::ObjectCreationExpressionNode;
use crate::autonodes::optional_type::OptionalTypeNode;
use crate::autonodes::pair::PairNode;
use crate::autonodes::parenthesized_expression::ParenthesizedExpressionNode;
use crate::autonodes::php_tag::PhpTagNode;
use crate::autonodes::primitive_type::PrimitiveTypeNode;
use crate::autonodes::print_intrinsic::PrintIntrinsicNode;
use crate::autonodes::program::ProgramNode;
use crate::autonodes::property_declaration::PropertyDeclarationNode;
use crate::autonodes::property_element::PropertyElementNode;
use crate::autonodes::property_initializer::PropertyInitializerNode;
use crate::autonodes::property_promotion_parameter::PropertyPromotionParameterNode;
use crate::autonodes::qualified_name::QualifiedNameNode;
use crate::autonodes::readonly_modifier::ReadonlyModifierNode;
use crate::autonodes::reference_assignment_expression::ReferenceAssignmentExpressionNode;
use crate::autonodes::reference_modifier::ReferenceModifierNode;
use crate::autonodes::relative_scope::RelativeScopeNode;
use crate::autonodes::require_expression::RequireExpressionNode;
use crate::autonodes::require_once_expression::RequireOnceExpressionNode;
use crate::autonodes::return_statement::ReturnStatementNode;
use crate::autonodes::scoped_call_expression::ScopedCallExpressionNode;
use crate::autonodes::scoped_property_access_expression::ScopedPropertyAccessExpressionNode;
use crate::autonodes::sequence_expression::SequenceExpressionNode;
use crate::autonodes::shell_command_expression::ShellCommandExpressionNode;
use crate::autonodes::simple_parameter::SimpleParameterNode;
use crate::autonodes::static_modifier::StaticModifierNode;
use crate::autonodes::static_variable_declaration::StaticVariableDeclarationNode;
use crate::autonodes::string::StringNode;
use crate::autonodes::string_value::StringValueNode;
use crate::autonodes::subscript_expression::SubscriptExpressionNode;
use crate::autonodes::switch_block::SwitchBlockNode;
use crate::autonodes::switch_statement::SwitchStatementNode;
use crate::autonodes::text::TextNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::throw_expression::ThrowExpressionNode;
use crate::autonodes::trait_declaration::TraitDeclarationNode;
use crate::autonodes::try_statement::TryStatementNode;
use crate::autonodes::type_list::TypeListNode;
use crate::autonodes::unary_op_expression::UnaryOpExpressionNode;
use crate::autonodes::union_type::UnionTypeNode;
use crate::autonodes::unset_statement::UnsetStatementNode;
use crate::autonodes::update_expression::UpdateExpressionNode;
use crate::autonodes::use_as_clause::UseAsClauseNode;
use crate::autonodes::use_declaration::UseDeclarationNode;
use crate::autonodes::use_instead_of_clause::UseInsteadOfClauseNode;
use crate::autonodes::use_list::UseListNode;
use crate::autonodes::var_modifier::VarModifierNode;
use crate::autonodes::variable_name::VariableNameNode;
use crate::autonodes::variadic_parameter::VariadicParameterNode;
use crate::autonodes::variadic_placeholder::VariadicPlaceholderNode;
use crate::autonodes::variadic_unpacking::VariadicUnpackingNode;
use crate::autonodes::visibility_modifier::VisibilityModifierNode;
use crate::autonodes::while_statement::WhileStatementNode;
use crate::autonodes::yield_expression::YieldExpressionNode;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::parser::Range;
use tree_sitter::Node;

use crate::operators::operator::Operators;

#[derive(Debug, Clone)]
pub enum AnyNodeRef<'a> {
    StaticExpr(&'static str, Range),
    Error(&'a ErrorNode),
    Operator(Operators<'a>),
    _Expression(&'a _ExpressionNode),
    _Literal(&'a _LiteralNode),
    _PrimaryExpression(&'a _PrimaryExpressionNode),
    _Statement(&'a _StatementNode),
    _Type(&'a _TypeNode),
    AbstractModifier(&'a AbstractModifierNode),
    AnonymousFunctionCreationExpression(&'a AnonymousFunctionCreationExpressionNode),
    AnonymousFunctionUseClause(&'a AnonymousFunctionUseClauseNode),
    Argument(&'a ArgumentNode),
    Arguments(&'a ArgumentsNode),
    ArrayCreationExpression(&'a ArrayCreationExpressionNode),
    ArrayElementInitializer(&'a ArrayElementInitializerNode),
    ArrowFunction(&'a ArrowFunctionNode),
    AssignmentExpression(&'a AssignmentExpressionNode),
    Attribute(&'a AttributeNode),
    AttributeGroup(&'a AttributeGroupNode),
    AttributeList(&'a AttributeListNode),
    AugmentedAssignmentExpression(&'a AugmentedAssignmentExpressionNode),
    BaseClause(&'a BaseClauseNode),
    BinaryExpression(&'a BinaryExpressionNode),
    Boolean(&'a BooleanNode),
    BreakStatement(&'a BreakStatementNode),
    ByRef(&'a ByRefNode),
    CaseStatement(&'a CaseStatementNode),
    CastExpression(&'a CastExpressionNode),
    CastType(&'a CastTypeNode),
    CatchClause(&'a CatchClauseNode),
    ClassConstantAccessExpression(&'a ClassConstantAccessExpressionNode),
    ClassConstantAccessIdentifier(&'a ClassConstantAccessIdentifierNode),
    ClassDeclaration(&'a ClassDeclarationNode),
    ClassInterfaceClause(&'a ClassInterfaceClauseNode),
    CloneExpression(&'a CloneExpressionNode),
    ColonBlock(&'a ColonBlockNode),
    CompoundStatement(&'a CompoundStatementNode),
    ConditionalExpression(&'a ConditionalExpressionNode),
    ConstDeclaration(&'a ConstDeclarationNode),
    ConstElement(&'a ConstElementNode),
    ContinueStatement(&'a ContinueStatementNode),
    DeclarationList(&'a DeclarationListNode),
    DeclareDirective(&'a DeclareDirectiveNode),
    DeclareStatement(&'a DeclareStatementNode),
    DefaultStatement(&'a DefaultStatementNode),
    DisjunctiveNormalFormType(&'a DisjunctiveNormalFormTypeNode),
    DoStatement(&'a DoStatementNode),
    DynamicVariableName(&'a DynamicVariableNameNode),
    EchoStatement(&'a EchoStatementNode),
    ElseClause(&'a ElseClauseNode),
    ElseIfClause(&'a ElseIfClauseNode),
    EmptyStatement(&'a EmptyStatementNode),
    EncapsedString(&'a EncapsedStringNode),
    EnumCase(&'a EnumCaseNode),
    EnumDeclaration(&'a EnumDeclarationNode),
    EnumDeclarationList(&'a EnumDeclarationListNode),
    ErrorSuppressionExpression(&'a ErrorSuppressionExpressionNode),
    ExpressionStatement(&'a ExpressionStatementNode),
    FinalModifier(&'a FinalModifierNode),
    FinallyClause(&'a FinallyClauseNode),
    ForStatement(&'a ForStatementNode),
    ForeachStatement(&'a ForeachStatementNode),
    FormalParameters(&'a FormalParametersNode),
    FunctionCallExpression(&'a FunctionCallExpressionNode),
    FunctionDefinition(&'a FunctionDefinitionNode),
    FunctionStaticDeclaration(&'a FunctionStaticDeclarationNode),
    GlobalDeclaration(&'a GlobalDeclarationNode),
    GotoStatement(&'a GotoStatementNode),
    Heredoc(&'a HeredocNode),
    HeredocBody(&'a HeredocBodyNode),
    IfStatement(&'a IfStatementNode),
    IncludeExpression(&'a IncludeExpressionNode),
    IncludeOnceExpression(&'a IncludeOnceExpressionNode),
    InterfaceDeclaration(&'a InterfaceDeclarationNode),
    IntersectionType(&'a IntersectionTypeNode),
    ListLiteral(&'a ListLiteralNode),
    MatchBlock(&'a MatchBlockNode),
    MatchConditionList(&'a MatchConditionListNode),
    MatchConditionalExpression(&'a MatchConditionalExpressionNode),
    MatchDefaultExpression(&'a MatchDefaultExpressionNode),
    MatchExpression(&'a MatchExpressionNode),
    MemberAccessExpression(&'a MemberAccessExpressionNode),
    MemberCallExpression(&'a MemberCallExpressionNode),
    MethodDeclaration(&'a MethodDeclarationNode),
    Name(&'a NameNode),
    NamedLabelStatement(&'a NamedLabelStatementNode),
    NamedType(&'a NamedTypeNode),
    NamespaceAliasingClause(&'a NamespaceAliasingClauseNode),
    NamespaceDefinition(&'a NamespaceDefinitionNode),
    NamespaceName(&'a NamespaceNameNode),
    NamespaceNameAsPrefix(&'a NamespaceNameAsPrefixNode),
    NamespaceUseClause(&'a NamespaceUseClauseNode),
    NamespaceUseDeclaration(&'a NamespaceUseDeclarationNode),
    NamespaceUseGroup(&'a NamespaceUseGroupNode),
    NamespaceUseGroupClause(&'a NamespaceUseGroupClauseNode),
    Nowdoc(&'a NowdocNode),
    NowdocBody(&'a NowdocBodyNode),
    Null(&'a NullNode),
    NullsafeMemberAccessExpression(&'a NullsafeMemberAccessExpressionNode),
    NullsafeMemberCallExpression(&'a NullsafeMemberCallExpressionNode),
    ObjectCreationExpression(&'a ObjectCreationExpressionNode),
    OptionalType(&'a OptionalTypeNode),
    Pair(&'a PairNode),
    ParenthesizedExpression(&'a ParenthesizedExpressionNode),
    PrimitiveType(&'a PrimitiveTypeNode),
    PrintIntrinsic(&'a PrintIntrinsicNode),
    Program(&'a ProgramNode),
    PropertyDeclaration(&'a PropertyDeclarationNode),
    PropertyElement(&'a PropertyElementNode),
    PropertyInitializer(&'a PropertyInitializerNode),
    PropertyPromotionParameter(&'a PropertyPromotionParameterNode),
    QualifiedName(&'a QualifiedNameNode),
    ReadonlyModifier(&'a ReadonlyModifierNode),
    ReferenceAssignmentExpression(&'a ReferenceAssignmentExpressionNode),
    ReferenceModifier(&'a ReferenceModifierNode),
    RelativeScope(&'a RelativeScopeNode),
    RequireExpression(&'a RequireExpressionNode),
    RequireOnceExpression(&'a RequireOnceExpressionNode),
    ReturnStatement(&'a ReturnStatementNode),
    ScopedCallExpression(&'a ScopedCallExpressionNode),
    ScopedPropertyAccessExpression(&'a ScopedPropertyAccessExpressionNode),
    SequenceExpression(&'a SequenceExpressionNode),
    ShellCommandExpression(&'a ShellCommandExpressionNode),
    SimpleParameter(&'a SimpleParameterNode),
    StaticModifier(&'a StaticModifierNode),
    StaticVariableDeclaration(&'a StaticVariableDeclarationNode),
    String(&'a StringNode),
    SubscriptExpression(&'a SubscriptExpressionNode),
    SwitchBlock(&'a SwitchBlockNode),
    SwitchStatement(&'a SwitchStatementNode),
    Text(&'a TextNode),
    TextInterpolation(&'a TextInterpolationNode),
    ThrowExpression(&'a ThrowExpressionNode),
    TraitDeclaration(&'a TraitDeclarationNode),
    TryStatement(&'a TryStatementNode),
    TypeList(&'a TypeListNode),
    UnaryOpExpression(&'a UnaryOpExpressionNode),
    UnionType(&'a UnionTypeNode),
    UnsetStatement(&'a UnsetStatementNode),
    UpdateExpression(&'a UpdateExpressionNode),
    UseAsClause(&'a UseAsClauseNode),
    UseDeclaration(&'a UseDeclarationNode),
    UseInsteadOfClause(&'a UseInsteadOfClauseNode),
    UseList(&'a UseListNode),
    VariableName(&'a VariableNameNode),
    VariadicParameter(&'a VariadicParameterNode),
    VariadicPlaceholder(&'a VariadicPlaceholderNode),
    VariadicUnpacking(&'a VariadicUnpackingNode),
    VisibilityModifier(&'a VisibilityModifierNode),
    WhileStatement(&'a WhileStatementNode),
    YieldExpression(&'a YieldExpressionNode),
    BottomType(&'a BottomTypeNode),
    Comment(&'a CommentNode),
    EscapeSequence(&'a EscapeSequenceNode),
    Float(&'a FloatNode),
    HeredocEnd(&'a HeredocEndNode),
    HeredocStart(&'a HeredocStartNode),
    Integer(&'a IntegerNode),
    NowdocString(&'a NowdocStringNode),
    PhpTag(&'a PhpTagNode),
    StringValue(&'a StringValueNode),
    VarModifier(&'a VarModifierNode),
}

impl<'a> AnyNodeRef<'a> {
    pub fn kind(&self) -> &'static str {
        match self {
            AnyNodeRef::StaticExpr(x, _) => x,
            AnyNodeRef::Error(e) => e.kind(),
            AnyNodeRef::Operator(op) => op.kind(),
            AnyNodeRef::_Expression(n) => n.kind(),
            AnyNodeRef::_Literal(n) => n.kind(),
            AnyNodeRef::_PrimaryExpression(n) => n.kind(),
            AnyNodeRef::_Statement(n) => n.kind(),
            AnyNodeRef::_Type(n) => n.kind(),
            AnyNodeRef::AbstractModifier(n) => n.kind(),
            AnyNodeRef::AnonymousFunctionCreationExpression(n) => n.kind(),
            AnyNodeRef::AnonymousFunctionUseClause(n) => n.kind(),
            AnyNodeRef::Argument(n) => n.kind(),
            AnyNodeRef::Arguments(n) => n.kind(),
            AnyNodeRef::ArrayCreationExpression(n) => n.kind(),
            AnyNodeRef::ArrayElementInitializer(n) => n.kind(),
            AnyNodeRef::ArrowFunction(n) => n.kind(),
            AnyNodeRef::AssignmentExpression(n) => n.kind(),
            AnyNodeRef::Attribute(n) => n.kind(),
            AnyNodeRef::AttributeGroup(n) => n.kind(),
            AnyNodeRef::AttributeList(n) => n.kind(),
            AnyNodeRef::AugmentedAssignmentExpression(n) => n.kind(),
            AnyNodeRef::BaseClause(n) => n.kind(),
            AnyNodeRef::BinaryExpression(n) => n.kind(),
            AnyNodeRef::Boolean(n) => n.kind(),
            AnyNodeRef::BreakStatement(n) => n.kind(),
            AnyNodeRef::ByRef(n) => n.kind(),
            AnyNodeRef::CaseStatement(n) => n.kind(),
            AnyNodeRef::CastExpression(n) => n.kind(),
            AnyNodeRef::CastType(n) => n.kind(),
            AnyNodeRef::CatchClause(n) => n.kind(),
            AnyNodeRef::ClassConstantAccessExpression(n) => n.kind(),
            AnyNodeRef::ClassConstantAccessIdentifier(n) => n.kind(),
            AnyNodeRef::ClassDeclaration(n) => n.kind(),
            AnyNodeRef::ClassInterfaceClause(n) => n.kind(),
            AnyNodeRef::CloneExpression(n) => n.kind(),
            AnyNodeRef::ColonBlock(n) => n.kind(),
            AnyNodeRef::CompoundStatement(n) => n.kind(),
            AnyNodeRef::ConditionalExpression(n) => n.kind(),
            AnyNodeRef::ConstDeclaration(n) => n.kind(),
            AnyNodeRef::ConstElement(n) => n.kind(),
            AnyNodeRef::ContinueStatement(n) => n.kind(),
            AnyNodeRef::DeclarationList(n) => n.kind(),
            AnyNodeRef::DeclareDirective(n) => n.kind(),
            AnyNodeRef::DeclareStatement(n) => n.kind(),
            AnyNodeRef::DefaultStatement(n) => n.kind(),
            AnyNodeRef::DisjunctiveNormalFormType(n) => n.kind(),
            AnyNodeRef::DoStatement(n) => n.kind(),
            AnyNodeRef::DynamicVariableName(n) => n.kind(),
            AnyNodeRef::EchoStatement(n) => n.kind(),
            AnyNodeRef::ElseClause(n) => n.kind(),
            AnyNodeRef::ElseIfClause(n) => n.kind(),
            AnyNodeRef::EmptyStatement(n) => n.kind(),
            AnyNodeRef::EncapsedString(n) => n.kind(),
            AnyNodeRef::EnumCase(n) => n.kind(),
            AnyNodeRef::EnumDeclaration(n) => n.kind(),
            AnyNodeRef::EnumDeclarationList(n) => n.kind(),
            AnyNodeRef::ErrorSuppressionExpression(n) => n.kind(),
            AnyNodeRef::ExpressionStatement(n) => n.kind(),
            AnyNodeRef::FinalModifier(n) => n.kind(),
            AnyNodeRef::FinallyClause(n) => n.kind(),
            AnyNodeRef::ForStatement(n) => n.kind(),
            AnyNodeRef::ForeachStatement(n) => n.kind(),
            AnyNodeRef::FormalParameters(n) => n.kind(),
            AnyNodeRef::FunctionCallExpression(n) => n.kind(),
            AnyNodeRef::FunctionDefinition(n) => n.kind(),
            AnyNodeRef::FunctionStaticDeclaration(n) => n.kind(),
            AnyNodeRef::GlobalDeclaration(n) => n.kind(),
            AnyNodeRef::GotoStatement(n) => n.kind(),
            AnyNodeRef::Heredoc(n) => n.kind(),
            AnyNodeRef::HeredocBody(n) => n.kind(),
            AnyNodeRef::IfStatement(n) => n.kind(),
            AnyNodeRef::IncludeExpression(n) => n.kind(),
            AnyNodeRef::IncludeOnceExpression(n) => n.kind(),
            AnyNodeRef::InterfaceDeclaration(n) => n.kind(),
            AnyNodeRef::IntersectionType(n) => n.kind(),
            AnyNodeRef::ListLiteral(n) => n.kind(),
            AnyNodeRef::MatchBlock(n) => n.kind(),
            AnyNodeRef::MatchConditionList(n) => n.kind(),
            AnyNodeRef::MatchConditionalExpression(n) => n.kind(),
            AnyNodeRef::MatchDefaultExpression(n) => n.kind(),
            AnyNodeRef::MatchExpression(n) => n.kind(),
            AnyNodeRef::MemberAccessExpression(n) => n.kind(),
            AnyNodeRef::MemberCallExpression(n) => n.kind(),
            AnyNodeRef::MethodDeclaration(n) => n.kind(),
            AnyNodeRef::Name(n) => n.kind(),
            AnyNodeRef::NamedLabelStatement(n) => n.kind(),
            AnyNodeRef::NamedType(n) => n.kind(),
            AnyNodeRef::NamespaceAliasingClause(n) => n.kind(),
            AnyNodeRef::NamespaceDefinition(n) => n.kind(),
            AnyNodeRef::NamespaceName(n) => n.kind(),
            AnyNodeRef::NamespaceNameAsPrefix(n) => n.kind(),
            AnyNodeRef::NamespaceUseClause(n) => n.kind(),
            AnyNodeRef::NamespaceUseDeclaration(n) => n.kind(),
            AnyNodeRef::NamespaceUseGroup(n) => n.kind(),
            AnyNodeRef::NamespaceUseGroupClause(n) => n.kind(),
            AnyNodeRef::Nowdoc(n) => n.kind(),
            AnyNodeRef::NowdocBody(n) => n.kind(),
            AnyNodeRef::Null(n) => n.kind(),
            AnyNodeRef::NullsafeMemberAccessExpression(n) => n.kind(),
            AnyNodeRef::NullsafeMemberCallExpression(n) => n.kind(),
            AnyNodeRef::ObjectCreationExpression(n) => n.kind(),
            AnyNodeRef::OptionalType(n) => n.kind(),
            AnyNodeRef::Pair(n) => n.kind(),
            AnyNodeRef::ParenthesizedExpression(n) => n.kind(),
            AnyNodeRef::PrimitiveType(n) => n.kind(),
            AnyNodeRef::PrintIntrinsic(n) => n.kind(),
            AnyNodeRef::Program(n) => n.kind(),
            AnyNodeRef::PropertyDeclaration(n) => n.kind(),
            AnyNodeRef::PropertyElement(n) => n.kind(),
            AnyNodeRef::PropertyInitializer(n) => n.kind(),
            AnyNodeRef::PropertyPromotionParameter(n) => n.kind(),
            AnyNodeRef::QualifiedName(n) => n.kind(),
            AnyNodeRef::ReadonlyModifier(n) => n.kind(),
            AnyNodeRef::ReferenceAssignmentExpression(n) => n.kind(),
            AnyNodeRef::ReferenceModifier(n) => n.kind(),
            AnyNodeRef::RelativeScope(n) => n.kind(),
            AnyNodeRef::RequireExpression(n) => n.kind(),
            AnyNodeRef::RequireOnceExpression(n) => n.kind(),
            AnyNodeRef::ReturnStatement(n) => n.kind(),
            AnyNodeRef::ScopedCallExpression(n) => n.kind(),
            AnyNodeRef::ScopedPropertyAccessExpression(n) => n.kind(),
            AnyNodeRef::SequenceExpression(n) => n.kind(),
            AnyNodeRef::ShellCommandExpression(n) => n.kind(),
            AnyNodeRef::SimpleParameter(n) => n.kind(),
            AnyNodeRef::StaticModifier(n) => n.kind(),
            AnyNodeRef::StaticVariableDeclaration(n) => n.kind(),
            AnyNodeRef::String(n) => n.kind(),
            AnyNodeRef::SubscriptExpression(n) => n.kind(),
            AnyNodeRef::SwitchBlock(n) => n.kind(),
            AnyNodeRef::SwitchStatement(n) => n.kind(),
            AnyNodeRef::Text(n) => n.kind(),
            AnyNodeRef::TextInterpolation(n) => n.kind(),
            AnyNodeRef::ThrowExpression(n) => n.kind(),
            AnyNodeRef::TraitDeclaration(n) => n.kind(),
            AnyNodeRef::TryStatement(n) => n.kind(),
            AnyNodeRef::TypeList(n) => n.kind(),
            AnyNodeRef::UnaryOpExpression(n) => n.kind(),
            AnyNodeRef::UnionType(n) => n.kind(),
            AnyNodeRef::UnsetStatement(n) => n.kind(),
            AnyNodeRef::UpdateExpression(n) => n.kind(),
            AnyNodeRef::UseAsClause(n) => n.kind(),
            AnyNodeRef::UseDeclaration(n) => n.kind(),
            AnyNodeRef::UseInsteadOfClause(n) => n.kind(),
            AnyNodeRef::UseList(n) => n.kind(),
            AnyNodeRef::VariableName(n) => n.kind(),
            AnyNodeRef::VariadicParameter(n) => n.kind(),
            AnyNodeRef::VariadicPlaceholder(n) => n.kind(),
            AnyNodeRef::VariadicUnpacking(n) => n.kind(),
            AnyNodeRef::VisibilityModifier(n) => n.kind(),
            AnyNodeRef::WhileStatement(n) => n.kind(),
            AnyNodeRef::YieldExpression(n) => n.kind(),
            AnyNodeRef::BottomType(n) => n.kind(),
            AnyNodeRef::Comment(n) => n.kind(),
            AnyNodeRef::EscapeSequence(n) => n.kind(),
            AnyNodeRef::Float(n) => n.kind(),
            AnyNodeRef::HeredocEnd(n) => n.kind(),
            AnyNodeRef::HeredocStart(n) => n.kind(),
            AnyNodeRef::Integer(n) => n.kind(),
            AnyNodeRef::NowdocString(n) => n.kind(),
            AnyNodeRef::PhpTag(n) => n.kind(),
            AnyNodeRef::StringValue(n) => n.kind(),
            AnyNodeRef::VarModifier(n) => n.kind(),
        }
    }
}

impl<'a> NodeAccess for AnyNodeRef<'a> {
    fn brief_desc(&self) -> String {
        match self {
            AnyNodeRef::StaticExpr(x, _) => x.to_string(),
            AnyNodeRef::Error(e) => e.brief_desc(),
            AnyNodeRef::Operator(op) => op.brief_desc(),
            AnyNodeRef::_Expression(n) => n.brief_desc(),
            AnyNodeRef::_Literal(n) => n.brief_desc(),
            AnyNodeRef::_PrimaryExpression(n) => n.brief_desc(),
            AnyNodeRef::_Statement(n) => n.brief_desc(),
            AnyNodeRef::_Type(n) => n.brief_desc(),
            AnyNodeRef::AbstractModifier(n) => n.brief_desc(),
            AnyNodeRef::AnonymousFunctionCreationExpression(n) => n.brief_desc(),
            AnyNodeRef::AnonymousFunctionUseClause(n) => n.brief_desc(),
            AnyNodeRef::Argument(n) => n.brief_desc(),
            AnyNodeRef::Arguments(n) => n.brief_desc(),
            AnyNodeRef::ArrayCreationExpression(n) => n.brief_desc(),
            AnyNodeRef::ArrayElementInitializer(n) => n.brief_desc(),
            AnyNodeRef::ArrowFunction(n) => n.brief_desc(),
            AnyNodeRef::AssignmentExpression(n) => n.brief_desc(),
            AnyNodeRef::Attribute(n) => n.brief_desc(),
            AnyNodeRef::AttributeGroup(n) => n.brief_desc(),
            AnyNodeRef::AttributeList(n) => n.brief_desc(),
            AnyNodeRef::AugmentedAssignmentExpression(n) => n.brief_desc(),
            AnyNodeRef::BaseClause(n) => n.brief_desc(),
            AnyNodeRef::BinaryExpression(n) => n.brief_desc(),
            AnyNodeRef::Boolean(n) => n.brief_desc(),
            AnyNodeRef::BreakStatement(n) => n.brief_desc(),
            AnyNodeRef::ByRef(n) => n.brief_desc(),
            AnyNodeRef::CaseStatement(n) => n.brief_desc(),
            AnyNodeRef::CastExpression(n) => n.brief_desc(),
            AnyNodeRef::CastType(n) => n.brief_desc(),
            AnyNodeRef::CatchClause(n) => n.brief_desc(),
            AnyNodeRef::ClassConstantAccessExpression(n) => n.brief_desc(),
            AnyNodeRef::ClassConstantAccessIdentifier(n) => n.brief_desc(),
            AnyNodeRef::ClassDeclaration(n) => n.brief_desc(),
            AnyNodeRef::ClassInterfaceClause(n) => n.brief_desc(),
            AnyNodeRef::CloneExpression(n) => n.brief_desc(),
            AnyNodeRef::ColonBlock(n) => n.brief_desc(),
            AnyNodeRef::CompoundStatement(n) => n.brief_desc(),
            AnyNodeRef::ConditionalExpression(n) => n.brief_desc(),
            AnyNodeRef::ConstDeclaration(n) => n.brief_desc(),
            AnyNodeRef::ConstElement(n) => n.brief_desc(),
            AnyNodeRef::ContinueStatement(n) => n.brief_desc(),
            AnyNodeRef::DeclarationList(n) => n.brief_desc(),
            AnyNodeRef::DeclareDirective(n) => n.brief_desc(),
            AnyNodeRef::DeclareStatement(n) => n.brief_desc(),
            AnyNodeRef::DefaultStatement(n) => n.brief_desc(),
            AnyNodeRef::DisjunctiveNormalFormType(n) => n.brief_desc(),
            AnyNodeRef::DoStatement(n) => n.brief_desc(),
            AnyNodeRef::DynamicVariableName(n) => n.brief_desc(),
            AnyNodeRef::EchoStatement(n) => n.brief_desc(),
            AnyNodeRef::ElseClause(n) => n.brief_desc(),
            AnyNodeRef::ElseIfClause(n) => n.brief_desc(),
            AnyNodeRef::EmptyStatement(n) => n.brief_desc(),
            AnyNodeRef::EncapsedString(n) => n.brief_desc(),
            AnyNodeRef::EnumCase(n) => n.brief_desc(),
            AnyNodeRef::EnumDeclaration(n) => n.brief_desc(),
            AnyNodeRef::EnumDeclarationList(n) => n.brief_desc(),
            AnyNodeRef::ErrorSuppressionExpression(n) => n.brief_desc(),
            AnyNodeRef::ExpressionStatement(n) => n.brief_desc(),
            AnyNodeRef::FinalModifier(n) => n.brief_desc(),
            AnyNodeRef::FinallyClause(n) => n.brief_desc(),
            AnyNodeRef::ForStatement(n) => n.brief_desc(),
            AnyNodeRef::ForeachStatement(n) => n.brief_desc(),
            AnyNodeRef::FormalParameters(n) => n.brief_desc(),
            AnyNodeRef::FunctionCallExpression(n) => n.brief_desc(),
            AnyNodeRef::FunctionDefinition(n) => n.brief_desc(),
            AnyNodeRef::FunctionStaticDeclaration(n) => n.brief_desc(),
            AnyNodeRef::GlobalDeclaration(n) => n.brief_desc(),
            AnyNodeRef::GotoStatement(n) => n.brief_desc(),
            AnyNodeRef::Heredoc(n) => n.brief_desc(),
            AnyNodeRef::HeredocBody(n) => n.brief_desc(),
            AnyNodeRef::IfStatement(n) => n.brief_desc(),
            AnyNodeRef::IncludeExpression(n) => n.brief_desc(),
            AnyNodeRef::IncludeOnceExpression(n) => n.brief_desc(),
            AnyNodeRef::InterfaceDeclaration(n) => n.brief_desc(),
            AnyNodeRef::IntersectionType(n) => n.brief_desc(),
            AnyNodeRef::ListLiteral(n) => n.brief_desc(),
            AnyNodeRef::MatchBlock(n) => n.brief_desc(),
            AnyNodeRef::MatchConditionList(n) => n.brief_desc(),
            AnyNodeRef::MatchConditionalExpression(n) => n.brief_desc(),
            AnyNodeRef::MatchDefaultExpression(n) => n.brief_desc(),
            AnyNodeRef::MatchExpression(n) => n.brief_desc(),
            AnyNodeRef::MemberAccessExpression(n) => n.brief_desc(),
            AnyNodeRef::MemberCallExpression(n) => n.brief_desc(),
            AnyNodeRef::MethodDeclaration(n) => n.brief_desc(),
            AnyNodeRef::Name(n) => n.brief_desc(),
            AnyNodeRef::NamedLabelStatement(n) => n.brief_desc(),
            AnyNodeRef::NamedType(n) => n.brief_desc(),
            AnyNodeRef::NamespaceAliasingClause(n) => n.brief_desc(),
            AnyNodeRef::NamespaceDefinition(n) => n.brief_desc(),
            AnyNodeRef::NamespaceName(n) => n.brief_desc(),
            AnyNodeRef::NamespaceNameAsPrefix(n) => n.brief_desc(),
            AnyNodeRef::NamespaceUseClause(n) => n.brief_desc(),
            AnyNodeRef::NamespaceUseDeclaration(n) => n.brief_desc(),
            AnyNodeRef::NamespaceUseGroup(n) => n.brief_desc(),
            AnyNodeRef::NamespaceUseGroupClause(n) => n.brief_desc(),
            AnyNodeRef::Nowdoc(n) => n.brief_desc(),
            AnyNodeRef::NowdocBody(n) => n.brief_desc(),
            AnyNodeRef::Null(n) => n.brief_desc(),
            AnyNodeRef::NullsafeMemberAccessExpression(n) => n.brief_desc(),
            AnyNodeRef::NullsafeMemberCallExpression(n) => n.brief_desc(),
            AnyNodeRef::ObjectCreationExpression(n) => n.brief_desc(),
            AnyNodeRef::OptionalType(n) => n.brief_desc(),
            AnyNodeRef::Pair(n) => n.brief_desc(),
            AnyNodeRef::ParenthesizedExpression(n) => n.brief_desc(),
            AnyNodeRef::PrimitiveType(n) => n.brief_desc(),
            AnyNodeRef::PrintIntrinsic(n) => n.brief_desc(),
            AnyNodeRef::Program(n) => n.brief_desc(),
            AnyNodeRef::PropertyDeclaration(n) => n.brief_desc(),
            AnyNodeRef::PropertyElement(n) => n.brief_desc(),
            AnyNodeRef::PropertyInitializer(n) => n.brief_desc(),
            AnyNodeRef::PropertyPromotionParameter(n) => n.brief_desc(),
            AnyNodeRef::QualifiedName(n) => n.brief_desc(),
            AnyNodeRef::ReadonlyModifier(n) => n.brief_desc(),
            AnyNodeRef::ReferenceAssignmentExpression(n) => n.brief_desc(),
            AnyNodeRef::ReferenceModifier(n) => n.brief_desc(),
            AnyNodeRef::RelativeScope(n) => n.brief_desc(),
            AnyNodeRef::RequireExpression(n) => n.brief_desc(),
            AnyNodeRef::RequireOnceExpression(n) => n.brief_desc(),
            AnyNodeRef::ReturnStatement(n) => n.brief_desc(),
            AnyNodeRef::ScopedCallExpression(n) => n.brief_desc(),
            AnyNodeRef::ScopedPropertyAccessExpression(n) => n.brief_desc(),
            AnyNodeRef::SequenceExpression(n) => n.brief_desc(),
            AnyNodeRef::ShellCommandExpression(n) => n.brief_desc(),
            AnyNodeRef::SimpleParameter(n) => n.brief_desc(),
            AnyNodeRef::StaticModifier(n) => n.brief_desc(),
            AnyNodeRef::StaticVariableDeclaration(n) => n.brief_desc(),
            AnyNodeRef::String(n) => n.brief_desc(),
            AnyNodeRef::SubscriptExpression(n) => n.brief_desc(),
            AnyNodeRef::SwitchBlock(n) => n.brief_desc(),
            AnyNodeRef::SwitchStatement(n) => n.brief_desc(),
            AnyNodeRef::Text(n) => n.brief_desc(),
            AnyNodeRef::TextInterpolation(n) => n.brief_desc(),
            AnyNodeRef::ThrowExpression(n) => n.brief_desc(),
            AnyNodeRef::TraitDeclaration(n) => n.brief_desc(),
            AnyNodeRef::TryStatement(n) => n.brief_desc(),
            AnyNodeRef::TypeList(n) => n.brief_desc(),
            AnyNodeRef::UnaryOpExpression(n) => n.brief_desc(),
            AnyNodeRef::UnionType(n) => n.brief_desc(),
            AnyNodeRef::UnsetStatement(n) => n.brief_desc(),
            AnyNodeRef::UpdateExpression(n) => n.brief_desc(),
            AnyNodeRef::UseAsClause(n) => n.brief_desc(),
            AnyNodeRef::UseDeclaration(n) => n.brief_desc(),
            AnyNodeRef::UseInsteadOfClause(n) => n.brief_desc(),
            AnyNodeRef::UseList(n) => n.brief_desc(),
            AnyNodeRef::VariableName(n) => n.brief_desc(),
            AnyNodeRef::VariadicParameter(n) => n.brief_desc(),
            AnyNodeRef::VariadicPlaceholder(n) => n.brief_desc(),
            AnyNodeRef::VariadicUnpacking(n) => n.brief_desc(),
            AnyNodeRef::VisibilityModifier(n) => n.brief_desc(),
            AnyNodeRef::WhileStatement(n) => n.brief_desc(),
            AnyNodeRef::YieldExpression(n) => n.brief_desc(),
            AnyNodeRef::BottomType(n) => n.brief_desc(),
            AnyNodeRef::Comment(n) => n.brief_desc(),
            AnyNodeRef::EscapeSequence(n) => n.brief_desc(),
            AnyNodeRef::Float(n) => n.brief_desc(),
            AnyNodeRef::HeredocEnd(n) => n.brief_desc(),
            AnyNodeRef::HeredocStart(n) => n.brief_desc(),
            AnyNodeRef::Integer(n) => n.brief_desc(),
            AnyNodeRef::NowdocString(n) => n.brief_desc(),
            AnyNodeRef::PhpTag(n) => n.brief_desc(),
            AnyNodeRef::StringValue(n) => n.brief_desc(),
            AnyNodeRef::VarModifier(n) => n.brief_desc(),
        }
    }

    fn range(&self) -> Range {
        match self {
            AnyNodeRef::StaticExpr(_, r) => *r,
            AnyNodeRef::Error(e) => e.range(),
            AnyNodeRef::Operator(op) => op.range(),
            AnyNodeRef::_Expression(n) => n.range(),
            AnyNodeRef::_Literal(n) => n.range(),
            AnyNodeRef::_PrimaryExpression(n) => n.range(),
            AnyNodeRef::_Statement(n) => n.range(),
            AnyNodeRef::_Type(n) => n.range(),
            AnyNodeRef::AbstractModifier(n) => n.range(),
            AnyNodeRef::AnonymousFunctionCreationExpression(n) => n.range(),
            AnyNodeRef::AnonymousFunctionUseClause(n) => n.range(),
            AnyNodeRef::Argument(n) => n.range(),
            AnyNodeRef::Arguments(n) => n.range(),
            AnyNodeRef::ArrayCreationExpression(n) => n.range(),
            AnyNodeRef::ArrayElementInitializer(n) => n.range(),
            AnyNodeRef::ArrowFunction(n) => n.range(),
            AnyNodeRef::AssignmentExpression(n) => n.range(),
            AnyNodeRef::Attribute(n) => n.range(),
            AnyNodeRef::AttributeGroup(n) => n.range(),
            AnyNodeRef::AttributeList(n) => n.range(),
            AnyNodeRef::AugmentedAssignmentExpression(n) => n.range(),
            AnyNodeRef::BaseClause(n) => n.range(),
            AnyNodeRef::BinaryExpression(n) => n.range(),
            AnyNodeRef::Boolean(n) => n.range(),
            AnyNodeRef::BreakStatement(n) => n.range(),
            AnyNodeRef::ByRef(n) => n.range(),
            AnyNodeRef::CaseStatement(n) => n.range(),
            AnyNodeRef::CastExpression(n) => n.range(),
            AnyNodeRef::CastType(n) => n.range(),
            AnyNodeRef::CatchClause(n) => n.range(),
            AnyNodeRef::ClassConstantAccessExpression(n) => n.range(),
            AnyNodeRef::ClassConstantAccessIdentifier(n) => n.range(),
            AnyNodeRef::ClassDeclaration(n) => n.range(),
            AnyNodeRef::ClassInterfaceClause(n) => n.range(),
            AnyNodeRef::CloneExpression(n) => n.range(),
            AnyNodeRef::ColonBlock(n) => n.range(),
            AnyNodeRef::CompoundStatement(n) => n.range(),
            AnyNodeRef::ConditionalExpression(n) => n.range(),
            AnyNodeRef::ConstDeclaration(n) => n.range(),
            AnyNodeRef::ConstElement(n) => n.range(),
            AnyNodeRef::ContinueStatement(n) => n.range(),
            AnyNodeRef::DeclarationList(n) => n.range(),
            AnyNodeRef::DeclareDirective(n) => n.range(),
            AnyNodeRef::DeclareStatement(n) => n.range(),
            AnyNodeRef::DefaultStatement(n) => n.range(),
            AnyNodeRef::DisjunctiveNormalFormType(n) => n.range(),
            AnyNodeRef::DoStatement(n) => n.range(),
            AnyNodeRef::DynamicVariableName(n) => n.range(),
            AnyNodeRef::EchoStatement(n) => n.range(),
            AnyNodeRef::ElseClause(n) => n.range(),
            AnyNodeRef::ElseIfClause(n) => n.range(),
            AnyNodeRef::EmptyStatement(n) => n.range(),
            AnyNodeRef::EncapsedString(n) => n.range(),
            AnyNodeRef::EnumCase(n) => n.range(),
            AnyNodeRef::EnumDeclaration(n) => n.range(),
            AnyNodeRef::EnumDeclarationList(n) => n.range(),
            AnyNodeRef::ErrorSuppressionExpression(n) => n.range(),
            AnyNodeRef::ExpressionStatement(n) => n.range(),
            AnyNodeRef::FinalModifier(n) => n.range(),
            AnyNodeRef::FinallyClause(n) => n.range(),
            AnyNodeRef::ForStatement(n) => n.range(),
            AnyNodeRef::ForeachStatement(n) => n.range(),
            AnyNodeRef::FormalParameters(n) => n.range(),
            AnyNodeRef::FunctionCallExpression(n) => n.range(),
            AnyNodeRef::FunctionDefinition(n) => n.range(),
            AnyNodeRef::FunctionStaticDeclaration(n) => n.range(),
            AnyNodeRef::GlobalDeclaration(n) => n.range(),
            AnyNodeRef::GotoStatement(n) => n.range(),
            AnyNodeRef::Heredoc(n) => n.range(),
            AnyNodeRef::HeredocBody(n) => n.range(),
            AnyNodeRef::IfStatement(n) => n.range(),
            AnyNodeRef::IncludeExpression(n) => n.range(),
            AnyNodeRef::IncludeOnceExpression(n) => n.range(),
            AnyNodeRef::InterfaceDeclaration(n) => n.range(),
            AnyNodeRef::IntersectionType(n) => n.range(),
            AnyNodeRef::ListLiteral(n) => n.range(),
            AnyNodeRef::MatchBlock(n) => n.range(),
            AnyNodeRef::MatchConditionList(n) => n.range(),
            AnyNodeRef::MatchConditionalExpression(n) => n.range(),
            AnyNodeRef::MatchDefaultExpression(n) => n.range(),
            AnyNodeRef::MatchExpression(n) => n.range(),
            AnyNodeRef::MemberAccessExpression(n) => n.range(),
            AnyNodeRef::MemberCallExpression(n) => n.range(),
            AnyNodeRef::MethodDeclaration(n) => n.range(),
            AnyNodeRef::Name(n) => n.range(),
            AnyNodeRef::NamedLabelStatement(n) => n.range(),
            AnyNodeRef::NamedType(n) => n.range(),
            AnyNodeRef::NamespaceAliasingClause(n) => n.range(),
            AnyNodeRef::NamespaceDefinition(n) => n.range(),
            AnyNodeRef::NamespaceName(n) => n.range(),
            AnyNodeRef::NamespaceNameAsPrefix(n) => n.range(),
            AnyNodeRef::NamespaceUseClause(n) => n.range(),
            AnyNodeRef::NamespaceUseDeclaration(n) => n.range(),
            AnyNodeRef::NamespaceUseGroup(n) => n.range(),
            AnyNodeRef::NamespaceUseGroupClause(n) => n.range(),
            AnyNodeRef::Nowdoc(n) => n.range(),
            AnyNodeRef::NowdocBody(n) => n.range(),
            AnyNodeRef::Null(n) => n.range(),
            AnyNodeRef::NullsafeMemberAccessExpression(n) => n.range(),
            AnyNodeRef::NullsafeMemberCallExpression(n) => n.range(),
            AnyNodeRef::ObjectCreationExpression(n) => n.range(),
            AnyNodeRef::OptionalType(n) => n.range(),
            AnyNodeRef::Pair(n) => n.range(),
            AnyNodeRef::ParenthesizedExpression(n) => n.range(),
            AnyNodeRef::PrimitiveType(n) => n.range(),
            AnyNodeRef::PrintIntrinsic(n) => n.range(),
            AnyNodeRef::Program(n) => n.range(),
            AnyNodeRef::PropertyDeclaration(n) => n.range(),
            AnyNodeRef::PropertyElement(n) => n.range(),
            AnyNodeRef::PropertyInitializer(n) => n.range(),
            AnyNodeRef::PropertyPromotionParameter(n) => n.range(),
            AnyNodeRef::QualifiedName(n) => n.range(),
            AnyNodeRef::ReadonlyModifier(n) => n.range(),
            AnyNodeRef::ReferenceAssignmentExpression(n) => n.range(),
            AnyNodeRef::ReferenceModifier(n) => n.range(),
            AnyNodeRef::RelativeScope(n) => n.range(),
            AnyNodeRef::RequireExpression(n) => n.range(),
            AnyNodeRef::RequireOnceExpression(n) => n.range(),
            AnyNodeRef::ReturnStatement(n) => n.range(),
            AnyNodeRef::ScopedCallExpression(n) => n.range(),
            AnyNodeRef::ScopedPropertyAccessExpression(n) => n.range(),
            AnyNodeRef::SequenceExpression(n) => n.range(),
            AnyNodeRef::ShellCommandExpression(n) => n.range(),
            AnyNodeRef::SimpleParameter(n) => n.range(),
            AnyNodeRef::StaticModifier(n) => n.range(),
            AnyNodeRef::StaticVariableDeclaration(n) => n.range(),
            AnyNodeRef::String(n) => n.range(),
            AnyNodeRef::SubscriptExpression(n) => n.range(),
            AnyNodeRef::SwitchBlock(n) => n.range(),
            AnyNodeRef::SwitchStatement(n) => n.range(),
            AnyNodeRef::Text(n) => n.range(),
            AnyNodeRef::TextInterpolation(n) => n.range(),
            AnyNodeRef::ThrowExpression(n) => n.range(),
            AnyNodeRef::TraitDeclaration(n) => n.range(),
            AnyNodeRef::TryStatement(n) => n.range(),
            AnyNodeRef::TypeList(n) => n.range(),
            AnyNodeRef::UnaryOpExpression(n) => n.range(),
            AnyNodeRef::UnionType(n) => n.range(),
            AnyNodeRef::UnsetStatement(n) => n.range(),
            AnyNodeRef::UpdateExpression(n) => n.range(),
            AnyNodeRef::UseAsClause(n) => n.range(),
            AnyNodeRef::UseDeclaration(n) => n.range(),
            AnyNodeRef::UseInsteadOfClause(n) => n.range(),
            AnyNodeRef::UseList(n) => n.range(),
            AnyNodeRef::VariableName(n) => n.range(),
            AnyNodeRef::VariadicParameter(n) => n.range(),
            AnyNodeRef::VariadicPlaceholder(n) => n.range(),
            AnyNodeRef::VariadicUnpacking(n) => n.range(),
            AnyNodeRef::VisibilityModifier(n) => n.range(),
            AnyNodeRef::WhileStatement(n) => n.range(),
            AnyNodeRef::YieldExpression(n) => n.range(),
            AnyNodeRef::BottomType(n) => n.range(),
            AnyNodeRef::Comment(n) => n.range(),
            AnyNodeRef::EscapeSequence(n) => n.range(),
            AnyNodeRef::Float(n) => n.range(),
            AnyNodeRef::HeredocEnd(n) => n.range(),
            AnyNodeRef::HeredocStart(n) => n.range(),
            AnyNodeRef::Integer(n) => n.range(),
            AnyNodeRef::NowdocString(n) => n.range(),
            AnyNodeRef::PhpTag(n) => n.range(),
            AnyNodeRef::StringValue(n) => n.range(),
            AnyNodeRef::VarModifier(n) => n.range(),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        self.clone()
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            AnyNodeRef::StaticExpr(_, _) => vec![],
            AnyNodeRef::Error(e) => e.children_any(),
            AnyNodeRef::Operator(op) => op.children_any(),
            AnyNodeRef::_Expression(n) => n.children_any(),
            AnyNodeRef::_Literal(n) => n.children_any(),
            AnyNodeRef::_PrimaryExpression(n) => n.children_any(),
            AnyNodeRef::_Statement(n) => n.children_any(),
            AnyNodeRef::_Type(n) => n.children_any(),
            AnyNodeRef::AbstractModifier(n) => n.children_any(),
            AnyNodeRef::AnonymousFunctionCreationExpression(n) => n.children_any(),
            AnyNodeRef::AnonymousFunctionUseClause(n) => n.children_any(),
            AnyNodeRef::Argument(n) => n.children_any(),
            AnyNodeRef::Arguments(n) => n.children_any(),
            AnyNodeRef::ArrayCreationExpression(n) => n.children_any(),
            AnyNodeRef::ArrayElementInitializer(n) => n.children_any(),
            AnyNodeRef::ArrowFunction(n) => n.children_any(),
            AnyNodeRef::AssignmentExpression(n) => n.children_any(),
            AnyNodeRef::Attribute(n) => n.children_any(),
            AnyNodeRef::AttributeGroup(n) => n.children_any(),
            AnyNodeRef::AttributeList(n) => n.children_any(),
            AnyNodeRef::AugmentedAssignmentExpression(n) => n.children_any(),
            AnyNodeRef::BaseClause(n) => n.children_any(),
            AnyNodeRef::BinaryExpression(n) => n.children_any(),
            AnyNodeRef::Boolean(n) => n.children_any(),
            AnyNodeRef::BreakStatement(n) => n.children_any(),
            AnyNodeRef::ByRef(n) => n.children_any(),
            AnyNodeRef::CaseStatement(n) => n.children_any(),
            AnyNodeRef::CastExpression(n) => n.children_any(),
            AnyNodeRef::CastType(n) => n.children_any(),
            AnyNodeRef::CatchClause(n) => n.children_any(),
            AnyNodeRef::ClassConstantAccessExpression(n) => n.children_any(),
            AnyNodeRef::ClassConstantAccessIdentifier(n) => n.children_any(),
            AnyNodeRef::ClassDeclaration(n) => n.children_any(),
            AnyNodeRef::ClassInterfaceClause(n) => n.children_any(),
            AnyNodeRef::CloneExpression(n) => n.children_any(),
            AnyNodeRef::ColonBlock(n) => n.children_any(),
            AnyNodeRef::CompoundStatement(n) => n.children_any(),
            AnyNodeRef::ConditionalExpression(n) => n.children_any(),
            AnyNodeRef::ConstDeclaration(n) => n.children_any(),
            AnyNodeRef::ConstElement(n) => n.children_any(),
            AnyNodeRef::ContinueStatement(n) => n.children_any(),
            AnyNodeRef::DeclarationList(n) => n.children_any(),
            AnyNodeRef::DeclareDirective(n) => n.children_any(),
            AnyNodeRef::DeclareStatement(n) => n.children_any(),
            AnyNodeRef::DefaultStatement(n) => n.children_any(),
            AnyNodeRef::DisjunctiveNormalFormType(n) => n.children_any(),
            AnyNodeRef::DoStatement(n) => n.children_any(),
            AnyNodeRef::DynamicVariableName(n) => n.children_any(),
            AnyNodeRef::EchoStatement(n) => n.children_any(),
            AnyNodeRef::ElseClause(n) => n.children_any(),
            AnyNodeRef::ElseIfClause(n) => n.children_any(),
            AnyNodeRef::EmptyStatement(n) => n.children_any(),
            AnyNodeRef::EncapsedString(n) => n.children_any(),
            AnyNodeRef::EnumCase(n) => n.children_any(),
            AnyNodeRef::EnumDeclaration(n) => n.children_any(),
            AnyNodeRef::EnumDeclarationList(n) => n.children_any(),
            AnyNodeRef::ErrorSuppressionExpression(n) => n.children_any(),
            AnyNodeRef::ExpressionStatement(n) => n.children_any(),
            AnyNodeRef::FinalModifier(n) => n.children_any(),
            AnyNodeRef::FinallyClause(n) => n.children_any(),
            AnyNodeRef::ForStatement(n) => n.children_any(),
            AnyNodeRef::ForeachStatement(n) => n.children_any(),
            AnyNodeRef::FormalParameters(n) => n.children_any(),
            AnyNodeRef::FunctionCallExpression(n) => n.children_any(),
            AnyNodeRef::FunctionDefinition(n) => n.children_any(),
            AnyNodeRef::FunctionStaticDeclaration(n) => n.children_any(),
            AnyNodeRef::GlobalDeclaration(n) => n.children_any(),
            AnyNodeRef::GotoStatement(n) => n.children_any(),
            AnyNodeRef::Heredoc(n) => n.children_any(),
            AnyNodeRef::HeredocBody(n) => n.children_any(),
            AnyNodeRef::IfStatement(n) => n.children_any(),
            AnyNodeRef::IncludeExpression(n) => n.children_any(),
            AnyNodeRef::IncludeOnceExpression(n) => n.children_any(),
            AnyNodeRef::InterfaceDeclaration(n) => n.children_any(),
            AnyNodeRef::IntersectionType(n) => n.children_any(),
            AnyNodeRef::ListLiteral(n) => n.children_any(),
            AnyNodeRef::MatchBlock(n) => n.children_any(),
            AnyNodeRef::MatchConditionList(n) => n.children_any(),
            AnyNodeRef::MatchConditionalExpression(n) => n.children_any(),
            AnyNodeRef::MatchDefaultExpression(n) => n.children_any(),
            AnyNodeRef::MatchExpression(n) => n.children_any(),
            AnyNodeRef::MemberAccessExpression(n) => n.children_any(),
            AnyNodeRef::MemberCallExpression(n) => n.children_any(),
            AnyNodeRef::MethodDeclaration(n) => n.children_any(),
            AnyNodeRef::Name(n) => n.children_any(),
            AnyNodeRef::NamedLabelStatement(n) => n.children_any(),
            AnyNodeRef::NamedType(n) => n.children_any(),
            AnyNodeRef::NamespaceAliasingClause(n) => n.children_any(),
            AnyNodeRef::NamespaceDefinition(n) => n.children_any(),
            AnyNodeRef::NamespaceName(n) => n.children_any(),
            AnyNodeRef::NamespaceNameAsPrefix(n) => n.children_any(),
            AnyNodeRef::NamespaceUseClause(n) => n.children_any(),
            AnyNodeRef::NamespaceUseDeclaration(n) => n.children_any(),
            AnyNodeRef::NamespaceUseGroup(n) => n.children_any(),
            AnyNodeRef::NamespaceUseGroupClause(n) => n.children_any(),
            AnyNodeRef::Nowdoc(n) => n.children_any(),
            AnyNodeRef::NowdocBody(n) => n.children_any(),
            AnyNodeRef::Null(n) => n.children_any(),
            AnyNodeRef::NullsafeMemberAccessExpression(n) => n.children_any(),
            AnyNodeRef::NullsafeMemberCallExpression(n) => n.children_any(),
            AnyNodeRef::ObjectCreationExpression(n) => n.children_any(),
            AnyNodeRef::OptionalType(n) => n.children_any(),
            AnyNodeRef::Pair(n) => n.children_any(),
            AnyNodeRef::ParenthesizedExpression(n) => n.children_any(),
            AnyNodeRef::PrimitiveType(n) => n.children_any(),
            AnyNodeRef::PrintIntrinsic(n) => n.children_any(),
            AnyNodeRef::Program(n) => n.children_any(),
            AnyNodeRef::PropertyDeclaration(n) => n.children_any(),
            AnyNodeRef::PropertyElement(n) => n.children_any(),
            AnyNodeRef::PropertyInitializer(n) => n.children_any(),
            AnyNodeRef::PropertyPromotionParameter(n) => n.children_any(),
            AnyNodeRef::QualifiedName(n) => n.children_any(),
            AnyNodeRef::ReadonlyModifier(n) => n.children_any(),
            AnyNodeRef::ReferenceAssignmentExpression(n) => n.children_any(),
            AnyNodeRef::ReferenceModifier(n) => n.children_any(),
            AnyNodeRef::RelativeScope(n) => n.children_any(),
            AnyNodeRef::RequireExpression(n) => n.children_any(),
            AnyNodeRef::RequireOnceExpression(n) => n.children_any(),
            AnyNodeRef::ReturnStatement(n) => n.children_any(),
            AnyNodeRef::ScopedCallExpression(n) => n.children_any(),
            AnyNodeRef::ScopedPropertyAccessExpression(n) => n.children_any(),
            AnyNodeRef::SequenceExpression(n) => n.children_any(),
            AnyNodeRef::ShellCommandExpression(n) => n.children_any(),
            AnyNodeRef::SimpleParameter(n) => n.children_any(),
            AnyNodeRef::StaticModifier(n) => n.children_any(),
            AnyNodeRef::StaticVariableDeclaration(n) => n.children_any(),
            AnyNodeRef::String(n) => n.children_any(),
            AnyNodeRef::SubscriptExpression(n) => n.children_any(),
            AnyNodeRef::SwitchBlock(n) => n.children_any(),
            AnyNodeRef::SwitchStatement(n) => n.children_any(),
            AnyNodeRef::Text(n) => n.children_any(),
            AnyNodeRef::TextInterpolation(n) => n.children_any(),
            AnyNodeRef::ThrowExpression(n) => n.children_any(),
            AnyNodeRef::TraitDeclaration(n) => n.children_any(),
            AnyNodeRef::TryStatement(n) => n.children_any(),
            AnyNodeRef::TypeList(n) => n.children_any(),
            AnyNodeRef::UnaryOpExpression(n) => n.children_any(),
            AnyNodeRef::UnionType(n) => n.children_any(),
            AnyNodeRef::UnsetStatement(n) => n.children_any(),
            AnyNodeRef::UpdateExpression(n) => n.children_any(),
            AnyNodeRef::UseAsClause(n) => n.children_any(),
            AnyNodeRef::UseDeclaration(n) => n.children_any(),
            AnyNodeRef::UseInsteadOfClause(n) => n.children_any(),
            AnyNodeRef::UseList(n) => n.children_any(),
            AnyNodeRef::VariableName(n) => n.children_any(),
            AnyNodeRef::VariadicParameter(n) => n.children_any(),
            AnyNodeRef::VariadicPlaceholder(n) => n.children_any(),
            AnyNodeRef::VariadicUnpacking(n) => n.children_any(),
            AnyNodeRef::VisibilityModifier(n) => n.children_any(),
            AnyNodeRef::WhileStatement(n) => n.children_any(),
            AnyNodeRef::YieldExpression(n) => n.children_any(),
            AnyNodeRef::BottomType(n) => n.children_any(),
            AnyNodeRef::Comment(n) => n.children_any(),
            AnyNodeRef::EscapeSequence(n) => n.children_any(),
            AnyNodeRef::Float(n) => n.children_any(),
            AnyNodeRef::HeredocEnd(n) => n.children_any(),
            AnyNodeRef::HeredocStart(n) => n.children_any(),
            AnyNodeRef::Integer(n) => n.children_any(),
            AnyNodeRef::NowdocString(n) => n.children_any(),
            AnyNodeRef::PhpTag(n) => n.children_any(),
            AnyNodeRef::StringValue(n) => n.children_any(),
            AnyNodeRef::VarModifier(n) => n.children_any(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AnyNode {
    _Expression(Box<_ExpressionNode>),
    _Literal(Box<_LiteralNode>),
    _PrimaryExpression(Box<_PrimaryExpressionNode>),
    _Statement(Box<_StatementNode>),
    _Type(Box<_TypeNode>),
    AbstractModifier(Box<AbstractModifierNode>),
    AnonymousFunctionCreationExpression(Box<AnonymousFunctionCreationExpressionNode>),
    AnonymousFunctionUseClause(Box<AnonymousFunctionUseClauseNode>),
    Argument(Box<ArgumentNode>),
    Arguments(Box<ArgumentsNode>),
    ArrayCreationExpression(Box<ArrayCreationExpressionNode>),
    ArrayElementInitializer(Box<ArrayElementInitializerNode>),
    ArrowFunction(Box<ArrowFunctionNode>),
    AssignmentExpression(Box<AssignmentExpressionNode>),
    Attribute(Box<AttributeNode>),
    AttributeGroup(Box<AttributeGroupNode>),
    AttributeList(Box<AttributeListNode>),
    AugmentedAssignmentExpression(Box<AugmentedAssignmentExpressionNode>),
    BaseClause(Box<BaseClauseNode>),
    BinaryExpression(Box<BinaryExpressionNode>),
    Boolean(Box<BooleanNode>),
    BreakStatement(Box<BreakStatementNode>),
    ByRef(Box<ByRefNode>),
    CaseStatement(Box<CaseStatementNode>),
    CastExpression(Box<CastExpressionNode>),
    CastType(Box<CastTypeNode>),
    CatchClause(Box<CatchClauseNode>),
    ClassConstantAccessExpression(Box<ClassConstantAccessExpressionNode>),
    ClassConstantAccessIdentifier(Box<ClassConstantAccessIdentifierNode>),
    ClassDeclaration(Box<ClassDeclarationNode>),
    ClassInterfaceClause(Box<ClassInterfaceClauseNode>),
    CloneExpression(Box<CloneExpressionNode>),
    ColonBlock(Box<ColonBlockNode>),
    CompoundStatement(Box<CompoundStatementNode>),
    ConditionalExpression(Box<ConditionalExpressionNode>),
    ConstDeclaration(Box<ConstDeclarationNode>),
    ConstElement(Box<ConstElementNode>),
    ContinueStatement(Box<ContinueStatementNode>),
    DeclarationList(Box<DeclarationListNode>),
    DeclareDirective(Box<DeclareDirectiveNode>),
    DeclareStatement(Box<DeclareStatementNode>),
    DefaultStatement(Box<DefaultStatementNode>),
    DisjunctiveNormalFormType(Box<DisjunctiveNormalFormTypeNode>),
    DoStatement(Box<DoStatementNode>),
    DynamicVariableName(Box<DynamicVariableNameNode>),
    EchoStatement(Box<EchoStatementNode>),
    ElseClause(Box<ElseClauseNode>),
    ElseIfClause(Box<ElseIfClauseNode>),
    EmptyStatement(Box<EmptyStatementNode>),
    EncapsedString(Box<EncapsedStringNode>),
    EnumCase(Box<EnumCaseNode>),
    EnumDeclaration(Box<EnumDeclarationNode>),
    EnumDeclarationList(Box<EnumDeclarationListNode>),
    ErrorSuppressionExpression(Box<ErrorSuppressionExpressionNode>),
    ExpressionStatement(Box<ExpressionStatementNode>),
    FinalModifier(Box<FinalModifierNode>),
    FinallyClause(Box<FinallyClauseNode>),
    ForStatement(Box<ForStatementNode>),
    ForeachStatement(Box<ForeachStatementNode>),
    FormalParameters(Box<FormalParametersNode>),
    FunctionCallExpression(Box<FunctionCallExpressionNode>),
    FunctionDefinition(Box<FunctionDefinitionNode>),
    FunctionStaticDeclaration(Box<FunctionStaticDeclarationNode>),
    GlobalDeclaration(Box<GlobalDeclarationNode>),
    GotoStatement(Box<GotoStatementNode>),
    Heredoc(Box<HeredocNode>),
    HeredocBody(Box<HeredocBodyNode>),
    IfStatement(Box<IfStatementNode>),
    IncludeExpression(Box<IncludeExpressionNode>),
    IncludeOnceExpression(Box<IncludeOnceExpressionNode>),
    InterfaceDeclaration(Box<InterfaceDeclarationNode>),
    IntersectionType(Box<IntersectionTypeNode>),
    ListLiteral(Box<ListLiteralNode>),
    MatchBlock(Box<MatchBlockNode>),
    MatchConditionList(Box<MatchConditionListNode>),
    MatchConditionalExpression(Box<MatchConditionalExpressionNode>),
    MatchDefaultExpression(Box<MatchDefaultExpressionNode>),
    MatchExpression(Box<MatchExpressionNode>),
    MemberAccessExpression(Box<MemberAccessExpressionNode>),
    MemberCallExpression(Box<MemberCallExpressionNode>),
    MethodDeclaration(Box<MethodDeclarationNode>),
    Name(Box<NameNode>),
    NamedLabelStatement(Box<NamedLabelStatementNode>),
    NamedType(Box<NamedTypeNode>),
    NamespaceAliasingClause(Box<NamespaceAliasingClauseNode>),
    NamespaceDefinition(Box<NamespaceDefinitionNode>),
    NamespaceName(Box<NamespaceNameNode>),
    NamespaceNameAsPrefix(Box<NamespaceNameAsPrefixNode>),
    NamespaceUseClause(Box<NamespaceUseClauseNode>),
    NamespaceUseDeclaration(Box<NamespaceUseDeclarationNode>),
    NamespaceUseGroup(Box<NamespaceUseGroupNode>),
    NamespaceUseGroupClause(Box<NamespaceUseGroupClauseNode>),
    Nowdoc(Box<NowdocNode>),
    NowdocBody(Box<NowdocBodyNode>),
    Null(Box<NullNode>),
    NullsafeMemberAccessExpression(Box<NullsafeMemberAccessExpressionNode>),
    NullsafeMemberCallExpression(Box<NullsafeMemberCallExpressionNode>),
    ObjectCreationExpression(Box<ObjectCreationExpressionNode>),
    OptionalType(Box<OptionalTypeNode>),
    Pair(Box<PairNode>),
    ParenthesizedExpression(Box<ParenthesizedExpressionNode>),
    PrimitiveType(Box<PrimitiveTypeNode>),
    PrintIntrinsic(Box<PrintIntrinsicNode>),
    Program(Box<ProgramNode>),
    PropertyDeclaration(Box<PropertyDeclarationNode>),
    PropertyElement(Box<PropertyElementNode>),
    PropertyInitializer(Box<PropertyInitializerNode>),
    PropertyPromotionParameter(Box<PropertyPromotionParameterNode>),
    QualifiedName(Box<QualifiedNameNode>),
    ReadonlyModifier(Box<ReadonlyModifierNode>),
    ReferenceAssignmentExpression(Box<ReferenceAssignmentExpressionNode>),
    ReferenceModifier(Box<ReferenceModifierNode>),
    RelativeScope(Box<RelativeScopeNode>),
    RequireExpression(Box<RequireExpressionNode>),
    RequireOnceExpression(Box<RequireOnceExpressionNode>),
    ReturnStatement(Box<ReturnStatementNode>),
    ScopedCallExpression(Box<ScopedCallExpressionNode>),
    ScopedPropertyAccessExpression(Box<ScopedPropertyAccessExpressionNode>),
    SequenceExpression(Box<SequenceExpressionNode>),
    ShellCommandExpression(Box<ShellCommandExpressionNode>),
    SimpleParameter(Box<SimpleParameterNode>),
    StaticModifier(Box<StaticModifierNode>),
    StaticVariableDeclaration(Box<StaticVariableDeclarationNode>),
    String(Box<StringNode>),
    SubscriptExpression(Box<SubscriptExpressionNode>),
    SwitchBlock(Box<SwitchBlockNode>),
    SwitchStatement(Box<SwitchStatementNode>),
    Text(Box<TextNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    ThrowExpression(Box<ThrowExpressionNode>),
    TraitDeclaration(Box<TraitDeclarationNode>),
    TryStatement(Box<TryStatementNode>),
    TypeList(Box<TypeListNode>),
    UnaryOpExpression(Box<UnaryOpExpressionNode>),
    UnionType(Box<UnionTypeNode>),
    UnsetStatement(Box<UnsetStatementNode>),
    UpdateExpression(Box<UpdateExpressionNode>),
    UseAsClause(Box<UseAsClauseNode>),
    UseDeclaration(Box<UseDeclarationNode>),
    UseInsteadOfClause(Box<UseInsteadOfClauseNode>),
    UseList(Box<UseListNode>),
    VariableName(Box<VariableNameNode>),
    VariadicParameter(Box<VariadicParameterNode>),
    VariadicPlaceholder(Box<VariadicPlaceholderNode>),
    VariadicUnpacking(Box<VariadicUnpackingNode>),
    VisibilityModifier(Box<VisibilityModifierNode>),
    WhileStatement(Box<WhileStatementNode>),
    YieldExpression(Box<YieldExpressionNode>),
    BottomType(Box<BottomTypeNode>),
    Comment(Box<CommentNode>),
    EscapeSequence(Box<EscapeSequenceNode>),
    Float(Box<FloatNode>),
    HeredocEnd(Box<HeredocEndNode>),
    HeredocStart(Box<HeredocStartNode>),
    Integer(Box<IntegerNode>),
    NowdocString(Box<NowdocStringNode>),
    PhpTag(Box<PhpTagNode>),
    StringValue(Box<StringValueNode>),
    VarModifier(Box<VarModifierNode>),
}

impl AnyNode {
    pub fn kind(&self) -> &'static str {
        self.as_any().kind()
    }

    pub fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            // "comment" =>
            // "text_interpolation" =>
            "_expression" => AnyNode::_Expression(Box::new(_ExpressionNode::parse(node, source)?)),
            "_literal" => AnyNode::_Literal(Box::new(_LiteralNode::parse(node, source)?)),
            "_primary_expression" => {
                AnyNode::_PrimaryExpression(Box::new(_PrimaryExpressionNode::parse(node, source)?))
            }
            "_statement" => AnyNode::_Statement(Box::new(_StatementNode::parse(node, source)?)),
            "_type" => AnyNode::_Type(Box::new(_TypeNode::parse(node, source)?)),
            "abstract_modifier" => {
                AnyNode::AbstractModifier(Box::new(AbstractModifierNode::parse(node, source)?))
            }
            "anonymous_function_creation_expression" => {
                AnyNode::AnonymousFunctionCreationExpression(Box::new(
                    AnonymousFunctionCreationExpressionNode::parse(node, source)?,
                ))
            }
            "anonymous_function_use_clause" => AnyNode::AnonymousFunctionUseClause(Box::new(
                AnonymousFunctionUseClauseNode::parse(node, source)?,
            )),
            "argument" => AnyNode::Argument(Box::new(ArgumentNode::parse(node, source)?)),
            "arguments" => AnyNode::Arguments(Box::new(ArgumentsNode::parse(node, source)?)),
            "array_creation_expression" => AnyNode::ArrayCreationExpression(Box::new(
                ArrayCreationExpressionNode::parse(node, source)?,
            )),
            "array_element_initializer" => AnyNode::ArrayElementInitializer(Box::new(
                ArrayElementInitializerNode::parse(node, source)?,
            )),
            "arrow_function" => {
                AnyNode::ArrowFunction(Box::new(ArrowFunctionNode::parse(node, source)?))
            }
            "assignment_expression" => AnyNode::AssignmentExpression(Box::new(
                AssignmentExpressionNode::parse(node, source)?,
            )),
            "attribute" => AnyNode::Attribute(Box::new(AttributeNode::parse(node, source)?)),
            "attribute_group" => {
                AnyNode::AttributeGroup(Box::new(AttributeGroupNode::parse(node, source)?))
            }
            "attribute_list" => {
                AnyNode::AttributeList(Box::new(AttributeListNode::parse(node, source)?))
            }
            "augmented_assignment_expression" => AnyNode::AugmentedAssignmentExpression(Box::new(
                AugmentedAssignmentExpressionNode::parse(node, source)?,
            )),
            "base_clause" => AnyNode::BaseClause(Box::new(BaseClauseNode::parse(node, source)?)),
            "binary_expression" => {
                AnyNode::BinaryExpression(Box::new(BinaryExpressionNode::parse(node, source)?))
            }
            "boolean" => AnyNode::Boolean(Box::new(BooleanNode::parse(node, source)?)),
            "break_statement" => {
                AnyNode::BreakStatement(Box::new(BreakStatementNode::parse(node, source)?))
            }
            "by_ref" => AnyNode::ByRef(Box::new(ByRefNode::parse(node, source)?)),
            "case_statement" => {
                AnyNode::CaseStatement(Box::new(CaseStatementNode::parse(node, source)?))
            }
            "cast_expression" => {
                AnyNode::CastExpression(Box::new(CastExpressionNode::parse(node, source)?))
            }
            "cast_type" => AnyNode::CastType(Box::new(CastTypeNode::parse(node, source)?)),
            "catch_clause" => AnyNode::CatchClause(Box::new(CatchClauseNode::parse(node, source)?)),
            "class_constant_access_expression" => AnyNode::ClassConstantAccessExpression(Box::new(
                ClassConstantAccessExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_identifier" => AnyNode::ClassConstantAccessIdentifier(Box::new(
                ClassConstantAccessIdentifierNode::parse(node, source)?,
            )),
            "class_declaration" => {
                AnyNode::ClassDeclaration(Box::new(ClassDeclarationNode::parse(node, source)?))
            }
            "class_interface_clause" => AnyNode::ClassInterfaceClause(Box::new(
                ClassInterfaceClauseNode::parse(node, source)?,
            )),
            "clone_expression" => {
                AnyNode::CloneExpression(Box::new(CloneExpressionNode::parse(node, source)?))
            }
            "colon_block" => AnyNode::ColonBlock(Box::new(ColonBlockNode::parse(node, source)?)),
            "compound_statement" => {
                AnyNode::CompoundStatement(Box::new(CompoundStatementNode::parse(node, source)?))
            }
            "conditional_expression" => AnyNode::ConditionalExpression(Box::new(
                ConditionalExpressionNode::parse(node, source)?,
            )),
            "const_declaration" => {
                AnyNode::ConstDeclaration(Box::new(ConstDeclarationNode::parse(node, source)?))
            }
            "const_element" => {
                AnyNode::ConstElement(Box::new(ConstElementNode::parse(node, source)?))
            }
            "continue_statement" => {
                AnyNode::ContinueStatement(Box::new(ContinueStatementNode::parse(node, source)?))
            }
            "declaration_list" => {
                AnyNode::DeclarationList(Box::new(DeclarationListNode::parse(node, source)?))
            }
            "declare_directive" => {
                AnyNode::DeclareDirective(Box::new(DeclareDirectiveNode::parse(node, source)?))
            }
            "declare_statement" => {
                AnyNode::DeclareStatement(Box::new(DeclareStatementNode::parse(node, source)?))
            }
            "default_statement" => {
                AnyNode::DefaultStatement(Box::new(DefaultStatementNode::parse(node, source)?))
            }
            "disjunctive_normal_form_type" => AnyNode::DisjunctiveNormalFormType(Box::new(
                DisjunctiveNormalFormTypeNode::parse(node, source)?,
            )),
            "do_statement" => AnyNode::DoStatement(Box::new(DoStatementNode::parse(node, source)?)),
            "dynamic_variable_name" => AnyNode::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "echo_statement" => {
                AnyNode::EchoStatement(Box::new(EchoStatementNode::parse(node, source)?))
            }
            "else_clause" => AnyNode::ElseClause(Box::new(ElseClauseNode::parse(node, source)?)),
            "else_if_clause" => {
                AnyNode::ElseIfClause(Box::new(ElseIfClauseNode::parse(node, source)?))
            }
            "empty_statement" => {
                AnyNode::EmptyStatement(Box::new(EmptyStatementNode::parse(node, source)?))
            }
            "encapsed_string" => {
                AnyNode::EncapsedString(Box::new(EncapsedStringNode::parse(node, source)?))
            }
            "enum_case" => AnyNode::EnumCase(Box::new(EnumCaseNode::parse(node, source)?)),
            "enum_declaration" => {
                AnyNode::EnumDeclaration(Box::new(EnumDeclarationNode::parse(node, source)?))
            }
            "enum_declaration_list" => AnyNode::EnumDeclarationList(Box::new(
                EnumDeclarationListNode::parse(node, source)?,
            )),
            "error_suppression_expression" => AnyNode::ErrorSuppressionExpression(Box::new(
                ErrorSuppressionExpressionNode::parse(node, source)?,
            )),
            "expression_statement" => AnyNode::ExpressionStatement(Box::new(
                ExpressionStatementNode::parse(node, source)?,
            )),
            "final_modifier" => {
                AnyNode::FinalModifier(Box::new(FinalModifierNode::parse(node, source)?))
            }
            "finally_clause" => {
                AnyNode::FinallyClause(Box::new(FinallyClauseNode::parse(node, source)?))
            }
            "for_statement" => {
                AnyNode::ForStatement(Box::new(ForStatementNode::parse(node, source)?))
            }
            "foreach_statement" => {
                AnyNode::ForeachStatement(Box::new(ForeachStatementNode::parse(node, source)?))
            }
            "formal_parameters" => {
                AnyNode::FormalParameters(Box::new(FormalParametersNode::parse(node, source)?))
            }
            "function_call_expression" => AnyNode::FunctionCallExpression(Box::new(
                FunctionCallExpressionNode::parse(node, source)?,
            )),
            "function_definition" => {
                AnyNode::FunctionDefinition(Box::new(FunctionDefinitionNode::parse(node, source)?))
            }
            "function_static_declaration" => AnyNode::FunctionStaticDeclaration(Box::new(
                FunctionStaticDeclarationNode::parse(node, source)?,
            )),
            "global_declaration" => {
                AnyNode::GlobalDeclaration(Box::new(GlobalDeclarationNode::parse(node, source)?))
            }
            "goto_statement" => {
                AnyNode::GotoStatement(Box::new(GotoStatementNode::parse(node, source)?))
            }
            "heredoc" => AnyNode::Heredoc(Box::new(HeredocNode::parse(node, source)?)),
            "heredoc_body" => AnyNode::HeredocBody(Box::new(HeredocBodyNode::parse(node, source)?)),
            "if_statement" => AnyNode::IfStatement(Box::new(IfStatementNode::parse(node, source)?)),
            "include_expression" => {
                AnyNode::IncludeExpression(Box::new(IncludeExpressionNode::parse(node, source)?))
            }
            "include_once_expression" => AnyNode::IncludeOnceExpression(Box::new(
                IncludeOnceExpressionNode::parse(node, source)?,
            )),
            "interface_declaration" => AnyNode::InterfaceDeclaration(Box::new(
                InterfaceDeclarationNode::parse(node, source)?,
            )),
            "intersection_type" => {
                AnyNode::IntersectionType(Box::new(IntersectionTypeNode::parse(node, source)?))
            }
            "list_literal" => AnyNode::ListLiteral(Box::new(ListLiteralNode::parse(node, source)?)),
            "match_block" => AnyNode::MatchBlock(Box::new(MatchBlockNode::parse(node, source)?)),
            "match_condition_list" => {
                AnyNode::MatchConditionList(Box::new(MatchConditionListNode::parse(node, source)?))
            }
            "match_conditional_expression" => AnyNode::MatchConditionalExpression(Box::new(
                MatchConditionalExpressionNode::parse(node, source)?,
            )),
            "match_default_expression" => AnyNode::MatchDefaultExpression(Box::new(
                MatchDefaultExpressionNode::parse(node, source)?,
            )),
            "match_expression" => {
                AnyNode::MatchExpression(Box::new(MatchExpressionNode::parse(node, source)?))
            }
            "member_access_expression" => AnyNode::MemberAccessExpression(Box::new(
                MemberAccessExpressionNode::parse(node, source)?,
            )),
            "member_call_expression" => AnyNode::MemberCallExpression(Box::new(
                MemberCallExpressionNode::parse(node, source)?,
            )),
            "method_declaration" => {
                AnyNode::MethodDeclaration(Box::new(MethodDeclarationNode::parse(node, source)?))
            }
            "name" => AnyNode::Name(Box::new(NameNode::parse(node, source)?)),
            "named_label_statement" => AnyNode::NamedLabelStatement(Box::new(
                NamedLabelStatementNode::parse(node, source)?,
            )),
            "named_type" => AnyNode::NamedType(Box::new(NamedTypeNode::parse(node, source)?)),
            "namespace_aliasing_clause" => AnyNode::NamespaceAliasingClause(Box::new(
                NamespaceAliasingClauseNode::parse(node, source)?,
            )),
            "namespace_definition" => AnyNode::NamespaceDefinition(Box::new(
                NamespaceDefinitionNode::parse(node, source)?,
            )),
            "namespace_name" => {
                AnyNode::NamespaceName(Box::new(NamespaceNameNode::parse(node, source)?))
            }
            "namespace_name_as_prefix" => AnyNode::NamespaceNameAsPrefix(Box::new(
                NamespaceNameAsPrefixNode::parse(node, source)?,
            )),
            "namespace_use_clause" => {
                AnyNode::NamespaceUseClause(Box::new(NamespaceUseClauseNode::parse(node, source)?))
            }
            "namespace_use_declaration" => AnyNode::NamespaceUseDeclaration(Box::new(
                NamespaceUseDeclarationNode::parse(node, source)?,
            )),
            "namespace_use_group" => {
                AnyNode::NamespaceUseGroup(Box::new(NamespaceUseGroupNode::parse(node, source)?))
            }
            "namespace_use_group_clause" => AnyNode::NamespaceUseGroupClause(Box::new(
                NamespaceUseGroupClauseNode::parse(node, source)?,
            )),
            "nowdoc" => AnyNode::Nowdoc(Box::new(NowdocNode::parse(node, source)?)),
            "nowdoc_body" => AnyNode::NowdocBody(Box::new(NowdocBodyNode::parse(node, source)?)),
            "null" => AnyNode::Null(Box::new(NullNode::parse(node, source)?)),
            "nullsafe_member_access_expression" => AnyNode::NullsafeMemberAccessExpression(
                Box::new(NullsafeMemberAccessExpressionNode::parse(node, source)?),
            ),
            "nullsafe_member_call_expression" => AnyNode::NullsafeMemberCallExpression(Box::new(
                NullsafeMemberCallExpressionNode::parse(node, source)?,
            )),
            "object_creation_expression" => AnyNode::ObjectCreationExpression(Box::new(
                ObjectCreationExpressionNode::parse(node, source)?,
            )),
            "optional_type" => {
                AnyNode::OptionalType(Box::new(OptionalTypeNode::parse(node, source)?))
            }
            "pair" => AnyNode::Pair(Box::new(PairNode::parse(node, source)?)),
            "parenthesized_expression" => AnyNode::ParenthesizedExpression(Box::new(
                ParenthesizedExpressionNode::parse(node, source)?,
            )),
            "primitive_type" => {
                AnyNode::PrimitiveType(Box::new(PrimitiveTypeNode::parse(node, source)?))
            }
            "print_intrinsic" => {
                AnyNode::PrintIntrinsic(Box::new(PrintIntrinsicNode::parse(node, source)?))
            }
            "program" => AnyNode::Program(Box::new(ProgramNode::parse(node, source)?)),
            "property_declaration" => AnyNode::PropertyDeclaration(Box::new(
                PropertyDeclarationNode::parse(node, source)?,
            )),
            "property_element" => {
                AnyNode::PropertyElement(Box::new(PropertyElementNode::parse(node, source)?))
            }
            "property_initializer" => AnyNode::PropertyInitializer(Box::new(
                PropertyInitializerNode::parse(node, source)?,
            )),
            "property_promotion_parameter" => AnyNode::PropertyPromotionParameter(Box::new(
                PropertyPromotionParameterNode::parse(node, source)?,
            )),
            "qualified_name" => {
                AnyNode::QualifiedName(Box::new(QualifiedNameNode::parse(node, source)?))
            }
            "readonly_modifier" => {
                AnyNode::ReadonlyModifier(Box::new(ReadonlyModifierNode::parse(node, source)?))
            }
            "reference_assignment_expression" => AnyNode::ReferenceAssignmentExpression(Box::new(
                ReferenceAssignmentExpressionNode::parse(node, source)?,
            )),
            "reference_modifier" => {
                AnyNode::ReferenceModifier(Box::new(ReferenceModifierNode::parse(node, source)?))
            }
            "relative_scope" => {
                AnyNode::RelativeScope(Box::new(RelativeScopeNode::parse(node, source)?))
            }
            "require_expression" => {
                AnyNode::RequireExpression(Box::new(RequireExpressionNode::parse(node, source)?))
            }
            "require_once_expression" => AnyNode::RequireOnceExpression(Box::new(
                RequireOnceExpressionNode::parse(node, source)?,
            )),
            "return_statement" => {
                AnyNode::ReturnStatement(Box::new(ReturnStatementNode::parse(node, source)?))
            }
            "scoped_call_expression" => AnyNode::ScopedCallExpression(Box::new(
                ScopedCallExpressionNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => AnyNode::ScopedPropertyAccessExpression(
                Box::new(ScopedPropertyAccessExpressionNode::parse(node, source)?),
            ),
            "sequence_expression" => {
                AnyNode::SequenceExpression(Box::new(SequenceExpressionNode::parse(node, source)?))
            }
            "shell_command_expression" => AnyNode::ShellCommandExpression(Box::new(
                ShellCommandExpressionNode::parse(node, source)?,
            )),
            "simple_parameter" => {
                AnyNode::SimpleParameter(Box::new(SimpleParameterNode::parse(node, source)?))
            }
            "static_modifier" => {
                AnyNode::StaticModifier(Box::new(StaticModifierNode::parse(node, source)?))
            }
            "static_variable_declaration" => AnyNode::StaticVariableDeclaration(Box::new(
                StaticVariableDeclarationNode::parse(node, source)?,
            )),
            "string" => AnyNode::String(Box::new(StringNode::parse(node, source)?)),
            "subscript_expression" => AnyNode::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "switch_block" => AnyNode::SwitchBlock(Box::new(SwitchBlockNode::parse(node, source)?)),
            "switch_statement" => {
                AnyNode::SwitchStatement(Box::new(SwitchStatementNode::parse(node, source)?))
            }
            "text" => AnyNode::Text(Box::new(TextNode::parse(node, source)?)),
            "text_interpolation" => {
                AnyNode::TextInterpolation(Box::new(TextInterpolationNode::parse(node, source)?))
            }
            "throw_expression" => {
                AnyNode::ThrowExpression(Box::new(ThrowExpressionNode::parse(node, source)?))
            }
            "trait_declaration" => {
                AnyNode::TraitDeclaration(Box::new(TraitDeclarationNode::parse(node, source)?))
            }
            "try_statement" => {
                AnyNode::TryStatement(Box::new(TryStatementNode::parse(node, source)?))
            }
            "type_list" => AnyNode::TypeList(Box::new(TypeListNode::parse(node, source)?)),
            "unary_op_expression" => {
                AnyNode::UnaryOpExpression(Box::new(UnaryOpExpressionNode::parse(node, source)?))
            }
            "union_type" => AnyNode::UnionType(Box::new(UnionTypeNode::parse(node, source)?)),
            "unset_statement" => {
                AnyNode::UnsetStatement(Box::new(UnsetStatementNode::parse(node, source)?))
            }
            "update_expression" => {
                AnyNode::UpdateExpression(Box::new(UpdateExpressionNode::parse(node, source)?))
            }
            "use_as_clause" => {
                AnyNode::UseAsClause(Box::new(UseAsClauseNode::parse(node, source)?))
            }
            "use_declaration" => {
                AnyNode::UseDeclaration(Box::new(UseDeclarationNode::parse(node, source)?))
            }
            "use_instead_of_clause" => {
                AnyNode::UseInsteadOfClause(Box::new(UseInsteadOfClauseNode::parse(node, source)?))
            }
            "use_list" => AnyNode::UseList(Box::new(UseListNode::parse(node, source)?)),
            "variable_name" => {
                AnyNode::VariableName(Box::new(VariableNameNode::parse(node, source)?))
            }
            "variadic_parameter" => {
                AnyNode::VariadicParameter(Box::new(VariadicParameterNode::parse(node, source)?))
            }
            "variadic_placeholder" => AnyNode::VariadicPlaceholder(Box::new(
                VariadicPlaceholderNode::parse(node, source)?,
            )),
            "variadic_unpacking" => {
                AnyNode::VariadicUnpacking(Box::new(VariadicUnpackingNode::parse(node, source)?))
            }
            "visibility_modifier" => {
                AnyNode::VisibilityModifier(Box::new(VisibilityModifierNode::parse(node, source)?))
            }
            "while_statement" => {
                AnyNode::WhileStatement(Box::new(WhileStatementNode::parse(node, source)?))
            }
            "yield_expression" => {
                AnyNode::YieldExpression(Box::new(YieldExpressionNode::parse(node, source)?))
            }

            "bottom_type" => AnyNode::BottomType(Box::new(BottomTypeNode::parse(node, source)?)),

            "comment" => AnyNode::Comment(Box::new(CommentNode::parse(node, source)?)),

            "escape_sequence" => {
                AnyNode::EscapeSequence(Box::new(EscapeSequenceNode::parse(node, source)?))
            }

            "float" => AnyNode::Float(Box::new(FloatNode::parse(node, source)?)),

            "heredoc_end" => AnyNode::HeredocEnd(Box::new(HeredocEndNode::parse(node, source)?)),
            "heredoc_start" => {
                AnyNode::HeredocStart(Box::new(HeredocStartNode::parse(node, source)?))
            }

            "integer" => AnyNode::Integer(Box::new(IntegerNode::parse(node, source)?)),

            "nowdoc_string" => {
                AnyNode::NowdocString(Box::new(NowdocStringNode::parse(node, source)?))
            }

            "php_tag" => AnyNode::PhpTag(Box::new(PhpTagNode::parse(node, source)?)),

            "string_value" => AnyNode::StringValue(Box::new(StringValueNode::parse(node, source)?)),

            "var_modifier" => AnyNode::VarModifier(Box::new(VarModifierNode::parse(node, source)?)),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Unknown node kind {}", node.kind()),
                ))
            }
        })
    }

    pub fn parse_vec<'a, I>(children: I, source: &[u8]) -> Result<Vec<Self>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
    {
        let mut res: Vec<Self> = vec![];
        for child in children {
            res.push(Self::parse(child, source)?);
        }
        Ok(res)
    }
}

impl NodeAccess for AnyNode {
    fn brief_desc(&self) -> String {
        match self {
            AnyNode::_Expression(x) => x.brief_desc(),
            AnyNode::_Literal(x) => x.brief_desc(),
            AnyNode::_PrimaryExpression(x) => x.brief_desc(),
            AnyNode::_Statement(x) => x.brief_desc(),
            AnyNode::_Type(x) => x.brief_desc(),
            AnyNode::AbstractModifier(x) => x.brief_desc(),
            AnyNode::AnonymousFunctionCreationExpression(x) => x.brief_desc(),
            AnyNode::AnonymousFunctionUseClause(x) => x.brief_desc(),
            AnyNode::Argument(x) => x.brief_desc(),
            AnyNode::Arguments(x) => x.brief_desc(),
            AnyNode::ArrayCreationExpression(x) => x.brief_desc(),
            AnyNode::ArrayElementInitializer(x) => x.brief_desc(),
            AnyNode::ArrowFunction(x) => x.brief_desc(),
            AnyNode::AssignmentExpression(x) => x.brief_desc(),
            AnyNode::Attribute(x) => x.brief_desc(),
            AnyNode::AttributeGroup(x) => x.brief_desc(),
            AnyNode::AttributeList(x) => x.brief_desc(),
            AnyNode::AugmentedAssignmentExpression(x) => x.brief_desc(),
            AnyNode::BaseClause(x) => x.brief_desc(),
            AnyNode::BinaryExpression(x) => x.brief_desc(),
            AnyNode::Boolean(x) => x.brief_desc(),
            AnyNode::BreakStatement(x) => x.brief_desc(),
            AnyNode::ByRef(x) => x.brief_desc(),
            AnyNode::CaseStatement(x) => x.brief_desc(),
            AnyNode::CastExpression(x) => x.brief_desc(),
            AnyNode::CastType(x) => x.brief_desc(),
            AnyNode::CatchClause(x) => x.brief_desc(),
            AnyNode::ClassConstantAccessExpression(x) => x.brief_desc(),
            AnyNode::ClassConstantAccessIdentifier(x) => x.brief_desc(),
            AnyNode::ClassDeclaration(x) => x.brief_desc(),
            AnyNode::ClassInterfaceClause(x) => x.brief_desc(),
            AnyNode::CloneExpression(x) => x.brief_desc(),
            AnyNode::ColonBlock(x) => x.brief_desc(),
            AnyNode::CompoundStatement(x) => x.brief_desc(),
            AnyNode::ConditionalExpression(x) => x.brief_desc(),
            AnyNode::ConstDeclaration(x) => x.brief_desc(),
            AnyNode::ConstElement(x) => x.brief_desc(),
            AnyNode::ContinueStatement(x) => x.brief_desc(),
            AnyNode::DeclarationList(x) => x.brief_desc(),
            AnyNode::DeclareDirective(x) => x.brief_desc(),
            AnyNode::DeclareStatement(x) => x.brief_desc(),
            AnyNode::DefaultStatement(x) => x.brief_desc(),
            AnyNode::DisjunctiveNormalFormType(x) => x.brief_desc(),
            AnyNode::DoStatement(x) => x.brief_desc(),
            AnyNode::DynamicVariableName(x) => x.brief_desc(),
            AnyNode::EchoStatement(x) => x.brief_desc(),
            AnyNode::ElseClause(x) => x.brief_desc(),
            AnyNode::ElseIfClause(x) => x.brief_desc(),
            AnyNode::EmptyStatement(x) => x.brief_desc(),
            AnyNode::EncapsedString(x) => x.brief_desc(),
            AnyNode::EnumCase(x) => x.brief_desc(),
            AnyNode::EnumDeclaration(x) => x.brief_desc(),
            AnyNode::EnumDeclarationList(x) => x.brief_desc(),
            AnyNode::ErrorSuppressionExpression(x) => x.brief_desc(),
            AnyNode::ExpressionStatement(x) => x.brief_desc(),
            AnyNode::FinalModifier(x) => x.brief_desc(),
            AnyNode::FinallyClause(x) => x.brief_desc(),
            AnyNode::ForStatement(x) => x.brief_desc(),
            AnyNode::ForeachStatement(x) => x.brief_desc(),
            AnyNode::FormalParameters(x) => x.brief_desc(),
            AnyNode::FunctionCallExpression(x) => x.brief_desc(),
            AnyNode::FunctionDefinition(x) => x.brief_desc(),
            AnyNode::FunctionStaticDeclaration(x) => x.brief_desc(),
            AnyNode::GlobalDeclaration(x) => x.brief_desc(),
            AnyNode::GotoStatement(x) => x.brief_desc(),
            AnyNode::Heredoc(x) => x.brief_desc(),
            AnyNode::HeredocBody(x) => x.brief_desc(),
            AnyNode::IfStatement(x) => x.brief_desc(),
            AnyNode::IncludeExpression(x) => x.brief_desc(),
            AnyNode::IncludeOnceExpression(x) => x.brief_desc(),
            AnyNode::InterfaceDeclaration(x) => x.brief_desc(),
            AnyNode::IntersectionType(x) => x.brief_desc(),
            AnyNode::ListLiteral(x) => x.brief_desc(),
            AnyNode::MatchBlock(x) => x.brief_desc(),
            AnyNode::MatchConditionList(x) => x.brief_desc(),
            AnyNode::MatchConditionalExpression(x) => x.brief_desc(),
            AnyNode::MatchDefaultExpression(x) => x.brief_desc(),
            AnyNode::MatchExpression(x) => x.brief_desc(),
            AnyNode::MemberAccessExpression(x) => x.brief_desc(),
            AnyNode::MemberCallExpression(x) => x.brief_desc(),
            AnyNode::MethodDeclaration(x) => x.brief_desc(),
            AnyNode::Name(x) => x.brief_desc(),
            AnyNode::NamedLabelStatement(x) => x.brief_desc(),
            AnyNode::NamedType(x) => x.brief_desc(),
            AnyNode::NamespaceAliasingClause(x) => x.brief_desc(),
            AnyNode::NamespaceDefinition(x) => x.brief_desc(),
            AnyNode::NamespaceName(x) => x.brief_desc(),
            AnyNode::NamespaceNameAsPrefix(x) => x.brief_desc(),
            AnyNode::NamespaceUseClause(x) => x.brief_desc(),
            AnyNode::NamespaceUseDeclaration(x) => x.brief_desc(),
            AnyNode::NamespaceUseGroup(x) => x.brief_desc(),
            AnyNode::NamespaceUseGroupClause(x) => x.brief_desc(),
            AnyNode::Nowdoc(x) => x.brief_desc(),
            AnyNode::NowdocBody(x) => x.brief_desc(),
            AnyNode::Null(x) => x.brief_desc(),
            AnyNode::NullsafeMemberAccessExpression(x) => x.brief_desc(),
            AnyNode::NullsafeMemberCallExpression(x) => x.brief_desc(),
            AnyNode::ObjectCreationExpression(x) => x.brief_desc(),
            AnyNode::OptionalType(x) => x.brief_desc(),
            AnyNode::Pair(x) => x.brief_desc(),
            AnyNode::ParenthesizedExpression(x) => x.brief_desc(),
            AnyNode::PrimitiveType(x) => x.brief_desc(),
            AnyNode::PrintIntrinsic(x) => x.brief_desc(),
            AnyNode::Program(x) => x.brief_desc(),
            AnyNode::PropertyDeclaration(x) => x.brief_desc(),
            AnyNode::PropertyElement(x) => x.brief_desc(),
            AnyNode::PropertyInitializer(x) => x.brief_desc(),
            AnyNode::PropertyPromotionParameter(x) => x.brief_desc(),
            AnyNode::QualifiedName(x) => x.brief_desc(),
            AnyNode::ReadonlyModifier(x) => x.brief_desc(),
            AnyNode::ReferenceAssignmentExpression(x) => x.brief_desc(),
            AnyNode::ReferenceModifier(x) => x.brief_desc(),
            AnyNode::RelativeScope(x) => x.brief_desc(),
            AnyNode::RequireExpression(x) => x.brief_desc(),
            AnyNode::RequireOnceExpression(x) => x.brief_desc(),
            AnyNode::ReturnStatement(x) => x.brief_desc(),
            AnyNode::ScopedCallExpression(x) => x.brief_desc(),
            AnyNode::ScopedPropertyAccessExpression(x) => x.brief_desc(),
            AnyNode::SequenceExpression(x) => x.brief_desc(),
            AnyNode::ShellCommandExpression(x) => x.brief_desc(),
            AnyNode::SimpleParameter(x) => x.brief_desc(),
            AnyNode::StaticModifier(x) => x.brief_desc(),
            AnyNode::StaticVariableDeclaration(x) => x.brief_desc(),
            AnyNode::String(x) => x.brief_desc(),
            AnyNode::SubscriptExpression(x) => x.brief_desc(),
            AnyNode::SwitchBlock(x) => x.brief_desc(),
            AnyNode::SwitchStatement(x) => x.brief_desc(),
            AnyNode::Text(x) => x.brief_desc(),
            AnyNode::TextInterpolation(x) => x.brief_desc(),
            AnyNode::ThrowExpression(x) => x.brief_desc(),
            AnyNode::TraitDeclaration(x) => x.brief_desc(),
            AnyNode::TryStatement(x) => x.brief_desc(),
            AnyNode::TypeList(x) => x.brief_desc(),
            AnyNode::UnaryOpExpression(x) => x.brief_desc(),
            AnyNode::UnionType(x) => x.brief_desc(),
            AnyNode::UnsetStatement(x) => x.brief_desc(),
            AnyNode::UpdateExpression(x) => x.brief_desc(),
            AnyNode::UseAsClause(x) => x.brief_desc(),
            AnyNode::UseDeclaration(x) => x.brief_desc(),
            AnyNode::UseInsteadOfClause(x) => x.brief_desc(),
            AnyNode::UseList(x) => x.brief_desc(),
            AnyNode::VariableName(x) => x.brief_desc(),
            AnyNode::VariadicParameter(x) => x.brief_desc(),
            AnyNode::VariadicPlaceholder(x) => x.brief_desc(),
            AnyNode::VariadicUnpacking(x) => x.brief_desc(),
            AnyNode::VisibilityModifier(x) => x.brief_desc(),
            AnyNode::WhileStatement(x) => x.brief_desc(),
            AnyNode::YieldExpression(x) => x.brief_desc(),

            AnyNode::BottomType(x) => x.brief_desc(),

            AnyNode::Comment(x) => x.brief_desc(),

            AnyNode::EscapeSequence(x) => x.brief_desc(),

            AnyNode::Float(x) => x.brief_desc(),

            AnyNode::HeredocEnd(x) => x.brief_desc(),
            AnyNode::HeredocStart(x) => x.brief_desc(),

            AnyNode::Integer(x) => x.brief_desc(),

            AnyNode::NowdocString(x) => x.brief_desc(),

            AnyNode::PhpTag(x) => x.brief_desc(),

            AnyNode::StringValue(x) => x.brief_desc(),

            AnyNode::VarModifier(x) => x.brief_desc(),
        }
    }

    fn range(&self) -> Range {
        match self {
            AnyNode::_Expression(x) => x.range(),
            AnyNode::_Literal(x) => x.range(),
            AnyNode::_PrimaryExpression(x) => x.range(),
            AnyNode::_Statement(x) => x.range(),
            AnyNode::_Type(x) => x.range(),
            AnyNode::AbstractModifier(x) => x.range(),
            AnyNode::AnonymousFunctionCreationExpression(x) => x.range(),
            AnyNode::AnonymousFunctionUseClause(x) => x.range(),
            AnyNode::Argument(x) => x.range(),
            AnyNode::Arguments(x) => x.range(),
            AnyNode::ArrayCreationExpression(x) => x.range(),
            AnyNode::ArrayElementInitializer(x) => x.range(),
            AnyNode::ArrowFunction(x) => x.range(),
            AnyNode::AssignmentExpression(x) => x.range(),
            AnyNode::Attribute(x) => x.range(),
            AnyNode::AttributeGroup(x) => x.range(),
            AnyNode::AttributeList(x) => x.range(),
            AnyNode::AugmentedAssignmentExpression(x) => x.range(),
            AnyNode::BaseClause(x) => x.range(),
            AnyNode::BinaryExpression(x) => x.range(),
            AnyNode::Boolean(x) => x.range(),
            AnyNode::BreakStatement(x) => x.range(),
            AnyNode::ByRef(x) => x.range(),
            AnyNode::CaseStatement(x) => x.range(),
            AnyNode::CastExpression(x) => x.range(),
            AnyNode::CastType(x) => x.range(),
            AnyNode::CatchClause(x) => x.range(),
            AnyNode::ClassConstantAccessExpression(x) => x.range(),
            AnyNode::ClassConstantAccessIdentifier(x) => x.range(),
            AnyNode::ClassDeclaration(x) => x.range(),
            AnyNode::ClassInterfaceClause(x) => x.range(),
            AnyNode::CloneExpression(x) => x.range(),
            AnyNode::ColonBlock(x) => x.range(),
            AnyNode::CompoundStatement(x) => x.range(),
            AnyNode::ConditionalExpression(x) => x.range(),
            AnyNode::ConstDeclaration(x) => x.range(),
            AnyNode::ConstElement(x) => x.range(),
            AnyNode::ContinueStatement(x) => x.range(),
            AnyNode::DeclarationList(x) => x.range(),
            AnyNode::DeclareDirective(x) => x.range(),
            AnyNode::DeclareStatement(x) => x.range(),
            AnyNode::DefaultStatement(x) => x.range(),
            AnyNode::DisjunctiveNormalFormType(x) => x.range(),
            AnyNode::DoStatement(x) => x.range(),
            AnyNode::DynamicVariableName(x) => x.range(),
            AnyNode::EchoStatement(x) => x.range(),
            AnyNode::ElseClause(x) => x.range(),
            AnyNode::ElseIfClause(x) => x.range(),
            AnyNode::EmptyStatement(x) => x.range(),
            AnyNode::EncapsedString(x) => x.range(),
            AnyNode::EnumCase(x) => x.range(),
            AnyNode::EnumDeclaration(x) => x.range(),
            AnyNode::EnumDeclarationList(x) => x.range(),
            AnyNode::ErrorSuppressionExpression(x) => x.range(),
            AnyNode::ExpressionStatement(x) => x.range(),
            AnyNode::FinalModifier(x) => x.range(),
            AnyNode::FinallyClause(x) => x.range(),
            AnyNode::ForStatement(x) => x.range(),
            AnyNode::ForeachStatement(x) => x.range(),
            AnyNode::FormalParameters(x) => x.range(),
            AnyNode::FunctionCallExpression(x) => x.range(),
            AnyNode::FunctionDefinition(x) => x.range(),
            AnyNode::FunctionStaticDeclaration(x) => x.range(),
            AnyNode::GlobalDeclaration(x) => x.range(),
            AnyNode::GotoStatement(x) => x.range(),
            AnyNode::Heredoc(x) => x.range(),
            AnyNode::HeredocBody(x) => x.range(),
            AnyNode::IfStatement(x) => x.range(),
            AnyNode::IncludeExpression(x) => x.range(),
            AnyNode::IncludeOnceExpression(x) => x.range(),
            AnyNode::InterfaceDeclaration(x) => x.range(),
            AnyNode::IntersectionType(x) => x.range(),
            AnyNode::ListLiteral(x) => x.range(),
            AnyNode::MatchBlock(x) => x.range(),
            AnyNode::MatchConditionList(x) => x.range(),
            AnyNode::MatchConditionalExpression(x) => x.range(),
            AnyNode::MatchDefaultExpression(x) => x.range(),
            AnyNode::MatchExpression(x) => x.range(),
            AnyNode::MemberAccessExpression(x) => x.range(),
            AnyNode::MemberCallExpression(x) => x.range(),
            AnyNode::MethodDeclaration(x) => x.range(),
            AnyNode::Name(x) => x.range(),
            AnyNode::NamedLabelStatement(x) => x.range(),
            AnyNode::NamedType(x) => x.range(),
            AnyNode::NamespaceAliasingClause(x) => x.range(),
            AnyNode::NamespaceDefinition(x) => x.range(),
            AnyNode::NamespaceName(x) => x.range(),
            AnyNode::NamespaceNameAsPrefix(x) => x.range(),
            AnyNode::NamespaceUseClause(x) => x.range(),
            AnyNode::NamespaceUseDeclaration(x) => x.range(),
            AnyNode::NamespaceUseGroup(x) => x.range(),
            AnyNode::NamespaceUseGroupClause(x) => x.range(),
            AnyNode::Nowdoc(x) => x.range(),
            AnyNode::NowdocBody(x) => x.range(),
            AnyNode::Null(x) => x.range(),
            AnyNode::NullsafeMemberAccessExpression(x) => x.range(),
            AnyNode::NullsafeMemberCallExpression(x) => x.range(),
            AnyNode::ObjectCreationExpression(x) => x.range(),
            AnyNode::OptionalType(x) => x.range(),
            AnyNode::Pair(x) => x.range(),
            AnyNode::ParenthesizedExpression(x) => x.range(),
            AnyNode::PrimitiveType(x) => x.range(),
            AnyNode::PrintIntrinsic(x) => x.range(),
            AnyNode::Program(x) => x.range(),
            AnyNode::PropertyDeclaration(x) => x.range(),
            AnyNode::PropertyElement(x) => x.range(),
            AnyNode::PropertyInitializer(x) => x.range(),
            AnyNode::PropertyPromotionParameter(x) => x.range(),
            AnyNode::QualifiedName(x) => x.range(),
            AnyNode::ReadonlyModifier(x) => x.range(),
            AnyNode::ReferenceAssignmentExpression(x) => x.range(),
            AnyNode::ReferenceModifier(x) => x.range(),
            AnyNode::RelativeScope(x) => x.range(),
            AnyNode::RequireExpression(x) => x.range(),
            AnyNode::RequireOnceExpression(x) => x.range(),
            AnyNode::ReturnStatement(x) => x.range(),
            AnyNode::ScopedCallExpression(x) => x.range(),
            AnyNode::ScopedPropertyAccessExpression(x) => x.range(),
            AnyNode::SequenceExpression(x) => x.range(),
            AnyNode::ShellCommandExpression(x) => x.range(),
            AnyNode::SimpleParameter(x) => x.range(),
            AnyNode::StaticModifier(x) => x.range(),
            AnyNode::StaticVariableDeclaration(x) => x.range(),
            AnyNode::String(x) => x.range(),
            AnyNode::SubscriptExpression(x) => x.range(),
            AnyNode::SwitchBlock(x) => x.range(),
            AnyNode::SwitchStatement(x) => x.range(),
            AnyNode::Text(x) => x.range(),
            AnyNode::TextInterpolation(x) => x.range(),
            AnyNode::ThrowExpression(x) => x.range(),
            AnyNode::TraitDeclaration(x) => x.range(),
            AnyNode::TryStatement(x) => x.range(),
            AnyNode::TypeList(x) => x.range(),
            AnyNode::UnaryOpExpression(x) => x.range(),
            AnyNode::UnionType(x) => x.range(),
            AnyNode::UnsetStatement(x) => x.range(),
            AnyNode::UpdateExpression(x) => x.range(),
            AnyNode::UseAsClause(x) => x.range(),
            AnyNode::UseDeclaration(x) => x.range(),
            AnyNode::UseInsteadOfClause(x) => x.range(),
            AnyNode::UseList(x) => x.range(),
            AnyNode::VariableName(x) => x.range(),
            AnyNode::VariadicParameter(x) => x.range(),
            AnyNode::VariadicPlaceholder(x) => x.range(),
            AnyNode::VariadicUnpacking(x) => x.range(),
            AnyNode::VisibilityModifier(x) => x.range(),
            AnyNode::WhileStatement(x) => x.range(),
            AnyNode::YieldExpression(x) => x.range(),

            AnyNode::BottomType(x) => x.range(),

            AnyNode::Comment(x) => x.range(),

            AnyNode::EscapeSequence(x) => x.range(),

            AnyNode::Float(x) => x.range(),

            AnyNode::HeredocEnd(x) => x.range(),
            AnyNode::HeredocStart(x) => x.range(),

            AnyNode::Integer(x) => x.range(),

            AnyNode::NowdocString(x) => x.range(),

            AnyNode::PhpTag(x) => x.range(),

            AnyNode::StringValue(x) => x.range(),

            AnyNode::VarModifier(x) => x.range(),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            AnyNode::_Expression(x) => x.as_any(),
            AnyNode::_Literal(x) => x.as_any(),
            AnyNode::_PrimaryExpression(x) => x.as_any(),
            AnyNode::_Statement(x) => x.as_any(),
            AnyNode::_Type(x) => x.as_any(),
            AnyNode::AbstractModifier(x) => x.as_any(),
            AnyNode::AnonymousFunctionCreationExpression(x) => x.as_any(),
            AnyNode::AnonymousFunctionUseClause(x) => x.as_any(),
            AnyNode::Argument(x) => x.as_any(),
            AnyNode::Arguments(x) => x.as_any(),
            AnyNode::ArrayCreationExpression(x) => x.as_any(),
            AnyNode::ArrayElementInitializer(x) => x.as_any(),
            AnyNode::ArrowFunction(x) => x.as_any(),
            AnyNode::AssignmentExpression(x) => x.as_any(),
            AnyNode::Attribute(x) => x.as_any(),
            AnyNode::AttributeGroup(x) => x.as_any(),
            AnyNode::AttributeList(x) => x.as_any(),
            AnyNode::AugmentedAssignmentExpression(x) => x.as_any(),
            AnyNode::BaseClause(x) => x.as_any(),
            AnyNode::BinaryExpression(x) => x.as_any(),
            AnyNode::Boolean(x) => x.as_any(),
            AnyNode::BreakStatement(x) => x.as_any(),
            AnyNode::ByRef(x) => x.as_any(),
            AnyNode::CaseStatement(x) => x.as_any(),
            AnyNode::CastExpression(x) => x.as_any(),
            AnyNode::CastType(x) => x.as_any(),
            AnyNode::CatchClause(x) => x.as_any(),
            AnyNode::ClassConstantAccessExpression(x) => x.as_any(),
            AnyNode::ClassConstantAccessIdentifier(x) => x.as_any(),
            AnyNode::ClassDeclaration(x) => x.as_any(),
            AnyNode::ClassInterfaceClause(x) => x.as_any(),
            AnyNode::CloneExpression(x) => x.as_any(),
            AnyNode::ColonBlock(x) => x.as_any(),
            AnyNode::CompoundStatement(x) => x.as_any(),
            AnyNode::ConditionalExpression(x) => x.as_any(),
            AnyNode::ConstDeclaration(x) => x.as_any(),
            AnyNode::ConstElement(x) => x.as_any(),
            AnyNode::ContinueStatement(x) => x.as_any(),
            AnyNode::DeclarationList(x) => x.as_any(),
            AnyNode::DeclareDirective(x) => x.as_any(),
            AnyNode::DeclareStatement(x) => x.as_any(),
            AnyNode::DefaultStatement(x) => x.as_any(),
            AnyNode::DisjunctiveNormalFormType(x) => x.as_any(),
            AnyNode::DoStatement(x) => x.as_any(),
            AnyNode::DynamicVariableName(x) => x.as_any(),
            AnyNode::EchoStatement(x) => x.as_any(),
            AnyNode::ElseClause(x) => x.as_any(),
            AnyNode::ElseIfClause(x) => x.as_any(),
            AnyNode::EmptyStatement(x) => x.as_any(),
            AnyNode::EncapsedString(x) => x.as_any(),
            AnyNode::EnumCase(x) => x.as_any(),
            AnyNode::EnumDeclaration(x) => x.as_any(),
            AnyNode::EnumDeclarationList(x) => x.as_any(),
            AnyNode::ErrorSuppressionExpression(x) => x.as_any(),
            AnyNode::ExpressionStatement(x) => x.as_any(),
            AnyNode::FinalModifier(x) => x.as_any(),
            AnyNode::FinallyClause(x) => x.as_any(),
            AnyNode::ForStatement(x) => x.as_any(),
            AnyNode::ForeachStatement(x) => x.as_any(),
            AnyNode::FormalParameters(x) => x.as_any(),
            AnyNode::FunctionCallExpression(x) => x.as_any(),
            AnyNode::FunctionDefinition(x) => x.as_any(),
            AnyNode::FunctionStaticDeclaration(x) => x.as_any(),
            AnyNode::GlobalDeclaration(x) => x.as_any(),
            AnyNode::GotoStatement(x) => x.as_any(),
            AnyNode::Heredoc(x) => x.as_any(),
            AnyNode::HeredocBody(x) => x.as_any(),
            AnyNode::IfStatement(x) => x.as_any(),
            AnyNode::IncludeExpression(x) => x.as_any(),
            AnyNode::IncludeOnceExpression(x) => x.as_any(),
            AnyNode::InterfaceDeclaration(x) => x.as_any(),
            AnyNode::IntersectionType(x) => x.as_any(),
            AnyNode::ListLiteral(x) => x.as_any(),
            AnyNode::MatchBlock(x) => x.as_any(),
            AnyNode::MatchConditionList(x) => x.as_any(),
            AnyNode::MatchConditionalExpression(x) => x.as_any(),
            AnyNode::MatchDefaultExpression(x) => x.as_any(),
            AnyNode::MatchExpression(x) => x.as_any(),
            AnyNode::MemberAccessExpression(x) => x.as_any(),
            AnyNode::MemberCallExpression(x) => x.as_any(),
            AnyNode::MethodDeclaration(x) => x.as_any(),
            AnyNode::Name(x) => x.as_any(),
            AnyNode::NamedLabelStatement(x) => x.as_any(),
            AnyNode::NamedType(x) => x.as_any(),
            AnyNode::NamespaceAliasingClause(x) => x.as_any(),
            AnyNode::NamespaceDefinition(x) => x.as_any(),
            AnyNode::NamespaceName(x) => x.as_any(),
            AnyNode::NamespaceNameAsPrefix(x) => x.as_any(),
            AnyNode::NamespaceUseClause(x) => x.as_any(),
            AnyNode::NamespaceUseDeclaration(x) => x.as_any(),
            AnyNode::NamespaceUseGroup(x) => x.as_any(),
            AnyNode::NamespaceUseGroupClause(x) => x.as_any(),
            AnyNode::Nowdoc(x) => x.as_any(),
            AnyNode::NowdocBody(x) => x.as_any(),
            AnyNode::Null(x) => x.as_any(),
            AnyNode::NullsafeMemberAccessExpression(x) => x.as_any(),
            AnyNode::NullsafeMemberCallExpression(x) => x.as_any(),
            AnyNode::ObjectCreationExpression(x) => x.as_any(),
            AnyNode::OptionalType(x) => x.as_any(),
            AnyNode::Pair(x) => x.as_any(),
            AnyNode::ParenthesizedExpression(x) => x.as_any(),
            AnyNode::PrimitiveType(x) => x.as_any(),
            AnyNode::PrintIntrinsic(x) => x.as_any(),
            AnyNode::Program(x) => x.as_any(),
            AnyNode::PropertyDeclaration(x) => x.as_any(),
            AnyNode::PropertyElement(x) => x.as_any(),
            AnyNode::PropertyInitializer(x) => x.as_any(),
            AnyNode::PropertyPromotionParameter(x) => x.as_any(),
            AnyNode::QualifiedName(x) => x.as_any(),
            AnyNode::ReadonlyModifier(x) => x.as_any(),
            AnyNode::ReferenceAssignmentExpression(x) => x.as_any(),
            AnyNode::ReferenceModifier(x) => x.as_any(),
            AnyNode::RelativeScope(x) => x.as_any(),
            AnyNode::RequireExpression(x) => x.as_any(),
            AnyNode::RequireOnceExpression(x) => x.as_any(),
            AnyNode::ReturnStatement(x) => x.as_any(),
            AnyNode::ScopedCallExpression(x) => x.as_any(),
            AnyNode::ScopedPropertyAccessExpression(x) => x.as_any(),
            AnyNode::SequenceExpression(x) => x.as_any(),
            AnyNode::ShellCommandExpression(x) => x.as_any(),
            AnyNode::SimpleParameter(x) => x.as_any(),
            AnyNode::StaticModifier(x) => x.as_any(),
            AnyNode::StaticVariableDeclaration(x) => x.as_any(),
            AnyNode::String(x) => x.as_any(),
            AnyNode::SubscriptExpression(x) => x.as_any(),
            AnyNode::SwitchBlock(x) => x.as_any(),
            AnyNode::SwitchStatement(x) => x.as_any(),
            AnyNode::Text(x) => x.as_any(),
            AnyNode::TextInterpolation(x) => x.as_any(),
            AnyNode::ThrowExpression(x) => x.as_any(),
            AnyNode::TraitDeclaration(x) => x.as_any(),
            AnyNode::TryStatement(x) => x.as_any(),
            AnyNode::TypeList(x) => x.as_any(),
            AnyNode::UnaryOpExpression(x) => x.as_any(),
            AnyNode::UnionType(x) => x.as_any(),
            AnyNode::UnsetStatement(x) => x.as_any(),
            AnyNode::UpdateExpression(x) => x.as_any(),
            AnyNode::UseAsClause(x) => x.as_any(),
            AnyNode::UseDeclaration(x) => x.as_any(),
            AnyNode::UseInsteadOfClause(x) => x.as_any(),
            AnyNode::UseList(x) => x.as_any(),
            AnyNode::VariableName(x) => x.as_any(),
            AnyNode::VariadicParameter(x) => x.as_any(),
            AnyNode::VariadicPlaceholder(x) => x.as_any(),
            AnyNode::VariadicUnpacking(x) => x.as_any(),
            AnyNode::VisibilityModifier(x) => x.as_any(),
            AnyNode::WhileStatement(x) => x.as_any(),
            AnyNode::YieldExpression(x) => x.as_any(),

            AnyNode::BottomType(x) => x.as_any(),

            AnyNode::Comment(x) => x.as_any(),

            AnyNode::EscapeSequence(x) => x.as_any(),

            AnyNode::Float(x) => x.as_any(),

            AnyNode::HeredocEnd(x) => x.as_any(),
            AnyNode::HeredocStart(x) => x.as_any(),

            AnyNode::Integer(x) => x.as_any(),

            AnyNode::NowdocString(x) => x.as_any(),

            AnyNode::PhpTag(x) => x.as_any(),

            AnyNode::StringValue(x) => x.as_any(),

            AnyNode::VarModifier(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        todo!("NEKJ");
    }
}
