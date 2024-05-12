use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::array_creation_expression::ArrayCreationExpressionNode;
use crate::autonodes::cast_expression::CastExpressionNode;
use crate::autonodes::class_constant_access_identifier::ClassConstantAccessIdentifierNode;
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
use crate::autonodes::relative_scope::RelativeScopeNode;
use crate::autonodes::scoped_call_expression::ScopedCallExpressionNode;
use crate::autonodes::scoped_property_access_expression::ScopedPropertyAccessExpressionNode;
use crate::autonodes::string::StringNode;
use crate::autonodes::subscript_expression::SubscriptExpressionNode;
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
pub enum ClassConstantAccessExpressionClass {
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
    RelativeScope(Box<RelativeScopeNode>),
    ScopedCallExpression(Box<ScopedCallExpressionNode>),
    ScopedPropertyAccessExpression(Box<ScopedPropertyAccessExpressionNode>),
    String(Box<StringNode>),
    SubscriptExpression(Box<SubscriptExpressionNode>),
    VariableName(Box<VariableNameNode>),
    Extra(ExtraChild),
}

impl NodeParser for ClassConstantAccessExpressionClass {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ClassConstantAccessExpressionClass::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => ClassConstantAccessExpressionClass::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "array_creation_expression" => {
                ClassConstantAccessExpressionClass::ArrayCreationExpression(Box::new(
                    ArrayCreationExpressionNode::parse(node, source)?,
                ))
            }
            "cast_expression" => ClassConstantAccessExpressionClass::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                ClassConstantAccessExpressionClass::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => ClassConstantAccessExpressionClass::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "encapsed_string" => ClassConstantAccessExpressionClass::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => {
                ClassConstantAccessExpressionClass::FunctionCallExpression(Box::new(
                    FunctionCallExpressionNode::parse(node, source)?,
                ))
            }
            "heredoc" => ClassConstantAccessExpressionClass::Heredoc(Box::new(HeredocNode::parse(
                node, source,
            )?)),
            "member_access_expression" => {
                ClassConstantAccessExpressionClass::MemberAccessExpression(Box::new(
                    MemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "member_call_expression" => ClassConstantAccessExpressionClass::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "name" => {
                ClassConstantAccessExpressionClass::Name(Box::new(NameNode::parse(node, source)?))
            }
            "nowdoc" => ClassConstantAccessExpressionClass::Nowdoc(Box::new(NowdocNode::parse(
                node, source,
            )?)),
            "nullsafe_member_access_expression" => {
                ClassConstantAccessExpressionClass::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                ClassConstantAccessExpressionClass::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => {
                ClassConstantAccessExpressionClass::ParenthesizedExpression(Box::new(
                    ParenthesizedExpressionNode::parse(node, source)?,
                ))
            }
            "qualified_name" => ClassConstantAccessExpressionClass::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "relative_scope" => ClassConstantAccessExpressionClass::RelativeScope(Box::new(
                RelativeScopeNode::parse(node, source)?,
            )),
            "scoped_call_expression" => ClassConstantAccessExpressionClass::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "scoped_property_access_expression" => {
                ClassConstantAccessExpressionClass::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "string" => ClassConstantAccessExpressionClass::String(Box::new(StringNode::parse(
                node, source,
            )?)),
            "subscript_expression" => ClassConstantAccessExpressionClass::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => ClassConstantAccessExpressionClass::VariableName(Box::new(
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

impl ClassConstantAccessExpressionClass {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ClassConstantAccessExpressionClass::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => ClassConstantAccessExpressionClass::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "array_creation_expression" => {
                ClassConstantAccessExpressionClass::ArrayCreationExpression(Box::new(
                    ArrayCreationExpressionNode::parse(node, source)?,
                ))
            }
            "cast_expression" => ClassConstantAccessExpressionClass::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                ClassConstantAccessExpressionClass::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => ClassConstantAccessExpressionClass::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "encapsed_string" => ClassConstantAccessExpressionClass::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => {
                ClassConstantAccessExpressionClass::FunctionCallExpression(Box::new(
                    FunctionCallExpressionNode::parse(node, source)?,
                ))
            }
            "heredoc" => ClassConstantAccessExpressionClass::Heredoc(Box::new(HeredocNode::parse(
                node, source,
            )?)),
            "member_access_expression" => {
                ClassConstantAccessExpressionClass::MemberAccessExpression(Box::new(
                    MemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "member_call_expression" => ClassConstantAccessExpressionClass::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "name" => {
                ClassConstantAccessExpressionClass::Name(Box::new(NameNode::parse(node, source)?))
            }
            "nowdoc" => ClassConstantAccessExpressionClass::Nowdoc(Box::new(NowdocNode::parse(
                node, source,
            )?)),
            "nullsafe_member_access_expression" => {
                ClassConstantAccessExpressionClass::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                ClassConstantAccessExpressionClass::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => {
                ClassConstantAccessExpressionClass::ParenthesizedExpression(Box::new(
                    ParenthesizedExpressionNode::parse(node, source)?,
                ))
            }
            "qualified_name" => ClassConstantAccessExpressionClass::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "relative_scope" => ClassConstantAccessExpressionClass::RelativeScope(Box::new(
                RelativeScopeNode::parse(node, source)?,
            )),
            "scoped_call_expression" => ClassConstantAccessExpressionClass::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "scoped_property_access_expression" => {
                ClassConstantAccessExpressionClass::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "string" => ClassConstantAccessExpressionClass::String(Box::new(StringNode::parse(
                node, source,
            )?)),
            "subscript_expression" => ClassConstantAccessExpressionClass::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => ClassConstantAccessExpressionClass::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            ClassConstantAccessExpressionClass::Extra(y) => y.kind(),
            ClassConstantAccessExpressionClass::ArrayCreationExpression(y) => y.kind(),
            ClassConstantAccessExpressionClass::CastExpression(y) => y.kind(),
            ClassConstantAccessExpressionClass::ClassConstantAccessExpression(y) => y.kind(),
            ClassConstantAccessExpressionClass::DynamicVariableName(y) => y.kind(),
            ClassConstantAccessExpressionClass::EncapsedString(y) => y.kind(),
            ClassConstantAccessExpressionClass::FunctionCallExpression(y) => y.kind(),
            ClassConstantAccessExpressionClass::Heredoc(y) => y.kind(),
            ClassConstantAccessExpressionClass::MemberAccessExpression(y) => y.kind(),
            ClassConstantAccessExpressionClass::MemberCallExpression(y) => y.kind(),
            ClassConstantAccessExpressionClass::Name(y) => y.kind(),
            ClassConstantAccessExpressionClass::Nowdoc(y) => y.kind(),
            ClassConstantAccessExpressionClass::NullsafeMemberAccessExpression(y) => y.kind(),
            ClassConstantAccessExpressionClass::NullsafeMemberCallExpression(y) => y.kind(),
            ClassConstantAccessExpressionClass::ParenthesizedExpression(y) => y.kind(),
            ClassConstantAccessExpressionClass::QualifiedName(y) => y.kind(),
            ClassConstantAccessExpressionClass::RelativeScope(y) => y.kind(),
            ClassConstantAccessExpressionClass::ScopedCallExpression(y) => y.kind(),
            ClassConstantAccessExpressionClass::ScopedPropertyAccessExpression(y) => y.kind(),
            ClassConstantAccessExpressionClass::String(y) => y.kind(),
            ClassConstantAccessExpressionClass::SubscriptExpression(y) => y.kind(),
            ClassConstantAccessExpressionClass::VariableName(y) => y.kind(),
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
            ClassConstantAccessExpressionClass::Extra(x) => x.get_utype(state, emitter),
            ClassConstantAccessExpressionClass::ArrayCreationExpression(x) => {
                x.get_utype(state, emitter)
            }
            ClassConstantAccessExpressionClass::CastExpression(x) => x.get_utype(state, emitter),
            ClassConstantAccessExpressionClass::ClassConstantAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            ClassConstantAccessExpressionClass::DynamicVariableName(x) => {
                x.get_utype(state, emitter)
            }
            ClassConstantAccessExpressionClass::EncapsedString(x) => x.get_utype(state, emitter),
            ClassConstantAccessExpressionClass::FunctionCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            ClassConstantAccessExpressionClass::Heredoc(x) => x.get_utype(state, emitter),
            ClassConstantAccessExpressionClass::MemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            ClassConstantAccessExpressionClass::MemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            ClassConstantAccessExpressionClass::Name(x) => x.get_utype(state, emitter),
            ClassConstantAccessExpressionClass::Nowdoc(x) => x.get_utype(state, emitter),
            ClassConstantAccessExpressionClass::NullsafeMemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            ClassConstantAccessExpressionClass::NullsafeMemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            ClassConstantAccessExpressionClass::ParenthesizedExpression(x) => {
                x.get_utype(state, emitter)
            }
            ClassConstantAccessExpressionClass::QualifiedName(x) => x.get_utype(state, emitter),
            ClassConstantAccessExpressionClass::RelativeScope(x) => x.get_utype(state, emitter),
            ClassConstantAccessExpressionClass::ScopedCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            ClassConstantAccessExpressionClass::ScopedPropertyAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            ClassConstantAccessExpressionClass::String(x) => x.get_utype(state, emitter),
            ClassConstantAccessExpressionClass::SubscriptExpression(x) => {
                x.get_utype(state, emitter)
            }
            ClassConstantAccessExpressionClass::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ClassConstantAccessExpressionClass::Extra(x) => x.get_php_value(state, emitter),
            ClassConstantAccessExpressionClass::ArrayCreationExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ClassConstantAccessExpressionClass::CastExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ClassConstantAccessExpressionClass::ClassConstantAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ClassConstantAccessExpressionClass::DynamicVariableName(x) => {
                x.get_php_value(state, emitter)
            }
            ClassConstantAccessExpressionClass::EncapsedString(x) => {
                x.get_php_value(state, emitter)
            }
            ClassConstantAccessExpressionClass::FunctionCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ClassConstantAccessExpressionClass::Heredoc(x) => x.get_php_value(state, emitter),
            ClassConstantAccessExpressionClass::MemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ClassConstantAccessExpressionClass::MemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ClassConstantAccessExpressionClass::Name(x) => x.get_php_value(state, emitter),
            ClassConstantAccessExpressionClass::Nowdoc(x) => x.get_php_value(state, emitter),
            ClassConstantAccessExpressionClass::NullsafeMemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ClassConstantAccessExpressionClass::NullsafeMemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ClassConstantAccessExpressionClass::ParenthesizedExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ClassConstantAccessExpressionClass::QualifiedName(x) => x.get_php_value(state, emitter),
            ClassConstantAccessExpressionClass::RelativeScope(x) => x.get_php_value(state, emitter),
            ClassConstantAccessExpressionClass::ScopedCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ClassConstantAccessExpressionClass::ScopedPropertyAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ClassConstantAccessExpressionClass::String(x) => x.get_php_value(state, emitter),
            ClassConstantAccessExpressionClass::SubscriptExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ClassConstantAccessExpressionClass::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ClassConstantAccessExpressionClass::Extra(x) => x.read_from(state, emitter),
            ClassConstantAccessExpressionClass::ArrayCreationExpression(x) => {
                x.read_from(state, emitter)
            }
            ClassConstantAccessExpressionClass::CastExpression(x) => x.read_from(state, emitter),
            ClassConstantAccessExpressionClass::ClassConstantAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            ClassConstantAccessExpressionClass::DynamicVariableName(x) => {
                x.read_from(state, emitter)
            }
            ClassConstantAccessExpressionClass::EncapsedString(x) => x.read_from(state, emitter),
            ClassConstantAccessExpressionClass::FunctionCallExpression(x) => {
                x.read_from(state, emitter)
            }
            ClassConstantAccessExpressionClass::Heredoc(x) => x.read_from(state, emitter),
            ClassConstantAccessExpressionClass::MemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            ClassConstantAccessExpressionClass::MemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            ClassConstantAccessExpressionClass::Name(x) => x.read_from(state, emitter),
            ClassConstantAccessExpressionClass::Nowdoc(x) => x.read_from(state, emitter),
            ClassConstantAccessExpressionClass::NullsafeMemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            ClassConstantAccessExpressionClass::NullsafeMemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            ClassConstantAccessExpressionClass::ParenthesizedExpression(x) => {
                x.read_from(state, emitter)
            }
            ClassConstantAccessExpressionClass::QualifiedName(x) => x.read_from(state, emitter),
            ClassConstantAccessExpressionClass::RelativeScope(x) => x.read_from(state, emitter),
            ClassConstantAccessExpressionClass::ScopedCallExpression(x) => {
                x.read_from(state, emitter)
            }
            ClassConstantAccessExpressionClass::ScopedPropertyAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            ClassConstantAccessExpressionClass::String(x) => x.read_from(state, emitter),
            ClassConstantAccessExpressionClass::SubscriptExpression(x) => {
                x.read_from(state, emitter)
            }
            ClassConstantAccessExpressionClass::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ClassConstantAccessExpressionClass {
    fn brief_desc(&self) -> String {
        match self {
            ClassConstantAccessExpressionClass::Extra(x) => format!(
                "ClassConstantAccessExpressionClass::extra({})",
                x.brief_desc()
            ),
            ClassConstantAccessExpressionClass::ArrayCreationExpression(x) => format!(
                "ClassConstantAccessExpressionClass::array_creation_expression({})",
                x.brief_desc()
            ),
            ClassConstantAccessExpressionClass::CastExpression(x) => format!(
                "ClassConstantAccessExpressionClass::cast_expression({})",
                x.brief_desc()
            ),
            ClassConstantAccessExpressionClass::ClassConstantAccessExpression(x) => format!(
                "ClassConstantAccessExpressionClass::class_constant_access_expression({})",
                x.brief_desc()
            ),
            ClassConstantAccessExpressionClass::DynamicVariableName(x) => format!(
                "ClassConstantAccessExpressionClass::dynamic_variable_name({})",
                x.brief_desc()
            ),
            ClassConstantAccessExpressionClass::EncapsedString(x) => format!(
                "ClassConstantAccessExpressionClass::encapsed_string({})",
                x.brief_desc()
            ),
            ClassConstantAccessExpressionClass::FunctionCallExpression(x) => format!(
                "ClassConstantAccessExpressionClass::function_call_expression({})",
                x.brief_desc()
            ),
            ClassConstantAccessExpressionClass::Heredoc(x) => format!(
                "ClassConstantAccessExpressionClass::heredoc({})",
                x.brief_desc()
            ),
            ClassConstantAccessExpressionClass::MemberAccessExpression(x) => format!(
                "ClassConstantAccessExpressionClass::member_access_expression({})",
                x.brief_desc()
            ),
            ClassConstantAccessExpressionClass::MemberCallExpression(x) => format!(
                "ClassConstantAccessExpressionClass::member_call_expression({})",
                x.brief_desc()
            ),
            ClassConstantAccessExpressionClass::Name(x) => format!(
                "ClassConstantAccessExpressionClass::name({})",
                x.brief_desc()
            ),
            ClassConstantAccessExpressionClass::Nowdoc(x) => format!(
                "ClassConstantAccessExpressionClass::nowdoc({})",
                x.brief_desc()
            ),
            ClassConstantAccessExpressionClass::NullsafeMemberAccessExpression(x) => format!(
                "ClassConstantAccessExpressionClass::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            ClassConstantAccessExpressionClass::NullsafeMemberCallExpression(x) => format!(
                "ClassConstantAccessExpressionClass::nullsafe_member_call_expression({})",
                x.brief_desc()
            ),
            ClassConstantAccessExpressionClass::ParenthesizedExpression(x) => format!(
                "ClassConstantAccessExpressionClass::parenthesized_expression({})",
                x.brief_desc()
            ),
            ClassConstantAccessExpressionClass::QualifiedName(x) => format!(
                "ClassConstantAccessExpressionClass::qualified_name({})",
                x.brief_desc()
            ),
            ClassConstantAccessExpressionClass::RelativeScope(x) => format!(
                "ClassConstantAccessExpressionClass::relative_scope({})",
                x.brief_desc()
            ),
            ClassConstantAccessExpressionClass::ScopedCallExpression(x) => format!(
                "ClassConstantAccessExpressionClass::scoped_call_expression({})",
                x.brief_desc()
            ),
            ClassConstantAccessExpressionClass::ScopedPropertyAccessExpression(x) => format!(
                "ClassConstantAccessExpressionClass::scoped_property_access_expression({})",
                x.brief_desc()
            ),
            ClassConstantAccessExpressionClass::String(x) => format!(
                "ClassConstantAccessExpressionClass::string({})",
                x.brief_desc()
            ),
            ClassConstantAccessExpressionClass::SubscriptExpression(x) => format!(
                "ClassConstantAccessExpressionClass::subscript_expression({})",
                x.brief_desc()
            ),
            ClassConstantAccessExpressionClass::VariableName(x) => format!(
                "ClassConstantAccessExpressionClass::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ClassConstantAccessExpressionClass::Extra(x) => x.as_any(),
            ClassConstantAccessExpressionClass::ArrayCreationExpression(x) => x.as_any(),
            ClassConstantAccessExpressionClass::CastExpression(x) => x.as_any(),
            ClassConstantAccessExpressionClass::ClassConstantAccessExpression(x) => x.as_any(),
            ClassConstantAccessExpressionClass::DynamicVariableName(x) => x.as_any(),
            ClassConstantAccessExpressionClass::EncapsedString(x) => x.as_any(),
            ClassConstantAccessExpressionClass::FunctionCallExpression(x) => x.as_any(),
            ClassConstantAccessExpressionClass::Heredoc(x) => x.as_any(),
            ClassConstantAccessExpressionClass::MemberAccessExpression(x) => x.as_any(),
            ClassConstantAccessExpressionClass::MemberCallExpression(x) => x.as_any(),
            ClassConstantAccessExpressionClass::Name(x) => x.as_any(),
            ClassConstantAccessExpressionClass::Nowdoc(x) => x.as_any(),
            ClassConstantAccessExpressionClass::NullsafeMemberAccessExpression(x) => x.as_any(),
            ClassConstantAccessExpressionClass::NullsafeMemberCallExpression(x) => x.as_any(),
            ClassConstantAccessExpressionClass::ParenthesizedExpression(x) => x.as_any(),
            ClassConstantAccessExpressionClass::QualifiedName(x) => x.as_any(),
            ClassConstantAccessExpressionClass::RelativeScope(x) => x.as_any(),
            ClassConstantAccessExpressionClass::ScopedCallExpression(x) => x.as_any(),
            ClassConstantAccessExpressionClass::ScopedPropertyAccessExpression(x) => x.as_any(),
            ClassConstantAccessExpressionClass::String(x) => x.as_any(),
            ClassConstantAccessExpressionClass::SubscriptExpression(x) => x.as_any(),
            ClassConstantAccessExpressionClass::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ClassConstantAccessExpressionClass::Extra(x) => x.children_any(),
            ClassConstantAccessExpressionClass::ArrayCreationExpression(x) => x.children_any(),
            ClassConstantAccessExpressionClass::CastExpression(x) => x.children_any(),
            ClassConstantAccessExpressionClass::ClassConstantAccessExpression(x) => {
                x.children_any()
            }
            ClassConstantAccessExpressionClass::DynamicVariableName(x) => x.children_any(),
            ClassConstantAccessExpressionClass::EncapsedString(x) => x.children_any(),
            ClassConstantAccessExpressionClass::FunctionCallExpression(x) => x.children_any(),
            ClassConstantAccessExpressionClass::Heredoc(x) => x.children_any(),
            ClassConstantAccessExpressionClass::MemberAccessExpression(x) => x.children_any(),
            ClassConstantAccessExpressionClass::MemberCallExpression(x) => x.children_any(),
            ClassConstantAccessExpressionClass::Name(x) => x.children_any(),
            ClassConstantAccessExpressionClass::Nowdoc(x) => x.children_any(),
            ClassConstantAccessExpressionClass::NullsafeMemberAccessExpression(x) => {
                x.children_any()
            }
            ClassConstantAccessExpressionClass::NullsafeMemberCallExpression(x) => x.children_any(),
            ClassConstantAccessExpressionClass::ParenthesizedExpression(x) => x.children_any(),
            ClassConstantAccessExpressionClass::QualifiedName(x) => x.children_any(),
            ClassConstantAccessExpressionClass::RelativeScope(x) => x.children_any(),
            ClassConstantAccessExpressionClass::ScopedCallExpression(x) => x.children_any(),
            ClassConstantAccessExpressionClass::ScopedPropertyAccessExpression(x) => {
                x.children_any()
            }
            ClassConstantAccessExpressionClass::String(x) => x.children_any(),
            ClassConstantAccessExpressionClass::SubscriptExpression(x) => x.children_any(),
            ClassConstantAccessExpressionClass::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ClassConstantAccessExpressionClass::Extra(x) => x.range(),
            ClassConstantAccessExpressionClass::ArrayCreationExpression(x) => x.range(),
            ClassConstantAccessExpressionClass::CastExpression(x) => x.range(),
            ClassConstantAccessExpressionClass::ClassConstantAccessExpression(x) => x.range(),
            ClassConstantAccessExpressionClass::DynamicVariableName(x) => x.range(),
            ClassConstantAccessExpressionClass::EncapsedString(x) => x.range(),
            ClassConstantAccessExpressionClass::FunctionCallExpression(x) => x.range(),
            ClassConstantAccessExpressionClass::Heredoc(x) => x.range(),
            ClassConstantAccessExpressionClass::MemberAccessExpression(x) => x.range(),
            ClassConstantAccessExpressionClass::MemberCallExpression(x) => x.range(),
            ClassConstantAccessExpressionClass::Name(x) => x.range(),
            ClassConstantAccessExpressionClass::Nowdoc(x) => x.range(),
            ClassConstantAccessExpressionClass::NullsafeMemberAccessExpression(x) => x.range(),
            ClassConstantAccessExpressionClass::NullsafeMemberCallExpression(x) => x.range(),
            ClassConstantAccessExpressionClass::ParenthesizedExpression(x) => x.range(),
            ClassConstantAccessExpressionClass::QualifiedName(x) => x.range(),
            ClassConstantAccessExpressionClass::RelativeScope(x) => x.range(),
            ClassConstantAccessExpressionClass::ScopedCallExpression(x) => x.range(),
            ClassConstantAccessExpressionClass::ScopedPropertyAccessExpression(x) => x.range(),
            ClassConstantAccessExpressionClass::String(x) => x.range(),
            ClassConstantAccessExpressionClass::SubscriptExpression(x) => x.range(),
            ClassConstantAccessExpressionClass::VariableName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClassConstantAccessExpressionNode {
    pub range: Range,
    pub class: Box<ClassConstantAccessExpressionClass>,
    pub constant: ClassConstantAccessIdentifierNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for ClassConstantAccessExpressionNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "class_constant_access_expression" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [class_constant_access_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let class: Box<ClassConstantAccessExpressionClass> =
            Result::from(node.parse_child("class", source).into())?;
        let constant: ClassConstantAccessIdentifierNode =
            Result::from(node.parse_child("constant", source).into())?;
        Ok(Self {
            range,
            class,
            constant,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl ClassConstantAccessExpressionNode {
    pub fn kind(&self) -> &'static str {
        "class_constant_access_expression"
    }
}

impl NodeAccess for ClassConstantAccessExpressionNode {
    fn brief_desc(&self) -> String {
        "ClassConstantAccessExpressionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::ClassConstantAccessExpression(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.class.as_any());
        child_vec.push(self.constant.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
