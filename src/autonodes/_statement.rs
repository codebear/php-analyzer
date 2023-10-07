use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::break_statement::BreakStatementNode;
use crate::autonodes::class_declaration::ClassDeclarationNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::compound_statement::CompoundStatementNode;
use crate::autonodes::const_declaration::ConstDeclarationNode;
use crate::autonodes::continue_statement::ContinueStatementNode;
use crate::autonodes::declare_statement::DeclareStatementNode;
use crate::autonodes::do_statement::DoStatementNode;
use crate::autonodes::echo_statement::EchoStatementNode;
use crate::autonodes::empty_statement::EmptyStatementNode;
use crate::autonodes::enum_declaration::EnumDeclarationNode;
use crate::autonodes::expression_statement::ExpressionStatementNode;
use crate::autonodes::for_statement::ForStatementNode;
use crate::autonodes::foreach_statement::ForeachStatementNode;
use crate::autonodes::function_definition::FunctionDefinitionNode;
use crate::autonodes::function_static_declaration::FunctionStaticDeclarationNode;
use crate::autonodes::global_declaration::GlobalDeclarationNode;
use crate::autonodes::goto_statement::GotoStatementNode;
use crate::autonodes::if_statement::IfStatementNode;
use crate::autonodes::interface_declaration::InterfaceDeclarationNode;
use crate::autonodes::named_label_statement::NamedLabelStatementNode;
use crate::autonodes::namespace_definition::NamespaceDefinitionNode;
use crate::autonodes::namespace_use_declaration::NamespaceUseDeclarationNode;
use crate::autonodes::return_statement::ReturnStatementNode;
use crate::autonodes::switch_statement::SwitchStatementNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::trait_declaration::TraitDeclarationNode;
use crate::autonodes::try_statement::TryStatementNode;
use crate::autonodes::unset_statement::UnsetStatementNode;
use crate::autonodes::while_statement::WhileStatementNode;
use crate::autotree::NodeAccess;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub enum _StatementNode {
    BreakStatement(Box<BreakStatementNode>),
    ClassDeclaration(Box<ClassDeclarationNode>),
    CompoundStatement(Box<CompoundStatementNode>),
    ConstDeclaration(Box<ConstDeclarationNode>),
    ContinueStatement(Box<ContinueStatementNode>),
    DeclareStatement(Box<DeclareStatementNode>),
    DoStatement(Box<DoStatementNode>),
    EchoStatement(Box<EchoStatementNode>),
    EmptyStatement(Box<EmptyStatementNode>),
    EnumDeclaration(Box<EnumDeclarationNode>),
    ExpressionStatement(Box<ExpressionStatementNode>),
    ForStatement(Box<ForStatementNode>),
    ForeachStatement(Box<ForeachStatementNode>),
    FunctionDefinition(Box<FunctionDefinitionNode>),
    FunctionStaticDeclaration(Box<FunctionStaticDeclarationNode>),
    GlobalDeclaration(Box<GlobalDeclarationNode>),
    GotoStatement(Box<GotoStatementNode>),
    IfStatement(Box<IfStatementNode>),
    InterfaceDeclaration(Box<InterfaceDeclarationNode>),
    NamedLabelStatement(Box<NamedLabelStatementNode>),
    NamespaceDefinition(Box<NamespaceDefinitionNode>),
    NamespaceUseDeclaration(Box<NamespaceUseDeclarationNode>),
    ReturnStatement(Box<ReturnStatementNode>),
    SwitchStatement(Box<SwitchStatementNode>),
    TraitDeclaration(Box<TraitDeclarationNode>),
    TryStatement(Box<TryStatementNode>),
    UnsetStatement(Box<UnsetStatementNode>),
    WhileStatement(Box<WhileStatementNode>),
    Extra(ExtraChild),
}

