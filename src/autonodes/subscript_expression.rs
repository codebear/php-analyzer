use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::array_creation_expression::ArrayCreationExpressionNode;
use crate::autonodes::cast_expression::CastExpressionNode;
use crate::autonodes::class_constant_access_expression::ClassConstantAccessExpressionNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::dynamic_variable_name::DynamicVariableNameNode;
use crate::autonodes::encapsed_string::EncapsedStringNode;
use crate::autonodes::function_call_expression::FunctionCallExpressionNode;
use crate::autonodes::heredoc::HeredocNode;
use crate::autonodes::member_access_expression::MemberAccessExpressionNode;
use crate::autonodes::member_call_expression::MemberCallExpressionNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::nowdoc::NowdocNode;
use crate::autonodes::nullsafe_member_access_expression::NullsafeMemberAccessExpressionNode;
use crate::autonodes::nullsafe_member_call_expression::NullsafeMemberCallExpressionNode;
use crate::autonodes::parenthesized_expression::ParenthesizedExpressionNode;
use crate::autonodes::qualified_name::QualifiedNameNode;
use crate::autonodes::scoped_call_expression::ScopedCallExpressionNode;
use crate::autonodes::scoped_property_access_expression::ScopedPropertyAccessExpressionNode;
use crate::autonodes::string::StringNode;
use crate::autonodes::variable_name::VariableNameNode;
use crate::autotree::ChildNodeParser;
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
pub enum SubscriptExpressionDereferenceable {
    ArrayCreationExpression(Box<ArrayCreationExpressionNode>),
    CastExpression(Box<CastExpressionNode>),
    ClassConstantAccessExpression(Box<ClassConstantAccessExpressionNode>),
    DynamicVariableName(Box<DynamicVariableNameNode>),
    EncapsedString(Box<EncapsedStringNode>),
    FunctionCallExpression(Box<FunctionCallExpressionNode>),
    Heredoc(Box<HeredocNode>),
    MemberAccessExpression(Box<MemberAccessExpressionNode>),
    MemberCallExpression(Box<MemberCallExpressionNode>),
    Name(Box<NameNode>),
    Nowdoc(Box<NowdocNode>),
    NullsafeMemberAccessExpression(Box<NullsafeMemberAccessExpressionNode>),
    NullsafeMemberCallExpression(Box<NullsafeMemberCallExpressionNode>),
    ParenthesizedExpression(Box<ParenthesizedExpressionNode>),
    QualifiedName(Box<QualifiedNameNode>),
    ScopedCallExpression(Box<ScopedCallExpressionNode>),
    ScopedPropertyAccessExpression(Box<ScopedPropertyAccessExpressionNode>),
    String(Box<StringNode>),
    SubscriptExpression(Box<SubscriptExpressionNode>),
    VariableName(Box<VariableNameNode>),
    Extra(ExtraChild),
}

