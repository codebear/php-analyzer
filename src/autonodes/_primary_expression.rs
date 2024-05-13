use crate::analysis::state::AnalysisState;
use crate::autonodes::_literal::_LiteralNode;
use crate::autonodes::anonymous_function_creation_expression::AnonymousFunctionCreationExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::array_creation_expression::ArrayCreationExpressionNode;
use crate::autonodes::arrow_function::ArrowFunctionNode;
use crate::autonodes::cast_expression::CastExpressionNode;
use crate::autonodes::class_constant_access_expression::ClassConstantAccessExpressionNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::dynamic_variable_name::DynamicVariableNameNode;
use crate::autonodes::function_call_expression::FunctionCallExpressionNode;
use crate::autonodes::member_access_expression::MemberAccessExpressionNode;
use crate::autonodes::member_call_expression::MemberCallExpressionNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::nullsafe_member_access_expression::NullsafeMemberAccessExpressionNode;
use crate::autonodes::nullsafe_member_call_expression::NullsafeMemberCallExpressionNode;
use crate::autonodes::object_creation_expression::ObjectCreationExpressionNode;
use crate::autonodes::parenthesized_expression::ParenthesizedExpressionNode;
use crate::autonodes::print_intrinsic::PrintIntrinsicNode;
use crate::autonodes::qualified_name::QualifiedNameNode;
use crate::autonodes::scoped_call_expression::ScopedCallExpressionNode;
use crate::autonodes::scoped_property_access_expression::ScopedPropertyAccessExpressionNode;
use crate::autonodes::shell_command_expression::ShellCommandExpressionNode;
use crate::autonodes::subscript_expression::SubscriptExpressionNode;
use crate::autonodes::throw_expression::ThrowExpressionNode;
use crate::autonodes::update_expression::UpdateExpressionNode;
use crate::autonodes::variable_name::VariableNameNode;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::parser::Range;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub enum _PrimaryExpressionNode {
    _Literal(Box<_LiteralNode>),
    AnonymousFunctionCreationExpression(Box<AnonymousFunctionCreationExpressionNode>),
    ArrayCreationExpression(Box<ArrayCreationExpressionNode>),
    ArrowFunction(Box<ArrowFunctionNode>),
    CastExpression(Box<CastExpressionNode>),
    ClassConstantAccessExpression(Box<ClassConstantAccessExpressionNode>),
    DynamicVariableName(Box<DynamicVariableNameNode>),
    FunctionCallExpression(Box<FunctionCallExpressionNode>),
    MemberAccessExpression(Box<MemberAccessExpressionNode>),
    MemberCallExpression(Box<MemberCallExpressionNode>),
    Name(Box<NameNode>),
    NullsafeMemberAccessExpression(Box<NullsafeMemberAccessExpressionNode>),
    NullsafeMemberCallExpression(Box<NullsafeMemberCallExpressionNode>),
    ObjectCreationExpression(Box<ObjectCreationExpressionNode>),
    ParenthesizedExpression(Box<ParenthesizedExpressionNode>),
    PrintIntrinsic(Box<PrintIntrinsicNode>),
    QualifiedName(Box<QualifiedNameNode>),
    ScopedCallExpression(Box<ScopedCallExpressionNode>),
    ScopedPropertyAccessExpression(Box<ScopedPropertyAccessExpressionNode>),
    ShellCommandExpression(Box<ShellCommandExpressionNode>),
    SubscriptExpression(Box<SubscriptExpressionNode>),
    ThrowExpression(Box<ThrowExpressionNode>),
    UpdateExpression(Box<UpdateExpressionNode>),
    VariableName(Box<VariableNameNode>),
    Extra(ExtraChild),
}

