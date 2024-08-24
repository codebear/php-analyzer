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
use crate::autonodes::nullsafe_member_call_expression::NullsafeMemberCallExpressionNode;
use crate::autonodes::parenthesized_expression::ParenthesizedExpressionNode;
use crate::autonodes::qualified_name::QualifiedNameNode;
use crate::autonodes::scoped_call_expression::ScopedCallExpressionNode;
use crate::autonodes::scoped_property_access_expression::ScopedPropertyAccessExpressionNode;
use crate::autonodes::string::StringNode;
use crate::autonodes::subscript_expression::SubscriptExpressionNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::variable_name::VariableNameNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::parser::Range;
use crate::types::union::PHPType;
use crate::value::PHPValue;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub enum NullsafeMemberAccessExpressionName {
    _Expression(Box<_ExpressionNode>),
    DynamicVariableName(Box<DynamicVariableNameNode>),
    Name(Box<NameNode>),
    VariableName(Box<VariableNameNode>),
    Extra(ExtraChild),
}

impl NodeParser for NullsafeMemberAccessExpressionName {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => NullsafeMemberAccessExpressionName::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                NullsafeMemberAccessExpressionName::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => NullsafeMemberAccessExpressionName::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "dynamic_variable_name" => NullsafeMemberAccessExpressionName::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "name" => {
                NullsafeMemberAccessExpressionName::Name(Box::new(NameNode::parse(node, source)?))
            }
            "variable_name" => NullsafeMemberAccessExpressionName::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(NullsafeMemberAccessExpressionName::_Expression)
                {
                    x
                } else {
                    return Err(ParseError::new(node.range(), format!("NullsafeMemberAccessExpressionName: Parse error, unexpected node-type: {}", node.kind())));
                }
            }
        })
    }
}