impl NodeParser for SubscriptExpressionDereferenceable {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => SubscriptExpressionDereferenceable::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => SubscriptExpressionDereferenceable::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "array_creation_expression" => {
                SubscriptExpressionDereferenceable::ArrayCreationExpression(Box::new(
                    ArrayCreationExpressionNode::parse(node, source)?,
                ))
            }
            "cast_expression" => SubscriptExpressionDereferenceable::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                SubscriptExpressionDereferenceable::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => SubscriptExpressionDereferenceable::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "encapsed_string" => SubscriptExpressionDereferenceable::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => {
                SubscriptExpressionDereferenceable::FunctionCallExpression(Box::new(
                    FunctionCallExpressionNode::parse(node, source)?,
                ))
            }
            "heredoc" => SubscriptExpressionDereferenceable::Heredoc(Box::new(HeredocNode::parse(
                node, source,
            )?)),
            "member_access_expression" => {
                SubscriptExpressionDereferenceable::MemberAccessExpression(Box::new(
                    MemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "member_call_expression" => SubscriptExpressionDereferenceable::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "name" => {
                SubscriptExpressionDereferenceable::Name(Box::new(NameNode::parse(node, source)?))
            }
            "nowdoc" => SubscriptExpressionDereferenceable::Nowdoc(Box::new(NowdocNode::parse(
                node, source,
            )?)),
            "nullsafe_member_access_expression" => {
                SubscriptExpressionDereferenceable::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                SubscriptExpressionDereferenceable::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => {
                SubscriptExpressionDereferenceable::ParenthesizedExpression(Box::new(
                    ParenthesizedExpressionNode::parse(node, source)?,
                ))
            }
            "qualified_name" => SubscriptExpressionDereferenceable::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_call_expression" => SubscriptExpressionDereferenceable::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "scoped_property_access_expression" => {
                SubscriptExpressionDereferenceable::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "string" => SubscriptExpressionDereferenceable::String(Box::new(StringNode::parse(
                node, source,
            )?)),
            "subscript_expression" => SubscriptExpressionDereferenceable::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => SubscriptExpressionDereferenceable::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }
}

impl SubscriptExpressionDereferenceable {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => SubscriptExpressionDereferenceable::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => SubscriptExpressionDereferenceable::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "array_creation_expression" => {
                SubscriptExpressionDereferenceable::ArrayCreationExpression(Box::new(
                    ArrayCreationExpressionNode::parse(node, source)?,
                ))
            }
            "cast_expression" => SubscriptExpressionDereferenceable::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                SubscriptExpressionDereferenceable::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => SubscriptExpressionDereferenceable::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "encapsed_string" => SubscriptExpressionDereferenceable::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => {
                SubscriptExpressionDereferenceable::FunctionCallExpression(Box::new(
                    FunctionCallExpressionNode::parse(node, source)?,
                ))
            }
            "heredoc" => SubscriptExpressionDereferenceable::Heredoc(Box::new(HeredocNode::parse(
                node, source,
            )?)),
            "member_access_expression" => {
                SubscriptExpressionDereferenceable::MemberAccessExpression(Box::new(
                    MemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "member_call_expression" => SubscriptExpressionDereferenceable::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "name" => {
                SubscriptExpressionDereferenceable::Name(Box::new(NameNode::parse(node, source)?))
            }
            "nowdoc" => SubscriptExpressionDereferenceable::Nowdoc(Box::new(NowdocNode::parse(
                node, source,
            )?)),
            "nullsafe_member_access_expression" => {
                SubscriptExpressionDereferenceable::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                SubscriptExpressionDereferenceable::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => {
                SubscriptExpressionDereferenceable::ParenthesizedExpression(Box::new(
                    ParenthesizedExpressionNode::parse(node, source)?,
                ))
            }
            "qualified_name" => SubscriptExpressionDereferenceable::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_call_expression" => SubscriptExpressionDereferenceable::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "scoped_property_access_expression" => {
                SubscriptExpressionDereferenceable::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "string" => SubscriptExpressionDereferenceable::String(Box::new(StringNode::parse(
                node, source,
            )?)),
            "subscript_expression" => SubscriptExpressionDereferenceable::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => SubscriptExpressionDereferenceable::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            SubscriptExpressionDereferenceable::Extra(y) => y.kind(),
            SubscriptExpressionDereferenceable::ArrayCreationExpression(y) => y.kind(),
            SubscriptExpressionDereferenceable::CastExpression(y) => y.kind(),
            SubscriptExpressionDereferenceable::ClassConstantAccessExpression(y) => y.kind(),
            SubscriptExpressionDereferenceable::DynamicVariableName(y) => y.kind(),
            SubscriptExpressionDereferenceable::EncapsedString(y) => y.kind(),
            SubscriptExpressionDereferenceable::FunctionCallExpression(y) => y.kind(),
            SubscriptExpressionDereferenceable::Heredoc(y) => y.kind(),
            SubscriptExpressionDereferenceable::MemberAccessExpression(y) => y.kind(),
            SubscriptExpressionDereferenceable::MemberCallExpression(y) => y.kind(),
            SubscriptExpressionDereferenceable::Name(y) => y.kind(),
            SubscriptExpressionDereferenceable::Nowdoc(y) => y.kind(),
            SubscriptExpressionDereferenceable::NullsafeMemberAccessExpression(y) => y.kind(),
            SubscriptExpressionDereferenceable::NullsafeMemberCallExpression(y) => y.kind(),
            SubscriptExpressionDereferenceable::ParenthesizedExpression(y) => y.kind(),
            SubscriptExpressionDereferenceable::QualifiedName(y) => y.kind(),
            SubscriptExpressionDereferenceable::ScopedCallExpression(y) => y.kind(),
            SubscriptExpressionDereferenceable::ScopedPropertyAccessExpression(y) => y.kind(),
            SubscriptExpressionDereferenceable::String(y) => y.kind(),
            SubscriptExpressionDereferenceable::SubscriptExpression(y) => y.kind(),
            SubscriptExpressionDereferenceable::VariableName(y) => y.kind(),
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
            SubscriptExpressionDereferenceable::Extra(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferenceable::ArrayCreationExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::CastExpression(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferenceable::ClassConstantAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::DynamicVariableName(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::EncapsedString(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferenceable::FunctionCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::Heredoc(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferenceable::MemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::MemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::Name(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferenceable::Nowdoc(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferenceable::NullsafeMemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::NullsafeMemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::ParenthesizedExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::QualifiedName(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferenceable::ScopedCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::ScopedPropertyAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::String(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferenceable::SubscriptExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferenceable::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            SubscriptExpressionDereferenceable::Extra(x) => x.get_php_value(state, emitter),
            SubscriptExpressionDereferenceable::ArrayCreationExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::CastExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::ClassConstantAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::DynamicVariableName(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::EncapsedString(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::FunctionCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::Heredoc(x) => x.get_php_value(state, emitter),
            SubscriptExpressionDereferenceable::MemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::MemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::Name(x) => x.get_php_value(state, emitter),
            SubscriptExpressionDereferenceable::Nowdoc(x) => x.get_php_value(state, emitter),
            SubscriptExpressionDereferenceable::NullsafeMemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::NullsafeMemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::ParenthesizedExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::QualifiedName(x) => x.get_php_value(state, emitter),
            SubscriptExpressionDereferenceable::ScopedCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::ScopedPropertyAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::String(x) => x.get_php_value(state, emitter),
            SubscriptExpressionDereferenceable::SubscriptExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferenceable::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            SubscriptExpressionDereferenceable::Extra(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferenceable::ArrayCreationExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::CastExpression(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferenceable::ClassConstantAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::DynamicVariableName(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::EncapsedString(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferenceable::FunctionCallExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::Heredoc(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferenceable::MemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::MemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::Name(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferenceable::Nowdoc(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferenceable::NullsafeMemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::NullsafeMemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::ParenthesizedExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::QualifiedName(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferenceable::ScopedCallExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::ScopedPropertyAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::String(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferenceable::SubscriptExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferenceable::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for SubscriptExpressionDereferenceable {
    fn brief_desc(&self) -> String {
        match self {
            SubscriptExpressionDereferenceable::Extra(x) => format!(
                "SubscriptExpressionDereferenceable::extra({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::ArrayCreationExpression(x) => format!(
                "SubscriptExpressionDereferenceable::array_creation_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::CastExpression(x) => format!(
                "SubscriptExpressionDereferenceable::cast_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::ClassConstantAccessExpression(x) => format!(
                "SubscriptExpressionDereferenceable::class_constant_access_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::DynamicVariableName(x) => format!(
                "SubscriptExpressionDereferenceable::dynamic_variable_name({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::EncapsedString(x) => format!(
                "SubscriptExpressionDereferenceable::encapsed_string({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::FunctionCallExpression(x) => format!(
                "SubscriptExpressionDereferenceable::function_call_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::Heredoc(x) => format!(
                "SubscriptExpressionDereferenceable::heredoc({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::MemberAccessExpression(x) => format!(
                "SubscriptExpressionDereferenceable::member_access_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::MemberCallExpression(x) => format!(
                "SubscriptExpressionDereferenceable::member_call_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::Name(x) => format!(
                "SubscriptExpressionDereferenceable::name({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::Nowdoc(x) => format!(
                "SubscriptExpressionDereferenceable::nowdoc({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::NullsafeMemberAccessExpression(x) => format!(
                "SubscriptExpressionDereferenceable::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::NullsafeMemberCallExpression(x) => format!(
                "SubscriptExpressionDereferenceable::nullsafe_member_call_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::ParenthesizedExpression(x) => format!(
                "SubscriptExpressionDereferenceable::parenthesized_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::QualifiedName(x) => format!(
                "SubscriptExpressionDereferenceable::qualified_name({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::ScopedCallExpression(x) => format!(
                "SubscriptExpressionDereferenceable::scoped_call_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::ScopedPropertyAccessExpression(x) => format!(
                "SubscriptExpressionDereferenceable::scoped_property_access_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::String(x) => format!(
                "SubscriptExpressionDereferenceable::string({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::SubscriptExpression(x) => format!(
                "SubscriptExpressionDereferenceable::subscript_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferenceable::VariableName(x) => format!(
                "SubscriptExpressionDereferenceable::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            SubscriptExpressionDereferenceable::Extra(x) => x.as_any(),
            SubscriptExpressionDereferenceable::ArrayCreationExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::CastExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::ClassConstantAccessExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::DynamicVariableName(x) => x.as_any(),
            SubscriptExpressionDereferenceable::EncapsedString(x) => x.as_any(),
            SubscriptExpressionDereferenceable::FunctionCallExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::Heredoc(x) => x.as_any(),
            SubscriptExpressionDereferenceable::MemberAccessExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::MemberCallExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::Name(x) => x.as_any(),
            SubscriptExpressionDereferenceable::Nowdoc(x) => x.as_any(),
            SubscriptExpressionDereferenceable::NullsafeMemberAccessExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::NullsafeMemberCallExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::ParenthesizedExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::QualifiedName(x) => x.as_any(),
            SubscriptExpressionDereferenceable::ScopedCallExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::ScopedPropertyAccessExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::String(x) => x.as_any(),
            SubscriptExpressionDereferenceable::SubscriptExpression(x) => x.as_any(),
            SubscriptExpressionDereferenceable::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            SubscriptExpressionDereferenceable::Extra(x) => x.children_any(),
            SubscriptExpressionDereferenceable::ArrayCreationExpression(x) => x.children_any(),
            SubscriptExpressionDereferenceable::CastExpression(x) => x.children_any(),
            SubscriptExpressionDereferenceable::ClassConstantAccessExpression(x) => {
                x.children_any()
            }
            SubscriptExpressionDereferenceable::DynamicVariableName(x) => x.children_any(),
            SubscriptExpressionDereferenceable::EncapsedString(x) => x.children_any(),
            SubscriptExpressionDereferenceable::FunctionCallExpression(x) => x.children_any(),
            SubscriptExpressionDereferenceable::Heredoc(x) => x.children_any(),
            SubscriptExpressionDereferenceable::MemberAccessExpression(x) => x.children_any(),
            SubscriptExpressionDereferenceable::MemberCallExpression(x) => x.children_any(),
            SubscriptExpressionDereferenceable::Name(x) => x.children_any(),
            SubscriptExpressionDereferenceable::Nowdoc(x) => x.children_any(),
            SubscriptExpressionDereferenceable::NullsafeMemberAccessExpression(x) => {
                x.children_any()
            }
            SubscriptExpressionDereferenceable::NullsafeMemberCallExpression(x) => x.children_any(),
            SubscriptExpressionDereferenceable::ParenthesizedExpression(x) => x.children_any(),
            SubscriptExpressionDereferenceable::QualifiedName(x) => x.children_any(),
            SubscriptExpressionDereferenceable::ScopedCallExpression(x) => x.children_any(),
            SubscriptExpressionDereferenceable::ScopedPropertyAccessExpression(x) => {
                x.children_any()
            }
            SubscriptExpressionDereferenceable::String(x) => x.children_any(),
            SubscriptExpressionDereferenceable::SubscriptExpression(x) => x.children_any(),
            SubscriptExpressionDereferenceable::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            SubscriptExpressionDereferenceable::Extra(x) => x.range(),
            SubscriptExpressionDereferenceable::ArrayCreationExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::CastExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::ClassConstantAccessExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::DynamicVariableName(x) => x.range(),
            SubscriptExpressionDereferenceable::EncapsedString(x) => x.range(),
            SubscriptExpressionDereferenceable::FunctionCallExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::Heredoc(x) => x.range(),
            SubscriptExpressionDereferenceable::MemberAccessExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::MemberCallExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::Name(x) => x.range(),
            SubscriptExpressionDereferenceable::Nowdoc(x) => x.range(),
            SubscriptExpressionDereferenceable::NullsafeMemberAccessExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::NullsafeMemberCallExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::ParenthesizedExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::QualifiedName(x) => x.range(),
            SubscriptExpressionDereferenceable::ScopedCallExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::ScopedPropertyAccessExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::String(x) => x.range(),
            SubscriptExpressionDereferenceable::SubscriptExpression(x) => x.range(),
            SubscriptExpressionDereferenceable::VariableName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SubscriptExpressionNode {
    pub range: Range,
    pub dereferenceable: Box<SubscriptExpressionDereferenceable>,
    pub index: Option<_ExpressionNode>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for SubscriptExpressionNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "subscript_expression" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [subscript_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let dereferenceable: Box<SubscriptExpressionDereferenceable> =
            Result::from(node.parse_child("dereferenceable", source).into())?;
        let index: Option<_ExpressionNode> =
            Result::from(node.parse_child("index", source).into())?;
        Ok(Self {
            range,
            dereferenceable,
            index,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl SubscriptExpressionNode {
    pub fn kind(&self) -> &'static str {
        "subscript_expression"
    }
}

impl NodeAccess for SubscriptExpressionNode {
    fn brief_desc(&self) -> String {
        "SubscriptExpressionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::SubscriptExpression(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.dereferenceable.as_any());
        if let Some(x) = &self.index {
            child_vec.push(x.as_any());
        }

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
