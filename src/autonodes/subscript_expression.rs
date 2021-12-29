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
use crate::autonodes::integer::IntegerNode;
use crate::autonodes::member_access_expression::MemberAccessExpressionNode;
use crate::autonodes::member_call_expression::MemberCallExpressionNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::nullsafe_member_access_expression::NullsafeMemberAccessExpressionNode;
use crate::autonodes::nullsafe_member_call_expression::NullsafeMemberCallExpressionNode;
use crate::autonodes::parenthesized_expression::ParenthesizedExpressionNode;
use crate::autonodes::qualified_name::QualifiedNameNode;
use crate::autonodes::scoped_call_expression::ScopedCallExpressionNode;
use crate::autonodes::scoped_property_access_expression::ScopedPropertyAccessExpressionNode;
use crate::autonodes::string::StringNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::unary_op_expression::UnaryOpExpressionNode;
use crate::autonodes::variable_name::VariableNameNode;
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
pub enum SubscriptExpressionDereferencable {
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
    NullsafeMemberAccessExpression(Box<NullsafeMemberAccessExpressionNode>),
    NullsafeMemberCallExpression(Box<NullsafeMemberCallExpressionNode>),
    ParenthesizedExpression(Box<ParenthesizedExpressionNode>),
    QualifiedName(Box<QualifiedNameNode>),
    ScopedCallExpression(Box<ScopedCallExpressionNode>),
    ScopedPropertyAccessExpression(Box<ScopedPropertyAccessExpressionNode>),
    String(Box<StringNode>),
    SubscriptExpression(Box<SubscriptExpressionNode>),
    VariableName(Box<VariableNameNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl SubscriptExpressionDereferencable {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => SubscriptExpressionDereferencable::Comment(Box::new(CommentNode::parse(
                node, source,
            )?)),
            "text_interpolation" => SubscriptExpressionDereferencable::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => {
                SubscriptExpressionDereferencable::Error(Box::new(ErrorNode::parse(node, source)?))
            }
            "array_creation_expression" => {
                SubscriptExpressionDereferencable::ArrayCreationExpression(Box::new(
                    ArrayCreationExpressionNode::parse(node, source)?,
                ))
            }
            "cast_expression" => SubscriptExpressionDereferencable::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                SubscriptExpressionDereferencable::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => SubscriptExpressionDereferencable::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "encapsed_string" => SubscriptExpressionDereferencable::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => {
                SubscriptExpressionDereferencable::FunctionCallExpression(Box::new(
                    FunctionCallExpressionNode::parse(node, source)?,
                ))
            }
            "heredoc" => SubscriptExpressionDereferencable::Heredoc(Box::new(HeredocNode::parse(
                node, source,
            )?)),
            "member_access_expression" => {
                SubscriptExpressionDereferencable::MemberAccessExpression(Box::new(
                    MemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "member_call_expression" => SubscriptExpressionDereferencable::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "name" => {
                SubscriptExpressionDereferencable::Name(Box::new(NameNode::parse(node, source)?))
            }
            "nullsafe_member_access_expression" => {
                SubscriptExpressionDereferencable::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                SubscriptExpressionDereferencable::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => {
                SubscriptExpressionDereferencable::ParenthesizedExpression(Box::new(
                    ParenthesizedExpressionNode::parse(node, source)?,
                ))
            }
            "qualified_name" => SubscriptExpressionDereferencable::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_call_expression" => SubscriptExpressionDereferencable::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "scoped_property_access_expression" => {
                SubscriptExpressionDereferencable::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "string" => SubscriptExpressionDereferencable::String(Box::new(StringNode::parse(
                node, source,
            )?)),
            "subscript_expression" => SubscriptExpressionDereferencable::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => SubscriptExpressionDereferencable::VariableName(Box::new(
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

    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => SubscriptExpressionDereferencable::Comment(Box::new(CommentNode::parse(
                node, source,
            )?)),
            "text_interpolation" => SubscriptExpressionDereferencable::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => {
                SubscriptExpressionDereferencable::Error(Box::new(ErrorNode::parse(node, source)?))
            }
            "array_creation_expression" => {
                SubscriptExpressionDereferencable::ArrayCreationExpression(Box::new(
                    ArrayCreationExpressionNode::parse(node, source)?,
                ))
            }
            "cast_expression" => SubscriptExpressionDereferencable::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                SubscriptExpressionDereferencable::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => SubscriptExpressionDereferencable::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "encapsed_string" => SubscriptExpressionDereferencable::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => {
                SubscriptExpressionDereferencable::FunctionCallExpression(Box::new(
                    FunctionCallExpressionNode::parse(node, source)?,
                ))
            }
            "heredoc" => SubscriptExpressionDereferencable::Heredoc(Box::new(HeredocNode::parse(
                node, source,
            )?)),
            "member_access_expression" => {
                SubscriptExpressionDereferencable::MemberAccessExpression(Box::new(
                    MemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "member_call_expression" => SubscriptExpressionDereferencable::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "name" => {
                SubscriptExpressionDereferencable::Name(Box::new(NameNode::parse(node, source)?))
            }
            "nullsafe_member_access_expression" => {
                SubscriptExpressionDereferencable::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                SubscriptExpressionDereferencable::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => {
                SubscriptExpressionDereferencable::ParenthesizedExpression(Box::new(
                    ParenthesizedExpressionNode::parse(node, source)?,
                ))
            }
            "qualified_name" => SubscriptExpressionDereferencable::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_call_expression" => SubscriptExpressionDereferencable::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "scoped_property_access_expression" => {
                SubscriptExpressionDereferencable::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "string" => SubscriptExpressionDereferencable::String(Box::new(StringNode::parse(
                node, source,
            )?)),
            "subscript_expression" => SubscriptExpressionDereferencable::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => SubscriptExpressionDereferencable::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

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
            SubscriptExpressionDereferencable::Comment(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferencable::TextInterpolation(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferencable::Error(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferencable::ArrayCreationExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferencable::CastExpression(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferencable::ClassConstantAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferencable::DynamicVariableName(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferencable::EncapsedString(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferencable::FunctionCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferencable::Heredoc(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferencable::MemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferencable::MemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferencable::Name(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferencable::NullsafeMemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferencable::NullsafeMemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferencable::ParenthesizedExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferencable::QualifiedName(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferencable::ScopedCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferencable::ScopedPropertyAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferencable::String(x) => x.get_utype(state, emitter),
            SubscriptExpressionDereferencable::SubscriptExpression(x) => {
                x.get_utype(state, emitter)
            }
            SubscriptExpressionDereferencable::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            SubscriptExpressionDereferencable::Comment(x) => x.get_php_value(state, emitter),
            SubscriptExpressionDereferencable::TextInterpolation(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferencable::Error(x) => x.get_php_value(state, emitter),
            SubscriptExpressionDereferencable::ArrayCreationExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferencable::CastExpression(x) => x.get_php_value(state, emitter),
            SubscriptExpressionDereferencable::ClassConstantAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferencable::DynamicVariableName(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferencable::EncapsedString(x) => x.get_php_value(state, emitter),
            SubscriptExpressionDereferencable::FunctionCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferencable::Heredoc(x) => x.get_php_value(state, emitter),
            SubscriptExpressionDereferencable::MemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferencable::MemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferencable::Name(x) => x.get_php_value(state, emitter),
            SubscriptExpressionDereferencable::NullsafeMemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferencable::NullsafeMemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferencable::ParenthesizedExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferencable::QualifiedName(x) => x.get_php_value(state, emitter),
            SubscriptExpressionDereferencable::ScopedCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferencable::ScopedPropertyAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferencable::String(x) => x.get_php_value(state, emitter),
            SubscriptExpressionDereferencable::SubscriptExpression(x) => {
                x.get_php_value(state, emitter)
            }
            SubscriptExpressionDereferencable::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            SubscriptExpressionDereferencable::Comment(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferencable::TextInterpolation(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferencable::Error(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferencable::ArrayCreationExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferencable::CastExpression(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferencable::ClassConstantAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferencable::DynamicVariableName(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferencable::EncapsedString(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferencable::FunctionCallExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferencable::Heredoc(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferencable::MemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferencable::MemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferencable::Name(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferencable::NullsafeMemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferencable::NullsafeMemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferencable::ParenthesizedExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferencable::QualifiedName(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferencable::ScopedCallExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferencable::ScopedPropertyAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferencable::String(x) => x.read_from(state, emitter),
            SubscriptExpressionDereferencable::SubscriptExpression(x) => {
                x.read_from(state, emitter)
            }
            SubscriptExpressionDereferencable::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for SubscriptExpressionDereferencable {
    fn brief_desc(&self) -> String {
        match self {
            SubscriptExpressionDereferencable::Comment(x) => format!(
                "SubscriptExpressionDereferencable::comment({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferencable::TextInterpolation(x) => format!(
                "SubscriptExpressionDereferencable::text_interpolation({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferencable::Error(x) => format!(
                "SubscriptExpressionDereferencable::ERROR({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferencable::ArrayCreationExpression(x) => format!(
                "SubscriptExpressionDereferencable::array_creation_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferencable::CastExpression(x) => format!(
                "SubscriptExpressionDereferencable::cast_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferencable::ClassConstantAccessExpression(x) => format!(
                "SubscriptExpressionDereferencable::class_constant_access_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferencable::DynamicVariableName(x) => format!(
                "SubscriptExpressionDereferencable::dynamic_variable_name({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferencable::EncapsedString(x) => format!(
                "SubscriptExpressionDereferencable::encapsed_string({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferencable::FunctionCallExpression(x) => format!(
                "SubscriptExpressionDereferencable::function_call_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferencable::Heredoc(x) => format!(
                "SubscriptExpressionDereferencable::heredoc({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferencable::MemberAccessExpression(x) => format!(
                "SubscriptExpressionDereferencable::member_access_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferencable::MemberCallExpression(x) => format!(
                "SubscriptExpressionDereferencable::member_call_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferencable::Name(x) => format!(
                "SubscriptExpressionDereferencable::name({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferencable::NullsafeMemberAccessExpression(x) => format!(
                "SubscriptExpressionDereferencable::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferencable::NullsafeMemberCallExpression(x) => format!(
                "SubscriptExpressionDereferencable::nullsafe_member_call_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferencable::ParenthesizedExpression(x) => format!(
                "SubscriptExpressionDereferencable::parenthesized_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferencable::QualifiedName(x) => format!(
                "SubscriptExpressionDereferencable::qualified_name({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferencable::ScopedCallExpression(x) => format!(
                "SubscriptExpressionDereferencable::scoped_call_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferencable::ScopedPropertyAccessExpression(x) => format!(
                "SubscriptExpressionDereferencable::scoped_property_access_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferencable::String(x) => format!(
                "SubscriptExpressionDereferencable::string({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferencable::SubscriptExpression(x) => format!(
                "SubscriptExpressionDereferencable::subscript_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionDereferencable::VariableName(x) => format!(
                "SubscriptExpressionDereferencable::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            SubscriptExpressionDereferencable::Comment(x) => x.as_any(),
            SubscriptExpressionDereferencable::TextInterpolation(x) => x.as_any(),
            SubscriptExpressionDereferencable::Error(x) => x.as_any(),
            SubscriptExpressionDereferencable::ArrayCreationExpression(x) => x.as_any(),
            SubscriptExpressionDereferencable::CastExpression(x) => x.as_any(),
            SubscriptExpressionDereferencable::ClassConstantAccessExpression(x) => x.as_any(),
            SubscriptExpressionDereferencable::DynamicVariableName(x) => x.as_any(),
            SubscriptExpressionDereferencable::EncapsedString(x) => x.as_any(),
            SubscriptExpressionDereferencable::FunctionCallExpression(x) => x.as_any(),
            SubscriptExpressionDereferencable::Heredoc(x) => x.as_any(),
            SubscriptExpressionDereferencable::MemberAccessExpression(x) => x.as_any(),
            SubscriptExpressionDereferencable::MemberCallExpression(x) => x.as_any(),
            SubscriptExpressionDereferencable::Name(x) => x.as_any(),
            SubscriptExpressionDereferencable::NullsafeMemberAccessExpression(x) => x.as_any(),
            SubscriptExpressionDereferencable::NullsafeMemberCallExpression(x) => x.as_any(),
            SubscriptExpressionDereferencable::ParenthesizedExpression(x) => x.as_any(),
            SubscriptExpressionDereferencable::QualifiedName(x) => x.as_any(),
            SubscriptExpressionDereferencable::ScopedCallExpression(x) => x.as_any(),
            SubscriptExpressionDereferencable::ScopedPropertyAccessExpression(x) => x.as_any(),
            SubscriptExpressionDereferencable::String(x) => x.as_any(),
            SubscriptExpressionDereferencable::SubscriptExpression(x) => x.as_any(),
            SubscriptExpressionDereferencable::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            SubscriptExpressionDereferencable::Comment(x) => x.children_any(),
            SubscriptExpressionDereferencable::TextInterpolation(x) => x.children_any(),
            SubscriptExpressionDereferencable::Error(x) => x.children_any(),
            SubscriptExpressionDereferencable::ArrayCreationExpression(x) => x.children_any(),
            SubscriptExpressionDereferencable::CastExpression(x) => x.children_any(),
            SubscriptExpressionDereferencable::ClassConstantAccessExpression(x) => x.children_any(),
            SubscriptExpressionDereferencable::DynamicVariableName(x) => x.children_any(),
            SubscriptExpressionDereferencable::EncapsedString(x) => x.children_any(),
            SubscriptExpressionDereferencable::FunctionCallExpression(x) => x.children_any(),
            SubscriptExpressionDereferencable::Heredoc(x) => x.children_any(),
            SubscriptExpressionDereferencable::MemberAccessExpression(x) => x.children_any(),
            SubscriptExpressionDereferencable::MemberCallExpression(x) => x.children_any(),
            SubscriptExpressionDereferencable::Name(x) => x.children_any(),
            SubscriptExpressionDereferencable::NullsafeMemberAccessExpression(x) => {
                x.children_any()
            }
            SubscriptExpressionDereferencable::NullsafeMemberCallExpression(x) => x.children_any(),
            SubscriptExpressionDereferencable::ParenthesizedExpression(x) => x.children_any(),
            SubscriptExpressionDereferencable::QualifiedName(x) => x.children_any(),
            SubscriptExpressionDereferencable::ScopedCallExpression(x) => x.children_any(),
            SubscriptExpressionDereferencable::ScopedPropertyAccessExpression(x) => {
                x.children_any()
            }
            SubscriptExpressionDereferencable::String(x) => x.children_any(),
            SubscriptExpressionDereferencable::SubscriptExpression(x) => x.children_any(),
            SubscriptExpressionDereferencable::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            SubscriptExpressionDereferencable::Comment(x) => x.range(),
            SubscriptExpressionDereferencable::TextInterpolation(x) => x.range(),
            SubscriptExpressionDereferencable::Error(x) => x.range(),
            SubscriptExpressionDereferencable::ArrayCreationExpression(x) => x.range(),
            SubscriptExpressionDereferencable::CastExpression(x) => x.range(),
            SubscriptExpressionDereferencable::ClassConstantAccessExpression(x) => x.range(),
            SubscriptExpressionDereferencable::DynamicVariableName(x) => x.range(),
            SubscriptExpressionDereferencable::EncapsedString(x) => x.range(),
            SubscriptExpressionDereferencable::FunctionCallExpression(x) => x.range(),
            SubscriptExpressionDereferencable::Heredoc(x) => x.range(),
            SubscriptExpressionDereferencable::MemberAccessExpression(x) => x.range(),
            SubscriptExpressionDereferencable::MemberCallExpression(x) => x.range(),
            SubscriptExpressionDereferencable::Name(x) => x.range(),
            SubscriptExpressionDereferencable::NullsafeMemberAccessExpression(x) => x.range(),
            SubscriptExpressionDereferencable::NullsafeMemberCallExpression(x) => x.range(),
            SubscriptExpressionDereferencable::ParenthesizedExpression(x) => x.range(),
            SubscriptExpressionDereferencable::QualifiedName(x) => x.range(),
            SubscriptExpressionDereferencable::ScopedCallExpression(x) => x.range(),
            SubscriptExpressionDereferencable::ScopedPropertyAccessExpression(x) => x.range(),
            SubscriptExpressionDereferencable::String(x) => x.range(),
            SubscriptExpressionDereferencable::SubscriptExpression(x) => x.range(),
            SubscriptExpressionDereferencable::VariableName(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SubscriptExpressionChildren {
    Integer(Box<IntegerNode>),
    Name(Box<NameNode>),
    UnaryOpExpression(Box<UnaryOpExpressionNode>),
    VariableName(Box<VariableNameNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl SubscriptExpressionChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => {
                SubscriptExpressionChildren::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => SubscriptExpressionChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => {
                SubscriptExpressionChildren::Error(Box::new(ErrorNode::parse(node, source)?))
            }
            "integer" => {
                SubscriptExpressionChildren::Integer(Box::new(IntegerNode::parse(node, source)?))
            }
            "name" => SubscriptExpressionChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "unary_op_expression" => SubscriptExpressionChildren::UnaryOpExpression(Box::new(
                UnaryOpExpressionNode::parse(node, source)?,
            )),
            "variable_name" => SubscriptExpressionChildren::VariableName(Box::new(
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

    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => {
                SubscriptExpressionChildren::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => SubscriptExpressionChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => {
                SubscriptExpressionChildren::Error(Box::new(ErrorNode::parse(node, source)?))
            }
            "integer" => {
                SubscriptExpressionChildren::Integer(Box::new(IntegerNode::parse(node, source)?))
            }
            "name" => SubscriptExpressionChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "unary_op_expression" => SubscriptExpressionChildren::UnaryOpExpression(Box::new(
                UnaryOpExpressionNode::parse(node, source)?,
            )),
            "variable_name" => SubscriptExpressionChildren::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

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
            SubscriptExpressionChildren::Comment(x) => x.get_utype(state, emitter),
            SubscriptExpressionChildren::TextInterpolation(x) => x.get_utype(state, emitter),
            SubscriptExpressionChildren::Error(x) => x.get_utype(state, emitter),
            SubscriptExpressionChildren::Integer(x) => x.get_utype(state, emitter),
            SubscriptExpressionChildren::Name(x) => x.get_utype(state, emitter),
            SubscriptExpressionChildren::UnaryOpExpression(x) => x.get_utype(state, emitter),
            SubscriptExpressionChildren::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            SubscriptExpressionChildren::Comment(x) => x.get_php_value(state, emitter),
            SubscriptExpressionChildren::TextInterpolation(x) => x.get_php_value(state, emitter),
            SubscriptExpressionChildren::Error(x) => x.get_php_value(state, emitter),
            SubscriptExpressionChildren::Integer(x) => x.get_php_value(state, emitter),
            SubscriptExpressionChildren::Name(x) => x.get_php_value(state, emitter),
            SubscriptExpressionChildren::UnaryOpExpression(x) => x.get_php_value(state, emitter),
            SubscriptExpressionChildren::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            SubscriptExpressionChildren::Comment(x) => x.read_from(state, emitter),
            SubscriptExpressionChildren::TextInterpolation(x) => x.read_from(state, emitter),
            SubscriptExpressionChildren::Error(x) => x.read_from(state, emitter),
            SubscriptExpressionChildren::Integer(x) => x.read_from(state, emitter),
            SubscriptExpressionChildren::Name(x) => x.read_from(state, emitter),
            SubscriptExpressionChildren::UnaryOpExpression(x) => x.read_from(state, emitter),
            SubscriptExpressionChildren::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for SubscriptExpressionChildren {
    fn brief_desc(&self) -> String {
        match self {
            SubscriptExpressionChildren::Comment(x) => {
                format!("SubscriptExpressionChildren::comment({})", x.brief_desc())
            }
            SubscriptExpressionChildren::TextInterpolation(x) => format!(
                "SubscriptExpressionChildren::text_interpolation({})",
                x.brief_desc()
            ),
            SubscriptExpressionChildren::Error(x) => {
                format!("SubscriptExpressionChildren::ERROR({})", x.brief_desc())
            }
            SubscriptExpressionChildren::Integer(x) => {
                format!("SubscriptExpressionChildren::integer({})", x.brief_desc())
            }
            SubscriptExpressionChildren::Name(x) => {
                format!("SubscriptExpressionChildren::name({})", x.brief_desc())
            }
            SubscriptExpressionChildren::UnaryOpExpression(x) => format!(
                "SubscriptExpressionChildren::unary_op_expression({})",
                x.brief_desc()
            ),
            SubscriptExpressionChildren::VariableName(x) => format!(
                "SubscriptExpressionChildren::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            SubscriptExpressionChildren::Comment(x) => x.as_any(),
            SubscriptExpressionChildren::TextInterpolation(x) => x.as_any(),
            SubscriptExpressionChildren::Error(x) => x.as_any(),
            SubscriptExpressionChildren::Integer(x) => x.as_any(),
            SubscriptExpressionChildren::Name(x) => x.as_any(),
            SubscriptExpressionChildren::UnaryOpExpression(x) => x.as_any(),
            SubscriptExpressionChildren::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            SubscriptExpressionChildren::Comment(x) => x.children_any(),
            SubscriptExpressionChildren::TextInterpolation(x) => x.children_any(),
            SubscriptExpressionChildren::Error(x) => x.children_any(),
            SubscriptExpressionChildren::Integer(x) => x.children_any(),
            SubscriptExpressionChildren::Name(x) => x.children_any(),
            SubscriptExpressionChildren::UnaryOpExpression(x) => x.children_any(),
            SubscriptExpressionChildren::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            SubscriptExpressionChildren::Comment(x) => x.range(),
            SubscriptExpressionChildren::TextInterpolation(x) => x.range(),
            SubscriptExpressionChildren::Error(x) => x.range(),
            SubscriptExpressionChildren::Integer(x) => x.range(),
            SubscriptExpressionChildren::Name(x) => x.range(),
            SubscriptExpressionChildren::UnaryOpExpression(x) => x.range(),
            SubscriptExpressionChildren::VariableName(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptExpressionNode {
    pub range: Range,
    pub dereferencable: Option<Box<SubscriptExpressionDereferencable>>,
    pub index: Option<_ExpressionNode>,
    pub children: Vec<Box<SubscriptExpressionChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl SubscriptExpressionNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "subscript_expression" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [subscript_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let mut skip_nodes: Vec<usize> = vec![];
        let dereferencable: Option<Box<SubscriptExpressionDereferencable>> = node
            .children_by_field_name("dereferencable", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode2| SubscriptExpressionDereferencable::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .into();
        let index: Option<_ExpressionNode> = node
            .children_by_field_name("index", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode1| _ExpressionNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        Ok(Self {
            range,
            dereferencable,
            index,
            children: SubscriptExpressionChildren::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| !skip_nodes.contains(&node.id()))
                    .filter(|node| node.kind() != "comment"),
                source,
            )?,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment")
                    .filter(|node| !skip_nodes.contains(&node.id())),
                source,
            )?,
        })
    }

    pub fn parse_vec<'a, I>(children: I, source: &Vec<u8>) -> Result<Vec<Box<Self>>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
    {
        let mut res: Vec<Box<Self>> = vec![];
        for child in children {
            if child.kind() == "comment" {
                continue;
            }
            res.push(Box::new(Self::parse(child, source)?));
        }
        Ok(res)
    }

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
        if let Some(x) = &self.dereferencable {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.index {
            child_vec.push(x.as_any());
        }
        child_vec.extend(self.children.iter().map(|n| n.as_any()));
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