impl NullsafeMemberAccessExpressionName {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => NullsafeMemberAccessExpressionName::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                NullsafeMemberAccessExpressionName::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => NullsafeMemberAccessExpressionName::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "dynamic_variable_name" => NullsafeMemberAccessExpressionName::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "name" => {
                NullsafeMemberAccessExpressionName::Name(Box::new(NameNode::parse(node, source)?))
            }
            "variable_name" => NullsafeMemberAccessExpressionName::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                return Ok(_ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(NullsafeMemberAccessExpressionName::_Expression))
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            NullsafeMemberAccessExpressionName::Extra(y) => y.kind(),
            NullsafeMemberAccessExpressionName::_Expression(y) => y.kind(),
            NullsafeMemberAccessExpressionName::DynamicVariableName(y) => y.kind(),
            NullsafeMemberAccessExpressionName::Name(y) => y.kind(),
            NullsafeMemberAccessExpressionName::VariableName(y) => y.kind(),
        }
    }

    pub fn parse_vec<'a, I>(children: I, source: &[u8]) -> Result<Vec<Box<Self>>, ParseError>
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
    ) -> Option<PHPType> {
        match self {
            NullsafeMemberAccessExpressionName::Extra(x) => x.get_utype(state, emitter),
            NullsafeMemberAccessExpressionName::_Expression(x) => x.get_utype(state, emitter),
            NullsafeMemberAccessExpressionName::DynamicVariableName(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberAccessExpressionName::Name(x) => x.get_utype(state, emitter),
            NullsafeMemberAccessExpressionName::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            NullsafeMemberAccessExpressionName::Extra(x) => x.get_php_value(state, emitter),
            NullsafeMemberAccessExpressionName::_Expression(x) => x.get_php_value(state, emitter),
            NullsafeMemberAccessExpressionName::DynamicVariableName(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberAccessExpressionName::Name(x) => x.get_php_value(state, emitter),
            NullsafeMemberAccessExpressionName::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            NullsafeMemberAccessExpressionName::Extra(x) => x.read_from(state, emitter),
            NullsafeMemberAccessExpressionName::_Expression(x) => x.read_from(state, emitter),
            NullsafeMemberAccessExpressionName::DynamicVariableName(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberAccessExpressionName::Name(x) => x.read_from(state, emitter),
            NullsafeMemberAccessExpressionName::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for NullsafeMemberAccessExpressionName {
    fn brief_desc(&self) -> String {
        match self {
            NullsafeMemberAccessExpressionName::Extra(x) => format!(
                "NullsafeMemberAccessExpressionName::extra({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionName::_Expression(x) => format!(
                "NullsafeMemberAccessExpressionName::_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionName::DynamicVariableName(x) => format!(
                "NullsafeMemberAccessExpressionName::dynamic_variable_name({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionName::Name(x) => format!(
                "NullsafeMemberAccessExpressionName::name({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionName::VariableName(x) => format!(
                "NullsafeMemberAccessExpressionName::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            NullsafeMemberAccessExpressionName::Extra(x) => x.as_any(),
            NullsafeMemberAccessExpressionName::_Expression(x) => x.as_any(),
            NullsafeMemberAccessExpressionName::DynamicVariableName(x) => x.as_any(),
            NullsafeMemberAccessExpressionName::Name(x) => x.as_any(),
            NullsafeMemberAccessExpressionName::VariableName(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            NullsafeMemberAccessExpressionName::Extra(x) => x.children_any(),
            NullsafeMemberAccessExpressionName::_Expression(x) => x.children_any(),
            NullsafeMemberAccessExpressionName::DynamicVariableName(x) => x.children_any(),
            NullsafeMemberAccessExpressionName::Name(x) => x.children_any(),
            NullsafeMemberAccessExpressionName::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            NullsafeMemberAccessExpressionName::Extra(x) => x.range(),
            NullsafeMemberAccessExpressionName::_Expression(x) => x.range(),
            NullsafeMemberAccessExpressionName::DynamicVariableName(x) => x.range(),
            NullsafeMemberAccessExpressionName::Name(x) => x.range(),
            NullsafeMemberAccessExpressionName::VariableName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum NullsafeMemberAccessExpressionObject {
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

impl NodeParser for NullsafeMemberAccessExpressionObject {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => NullsafeMemberAccessExpressionObject::Extra(ExtraChild::Comment(
                Box::new(CommentNode::parse(node, source)?),
            )),
            "text_interpolation" => {
                NullsafeMemberAccessExpressionObject::Extra(ExtraChild::TextInterpolation(
                    Box::new(TextInterpolationNode::parse(node, source)?),
                ))
            }
            "ERROR" => NullsafeMemberAccessExpressionObject::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "array_creation_expression" => {
                NullsafeMemberAccessExpressionObject::ArrayCreationExpression(Box::new(
                    ArrayCreationExpressionNode::parse(node, source)?,
                ))
            }
            "cast_expression" => NullsafeMemberAccessExpressionObject::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                NullsafeMemberAccessExpressionObject::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => NullsafeMemberAccessExpressionObject::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "encapsed_string" => NullsafeMemberAccessExpressionObject::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => {
                NullsafeMemberAccessExpressionObject::FunctionCallExpression(Box::new(
                    FunctionCallExpressionNode::parse(node, source)?,
                ))
            }
            "heredoc" => NullsafeMemberAccessExpressionObject::Heredoc(Box::new(
                HeredocNode::parse(node, source)?,
            )),
            "member_access_expression" => {
                NullsafeMemberAccessExpressionObject::MemberAccessExpression(Box::new(
                    MemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "member_call_expression" => NullsafeMemberAccessExpressionObject::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "name" => {
                NullsafeMemberAccessExpressionObject::Name(Box::new(NameNode::parse(node, source)?))
            }
            "nowdoc" => NullsafeMemberAccessExpressionObject::Nowdoc(Box::new(NowdocNode::parse(
                node, source,
            )?)),
            "nullsafe_member_access_expression" => {
                NullsafeMemberAccessExpressionObject::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                NullsafeMemberAccessExpressionObject::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => {
                NullsafeMemberAccessExpressionObject::ParenthesizedExpression(Box::new(
                    ParenthesizedExpressionNode::parse(node, source)?,
                ))
            }
            "qualified_name" => NullsafeMemberAccessExpressionObject::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_call_expression" => NullsafeMemberAccessExpressionObject::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "scoped_property_access_expression" => {
                NullsafeMemberAccessExpressionObject::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "string" => NullsafeMemberAccessExpressionObject::String(Box::new(StringNode::parse(
                node, source,
            )?)),
            "subscript_expression" => NullsafeMemberAccessExpressionObject::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => NullsafeMemberAccessExpressionObject::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!(
                    "NullsafeMemberAccessExpressionObject: Parse error, unexpected node-type: {}",
                    node.kind()
                ),
                ))
            }
        })
    }
}

impl NullsafeMemberAccessExpressionObject {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => NullsafeMemberAccessExpressionObject::Extra(ExtraChild::Comment(
                Box::new(CommentNode::parse(node, source)?),
            )),
            "text_interpolation" => {
                NullsafeMemberAccessExpressionObject::Extra(ExtraChild::TextInterpolation(
                    Box::new(TextInterpolationNode::parse(node, source)?),
                ))
            }
            "ERROR" => NullsafeMemberAccessExpressionObject::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "array_creation_expression" => {
                NullsafeMemberAccessExpressionObject::ArrayCreationExpression(Box::new(
                    ArrayCreationExpressionNode::parse(node, source)?,
                ))
            }
            "cast_expression" => NullsafeMemberAccessExpressionObject::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                NullsafeMemberAccessExpressionObject::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => NullsafeMemberAccessExpressionObject::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "encapsed_string" => NullsafeMemberAccessExpressionObject::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => {
                NullsafeMemberAccessExpressionObject::FunctionCallExpression(Box::new(
                    FunctionCallExpressionNode::parse(node, source)?,
                ))
            }
            "heredoc" => NullsafeMemberAccessExpressionObject::Heredoc(Box::new(
                HeredocNode::parse(node, source)?,
            )),
            "member_access_expression" => {
                NullsafeMemberAccessExpressionObject::MemberAccessExpression(Box::new(
                    MemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "member_call_expression" => NullsafeMemberAccessExpressionObject::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "name" => {
                NullsafeMemberAccessExpressionObject::Name(Box::new(NameNode::parse(node, source)?))
            }
            "nowdoc" => NullsafeMemberAccessExpressionObject::Nowdoc(Box::new(NowdocNode::parse(
                node, source,
            )?)),
            "nullsafe_member_access_expression" => {
                NullsafeMemberAccessExpressionObject::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                NullsafeMemberAccessExpressionObject::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => {
                NullsafeMemberAccessExpressionObject::ParenthesizedExpression(Box::new(
                    ParenthesizedExpressionNode::parse(node, source)?,
                ))
            }
            "qualified_name" => NullsafeMemberAccessExpressionObject::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_call_expression" => NullsafeMemberAccessExpressionObject::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "scoped_property_access_expression" => {
                NullsafeMemberAccessExpressionObject::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "string" => NullsafeMemberAccessExpressionObject::String(Box::new(StringNode::parse(
                node, source,
            )?)),
            "subscript_expression" => NullsafeMemberAccessExpressionObject::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => NullsafeMemberAccessExpressionObject::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            NullsafeMemberAccessExpressionObject::Extra(y) => y.kind(),
            NullsafeMemberAccessExpressionObject::ArrayCreationExpression(y) => y.kind(),
            NullsafeMemberAccessExpressionObject::CastExpression(y) => y.kind(),
            NullsafeMemberAccessExpressionObject::ClassConstantAccessExpression(y) => y.kind(),
            NullsafeMemberAccessExpressionObject::DynamicVariableName(y) => y.kind(),
            NullsafeMemberAccessExpressionObject::EncapsedString(y) => y.kind(),
            NullsafeMemberAccessExpressionObject::FunctionCallExpression(y) => y.kind(),
            NullsafeMemberAccessExpressionObject::Heredoc(y) => y.kind(),
            NullsafeMemberAccessExpressionObject::MemberAccessExpression(y) => y.kind(),
            NullsafeMemberAccessExpressionObject::MemberCallExpression(y) => y.kind(),
            NullsafeMemberAccessExpressionObject::Name(y) => y.kind(),
            NullsafeMemberAccessExpressionObject::Nowdoc(y) => y.kind(),
            NullsafeMemberAccessExpressionObject::NullsafeMemberAccessExpression(y) => y.kind(),
            NullsafeMemberAccessExpressionObject::NullsafeMemberCallExpression(y) => y.kind(),
            NullsafeMemberAccessExpressionObject::ParenthesizedExpression(y) => y.kind(),
            NullsafeMemberAccessExpressionObject::QualifiedName(y) => y.kind(),
            NullsafeMemberAccessExpressionObject::ScopedCallExpression(y) => y.kind(),
            NullsafeMemberAccessExpressionObject::ScopedPropertyAccessExpression(y) => y.kind(),
            NullsafeMemberAccessExpressionObject::String(y) => y.kind(),
            NullsafeMemberAccessExpressionObject::SubscriptExpression(y) => y.kind(),
            NullsafeMemberAccessExpressionObject::VariableName(y) => y.kind(),
        }
    }

    pub fn parse_vec<'a, I>(children: I, source: &[u8]) -> Result<Vec<Box<Self>>, ParseError>
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
    ) -> Option<PHPType> {
        match self {
            NullsafeMemberAccessExpressionObject::Extra(x) => x.get_utype(state, emitter),
            NullsafeMemberAccessExpressionObject::ArrayCreationExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::CastExpression(x) => x.get_utype(state, emitter),
            NullsafeMemberAccessExpressionObject::ClassConstantAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::DynamicVariableName(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::EncapsedString(x) => x.get_utype(state, emitter),
            NullsafeMemberAccessExpressionObject::FunctionCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::Heredoc(x) => x.get_utype(state, emitter),
            NullsafeMemberAccessExpressionObject::MemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::MemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::Name(x) => x.get_utype(state, emitter),
            NullsafeMemberAccessExpressionObject::Nowdoc(x) => x.get_utype(state, emitter),
            NullsafeMemberAccessExpressionObject::NullsafeMemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::NullsafeMemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::ParenthesizedExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::QualifiedName(x) => x.get_utype(state, emitter),
            NullsafeMemberAccessExpressionObject::ScopedCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::ScopedPropertyAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::String(x) => x.get_utype(state, emitter),
            NullsafeMemberAccessExpressionObject::SubscriptExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            NullsafeMemberAccessExpressionObject::Extra(x) => x.get_php_value(state, emitter),
            NullsafeMemberAccessExpressionObject::ArrayCreationExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::CastExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::ClassConstantAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::DynamicVariableName(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::EncapsedString(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::FunctionCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::Heredoc(x) => x.get_php_value(state, emitter),
            NullsafeMemberAccessExpressionObject::MemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::MemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::Name(x) => x.get_php_value(state, emitter),
            NullsafeMemberAccessExpressionObject::Nowdoc(x) => x.get_php_value(state, emitter),
            NullsafeMemberAccessExpressionObject::NullsafeMemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::NullsafeMemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::ParenthesizedExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::QualifiedName(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::ScopedCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::ScopedPropertyAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::String(x) => x.get_php_value(state, emitter),
            NullsafeMemberAccessExpressionObject::SubscriptExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::VariableName(x) => {
                x.get_php_value(state, emitter)
            }
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            NullsafeMemberAccessExpressionObject::Extra(x) => x.read_from(state, emitter),
            NullsafeMemberAccessExpressionObject::ArrayCreationExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::CastExpression(x) => x.read_from(state, emitter),
            NullsafeMemberAccessExpressionObject::ClassConstantAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::DynamicVariableName(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::EncapsedString(x) => x.read_from(state, emitter),
            NullsafeMemberAccessExpressionObject::FunctionCallExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::Heredoc(x) => x.read_from(state, emitter),
            NullsafeMemberAccessExpressionObject::MemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::MemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::Name(x) => x.read_from(state, emitter),
            NullsafeMemberAccessExpressionObject::Nowdoc(x) => x.read_from(state, emitter),
            NullsafeMemberAccessExpressionObject::NullsafeMemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::NullsafeMemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::ParenthesizedExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::QualifiedName(x) => x.read_from(state, emitter),
            NullsafeMemberAccessExpressionObject::ScopedCallExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::ScopedPropertyAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::String(x) => x.read_from(state, emitter),
            NullsafeMemberAccessExpressionObject::SubscriptExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberAccessExpressionObject::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for NullsafeMemberAccessExpressionObject {
    fn brief_desc(&self) -> String {
        match self {
            NullsafeMemberAccessExpressionObject::Extra(x) => format!(
                "NullsafeMemberAccessExpressionObject::extra({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionObject::ArrayCreationExpression(x) => format!(
                "NullsafeMemberAccessExpressionObject::array_creation_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionObject::CastExpression(x) => format!(
                "NullsafeMemberAccessExpressionObject::cast_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionObject::ClassConstantAccessExpression(x) => format!(
                "NullsafeMemberAccessExpressionObject::class_constant_access_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionObject::DynamicVariableName(x) => format!(
                "NullsafeMemberAccessExpressionObject::dynamic_variable_name({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionObject::EncapsedString(x) => format!(
                "NullsafeMemberAccessExpressionObject::encapsed_string({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionObject::FunctionCallExpression(x) => format!(
                "NullsafeMemberAccessExpressionObject::function_call_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionObject::Heredoc(x) => format!(
                "NullsafeMemberAccessExpressionObject::heredoc({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionObject::MemberAccessExpression(x) => format!(
                "NullsafeMemberAccessExpressionObject::member_access_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionObject::MemberCallExpression(x) => format!(
                "NullsafeMemberAccessExpressionObject::member_call_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionObject::Name(x) => format!(
                "NullsafeMemberAccessExpressionObject::name({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionObject::Nowdoc(x) => format!(
                "NullsafeMemberAccessExpressionObject::nowdoc({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionObject::NullsafeMemberAccessExpression(x) => format!(
                "NullsafeMemberAccessExpressionObject::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionObject::NullsafeMemberCallExpression(x) => format!(
                "NullsafeMemberAccessExpressionObject::nullsafe_member_call_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionObject::ParenthesizedExpression(x) => format!(
                "NullsafeMemberAccessExpressionObject::parenthesized_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionObject::QualifiedName(x) => format!(
                "NullsafeMemberAccessExpressionObject::qualified_name({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionObject::ScopedCallExpression(x) => format!(
                "NullsafeMemberAccessExpressionObject::scoped_call_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionObject::ScopedPropertyAccessExpression(x) => format!(
                "NullsafeMemberAccessExpressionObject::scoped_property_access_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionObject::String(x) => format!(
                "NullsafeMemberAccessExpressionObject::string({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionObject::SubscriptExpression(x) => format!(
                "NullsafeMemberAccessExpressionObject::subscript_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberAccessExpressionObject::VariableName(x) => format!(
                "NullsafeMemberAccessExpressionObject::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            NullsafeMemberAccessExpressionObject::Extra(x) => x.as_any(),
            NullsafeMemberAccessExpressionObject::ArrayCreationExpression(x) => x.as_any(),
            NullsafeMemberAccessExpressionObject::CastExpression(x) => x.as_any(),
            NullsafeMemberAccessExpressionObject::ClassConstantAccessExpression(x) => x.as_any(),
            NullsafeMemberAccessExpressionObject::DynamicVariableName(x) => x.as_any(),
            NullsafeMemberAccessExpressionObject::EncapsedString(x) => x.as_any(),
            NullsafeMemberAccessExpressionObject::FunctionCallExpression(x) => x.as_any(),
            NullsafeMemberAccessExpressionObject::Heredoc(x) => x.as_any(),
            NullsafeMemberAccessExpressionObject::MemberAccessExpression(x) => x.as_any(),
            NullsafeMemberAccessExpressionObject::MemberCallExpression(x) => x.as_any(),
            NullsafeMemberAccessExpressionObject::Name(x) => x.as_any(),
            NullsafeMemberAccessExpressionObject::Nowdoc(x) => x.as_any(),
            NullsafeMemberAccessExpressionObject::NullsafeMemberAccessExpression(x) => x.as_any(),
            NullsafeMemberAccessExpressionObject::NullsafeMemberCallExpression(x) => x.as_any(),
            NullsafeMemberAccessExpressionObject::ParenthesizedExpression(x) => x.as_any(),
            NullsafeMemberAccessExpressionObject::QualifiedName(x) => x.as_any(),
            NullsafeMemberAccessExpressionObject::ScopedCallExpression(x) => x.as_any(),
            NullsafeMemberAccessExpressionObject::ScopedPropertyAccessExpression(x) => x.as_any(),
            NullsafeMemberAccessExpressionObject::String(x) => x.as_any(),
            NullsafeMemberAccessExpressionObject::SubscriptExpression(x) => x.as_any(),
            NullsafeMemberAccessExpressionObject::VariableName(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            NullsafeMemberAccessExpressionObject::Extra(x) => x.children_any(),
            NullsafeMemberAccessExpressionObject::ArrayCreationExpression(x) => x.children_any(),
            NullsafeMemberAccessExpressionObject::CastExpression(x) => x.children_any(),
            NullsafeMemberAccessExpressionObject::ClassConstantAccessExpression(x) => {
                x.children_any()
            }
            NullsafeMemberAccessExpressionObject::DynamicVariableName(x) => x.children_any(),
            NullsafeMemberAccessExpressionObject::EncapsedString(x) => x.children_any(),
            NullsafeMemberAccessExpressionObject::FunctionCallExpression(x) => x.children_any(),
            NullsafeMemberAccessExpressionObject::Heredoc(x) => x.children_any(),
            NullsafeMemberAccessExpressionObject::MemberAccessExpression(x) => x.children_any(),
            NullsafeMemberAccessExpressionObject::MemberCallExpression(x) => x.children_any(),
            NullsafeMemberAccessExpressionObject::Name(x) => x.children_any(),
            NullsafeMemberAccessExpressionObject::Nowdoc(x) => x.children_any(),
            NullsafeMemberAccessExpressionObject::NullsafeMemberAccessExpression(x) => {
                x.children_any()
            }
            NullsafeMemberAccessExpressionObject::NullsafeMemberCallExpression(x) => {
                x.children_any()
            }
            NullsafeMemberAccessExpressionObject::ParenthesizedExpression(x) => x.children_any(),
            NullsafeMemberAccessExpressionObject::QualifiedName(x) => x.children_any(),
            NullsafeMemberAccessExpressionObject::ScopedCallExpression(x) => x.children_any(),
            NullsafeMemberAccessExpressionObject::ScopedPropertyAccessExpression(x) => {
                x.children_any()
            }
            NullsafeMemberAccessExpressionObject::String(x) => x.children_any(),
            NullsafeMemberAccessExpressionObject::SubscriptExpression(x) => x.children_any(),
            NullsafeMemberAccessExpressionObject::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            NullsafeMemberAccessExpressionObject::Extra(x) => x.range(),
            NullsafeMemberAccessExpressionObject::ArrayCreationExpression(x) => x.range(),
            NullsafeMemberAccessExpressionObject::CastExpression(x) => x.range(),
            NullsafeMemberAccessExpressionObject::ClassConstantAccessExpression(x) => x.range(),
            NullsafeMemberAccessExpressionObject::DynamicVariableName(x) => x.range(),
            NullsafeMemberAccessExpressionObject::EncapsedString(x) => x.range(),
            NullsafeMemberAccessExpressionObject::FunctionCallExpression(x) => x.range(),
            NullsafeMemberAccessExpressionObject::Heredoc(x) => x.range(),
            NullsafeMemberAccessExpressionObject::MemberAccessExpression(x) => x.range(),
            NullsafeMemberAccessExpressionObject::MemberCallExpression(x) => x.range(),
            NullsafeMemberAccessExpressionObject::Name(x) => x.range(),
            NullsafeMemberAccessExpressionObject::Nowdoc(x) => x.range(),
            NullsafeMemberAccessExpressionObject::NullsafeMemberAccessExpression(x) => x.range(),
            NullsafeMemberAccessExpressionObject::NullsafeMemberCallExpression(x) => x.range(),
            NullsafeMemberAccessExpressionObject::ParenthesizedExpression(x) => x.range(),
            NullsafeMemberAccessExpressionObject::QualifiedName(x) => x.range(),
            NullsafeMemberAccessExpressionObject::ScopedCallExpression(x) => x.range(),
            NullsafeMemberAccessExpressionObject::ScopedPropertyAccessExpression(x) => x.range(),
            NullsafeMemberAccessExpressionObject::String(x) => x.range(),
            NullsafeMemberAccessExpressionObject::SubscriptExpression(x) => x.range(),
            NullsafeMemberAccessExpressionObject::VariableName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NullsafeMemberAccessExpressionNode {
    pub range: Range,
    pub name: Box<NullsafeMemberAccessExpressionName>,
    pub object: Box<NullsafeMemberAccessExpressionObject>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for NullsafeMemberAccessExpressionNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "nullsafe_member_access_expression" {
            return Err(ParseError::new(range, format!("NullsafeMemberAccessExpressionNode: Node is of the wrong kind [{}] vs expected [nullsafe_member_access_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let name: Box<NullsafeMemberAccessExpressionName> =
            Into::<Result<_, _>>::into(node.parse_child("name", source))?;
        let object: Box<NullsafeMemberAccessExpressionObject> =
            Into::<Result<_, _>>::into(node.parse_child("object", source))?;
        Ok(Self {
            range,
            name,
            object,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl NullsafeMemberAccessExpressionNode {
    pub fn kind(&self) -> &'static str {
        "nullsafe_member_access_expression"
    }
}

impl NodeAccess for NullsafeMemberAccessExpressionNode {
    fn brief_desc(&self) -> String {
        "NullsafeMemberAccessExpressionNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::NullsafeMemberAccessExpression(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.name.as_any());
        child_vec.push(self.object.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