impl _StatementNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => _StatementNode::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "text_interpolation" => _StatementNode::Extra(ExtraChild::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            ))),
            "ERROR" => {
                _StatementNode::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "break_statement" => {
                _StatementNode::BreakStatement(Box::new(BreakStatementNode::parse(node, source)?))
            }
            "class_declaration" => _StatementNode::ClassDeclaration(Box::new(
                ClassDeclarationNode::parse(node, source)?,
            )),
            "compound_statement" => _StatementNode::CompoundStatement(Box::new(
                CompoundStatementNode::parse(node, source)?,
            )),
            "const_declaration" => _StatementNode::ConstDeclaration(Box::new(
                ConstDeclarationNode::parse(node, source)?,
            )),
            "continue_statement" => _StatementNode::ContinueStatement(Box::new(
                ContinueStatementNode::parse(node, source)?,
            )),
            "declare_statement" => _StatementNode::DeclareStatement(Box::new(
                DeclareStatementNode::parse(node, source)?,
            )),
            "do_statement" => {
                _StatementNode::DoStatement(Box::new(DoStatementNode::parse(node, source)?))
            }
            "echo_statement" => {
                _StatementNode::EchoStatement(Box::new(EchoStatementNode::parse(node, source)?))
            }
            "empty_statement" => {
                _StatementNode::EmptyStatement(Box::new(EmptyStatementNode::parse(node, source)?))
            }
            "enum_declaration" => {
                _StatementNode::EnumDeclaration(Box::new(EnumDeclarationNode::parse(node, source)?))
            }
            "expression_statement" => _StatementNode::ExpressionStatement(Box::new(
                ExpressionStatementNode::parse(node, source)?,
            )),
            "for_statement" => {
                _StatementNode::ForStatement(Box::new(ForStatementNode::parse(node, source)?))
            }
            "foreach_statement" => _StatementNode::ForeachStatement(Box::new(
                ForeachStatementNode::parse(node, source)?,
            )),
            "function_definition" => _StatementNode::FunctionDefinition(Box::new(
                FunctionDefinitionNode::parse(node, source)?,
            )),
            "function_static_declaration" => _StatementNode::FunctionStaticDeclaration(Box::new(
                FunctionStaticDeclarationNode::parse(node, source)?,
            )),
            "global_declaration" => _StatementNode::GlobalDeclaration(Box::new(
                GlobalDeclarationNode::parse(node, source)?,
            )),
            "goto_statement" => {
                _StatementNode::GotoStatement(Box::new(GotoStatementNode::parse(node, source)?))
            }
            "if_statement" => {
                _StatementNode::IfStatement(Box::new(IfStatementNode::parse(node, source)?))
            }
            "interface_declaration" => _StatementNode::InterfaceDeclaration(Box::new(
                InterfaceDeclarationNode::parse(node, source)?,
            )),
            "named_label_statement" => _StatementNode::NamedLabelStatement(Box::new(
                NamedLabelStatementNode::parse(node, source)?,
            )),
            "namespace_definition" => _StatementNode::NamespaceDefinition(Box::new(
                NamespaceDefinitionNode::parse(node, source)?,
            )),
            "namespace_use_declaration" => _StatementNode::NamespaceUseDeclaration(Box::new(
                NamespaceUseDeclarationNode::parse(node, source)?,
            )),
            "return_statement" => {
                _StatementNode::ReturnStatement(Box::new(ReturnStatementNode::parse(node, source)?))
            }
            "switch_statement" => {
                _StatementNode::SwitchStatement(Box::new(SwitchStatementNode::parse(node, source)?))
            }
            "trait_declaration" => _StatementNode::TraitDeclaration(Box::new(
                TraitDeclarationNode::parse(node, source)?,
            )),
            "try_statement" => {
                _StatementNode::TryStatement(Box::new(TryStatementNode::parse(node, source)?))
            }
            "unset_statement" => {
                _StatementNode::UnsetStatement(Box::new(UnsetStatementNode::parse(node, source)?))
            }
            "while_statement" => {
                _StatementNode::WhileStatement(Box::new(WhileStatementNode::parse(node, source)?))
            }

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }

    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => _StatementNode::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "text_interpolation" => _StatementNode::Extra(ExtraChild::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            ))),
            "ERROR" => {
                _StatementNode::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "break_statement" => {
                _StatementNode::BreakStatement(Box::new(BreakStatementNode::parse(node, source)?))
            }
            "class_declaration" => _StatementNode::ClassDeclaration(Box::new(
                ClassDeclarationNode::parse(node, source)?,
            )),
            "compound_statement" => _StatementNode::CompoundStatement(Box::new(
                CompoundStatementNode::parse(node, source)?,
            )),
            "const_declaration" => _StatementNode::ConstDeclaration(Box::new(
                ConstDeclarationNode::parse(node, source)?,
            )),
            "continue_statement" => _StatementNode::ContinueStatement(Box::new(
                ContinueStatementNode::parse(node, source)?,
            )),
            "declare_statement" => _StatementNode::DeclareStatement(Box::new(
                DeclareStatementNode::parse(node, source)?,
            )),
            "do_statement" => {
                _StatementNode::DoStatement(Box::new(DoStatementNode::parse(node, source)?))
            }
            "echo_statement" => {
                _StatementNode::EchoStatement(Box::new(EchoStatementNode::parse(node, source)?))
            }
            "empty_statement" => {
                _StatementNode::EmptyStatement(Box::new(EmptyStatementNode::parse(node, source)?))
            }
            "enum_declaration" => {
                _StatementNode::EnumDeclaration(Box::new(EnumDeclarationNode::parse(node, source)?))
            }
            "expression_statement" => _StatementNode::ExpressionStatement(Box::new(
                ExpressionStatementNode::parse(node, source)?,
            )),
            "for_statement" => {
                _StatementNode::ForStatement(Box::new(ForStatementNode::parse(node, source)?))
            }
            "foreach_statement" => _StatementNode::ForeachStatement(Box::new(
                ForeachStatementNode::parse(node, source)?,
            )),
            "function_definition" => _StatementNode::FunctionDefinition(Box::new(
                FunctionDefinitionNode::parse(node, source)?,
            )),
            "function_static_declaration" => _StatementNode::FunctionStaticDeclaration(Box::new(
                FunctionStaticDeclarationNode::parse(node, source)?,
            )),
            "global_declaration" => _StatementNode::GlobalDeclaration(Box::new(
                GlobalDeclarationNode::parse(node, source)?,
            )),
            "goto_statement" => {
                _StatementNode::GotoStatement(Box::new(GotoStatementNode::parse(node, source)?))
            }
            "if_statement" => {
                _StatementNode::IfStatement(Box::new(IfStatementNode::parse(node, source)?))
            }
            "interface_declaration" => _StatementNode::InterfaceDeclaration(Box::new(
                InterfaceDeclarationNode::parse(node, source)?,
            )),
            "named_label_statement" => _StatementNode::NamedLabelStatement(Box::new(
                NamedLabelStatementNode::parse(node, source)?,
            )),
            "namespace_definition" => _StatementNode::NamespaceDefinition(Box::new(
                NamespaceDefinitionNode::parse(node, source)?,
            )),
            "namespace_use_declaration" => _StatementNode::NamespaceUseDeclaration(Box::new(
                NamespaceUseDeclarationNode::parse(node, source)?,
            )),
            "return_statement" => {
                _StatementNode::ReturnStatement(Box::new(ReturnStatementNode::parse(node, source)?))
            }
            "switch_statement" => {
                _StatementNode::SwitchStatement(Box::new(SwitchStatementNode::parse(node, source)?))
            }
            "trait_declaration" => _StatementNode::TraitDeclaration(Box::new(
                TraitDeclarationNode::parse(node, source)?,
            )),
            "try_statement" => {
                _StatementNode::TryStatement(Box::new(TryStatementNode::parse(node, source)?))
            }
            "unset_statement" => {
                _StatementNode::UnsetStatement(Box::new(UnsetStatementNode::parse(node, source)?))
            }
            "while_statement" => {
                _StatementNode::WhileStatement(Box::new(WhileStatementNode::parse(node, source)?))
            }

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        self.as_any().kind()
    }

    pub fn parse_vec<'a, I>(children: I, source: &Vec<u8>) -> Result<Vec<Box<Self>>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
    {
        let mut res: Vec<Box<Self>> = vec![];
        for child in children {
            res.push(Box::new(Self::parse(child, source)?));
        }
        Ok(res)
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        match self {
            _StatementNode::Extra(x) => x.get_utype(state, emitter),
            _StatementNode::BreakStatement(x) => x.get_utype(state, emitter),
            _StatementNode::ClassDeclaration(x) => x.get_utype(state, emitter),
            _StatementNode::CompoundStatement(x) => x.get_utype(state, emitter),
            _StatementNode::ConstDeclaration(x) => x.get_utype(state, emitter),
            _StatementNode::ContinueStatement(x) => x.get_utype(state, emitter),
            _StatementNode::DeclareStatement(x) => x.get_utype(state, emitter),
            _StatementNode::DoStatement(x) => x.get_utype(state, emitter),
            _StatementNode::EchoStatement(x) => x.get_utype(state, emitter),
            _StatementNode::EmptyStatement(x) => x.get_utype(state, emitter),
            _StatementNode::EnumDeclaration(x) => x.get_utype(state, emitter),
            _StatementNode::ExpressionStatement(x) => x.get_utype(state, emitter),
            _StatementNode::ForStatement(x) => x.get_utype(state, emitter),
            _StatementNode::ForeachStatement(x) => x.get_utype(state, emitter),
            _StatementNode::FunctionDefinition(x) => x.get_utype(state, emitter),
            _StatementNode::FunctionStaticDeclaration(x) => x.get_utype(state, emitter),
            _StatementNode::GlobalDeclaration(x) => x.get_utype(state, emitter),
            _StatementNode::GotoStatement(x) => x.get_utype(state, emitter),
            _StatementNode::IfStatement(x) => x.get_utype(state, emitter),
            _StatementNode::InterfaceDeclaration(x) => x.get_utype(state, emitter),
            _StatementNode::NamedLabelStatement(x) => x.get_utype(state, emitter),
            _StatementNode::NamespaceDefinition(x) => x.get_utype(state, emitter),
            _StatementNode::NamespaceUseDeclaration(x) => x.get_utype(state, emitter),
            _StatementNode::ReturnStatement(x) => x.get_utype(state, emitter),
            _StatementNode::SwitchStatement(x) => x.get_utype(state, emitter),
            _StatementNode::TraitDeclaration(x) => x.get_utype(state, emitter),
            _StatementNode::TryStatement(x) => x.get_utype(state, emitter),
            _StatementNode::UnsetStatement(x) => x.get_utype(state, emitter),
            _StatementNode::WhileStatement(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            _StatementNode::Extra(x) => x.get_php_value(state, emitter),
            _StatementNode::BreakStatement(x) => x.get_php_value(state, emitter),
            _StatementNode::ClassDeclaration(x) => x.get_php_value(state, emitter),
            _StatementNode::CompoundStatement(x) => x.get_php_value(state, emitter),
            _StatementNode::ConstDeclaration(x) => x.get_php_value(state, emitter),
            _StatementNode::ContinueStatement(x) => x.get_php_value(state, emitter),
            _StatementNode::DeclareStatement(x) => x.get_php_value(state, emitter),
            _StatementNode::DoStatement(x) => x.get_php_value(state, emitter),
            _StatementNode::EchoStatement(x) => x.get_php_value(state, emitter),
            _StatementNode::EmptyStatement(x) => x.get_php_value(state, emitter),
            _StatementNode::EnumDeclaration(x) => x.get_php_value(state, emitter),
            _StatementNode::ExpressionStatement(x) => x.get_php_value(state, emitter),
            _StatementNode::ForStatement(x) => x.get_php_value(state, emitter),
            _StatementNode::ForeachStatement(x) => x.get_php_value(state, emitter),
            _StatementNode::FunctionDefinition(x) => x.get_php_value(state, emitter),
            _StatementNode::FunctionStaticDeclaration(x) => x.get_php_value(state, emitter),
            _StatementNode::GlobalDeclaration(x) => x.get_php_value(state, emitter),
            _StatementNode::GotoStatement(x) => x.get_php_value(state, emitter),
            _StatementNode::IfStatement(x) => x.get_php_value(state, emitter),
            _StatementNode::InterfaceDeclaration(x) => x.get_php_value(state, emitter),
            _StatementNode::NamedLabelStatement(x) => x.get_php_value(state, emitter),
            _StatementNode::NamespaceDefinition(x) => x.get_php_value(state, emitter),
            _StatementNode::NamespaceUseDeclaration(x) => x.get_php_value(state, emitter),
            _StatementNode::ReturnStatement(x) => x.get_php_value(state, emitter),
            _StatementNode::SwitchStatement(x) => x.get_php_value(state, emitter),
            _StatementNode::TraitDeclaration(x) => x.get_php_value(state, emitter),
            _StatementNode::TryStatement(x) => x.get_php_value(state, emitter),
            _StatementNode::UnsetStatement(x) => x.get_php_value(state, emitter),
            _StatementNode::WhileStatement(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            _StatementNode::Extra(x) => x.read_from(state, emitter),
            _StatementNode::BreakStatement(x) => x.read_from(state, emitter),
            _StatementNode::ClassDeclaration(x) => x.read_from(state, emitter),
            _StatementNode::CompoundStatement(x) => x.read_from(state, emitter),
            _StatementNode::ConstDeclaration(x) => x.read_from(state, emitter),
            _StatementNode::ContinueStatement(x) => x.read_from(state, emitter),
            _StatementNode::DeclareStatement(x) => x.read_from(state, emitter),
            _StatementNode::DoStatement(x) => x.read_from(state, emitter),
            _StatementNode::EchoStatement(x) => x.read_from(state, emitter),
            _StatementNode::EmptyStatement(x) => x.read_from(state, emitter),
            _StatementNode::EnumDeclaration(x) => x.read_from(state, emitter),
            _StatementNode::ExpressionStatement(x) => x.read_from(state, emitter),
            _StatementNode::ForStatement(x) => x.read_from(state, emitter),
            _StatementNode::ForeachStatement(x) => x.read_from(state, emitter),
            _StatementNode::FunctionDefinition(x) => x.read_from(state, emitter),
            _StatementNode::FunctionStaticDeclaration(x) => x.read_from(state, emitter),
            _StatementNode::GlobalDeclaration(x) => x.read_from(state, emitter),
            _StatementNode::GotoStatement(x) => x.read_from(state, emitter),
            _StatementNode::IfStatement(x) => x.read_from(state, emitter),
            _StatementNode::InterfaceDeclaration(x) => x.read_from(state, emitter),
            _StatementNode::NamedLabelStatement(x) => x.read_from(state, emitter),
            _StatementNode::NamespaceDefinition(x) => x.read_from(state, emitter),
            _StatementNode::NamespaceUseDeclaration(x) => x.read_from(state, emitter),
            _StatementNode::ReturnStatement(x) => x.read_from(state, emitter),
            _StatementNode::SwitchStatement(x) => x.read_from(state, emitter),
            _StatementNode::TraitDeclaration(x) => x.read_from(state, emitter),
            _StatementNode::TryStatement(x) => x.read_from(state, emitter),
            _StatementNode::UnsetStatement(x) => x.read_from(state, emitter),
            _StatementNode::WhileStatement(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for _StatementNode {
    fn brief_desc(&self) -> String {
        match self {
            _StatementNode::Extra(x) => format!("_StatementNode::extra({})", x.brief_desc()),
            _StatementNode::BreakStatement(x) => {
                format!("_StatementNode::break_statement({})", x.brief_desc())
            }
            _StatementNode::ClassDeclaration(x) => {
                format!("_StatementNode::class_declaration({})", x.brief_desc())
            }
            _StatementNode::CompoundStatement(x) => {
                format!("_StatementNode::compound_statement({})", x.brief_desc())
            }
            _StatementNode::ConstDeclaration(x) => {
                format!("_StatementNode::const_declaration({})", x.brief_desc())
            }
            _StatementNode::ContinueStatement(x) => {
                format!("_StatementNode::continue_statement({})", x.brief_desc())
            }
            _StatementNode::DeclareStatement(x) => {
                format!("_StatementNode::declare_statement({})", x.brief_desc())
            }
            _StatementNode::DoStatement(x) => {
                format!("_StatementNode::do_statement({})", x.brief_desc())
            }
            _StatementNode::EchoStatement(x) => {
                format!("_StatementNode::echo_statement({})", x.brief_desc())
            }
            _StatementNode::EmptyStatement(x) => {
                format!("_StatementNode::empty_statement({})", x.brief_desc())
            }
            _StatementNode::EnumDeclaration(x) => {
                format!("_StatementNode::enum_declaration({})", x.brief_desc())
            }
            _StatementNode::ExpressionStatement(x) => {
                format!("_StatementNode::expression_statement({})", x.brief_desc())
            }
            _StatementNode::ForStatement(x) => {
                format!("_StatementNode::for_statement({})", x.brief_desc())
            }
            _StatementNode::ForeachStatement(x) => {
                format!("_StatementNode::foreach_statement({})", x.brief_desc())
            }
            _StatementNode::FunctionDefinition(x) => {
                format!("_StatementNode::function_definition({})", x.brief_desc())
            }
            _StatementNode::FunctionStaticDeclaration(x) => format!(
                "_StatementNode::function_static_declaration({})",
                x.brief_desc()
            ),
            _StatementNode::GlobalDeclaration(x) => {
                format!("_StatementNode::global_declaration({})", x.brief_desc())
            }
            _StatementNode::GotoStatement(x) => {
                format!("_StatementNode::goto_statement({})", x.brief_desc())
            }
            _StatementNode::IfStatement(x) => {
                format!("_StatementNode::if_statement({})", x.brief_desc())
            }
            _StatementNode::InterfaceDeclaration(x) => {
                format!("_StatementNode::interface_declaration({})", x.brief_desc())
            }
            _StatementNode::NamedLabelStatement(x) => {
                format!("_StatementNode::named_label_statement({})", x.brief_desc())
            }
            _StatementNode::NamespaceDefinition(x) => {
                format!("_StatementNode::namespace_definition({})", x.brief_desc())
            }
            _StatementNode::NamespaceUseDeclaration(x) => format!(
                "_StatementNode::namespace_use_declaration({})",
                x.brief_desc()
            ),
            _StatementNode::ReturnStatement(x) => {
                format!("_StatementNode::return_statement({})", x.brief_desc())
            }
            _StatementNode::SwitchStatement(x) => {
                format!("_StatementNode::switch_statement({})", x.brief_desc())
            }
            _StatementNode::TraitDeclaration(x) => {
                format!("_StatementNode::trait_declaration({})", x.brief_desc())
            }
            _StatementNode::TryStatement(x) => {
                format!("_StatementNode::try_statement({})", x.brief_desc())
            }
            _StatementNode::UnsetStatement(x) => {
                format!("_StatementNode::unset_statement({})", x.brief_desc())
            }
            _StatementNode::WhileStatement(x) => {
                format!("_StatementNode::while_statement({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            _StatementNode::Extra(x) => x.as_any(),
            _StatementNode::BreakStatement(x) => x.as_any(),
            _StatementNode::ClassDeclaration(x) => x.as_any(),
            _StatementNode::CompoundStatement(x) => x.as_any(),
            _StatementNode::ConstDeclaration(x) => x.as_any(),
            _StatementNode::ContinueStatement(x) => x.as_any(),
            _StatementNode::DeclareStatement(x) => x.as_any(),
            _StatementNode::DoStatement(x) => x.as_any(),
            _StatementNode::EchoStatement(x) => x.as_any(),
            _StatementNode::EmptyStatement(x) => x.as_any(),
            _StatementNode::EnumDeclaration(x) => x.as_any(),
            _StatementNode::ExpressionStatement(x) => x.as_any(),
            _StatementNode::ForStatement(x) => x.as_any(),
            _StatementNode::ForeachStatement(x) => x.as_any(),
            _StatementNode::FunctionDefinition(x) => x.as_any(),
            _StatementNode::FunctionStaticDeclaration(x) => x.as_any(),
            _StatementNode::GlobalDeclaration(x) => x.as_any(),
            _StatementNode::GotoStatement(x) => x.as_any(),
            _StatementNode::IfStatement(x) => x.as_any(),
            _StatementNode::InterfaceDeclaration(x) => x.as_any(),
            _StatementNode::NamedLabelStatement(x) => x.as_any(),
            _StatementNode::NamespaceDefinition(x) => x.as_any(),
            _StatementNode::NamespaceUseDeclaration(x) => x.as_any(),
            _StatementNode::ReturnStatement(x) => x.as_any(),
            _StatementNode::SwitchStatement(x) => x.as_any(),
            _StatementNode::TraitDeclaration(x) => x.as_any(),
            _StatementNode::TryStatement(x) => x.as_any(),
            _StatementNode::UnsetStatement(x) => x.as_any(),
            _StatementNode::WhileStatement(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            _StatementNode::Extra(x) => x.children_any(),
            _StatementNode::BreakStatement(x) => x.children_any(),
            _StatementNode::ClassDeclaration(x) => x.children_any(),
            _StatementNode::CompoundStatement(x) => x.children_any(),
            _StatementNode::ConstDeclaration(x) => x.children_any(),
            _StatementNode::ContinueStatement(x) => x.children_any(),
            _StatementNode::DeclareStatement(x) => x.children_any(),
            _StatementNode::DoStatement(x) => x.children_any(),
            _StatementNode::EchoStatement(x) => x.children_any(),
            _StatementNode::EmptyStatement(x) => x.children_any(),
            _StatementNode::EnumDeclaration(x) => x.children_any(),
            _StatementNode::ExpressionStatement(x) => x.children_any(),
            _StatementNode::ForStatement(x) => x.children_any(),
            _StatementNode::ForeachStatement(x) => x.children_any(),
            _StatementNode::FunctionDefinition(x) => x.children_any(),
            _StatementNode::FunctionStaticDeclaration(x) => x.children_any(),
            _StatementNode::GlobalDeclaration(x) => x.children_any(),
            _StatementNode::GotoStatement(x) => x.children_any(),
            _StatementNode::IfStatement(x) => x.children_any(),
            _StatementNode::InterfaceDeclaration(x) => x.children_any(),
            _StatementNode::NamedLabelStatement(x) => x.children_any(),
            _StatementNode::NamespaceDefinition(x) => x.children_any(),
            _StatementNode::NamespaceUseDeclaration(x) => x.children_any(),
            _StatementNode::ReturnStatement(x) => x.children_any(),
            _StatementNode::SwitchStatement(x) => x.children_any(),
            _StatementNode::TraitDeclaration(x) => x.children_any(),
            _StatementNode::TryStatement(x) => x.children_any(),
            _StatementNode::UnsetStatement(x) => x.children_any(),
            _StatementNode::WhileStatement(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            _StatementNode::Extra(x) => x.range(),
            _StatementNode::BreakStatement(x) => x.range(),
            _StatementNode::ClassDeclaration(x) => x.range(),
            _StatementNode::CompoundStatement(x) => x.range(),
            _StatementNode::ConstDeclaration(x) => x.range(),
            _StatementNode::ContinueStatement(x) => x.range(),
            _StatementNode::DeclareStatement(x) => x.range(),
            _StatementNode::DoStatement(x) => x.range(),
            _StatementNode::EchoStatement(x) => x.range(),
            _StatementNode::EmptyStatement(x) => x.range(),
            _StatementNode::EnumDeclaration(x) => x.range(),
            _StatementNode::ExpressionStatement(x) => x.range(),
            _StatementNode::ForStatement(x) => x.range(),
            _StatementNode::ForeachStatement(x) => x.range(),
            _StatementNode::FunctionDefinition(x) => x.range(),
            _StatementNode::FunctionStaticDeclaration(x) => x.range(),
            _StatementNode::GlobalDeclaration(x) => x.range(),
            _StatementNode::GotoStatement(x) => x.range(),
            _StatementNode::IfStatement(x) => x.range(),
            _StatementNode::InterfaceDeclaration(x) => x.range(),
            _StatementNode::NamedLabelStatement(x) => x.range(),
            _StatementNode::NamespaceDefinition(x) => x.range(),
            _StatementNode::NamespaceUseDeclaration(x) => x.range(),
            _StatementNode::ReturnStatement(x) => x.range(),
            _StatementNode::SwitchStatement(x) => x.range(),
            _StatementNode::TraitDeclaration(x) => x.range(),
            _StatementNode::TryStatement(x) => x.range(),
            _StatementNode::UnsetStatement(x) => x.range(),
            _StatementNode::WhileStatement(x) => x.range(),
        }
    }
}
