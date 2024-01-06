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
use crate::autonodes::subscript_expression::SubscriptExpressionNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
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
pub enum MemberAccessExpressionName {
    _Expression(Box<_ExpressionNode>),
    DynamicVariableName(Box<DynamicVariableNameNode>),
    Name(Box<NameNode>),
    VariableName(Box<VariableNameNode>),
    Extra(ExtraChild),
}

impl MemberAccessExpressionName {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => MemberAccessExpressionName::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                MemberAccessExpressionName::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => MemberAccessExpressionName::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "dynamic_variable_name" => MemberAccessExpressionName::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "name" => MemberAccessExpressionName::Name(Box::new(NameNode::parse(node, source)?)),
            "variable_name" => MemberAccessExpressionName::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| MemberAccessExpressionName::_Expression(y))
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

    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => MemberAccessExpressionName::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                MemberAccessExpressionName::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => MemberAccessExpressionName::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "dynamic_variable_name" => MemberAccessExpressionName::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "name" => MemberAccessExpressionName::Name(Box::new(NameNode::parse(node, source)?)),
            "variable_name" => MemberAccessExpressionName::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                return Ok(
                    if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| MemberAccessExpressionName::_Expression(y))
                    {
                        Some(x)
                    } else {
                        None
                    },
                )
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            MemberAccessExpressionName::Extra(y) => y.kind(),
            MemberAccessExpressionName::_Expression(y) => y.kind(),
            MemberAccessExpressionName::DynamicVariableName(y) => y.kind(),
            MemberAccessExpressionName::Name(y) => y.kind(),
            MemberAccessExpressionName::VariableName(y) => y.kind(),
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
            MemberAccessExpressionName::Extra(x) => x.get_utype(state, emitter),
            MemberAccessExpressionName::_Expression(x) => x.get_utype(state, emitter),
            MemberAccessExpressionName::DynamicVariableName(x) => x.get_utype(state, emitter),
            MemberAccessExpressionName::Name(x) => x.get_utype(state, emitter),
            MemberAccessExpressionName::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            MemberAccessExpressionName::Extra(x) => x.get_php_value(state, emitter),
            MemberAccessExpressionName::_Expression(x) => x.get_php_value(state, emitter),
            MemberAccessExpressionName::DynamicVariableName(x) => x.get_php_value(state, emitter),
            MemberAccessExpressionName::Name(x) => x.get_php_value(state, emitter),
            MemberAccessExpressionName::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            MemberAccessExpressionName::Extra(x) => x.read_from(state, emitter),
            MemberAccessExpressionName::_Expression(x) => x.read_from(state, emitter),
            MemberAccessExpressionName::DynamicVariableName(x) => x.read_from(state, emitter),
            MemberAccessExpressionName::Name(x) => x.read_from(state, emitter),
            MemberAccessExpressionName::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for MemberAccessExpressionName {
    fn brief_desc(&self) -> String {
        match self {
            MemberAccessExpressionName::Extra(x) => {
                format!("MemberAccessExpressionName::extra({})", x.brief_desc())
            }
            MemberAccessExpressionName::_Expression(x) => format!(
                "MemberAccessExpressionName::_expression({})",
                x.brief_desc()
            ),
            MemberAccessExpressionName::DynamicVariableName(x) => format!(
                "MemberAccessExpressionName::dynamic_variable_name({})",
                x.brief_desc()
            ),
            MemberAccessExpressionName::Name(x) => {
                format!("MemberAccessExpressionName::name({})", x.brief_desc())
            }
            MemberAccessExpressionName::VariableName(x) => format!(
                "MemberAccessExpressionName::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            MemberAccessExpressionName::Extra(x) => x.as_any(),
            MemberAccessExpressionName::_Expression(x) => x.as_any(),
            MemberAccessExpressionName::DynamicVariableName(x) => x.as_any(),
            MemberAccessExpressionName::Name(x) => x.as_any(),
            MemberAccessExpressionName::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            MemberAccessExpressionName::Extra(x) => x.children_any(),
            MemberAccessExpressionName::_Expression(x) => x.children_any(),
            MemberAccessExpressionName::DynamicVariableName(x) => x.children_any(),
            MemberAccessExpressionName::Name(x) => x.children_any(),
            MemberAccessExpressionName::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            MemberAccessExpressionName::Extra(x) => x.range(),
            MemberAccessExpressionName::_Expression(x) => x.range(),
            MemberAccessExpressionName::DynamicVariableName(x) => x.range(),
            MemberAccessExpressionName::Name(x) => x.range(),
            MemberAccessExpressionName::VariableName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MemberAccessExpressionObject {
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

impl MemberAccessExpressionObject {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => MemberAccessExpressionObject::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                MemberAccessExpressionObject::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => MemberAccessExpressionObject::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "array_creation_expression" => MemberAccessExpressionObject::ArrayCreationExpression(
                Box::new(ArrayCreationExpressionNode::parse(node, source)?),
            ),
            "cast_expression" => MemberAccessExpressionObject::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                MemberAccessExpressionObject::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => MemberAccessExpressionObject::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "encapsed_string" => MemberAccessExpressionObject::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => MemberAccessExpressionObject::FunctionCallExpression(
                Box::new(FunctionCallExpressionNode::parse(node, source)?),
            ),
            "heredoc" => {
                MemberAccessExpressionObject::Heredoc(Box::new(HeredocNode::parse(node, source)?))
            }
            "member_access_expression" => MemberAccessExpressionObject::MemberAccessExpression(
                Box::new(MemberAccessExpressionNode::parse(node, source)?),
            ),
            "member_call_expression" => MemberAccessExpressionObject::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "name" => MemberAccessExpressionObject::Name(Box::new(NameNode::parse(node, source)?)),
            "nowdoc" => {
                MemberAccessExpressionObject::Nowdoc(Box::new(NowdocNode::parse(node, source)?))
            }
            "nullsafe_member_access_expression" => {
                MemberAccessExpressionObject::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                MemberAccessExpressionObject::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => MemberAccessExpressionObject::ParenthesizedExpression(
                Box::new(ParenthesizedExpressionNode::parse(node, source)?),
            ),
            "qualified_name" => MemberAccessExpressionObject::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_call_expression" => MemberAccessExpressionObject::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "scoped_property_access_expression" => {
                MemberAccessExpressionObject::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "string" => {
                MemberAccessExpressionObject::String(Box::new(StringNode::parse(node, source)?))
            }
            "subscript_expression" => MemberAccessExpressionObject::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => MemberAccessExpressionObject::VariableName(Box::new(
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
            "comment" => MemberAccessExpressionObject::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                MemberAccessExpressionObject::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => MemberAccessExpressionObject::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "array_creation_expression" => MemberAccessExpressionObject::ArrayCreationExpression(
                Box::new(ArrayCreationExpressionNode::parse(node, source)?),
            ),
            "cast_expression" => MemberAccessExpressionObject::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                MemberAccessExpressionObject::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => MemberAccessExpressionObject::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "encapsed_string" => MemberAccessExpressionObject::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => MemberAccessExpressionObject::FunctionCallExpression(
                Box::new(FunctionCallExpressionNode::parse(node, source)?),
            ),
            "heredoc" => {
                MemberAccessExpressionObject::Heredoc(Box::new(HeredocNode::parse(node, source)?))
            }
            "member_access_expression" => MemberAccessExpressionObject::MemberAccessExpression(
                Box::new(MemberAccessExpressionNode::parse(node, source)?),
            ),
            "member_call_expression" => MemberAccessExpressionObject::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "name" => MemberAccessExpressionObject::Name(Box::new(NameNode::parse(node, source)?)),
            "nowdoc" => {
                MemberAccessExpressionObject::Nowdoc(Box::new(NowdocNode::parse(node, source)?))
            }
            "nullsafe_member_access_expression" => {
                MemberAccessExpressionObject::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                MemberAccessExpressionObject::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => MemberAccessExpressionObject::ParenthesizedExpression(
                Box::new(ParenthesizedExpressionNode::parse(node, source)?),
            ),
            "qualified_name" => MemberAccessExpressionObject::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_call_expression" => MemberAccessExpressionObject::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "scoped_property_access_expression" => {
                MemberAccessExpressionObject::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "string" => {
                MemberAccessExpressionObject::String(Box::new(StringNode::parse(node, source)?))
            }
            "subscript_expression" => MemberAccessExpressionObject::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => MemberAccessExpressionObject::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            MemberAccessExpressionObject::Extra(y) => y.kind(),
            MemberAccessExpressionObject::ArrayCreationExpression(y) => y.kind(),
            MemberAccessExpressionObject::CastExpression(y) => y.kind(),
            MemberAccessExpressionObject::ClassConstantAccessExpression(y) => y.kind(),
            MemberAccessExpressionObject::DynamicVariableName(y) => y.kind(),
            MemberAccessExpressionObject::EncapsedString(y) => y.kind(),
            MemberAccessExpressionObject::FunctionCallExpression(y) => y.kind(),
            MemberAccessExpressionObject::Heredoc(y) => y.kind(),
            MemberAccessExpressionObject::MemberAccessExpression(y) => y.kind(),
            MemberAccessExpressionObject::MemberCallExpression(y) => y.kind(),
            MemberAccessExpressionObject::Name(y) => y.kind(),
            MemberAccessExpressionObject::Nowdoc(y) => y.kind(),
            MemberAccessExpressionObject::NullsafeMemberAccessExpression(y) => y.kind(),
            MemberAccessExpressionObject::NullsafeMemberCallExpression(y) => y.kind(),
            MemberAccessExpressionObject::ParenthesizedExpression(y) => y.kind(),
            MemberAccessExpressionObject::QualifiedName(y) => y.kind(),
            MemberAccessExpressionObject::ScopedCallExpression(y) => y.kind(),
            MemberAccessExpressionObject::ScopedPropertyAccessExpression(y) => y.kind(),
            MemberAccessExpressionObject::String(y) => y.kind(),
            MemberAccessExpressionObject::SubscriptExpression(y) => y.kind(),
            MemberAccessExpressionObject::VariableName(y) => y.kind(),
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
            MemberAccessExpressionObject::Extra(x) => x.get_utype(state, emitter),
            MemberAccessExpressionObject::ArrayCreationExpression(x) => x.get_utype(state, emitter),
            MemberAccessExpressionObject::CastExpression(x) => x.get_utype(state, emitter),
            MemberAccessExpressionObject::ClassConstantAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            MemberAccessExpressionObject::DynamicVariableName(x) => x.get_utype(state, emitter),
            MemberAccessExpressionObject::EncapsedString(x) => x.get_utype(state, emitter),
            MemberAccessExpressionObject::FunctionCallExpression(x) => x.get_utype(state, emitter),
            MemberAccessExpressionObject::Heredoc(x) => x.get_utype(state, emitter),
            MemberAccessExpressionObject::MemberAccessExpression(x) => x.get_utype(state, emitter),
            MemberAccessExpressionObject::MemberCallExpression(x) => x.get_utype(state, emitter),
            MemberAccessExpressionObject::Name(x) => x.get_utype(state, emitter),
            MemberAccessExpressionObject::Nowdoc(x) => x.get_utype(state, emitter),
            MemberAccessExpressionObject::NullsafeMemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            MemberAccessExpressionObject::NullsafeMemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            MemberAccessExpressionObject::ParenthesizedExpression(x) => x.get_utype(state, emitter),
            MemberAccessExpressionObject::QualifiedName(x) => x.get_utype(state, emitter),
            MemberAccessExpressionObject::ScopedCallExpression(x) => x.get_utype(state, emitter),
            MemberAccessExpressionObject::ScopedPropertyAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            MemberAccessExpressionObject::String(x) => x.get_utype(state, emitter),
            MemberAccessExpressionObject::SubscriptExpression(x) => x.get_utype(state, emitter),
            MemberAccessExpressionObject::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            MemberAccessExpressionObject::Extra(x) => x.get_php_value(state, emitter),
            MemberAccessExpressionObject::ArrayCreationExpression(x) => {
                x.get_php_value(state, emitter)
            }
            MemberAccessExpressionObject::CastExpression(x) => x.get_php_value(state, emitter),
            MemberAccessExpressionObject::ClassConstantAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            MemberAccessExpressionObject::DynamicVariableName(x) => x.get_php_value(state, emitter),
            MemberAccessExpressionObject::EncapsedString(x) => x.get_php_value(state, emitter),
            MemberAccessExpressionObject::FunctionCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            MemberAccessExpressionObject::Heredoc(x) => x.get_php_value(state, emitter),
            MemberAccessExpressionObject::MemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            MemberAccessExpressionObject::MemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            MemberAccessExpressionObject::Name(x) => x.get_php_value(state, emitter),
            MemberAccessExpressionObject::Nowdoc(x) => x.get_php_value(state, emitter),
            MemberAccessExpressionObject::NullsafeMemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            MemberAccessExpressionObject::NullsafeMemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            MemberAccessExpressionObject::ParenthesizedExpression(x) => {
                x.get_php_value(state, emitter)
            }
            MemberAccessExpressionObject::QualifiedName(x) => x.get_php_value(state, emitter),
            MemberAccessExpressionObject::ScopedCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            MemberAccessExpressionObject::ScopedPropertyAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            MemberAccessExpressionObject::String(x) => x.get_php_value(state, emitter),
            MemberAccessExpressionObject::SubscriptExpression(x) => x.get_php_value(state, emitter),
            MemberAccessExpressionObject::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            MemberAccessExpressionObject::Extra(x) => x.read_from(state, emitter),
            MemberAccessExpressionObject::ArrayCreationExpression(x) => x.read_from(state, emitter),
            MemberAccessExpressionObject::CastExpression(x) => x.read_from(state, emitter),
            MemberAccessExpressionObject::ClassConstantAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            MemberAccessExpressionObject::DynamicVariableName(x) => x.read_from(state, emitter),
            MemberAccessExpressionObject::EncapsedString(x) => x.read_from(state, emitter),
            MemberAccessExpressionObject::FunctionCallExpression(x) => x.read_from(state, emitter),
            MemberAccessExpressionObject::Heredoc(x) => x.read_from(state, emitter),
            MemberAccessExpressionObject::MemberAccessExpression(x) => x.read_from(state, emitter),
            MemberAccessExpressionObject::MemberCallExpression(x) => x.read_from(state, emitter),
            MemberAccessExpressionObject::Name(x) => x.read_from(state, emitter),
            MemberAccessExpressionObject::Nowdoc(x) => x.read_from(state, emitter),
            MemberAccessExpressionObject::NullsafeMemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            MemberAccessExpressionObject::NullsafeMemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            MemberAccessExpressionObject::ParenthesizedExpression(x) => x.read_from(state, emitter),
            MemberAccessExpressionObject::QualifiedName(x) => x.read_from(state, emitter),
            MemberAccessExpressionObject::ScopedCallExpression(x) => x.read_from(state, emitter),
            MemberAccessExpressionObject::ScopedPropertyAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            MemberAccessExpressionObject::String(x) => x.read_from(state, emitter),
            MemberAccessExpressionObject::SubscriptExpression(x) => x.read_from(state, emitter),
            MemberAccessExpressionObject::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for MemberAccessExpressionObject {
    fn brief_desc(&self) -> String {
        match self {
            MemberAccessExpressionObject::Extra(x) => {
                format!("MemberAccessExpressionObject::extra({})", x.brief_desc())
            }
            MemberAccessExpressionObject::ArrayCreationExpression(x) => format!(
                "MemberAccessExpressionObject::array_creation_expression({})",
                x.brief_desc()
            ),
            MemberAccessExpressionObject::CastExpression(x) => format!(
                "MemberAccessExpressionObject::cast_expression({})",
                x.brief_desc()
            ),
            MemberAccessExpressionObject::ClassConstantAccessExpression(x) => format!(
                "MemberAccessExpressionObject::class_constant_access_expression({})",
                x.brief_desc()
            ),
            MemberAccessExpressionObject::DynamicVariableName(x) => format!(
                "MemberAccessExpressionObject::dynamic_variable_name({})",
                x.brief_desc()
            ),
            MemberAccessExpressionObject::EncapsedString(x) => format!(
                "MemberAccessExpressionObject::encapsed_string({})",
                x.brief_desc()
            ),
            MemberAccessExpressionObject::FunctionCallExpression(x) => format!(
                "MemberAccessExpressionObject::function_call_expression({})",
                x.brief_desc()
            ),
            MemberAccessExpressionObject::Heredoc(x) => {
                format!("MemberAccessExpressionObject::heredoc({})", x.brief_desc())
            }
            MemberAccessExpressionObject::MemberAccessExpression(x) => format!(
                "MemberAccessExpressionObject::member_access_expression({})",
                x.brief_desc()
            ),
            MemberAccessExpressionObject::MemberCallExpression(x) => format!(
                "MemberAccessExpressionObject::member_call_expression({})",
                x.brief_desc()
            ),
            MemberAccessExpressionObject::Name(x) => {
                format!("MemberAccessExpressionObject::name({})", x.brief_desc())
            }
            MemberAccessExpressionObject::Nowdoc(x) => {
                format!("MemberAccessExpressionObject::nowdoc({})", x.brief_desc())
            }
            MemberAccessExpressionObject::NullsafeMemberAccessExpression(x) => format!(
                "MemberAccessExpressionObject::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            MemberAccessExpressionObject::NullsafeMemberCallExpression(x) => format!(
                "MemberAccessExpressionObject::nullsafe_member_call_expression({})",
                x.brief_desc()
            ),
            MemberAccessExpressionObject::ParenthesizedExpression(x) => format!(
                "MemberAccessExpressionObject::parenthesized_expression({})",
                x.brief_desc()
            ),
            MemberAccessExpressionObject::QualifiedName(x) => format!(
                "MemberAccessExpressionObject::qualified_name({})",
                x.brief_desc()
            ),
            MemberAccessExpressionObject::ScopedCallExpression(x) => format!(
                "MemberAccessExpressionObject::scoped_call_expression({})",
                x.brief_desc()
            ),
            MemberAccessExpressionObject::ScopedPropertyAccessExpression(x) => format!(
                "MemberAccessExpressionObject::scoped_property_access_expression({})",
                x.brief_desc()
            ),
            MemberAccessExpressionObject::String(x) => {
                format!("MemberAccessExpressionObject::string({})", x.brief_desc())
            }
            MemberAccessExpressionObject::SubscriptExpression(x) => format!(
                "MemberAccessExpressionObject::subscript_expression({})",
                x.brief_desc()
            ),
            MemberAccessExpressionObject::VariableName(x) => format!(
                "MemberAccessExpressionObject::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            MemberAccessExpressionObject::Extra(x) => x.as_any(),
            MemberAccessExpressionObject::ArrayCreationExpression(x) => x.as_any(),
            MemberAccessExpressionObject::CastExpression(x) => x.as_any(),
            MemberAccessExpressionObject::ClassConstantAccessExpression(x) => x.as_any(),
            MemberAccessExpressionObject::DynamicVariableName(x) => x.as_any(),
            MemberAccessExpressionObject::EncapsedString(x) => x.as_any(),
            MemberAccessExpressionObject::FunctionCallExpression(x) => x.as_any(),
            MemberAccessExpressionObject::Heredoc(x) => x.as_any(),
            MemberAccessExpressionObject::MemberAccessExpression(x) => x.as_any(),
            MemberAccessExpressionObject::MemberCallExpression(x) => x.as_any(),
            MemberAccessExpressionObject::Name(x) => x.as_any(),
            MemberAccessExpressionObject::Nowdoc(x) => x.as_any(),
            MemberAccessExpressionObject::NullsafeMemberAccessExpression(x) => x.as_any(),
            MemberAccessExpressionObject::NullsafeMemberCallExpression(x) => x.as_any(),
            MemberAccessExpressionObject::ParenthesizedExpression(x) => x.as_any(),
            MemberAccessExpressionObject::QualifiedName(x) => x.as_any(),
            MemberAccessExpressionObject::ScopedCallExpression(x) => x.as_any(),
            MemberAccessExpressionObject::ScopedPropertyAccessExpression(x) => x.as_any(),
            MemberAccessExpressionObject::String(x) => x.as_any(),
            MemberAccessExpressionObject::SubscriptExpression(x) => x.as_any(),
            MemberAccessExpressionObject::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            MemberAccessExpressionObject::Extra(x) => x.children_any(),
            MemberAccessExpressionObject::ArrayCreationExpression(x) => x.children_any(),
            MemberAccessExpressionObject::CastExpression(x) => x.children_any(),
            MemberAccessExpressionObject::ClassConstantAccessExpression(x) => x.children_any(),
            MemberAccessExpressionObject::DynamicVariableName(x) => x.children_any(),
            MemberAccessExpressionObject::EncapsedString(x) => x.children_any(),
            MemberAccessExpressionObject::FunctionCallExpression(x) => x.children_any(),
            MemberAccessExpressionObject::Heredoc(x) => x.children_any(),
            MemberAccessExpressionObject::MemberAccessExpression(x) => x.children_any(),
            MemberAccessExpressionObject::MemberCallExpression(x) => x.children_any(),
            MemberAccessExpressionObject::Name(x) => x.children_any(),
            MemberAccessExpressionObject::Nowdoc(x) => x.children_any(),
            MemberAccessExpressionObject::NullsafeMemberAccessExpression(x) => x.children_any(),
            MemberAccessExpressionObject::NullsafeMemberCallExpression(x) => x.children_any(),
            MemberAccessExpressionObject::ParenthesizedExpression(x) => x.children_any(),
            MemberAccessExpressionObject::QualifiedName(x) => x.children_any(),
            MemberAccessExpressionObject::ScopedCallExpression(x) => x.children_any(),
            MemberAccessExpressionObject::ScopedPropertyAccessExpression(x) => x.children_any(),
            MemberAccessExpressionObject::String(x) => x.children_any(),
            MemberAccessExpressionObject::SubscriptExpression(x) => x.children_any(),
            MemberAccessExpressionObject::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            MemberAccessExpressionObject::Extra(x) => x.range(),
            MemberAccessExpressionObject::ArrayCreationExpression(x) => x.range(),
            MemberAccessExpressionObject::CastExpression(x) => x.range(),
            MemberAccessExpressionObject::ClassConstantAccessExpression(x) => x.range(),
            MemberAccessExpressionObject::DynamicVariableName(x) => x.range(),
            MemberAccessExpressionObject::EncapsedString(x) => x.range(),
            MemberAccessExpressionObject::FunctionCallExpression(x) => x.range(),
            MemberAccessExpressionObject::Heredoc(x) => x.range(),
            MemberAccessExpressionObject::MemberAccessExpression(x) => x.range(),
            MemberAccessExpressionObject::MemberCallExpression(x) => x.range(),
            MemberAccessExpressionObject::Name(x) => x.range(),
            MemberAccessExpressionObject::Nowdoc(x) => x.range(),
            MemberAccessExpressionObject::NullsafeMemberAccessExpression(x) => x.range(),
            MemberAccessExpressionObject::NullsafeMemberCallExpression(x) => x.range(),
            MemberAccessExpressionObject::ParenthesizedExpression(x) => x.range(),
            MemberAccessExpressionObject::QualifiedName(x) => x.range(),
            MemberAccessExpressionObject::ScopedCallExpression(x) => x.range(),
            MemberAccessExpressionObject::ScopedPropertyAccessExpression(x) => x.range(),
            MemberAccessExpressionObject::String(x) => x.range(),
            MemberAccessExpressionObject::SubscriptExpression(x) => x.range(),
            MemberAccessExpressionObject::VariableName(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MemberAccessExpressionNode {
    pub range: Range,
    pub name: Box<MemberAccessExpressionName>,
    pub object: Box<MemberAccessExpressionObject>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl MemberAccessExpressionNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "member_access_expression" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [member_access_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let name: Box<MemberAccessExpressionName> = node
            .children_by_field_name("name", &mut node.walk())
            .map(|chnode2| MemberAccessExpressionName::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .expect("Field name should exist")
            .into();
        let object: Box<MemberAccessExpressionObject> = node
            .children_by_field_name("object", &mut node.walk())
            .map(|chnode2| MemberAccessExpressionObject::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .expect("Field object should exist")
            .into();
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
        "member_access_expression"
    }
}

impl NodeAccess for MemberAccessExpressionNode {
    fn brief_desc(&self) -> String {
        "MemberAccessExpressionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::MemberAccessExpression(self)
    }

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