impl NodeParser for _PrimaryExpressionNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => _PrimaryExpressionNode::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => _PrimaryExpressionNode::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "anonymous_function_creation_expression" => {
                _PrimaryExpressionNode::AnonymousFunctionCreationExpression(Box::new(
                    AnonymousFunctionCreationExpressionNode::parse(node, source)?,
                ))
            }
            "array_creation_expression" => _PrimaryExpressionNode::ArrayCreationExpression(
                Box::new(ArrayCreationExpressionNode::parse(node, source)?),
            ),
            "arrow_function" => _PrimaryExpressionNode::ArrowFunction(Box::new(
                ArrowFunctionNode::parse(node, source)?,
            )),
            "cast_expression" => _PrimaryExpressionNode::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                _PrimaryExpressionNode::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => _PrimaryExpressionNode::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "function_call_expression" => _PrimaryExpressionNode::FunctionCallExpression(Box::new(
                FunctionCallExpressionNode::parse(node, source)?,
            )),
            "member_access_expression" => _PrimaryExpressionNode::MemberAccessExpression(Box::new(
                MemberAccessExpressionNode::parse(node, source)?,
            )),
            "member_call_expression" => _PrimaryExpressionNode::MemberCallExpression(Box::new(
                MemberCallExpressionNode::parse(node, source)?,
            )),
            "name" => _PrimaryExpressionNode::Name(Box::new(NameNode::parse(node, source)?)),
            "nullsafe_member_access_expression" => {
                _PrimaryExpressionNode::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                _PrimaryExpressionNode::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "object_creation_expression" => _PrimaryExpressionNode::ObjectCreationExpression(
                Box::new(ObjectCreationExpressionNode::parse(node, source)?),
            ),
            "parenthesized_expression" => _PrimaryExpressionNode::ParenthesizedExpression(
                Box::new(ParenthesizedExpressionNode::parse(node, source)?),
            ),
            "print_intrinsic" => _PrimaryExpressionNode::PrintIntrinsic(Box::new(
                PrintIntrinsicNode::parse(node, source)?,
            )),
            "qualified_name" => _PrimaryExpressionNode::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_call_expression" => _PrimaryExpressionNode::ScopedCallExpression(Box::new(
                ScopedCallExpressionNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                _PrimaryExpressionNode::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "shell_command_expression" => _PrimaryExpressionNode::ShellCommandExpression(Box::new(
                ShellCommandExpressionNode::parse(node, source)?,
            )),
            "subscript_expression" => _PrimaryExpressionNode::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "throw_expression" => _PrimaryExpressionNode::ThrowExpression(Box::new(
                ThrowExpressionNode::parse(node, source)?,
            )),
            "update_expression" => _PrimaryExpressionNode::UpdateExpression(Box::new(
                UpdateExpressionNode::parse(node, source)?,
            )),
            "variable_name" => _PrimaryExpressionNode::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _LiteralNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(_PrimaryExpressionNode::_Literal)
                {
                    x
                } else {
                    return Err(ParseError::new(
                        node.range(),
                        format!("Parse error, unexpected node-type: {}", node.kind()),
                    ));
                }
            }
        })
    }
}

impl _PrimaryExpressionNode {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => _PrimaryExpressionNode::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => _PrimaryExpressionNode::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "anonymous_function_creation_expression" => {
                _PrimaryExpressionNode::AnonymousFunctionCreationExpression(Box::new(
                    AnonymousFunctionCreationExpressionNode::parse(node, source)?,
                ))
            }
            "array_creation_expression" => _PrimaryExpressionNode::ArrayCreationExpression(
                Box::new(ArrayCreationExpressionNode::parse(node, source)?),
            ),
            "arrow_function" => _PrimaryExpressionNode::ArrowFunction(Box::new(
                ArrowFunctionNode::parse(node, source)?,
            )),
            "cast_expression" => _PrimaryExpressionNode::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                _PrimaryExpressionNode::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => _PrimaryExpressionNode::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "function_call_expression" => _PrimaryExpressionNode::FunctionCallExpression(Box::new(
                FunctionCallExpressionNode::parse(node, source)?,
            )),
            "member_access_expression" => _PrimaryExpressionNode::MemberAccessExpression(Box::new(
                MemberAccessExpressionNode::parse(node, source)?,
            )),
            "member_call_expression" => _PrimaryExpressionNode::MemberCallExpression(Box::new(
                MemberCallExpressionNode::parse(node, source)?,
            )),
            "name" => _PrimaryExpressionNode::Name(Box::new(NameNode::parse(node, source)?)),
            "nullsafe_member_access_expression" => {
                _PrimaryExpressionNode::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                _PrimaryExpressionNode::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "object_creation_expression" => _PrimaryExpressionNode::ObjectCreationExpression(
                Box::new(ObjectCreationExpressionNode::parse(node, source)?),
            ),
            "parenthesized_expression" => _PrimaryExpressionNode::ParenthesizedExpression(
                Box::new(ParenthesizedExpressionNode::parse(node, source)?),
            ),
            "print_intrinsic" => _PrimaryExpressionNode::PrintIntrinsic(Box::new(
                PrintIntrinsicNode::parse(node, source)?,
            )),
            "qualified_name" => _PrimaryExpressionNode::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_call_expression" => _PrimaryExpressionNode::ScopedCallExpression(Box::new(
                ScopedCallExpressionNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                _PrimaryExpressionNode::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "shell_command_expression" => _PrimaryExpressionNode::ShellCommandExpression(Box::new(
                ShellCommandExpressionNode::parse(node, source)?,
            )),
            "subscript_expression" => _PrimaryExpressionNode::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "throw_expression" => _PrimaryExpressionNode::ThrowExpression(Box::new(
                ThrowExpressionNode::parse(node, source)?,
            )),
            "update_expression" => _PrimaryExpressionNode::UpdateExpression(Box::new(
                UpdateExpressionNode::parse(node, source)?,
            )),
            "variable_name" => _PrimaryExpressionNode::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                return Ok(
                    _LiteralNode::parse_opt(node, source)?
                        .map(Box::new)
                        .map(_PrimaryExpressionNode::_Literal),
                )
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            _PrimaryExpressionNode::Extra(y) => y.kind(),
            _PrimaryExpressionNode::_Literal(y) => y.kind(),
            _PrimaryExpressionNode::AnonymousFunctionCreationExpression(y) => y.kind(),
            _PrimaryExpressionNode::ArrayCreationExpression(y) => y.kind(),
            _PrimaryExpressionNode::ArrowFunction(y) => y.kind(),
            _PrimaryExpressionNode::CastExpression(y) => y.kind(),
            _PrimaryExpressionNode::ClassConstantAccessExpression(y) => y.kind(),
            _PrimaryExpressionNode::DynamicVariableName(y) => y.kind(),
            _PrimaryExpressionNode::FunctionCallExpression(y) => y.kind(),
            _PrimaryExpressionNode::MemberAccessExpression(y) => y.kind(),
            _PrimaryExpressionNode::MemberCallExpression(y) => y.kind(),
            _PrimaryExpressionNode::Name(y) => y.kind(),
            _PrimaryExpressionNode::NullsafeMemberAccessExpression(y) => y.kind(),
            _PrimaryExpressionNode::NullsafeMemberCallExpression(y) => y.kind(),
            _PrimaryExpressionNode::ObjectCreationExpression(y) => y.kind(),
            _PrimaryExpressionNode::ParenthesizedExpression(y) => y.kind(),
            _PrimaryExpressionNode::PrintIntrinsic(y) => y.kind(),
            _PrimaryExpressionNode::QualifiedName(y) => y.kind(),
            _PrimaryExpressionNode::ScopedCallExpression(y) => y.kind(),
            _PrimaryExpressionNode::ScopedPropertyAccessExpression(y) => y.kind(),
            _PrimaryExpressionNode::ShellCommandExpression(y) => y.kind(),
            _PrimaryExpressionNode::SubscriptExpression(y) => y.kind(),
            _PrimaryExpressionNode::ThrowExpression(y) => y.kind(),
            _PrimaryExpressionNode::UpdateExpression(y) => y.kind(),
            _PrimaryExpressionNode::VariableName(y) => y.kind(),
        }
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
            _PrimaryExpressionNode::Extra(x) => x.get_utype(state, emitter),
            _PrimaryExpressionNode::_Literal(x) => x.get_utype(state, emitter),
            _PrimaryExpressionNode::AnonymousFunctionCreationExpression(x) => {
                x.get_utype(state, emitter)
            }
            _PrimaryExpressionNode::ArrayCreationExpression(x) => x.get_utype(state, emitter),
            _PrimaryExpressionNode::ArrowFunction(x) => x.get_utype(state, emitter),
            _PrimaryExpressionNode::CastExpression(x) => x.get_utype(state, emitter),
            _PrimaryExpressionNode::ClassConstantAccessExpression(x) => x.get_utype(state, emitter),
            _PrimaryExpressionNode::DynamicVariableName(x) => x.get_utype(state, emitter),
            _PrimaryExpressionNode::FunctionCallExpression(x) => x.get_utype(state, emitter),
            _PrimaryExpressionNode::MemberAccessExpression(x) => x.get_utype(state, emitter),
            _PrimaryExpressionNode::MemberCallExpression(x) => x.get_utype(state, emitter),
            _PrimaryExpressionNode::Name(x) => x.get_utype(state, emitter),
            _PrimaryExpressionNode::NullsafeMemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            _PrimaryExpressionNode::NullsafeMemberCallExpression(x) => x.get_utype(state, emitter),
            _PrimaryExpressionNode::ObjectCreationExpression(x) => x.get_utype(state, emitter),
            _PrimaryExpressionNode::ParenthesizedExpression(x) => x.get_utype(state, emitter),
            _PrimaryExpressionNode::PrintIntrinsic(x) => x.get_utype(state, emitter),
            _PrimaryExpressionNode::QualifiedName(x) => x.get_utype(state, emitter),
            _PrimaryExpressionNode::ScopedCallExpression(x) => x.get_utype(state, emitter),
            _PrimaryExpressionNode::ScopedPropertyAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            _PrimaryExpressionNode::ShellCommandExpression(x) => x.get_utype(state, emitter),
            _PrimaryExpressionNode::SubscriptExpression(x) => x.get_utype(state, emitter),
            _PrimaryExpressionNode::ThrowExpression(x) => x.get_utype(state, emitter),
            _PrimaryExpressionNode::UpdateExpression(x) => x.get_utype(state, emitter),
            _PrimaryExpressionNode::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            _PrimaryExpressionNode::Extra(x) => x.get_php_value(state, emitter),
            _PrimaryExpressionNode::_Literal(x) => x.get_php_value(state, emitter),
            _PrimaryExpressionNode::AnonymousFunctionCreationExpression(x) => {
                x.get_php_value(state, emitter)
            }
            _PrimaryExpressionNode::ArrayCreationExpression(x) => x.get_php_value(state, emitter),
            _PrimaryExpressionNode::ArrowFunction(x) => x.get_php_value(state, emitter),
            _PrimaryExpressionNode::CastExpression(x) => x.get_php_value(state, emitter),
            _PrimaryExpressionNode::ClassConstantAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            _PrimaryExpressionNode::DynamicVariableName(x) => x.get_php_value(state, emitter),
            _PrimaryExpressionNode::FunctionCallExpression(x) => x.get_php_value(state, emitter),
            _PrimaryExpressionNode::MemberAccessExpression(x) => x.get_php_value(state, emitter),
            _PrimaryExpressionNode::MemberCallExpression(x) => x.get_php_value(state, emitter),
            _PrimaryExpressionNode::Name(x) => x.get_php_value(state, emitter),
            _PrimaryExpressionNode::NullsafeMemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            _PrimaryExpressionNode::NullsafeMemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            _PrimaryExpressionNode::ObjectCreationExpression(x) => x.get_php_value(state, emitter),
            _PrimaryExpressionNode::ParenthesizedExpression(x) => x.get_php_value(state, emitter),
            _PrimaryExpressionNode::PrintIntrinsic(x) => x.get_php_value(state, emitter),
            _PrimaryExpressionNode::QualifiedName(x) => x.get_php_value(state, emitter),
            _PrimaryExpressionNode::ScopedCallExpression(x) => x.get_php_value(state, emitter),
            _PrimaryExpressionNode::ScopedPropertyAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            _PrimaryExpressionNode::ShellCommandExpression(x) => x.get_php_value(state, emitter),
            _PrimaryExpressionNode::SubscriptExpression(x) => x.get_php_value(state, emitter),
            _PrimaryExpressionNode::ThrowExpression(x) => x.get_php_value(state, emitter),
            _PrimaryExpressionNode::UpdateExpression(x) => x.get_php_value(state, emitter),
            _PrimaryExpressionNode::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            _PrimaryExpressionNode::Extra(x) => x.read_from(state, emitter),
            _PrimaryExpressionNode::_Literal(x) => x.read_from(state, emitter),
            _PrimaryExpressionNode::AnonymousFunctionCreationExpression(x) => {
                x.read_from(state, emitter)
            }
            _PrimaryExpressionNode::ArrayCreationExpression(x) => x.read_from(state, emitter),
            _PrimaryExpressionNode::ArrowFunction(x) => x.read_from(state, emitter),
            _PrimaryExpressionNode::CastExpression(x) => x.read_from(state, emitter),
            _PrimaryExpressionNode::ClassConstantAccessExpression(x) => x.read_from(state, emitter),
            _PrimaryExpressionNode::DynamicVariableName(x) => x.read_from(state, emitter),
            _PrimaryExpressionNode::FunctionCallExpression(x) => x.read_from(state, emitter),
            _PrimaryExpressionNode::MemberAccessExpression(x) => x.read_from(state, emitter),
            _PrimaryExpressionNode::MemberCallExpression(x) => x.read_from(state, emitter),
            _PrimaryExpressionNode::Name(x) => x.read_from(state, emitter),
            _PrimaryExpressionNode::NullsafeMemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            _PrimaryExpressionNode::NullsafeMemberCallExpression(x) => x.read_from(state, emitter),
            _PrimaryExpressionNode::ObjectCreationExpression(x) => x.read_from(state, emitter),
            _PrimaryExpressionNode::ParenthesizedExpression(x) => x.read_from(state, emitter),
            _PrimaryExpressionNode::PrintIntrinsic(x) => x.read_from(state, emitter),
            _PrimaryExpressionNode::QualifiedName(x) => x.read_from(state, emitter),
            _PrimaryExpressionNode::ScopedCallExpression(x) => x.read_from(state, emitter),
            _PrimaryExpressionNode::ScopedPropertyAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            _PrimaryExpressionNode::ShellCommandExpression(x) => x.read_from(state, emitter),
            _PrimaryExpressionNode::SubscriptExpression(x) => x.read_from(state, emitter),
            _PrimaryExpressionNode::ThrowExpression(x) => x.read_from(state, emitter),
            _PrimaryExpressionNode::UpdateExpression(x) => x.read_from(state, emitter),
            _PrimaryExpressionNode::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for _PrimaryExpressionNode {
    fn brief_desc(&self) -> String {
        match self {
            _PrimaryExpressionNode::Extra(x) => {
                format!("_PrimaryExpressionNode::extra({})", x.brief_desc())
            }
            _PrimaryExpressionNode::_Literal(x) => {
                format!("_PrimaryExpressionNode::_literal({})", x.brief_desc())
            }
            _PrimaryExpressionNode::AnonymousFunctionCreationExpression(x) => format!(
                "_PrimaryExpressionNode::anonymous_function_creation_expression({})",
                x.brief_desc()
            ),
            _PrimaryExpressionNode::ArrayCreationExpression(x) => format!(
                "_PrimaryExpressionNode::array_creation_expression({})",
                x.brief_desc()
            ),
            _PrimaryExpressionNode::ArrowFunction(x) => {
                format!("_PrimaryExpressionNode::arrow_function({})", x.brief_desc())
            }
            _PrimaryExpressionNode::CastExpression(x) => format!(
                "_PrimaryExpressionNode::cast_expression({})",
                x.brief_desc()
            ),
            _PrimaryExpressionNode::ClassConstantAccessExpression(x) => format!(
                "_PrimaryExpressionNode::class_constant_access_expression({})",
                x.brief_desc()
            ),
            _PrimaryExpressionNode::DynamicVariableName(x) => format!(
                "_PrimaryExpressionNode::dynamic_variable_name({})",
                x.brief_desc()
            ),
            _PrimaryExpressionNode::FunctionCallExpression(x) => format!(
                "_PrimaryExpressionNode::function_call_expression({})",
                x.brief_desc()
            ),
            _PrimaryExpressionNode::MemberAccessExpression(x) => format!(
                "_PrimaryExpressionNode::member_access_expression({})",
                x.brief_desc()
            ),
            _PrimaryExpressionNode::MemberCallExpression(x) => format!(
                "_PrimaryExpressionNode::member_call_expression({})",
                x.brief_desc()
            ),
            _PrimaryExpressionNode::Name(x) => {
                format!("_PrimaryExpressionNode::name({})", x.brief_desc())
            }
            _PrimaryExpressionNode::NullsafeMemberAccessExpression(x) => format!(
                "_PrimaryExpressionNode::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            _PrimaryExpressionNode::NullsafeMemberCallExpression(x) => format!(
                "_PrimaryExpressionNode::nullsafe_member_call_expression({})",
                x.brief_desc()
            ),
            _PrimaryExpressionNode::ObjectCreationExpression(x) => format!(
                "_PrimaryExpressionNode::object_creation_expression({})",
                x.brief_desc()
            ),
            _PrimaryExpressionNode::ParenthesizedExpression(x) => format!(
                "_PrimaryExpressionNode::parenthesized_expression({})",
                x.brief_desc()
            ),
            _PrimaryExpressionNode::PrintIntrinsic(x) => format!(
                "_PrimaryExpressionNode::print_intrinsic({})",
                x.brief_desc()
            ),
            _PrimaryExpressionNode::QualifiedName(x) => {
                format!("_PrimaryExpressionNode::qualified_name({})", x.brief_desc())
            }
            _PrimaryExpressionNode::ScopedCallExpression(x) => format!(
                "_PrimaryExpressionNode::scoped_call_expression({})",
                x.brief_desc()
            ),
            _PrimaryExpressionNode::ScopedPropertyAccessExpression(x) => format!(
                "_PrimaryExpressionNode::scoped_property_access_expression({})",
                x.brief_desc()
            ),
            _PrimaryExpressionNode::ShellCommandExpression(x) => format!(
                "_PrimaryExpressionNode::shell_command_expression({})",
                x.brief_desc()
            ),
            _PrimaryExpressionNode::SubscriptExpression(x) => format!(
                "_PrimaryExpressionNode::subscript_expression({})",
                x.brief_desc()
            ),
            _PrimaryExpressionNode::ThrowExpression(x) => format!(
                "_PrimaryExpressionNode::throw_expression({})",
                x.brief_desc()
            ),
            _PrimaryExpressionNode::UpdateExpression(x) => format!(
                "_PrimaryExpressionNode::update_expression({})",
                x.brief_desc()
            ),
            _PrimaryExpressionNode::VariableName(x) => {
                format!("_PrimaryExpressionNode::variable_name({})", x.brief_desc())
            }
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            _PrimaryExpressionNode::Extra(x) => x.as_any(),
            _PrimaryExpressionNode::_Literal(x) => x.as_any(),
            _PrimaryExpressionNode::AnonymousFunctionCreationExpression(x) => x.as_any(),
            _PrimaryExpressionNode::ArrayCreationExpression(x) => x.as_any(),
            _PrimaryExpressionNode::ArrowFunction(x) => x.as_any(),
            _PrimaryExpressionNode::CastExpression(x) => x.as_any(),
            _PrimaryExpressionNode::ClassConstantAccessExpression(x) => x.as_any(),
            _PrimaryExpressionNode::DynamicVariableName(x) => x.as_any(),
            _PrimaryExpressionNode::FunctionCallExpression(x) => x.as_any(),
            _PrimaryExpressionNode::MemberAccessExpression(x) => x.as_any(),
            _PrimaryExpressionNode::MemberCallExpression(x) => x.as_any(),
            _PrimaryExpressionNode::Name(x) => x.as_any(),
            _PrimaryExpressionNode::NullsafeMemberAccessExpression(x) => x.as_any(),
            _PrimaryExpressionNode::NullsafeMemberCallExpression(x) => x.as_any(),
            _PrimaryExpressionNode::ObjectCreationExpression(x) => x.as_any(),
            _PrimaryExpressionNode::ParenthesizedExpression(x) => x.as_any(),
            _PrimaryExpressionNode::PrintIntrinsic(x) => x.as_any(),
            _PrimaryExpressionNode::QualifiedName(x) => x.as_any(),
            _PrimaryExpressionNode::ScopedCallExpression(x) => x.as_any(),
            _PrimaryExpressionNode::ScopedPropertyAccessExpression(x) => x.as_any(),
            _PrimaryExpressionNode::ShellCommandExpression(x) => x.as_any(),
            _PrimaryExpressionNode::SubscriptExpression(x) => x.as_any(),
            _PrimaryExpressionNode::ThrowExpression(x) => x.as_any(),
            _PrimaryExpressionNode::UpdateExpression(x) => x.as_any(),
            _PrimaryExpressionNode::VariableName(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            _PrimaryExpressionNode::Extra(x) => x.children_any(),
            _PrimaryExpressionNode::_Literal(x) => x.children_any(),
            _PrimaryExpressionNode::AnonymousFunctionCreationExpression(x) => x.children_any(),
            _PrimaryExpressionNode::ArrayCreationExpression(x) => x.children_any(),
            _PrimaryExpressionNode::ArrowFunction(x) => x.children_any(),
            _PrimaryExpressionNode::CastExpression(x) => x.children_any(),
            _PrimaryExpressionNode::ClassConstantAccessExpression(x) => x.children_any(),
            _PrimaryExpressionNode::DynamicVariableName(x) => x.children_any(),
            _PrimaryExpressionNode::FunctionCallExpression(x) => x.children_any(),
            _PrimaryExpressionNode::MemberAccessExpression(x) => x.children_any(),
            _PrimaryExpressionNode::MemberCallExpression(x) => x.children_any(),
            _PrimaryExpressionNode::Name(x) => x.children_any(),
            _PrimaryExpressionNode::NullsafeMemberAccessExpression(x) => x.children_any(),
            _PrimaryExpressionNode::NullsafeMemberCallExpression(x) => x.children_any(),
            _PrimaryExpressionNode::ObjectCreationExpression(x) => x.children_any(),
            _PrimaryExpressionNode::ParenthesizedExpression(x) => x.children_any(),
            _PrimaryExpressionNode::PrintIntrinsic(x) => x.children_any(),
            _PrimaryExpressionNode::QualifiedName(x) => x.children_any(),
            _PrimaryExpressionNode::ScopedCallExpression(x) => x.children_any(),
            _PrimaryExpressionNode::ScopedPropertyAccessExpression(x) => x.children_any(),
            _PrimaryExpressionNode::ShellCommandExpression(x) => x.children_any(),
            _PrimaryExpressionNode::SubscriptExpression(x) => x.children_any(),
            _PrimaryExpressionNode::ThrowExpression(x) => x.children_any(),
            _PrimaryExpressionNode::UpdateExpression(x) => x.children_any(),
            _PrimaryExpressionNode::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            _PrimaryExpressionNode::Extra(x) => x.range(),
            _PrimaryExpressionNode::_Literal(x) => x.range(),
            _PrimaryExpressionNode::AnonymousFunctionCreationExpression(x) => x.range(),
            _PrimaryExpressionNode::ArrayCreationExpression(x) => x.range(),
            _PrimaryExpressionNode::ArrowFunction(x) => x.range(),
            _PrimaryExpressionNode::CastExpression(x) => x.range(),
            _PrimaryExpressionNode::ClassConstantAccessExpression(x) => x.range(),
            _PrimaryExpressionNode::DynamicVariableName(x) => x.range(),
            _PrimaryExpressionNode::FunctionCallExpression(x) => x.range(),
            _PrimaryExpressionNode::MemberAccessExpression(x) => x.range(),
            _PrimaryExpressionNode::MemberCallExpression(x) => x.range(),
            _PrimaryExpressionNode::Name(x) => x.range(),
            _PrimaryExpressionNode::NullsafeMemberAccessExpression(x) => x.range(),
            _PrimaryExpressionNode::NullsafeMemberCallExpression(x) => x.range(),
            _PrimaryExpressionNode::ObjectCreationExpression(x) => x.range(),
            _PrimaryExpressionNode::ParenthesizedExpression(x) => x.range(),
            _PrimaryExpressionNode::PrintIntrinsic(x) => x.range(),
            _PrimaryExpressionNode::QualifiedName(x) => x.range(),
            _PrimaryExpressionNode::ScopedCallExpression(x) => x.range(),
            _PrimaryExpressionNode::ScopedPropertyAccessExpression(x) => x.range(),
            _PrimaryExpressionNode::ShellCommandExpression(x) => x.range(),
            _PrimaryExpressionNode::SubscriptExpression(x) => x.range(),
            _PrimaryExpressionNode::ThrowExpression(x) => x.range(),
            _PrimaryExpressionNode::UpdateExpression(x) => x.range(),
            _PrimaryExpressionNode::VariableName(x) => x.range(),
        }
    }
}
