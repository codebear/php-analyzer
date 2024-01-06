use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::arguments::ArgumentsNode;
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
pub enum NullsafeMemberCallExpressionName {
    _Expression(Box<_ExpressionNode>),
    DynamicVariableName(Box<DynamicVariableNameNode>),
    Name(Box<NameNode>),
    VariableName(Box<VariableNameNode>),
    Extra(ExtraChild),
}

impl NullsafeMemberCallExpressionName {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => NullsafeMemberCallExpressionName::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                NullsafeMemberCallExpressionName::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => NullsafeMemberCallExpressionName::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "dynamic_variable_name" => NullsafeMemberCallExpressionName::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "name" => {
                NullsafeMemberCallExpressionName::Name(Box::new(NameNode::parse(node, source)?))
            }
            "variable_name" => NullsafeMemberCallExpressionName::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| NullsafeMemberCallExpressionName::_Expression(y))
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
            "comment" => NullsafeMemberCallExpressionName::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                NullsafeMemberCallExpressionName::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => NullsafeMemberCallExpressionName::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "dynamic_variable_name" => NullsafeMemberCallExpressionName::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "name" => {
                NullsafeMemberCallExpressionName::Name(Box::new(NameNode::parse(node, source)?))
            }
            "variable_name" => NullsafeMemberCallExpressionName::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                return Ok(
                    if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| NullsafeMemberCallExpressionName::_Expression(y))
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
            NullsafeMemberCallExpressionName::Extra(y) => y.kind(),
            NullsafeMemberCallExpressionName::_Expression(y) => y.kind(),
            NullsafeMemberCallExpressionName::DynamicVariableName(y) => y.kind(),
            NullsafeMemberCallExpressionName::Name(y) => y.kind(),
            NullsafeMemberCallExpressionName::VariableName(y) => y.kind(),
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
            NullsafeMemberCallExpressionName::Extra(x) => x.get_utype(state, emitter),
            NullsafeMemberCallExpressionName::_Expression(x) => x.get_utype(state, emitter),
            NullsafeMemberCallExpressionName::DynamicVariableName(x) => x.get_utype(state, emitter),
            NullsafeMemberCallExpressionName::Name(x) => x.get_utype(state, emitter),
            NullsafeMemberCallExpressionName::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            NullsafeMemberCallExpressionName::Extra(x) => x.get_php_value(state, emitter),
            NullsafeMemberCallExpressionName::_Expression(x) => x.get_php_value(state, emitter),
            NullsafeMemberCallExpressionName::DynamicVariableName(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberCallExpressionName::Name(x) => x.get_php_value(state, emitter),
            NullsafeMemberCallExpressionName::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            NullsafeMemberCallExpressionName::Extra(x) => x.read_from(state, emitter),
            NullsafeMemberCallExpressionName::_Expression(x) => x.read_from(state, emitter),
            NullsafeMemberCallExpressionName::DynamicVariableName(x) => x.read_from(state, emitter),
            NullsafeMemberCallExpressionName::Name(x) => x.read_from(state, emitter),
            NullsafeMemberCallExpressionName::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for NullsafeMemberCallExpressionName {
    fn brief_desc(&self) -> String {
        match self {
            NullsafeMemberCallExpressionName::Extra(x) => format!(
                "NullsafeMemberCallExpressionName::extra({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionName::_Expression(x) => format!(
                "NullsafeMemberCallExpressionName::_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionName::DynamicVariableName(x) => format!(
                "NullsafeMemberCallExpressionName::dynamic_variable_name({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionName::Name(x) => {
                format!("NullsafeMemberCallExpressionName::name({})", x.brief_desc())
            }
            NullsafeMemberCallExpressionName::VariableName(x) => format!(
                "NullsafeMemberCallExpressionName::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            NullsafeMemberCallExpressionName::Extra(x) => x.as_any(),
            NullsafeMemberCallExpressionName::_Expression(x) => x.as_any(),
            NullsafeMemberCallExpressionName::DynamicVariableName(x) => x.as_any(),
            NullsafeMemberCallExpressionName::Name(x) => x.as_any(),
            NullsafeMemberCallExpressionName::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            NullsafeMemberCallExpressionName::Extra(x) => x.children_any(),
            NullsafeMemberCallExpressionName::_Expression(x) => x.children_any(),
            NullsafeMemberCallExpressionName::DynamicVariableName(x) => x.children_any(),
            NullsafeMemberCallExpressionName::Name(x) => x.children_any(),
            NullsafeMemberCallExpressionName::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            NullsafeMemberCallExpressionName::Extra(x) => x.range(),
            NullsafeMemberCallExpressionName::_Expression(x) => x.range(),
            NullsafeMemberCallExpressionName::DynamicVariableName(x) => x.range(),
            NullsafeMemberCallExpressionName::Name(x) => x.range(),
            NullsafeMemberCallExpressionName::VariableName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum NullsafeMemberCallExpressionObject {
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

impl NullsafeMemberCallExpressionObject {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => NullsafeMemberCallExpressionObject::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                NullsafeMemberCallExpressionObject::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => NullsafeMemberCallExpressionObject::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "array_creation_expression" => {
                NullsafeMemberCallExpressionObject::ArrayCreationExpression(Box::new(
                    ArrayCreationExpressionNode::parse(node, source)?,
                ))
            }
            "cast_expression" => NullsafeMemberCallExpressionObject::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                NullsafeMemberCallExpressionObject::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => NullsafeMemberCallExpressionObject::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "encapsed_string" => NullsafeMemberCallExpressionObject::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => {
                NullsafeMemberCallExpressionObject::FunctionCallExpression(Box::new(
                    FunctionCallExpressionNode::parse(node, source)?,
                ))
            }
            "heredoc" => NullsafeMemberCallExpressionObject::Heredoc(Box::new(HeredocNode::parse(
                node, source,
            )?)),
            "member_access_expression" => {
                NullsafeMemberCallExpressionObject::MemberAccessExpression(Box::new(
                    MemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "member_call_expression" => NullsafeMemberCallExpressionObject::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "name" => {
                NullsafeMemberCallExpressionObject::Name(Box::new(NameNode::parse(node, source)?))
            }
            "nowdoc" => NullsafeMemberCallExpressionObject::Nowdoc(Box::new(NowdocNode::parse(
                node, source,
            )?)),
            "nullsafe_member_access_expression" => {
                NullsafeMemberCallExpressionObject::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                NullsafeMemberCallExpressionObject::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => {
                NullsafeMemberCallExpressionObject::ParenthesizedExpression(Box::new(
                    ParenthesizedExpressionNode::parse(node, source)?,
                ))
            }
            "qualified_name" => NullsafeMemberCallExpressionObject::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_call_expression" => NullsafeMemberCallExpressionObject::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "scoped_property_access_expression" => {
                NullsafeMemberCallExpressionObject::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "string" => NullsafeMemberCallExpressionObject::String(Box::new(StringNode::parse(
                node, source,
            )?)),
            "subscript_expression" => NullsafeMemberCallExpressionObject::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => NullsafeMemberCallExpressionObject::VariableName(Box::new(
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
            "comment" => NullsafeMemberCallExpressionObject::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                NullsafeMemberCallExpressionObject::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => NullsafeMemberCallExpressionObject::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "array_creation_expression" => {
                NullsafeMemberCallExpressionObject::ArrayCreationExpression(Box::new(
                    ArrayCreationExpressionNode::parse(node, source)?,
                ))
            }
            "cast_expression" => NullsafeMemberCallExpressionObject::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                NullsafeMemberCallExpressionObject::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => NullsafeMemberCallExpressionObject::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "encapsed_string" => NullsafeMemberCallExpressionObject::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => {
                NullsafeMemberCallExpressionObject::FunctionCallExpression(Box::new(
                    FunctionCallExpressionNode::parse(node, source)?,
                ))
            }
            "heredoc" => NullsafeMemberCallExpressionObject::Heredoc(Box::new(HeredocNode::parse(
                node, source,
            )?)),
            "member_access_expression" => {
                NullsafeMemberCallExpressionObject::MemberAccessExpression(Box::new(
                    MemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "member_call_expression" => NullsafeMemberCallExpressionObject::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "name" => {
                NullsafeMemberCallExpressionObject::Name(Box::new(NameNode::parse(node, source)?))
            }
            "nowdoc" => NullsafeMemberCallExpressionObject::Nowdoc(Box::new(NowdocNode::parse(
                node, source,
            )?)),
            "nullsafe_member_access_expression" => {
                NullsafeMemberCallExpressionObject::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                NullsafeMemberCallExpressionObject::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => {
                NullsafeMemberCallExpressionObject::ParenthesizedExpression(Box::new(
                    ParenthesizedExpressionNode::parse(node, source)?,
                ))
            }
            "qualified_name" => NullsafeMemberCallExpressionObject::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_call_expression" => NullsafeMemberCallExpressionObject::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "scoped_property_access_expression" => {
                NullsafeMemberCallExpressionObject::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "string" => NullsafeMemberCallExpressionObject::String(Box::new(StringNode::parse(
                node, source,
            )?)),
            "subscript_expression" => NullsafeMemberCallExpressionObject::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => NullsafeMemberCallExpressionObject::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            NullsafeMemberCallExpressionObject::Extra(y) => y.kind(),
            NullsafeMemberCallExpressionObject::ArrayCreationExpression(y) => y.kind(),
            NullsafeMemberCallExpressionObject::CastExpression(y) => y.kind(),
            NullsafeMemberCallExpressionObject::ClassConstantAccessExpression(y) => y.kind(),
            NullsafeMemberCallExpressionObject::DynamicVariableName(y) => y.kind(),
            NullsafeMemberCallExpressionObject::EncapsedString(y) => y.kind(),
            NullsafeMemberCallExpressionObject::FunctionCallExpression(y) => y.kind(),
            NullsafeMemberCallExpressionObject::Heredoc(y) => y.kind(),
            NullsafeMemberCallExpressionObject::MemberAccessExpression(y) => y.kind(),
            NullsafeMemberCallExpressionObject::MemberCallExpression(y) => y.kind(),
            NullsafeMemberCallExpressionObject::Name(y) => y.kind(),
            NullsafeMemberCallExpressionObject::Nowdoc(y) => y.kind(),
            NullsafeMemberCallExpressionObject::NullsafeMemberAccessExpression(y) => y.kind(),
            NullsafeMemberCallExpressionObject::NullsafeMemberCallExpression(y) => y.kind(),
            NullsafeMemberCallExpressionObject::ParenthesizedExpression(y) => y.kind(),
            NullsafeMemberCallExpressionObject::QualifiedName(y) => y.kind(),
            NullsafeMemberCallExpressionObject::ScopedCallExpression(y) => y.kind(),
            NullsafeMemberCallExpressionObject::ScopedPropertyAccessExpression(y) => y.kind(),
            NullsafeMemberCallExpressionObject::String(y) => y.kind(),
            NullsafeMemberCallExpressionObject::SubscriptExpression(y) => y.kind(),
            NullsafeMemberCallExpressionObject::VariableName(y) => y.kind(),
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
            NullsafeMemberCallExpressionObject::Extra(x) => x.get_utype(state, emitter),
            NullsafeMemberCallExpressionObject::ArrayCreationExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberCallExpressionObject::CastExpression(x) => x.get_utype(state, emitter),
            NullsafeMemberCallExpressionObject::ClassConstantAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberCallExpressionObject::DynamicVariableName(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberCallExpressionObject::EncapsedString(x) => x.get_utype(state, emitter),
            NullsafeMemberCallExpressionObject::FunctionCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberCallExpressionObject::Heredoc(x) => x.get_utype(state, emitter),
            NullsafeMemberCallExpressionObject::MemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberCallExpressionObject::MemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberCallExpressionObject::Name(x) => x.get_utype(state, emitter),
            NullsafeMemberCallExpressionObject::Nowdoc(x) => x.get_utype(state, emitter),
            NullsafeMemberCallExpressionObject::NullsafeMemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberCallExpressionObject::NullsafeMemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberCallExpressionObject::ParenthesizedExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberCallExpressionObject::QualifiedName(x) => x.get_utype(state, emitter),
            NullsafeMemberCallExpressionObject::ScopedCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberCallExpressionObject::ScopedPropertyAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberCallExpressionObject::String(x) => x.get_utype(state, emitter),
            NullsafeMemberCallExpressionObject::SubscriptExpression(x) => {
                x.get_utype(state, emitter)
            }
            NullsafeMemberCallExpressionObject::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            NullsafeMemberCallExpressionObject::Extra(x) => x.get_php_value(state, emitter),
            NullsafeMemberCallExpressionObject::ArrayCreationExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberCallExpressionObject::CastExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberCallExpressionObject::ClassConstantAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberCallExpressionObject::DynamicVariableName(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberCallExpressionObject::EncapsedString(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberCallExpressionObject::FunctionCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberCallExpressionObject::Heredoc(x) => x.get_php_value(state, emitter),
            NullsafeMemberCallExpressionObject::MemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberCallExpressionObject::MemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberCallExpressionObject::Name(x) => x.get_php_value(state, emitter),
            NullsafeMemberCallExpressionObject::Nowdoc(x) => x.get_php_value(state, emitter),
            NullsafeMemberCallExpressionObject::NullsafeMemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberCallExpressionObject::NullsafeMemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberCallExpressionObject::ParenthesizedExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberCallExpressionObject::QualifiedName(x) => x.get_php_value(state, emitter),
            NullsafeMemberCallExpressionObject::ScopedCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberCallExpressionObject::ScopedPropertyAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberCallExpressionObject::String(x) => x.get_php_value(state, emitter),
            NullsafeMemberCallExpressionObject::SubscriptExpression(x) => {
                x.get_php_value(state, emitter)
            }
            NullsafeMemberCallExpressionObject::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            NullsafeMemberCallExpressionObject::Extra(x) => x.read_from(state, emitter),
            NullsafeMemberCallExpressionObject::ArrayCreationExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberCallExpressionObject::CastExpression(x) => x.read_from(state, emitter),
            NullsafeMemberCallExpressionObject::ClassConstantAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberCallExpressionObject::DynamicVariableName(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberCallExpressionObject::EncapsedString(x) => x.read_from(state, emitter),
            NullsafeMemberCallExpressionObject::FunctionCallExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberCallExpressionObject::Heredoc(x) => x.read_from(state, emitter),
            NullsafeMemberCallExpressionObject::MemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberCallExpressionObject::MemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberCallExpressionObject::Name(x) => x.read_from(state, emitter),
            NullsafeMemberCallExpressionObject::Nowdoc(x) => x.read_from(state, emitter),
            NullsafeMemberCallExpressionObject::NullsafeMemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberCallExpressionObject::NullsafeMemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberCallExpressionObject::ParenthesizedExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberCallExpressionObject::QualifiedName(x) => x.read_from(state, emitter),
            NullsafeMemberCallExpressionObject::ScopedCallExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberCallExpressionObject::ScopedPropertyAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberCallExpressionObject::String(x) => x.read_from(state, emitter),
            NullsafeMemberCallExpressionObject::SubscriptExpression(x) => {
                x.read_from(state, emitter)
            }
            NullsafeMemberCallExpressionObject::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for NullsafeMemberCallExpressionObject {
    fn brief_desc(&self) -> String {
        match self {
            NullsafeMemberCallExpressionObject::Extra(x) => format!(
                "NullsafeMemberCallExpressionObject::extra({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionObject::ArrayCreationExpression(x) => format!(
                "NullsafeMemberCallExpressionObject::array_creation_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionObject::CastExpression(x) => format!(
                "NullsafeMemberCallExpressionObject::cast_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionObject::ClassConstantAccessExpression(x) => format!(
                "NullsafeMemberCallExpressionObject::class_constant_access_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionObject::DynamicVariableName(x) => format!(
                "NullsafeMemberCallExpressionObject::dynamic_variable_name({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionObject::EncapsedString(x) => format!(
                "NullsafeMemberCallExpressionObject::encapsed_string({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionObject::FunctionCallExpression(x) => format!(
                "NullsafeMemberCallExpressionObject::function_call_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionObject::Heredoc(x) => format!(
                "NullsafeMemberCallExpressionObject::heredoc({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionObject::MemberAccessExpression(x) => format!(
                "NullsafeMemberCallExpressionObject::member_access_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionObject::MemberCallExpression(x) => format!(
                "NullsafeMemberCallExpressionObject::member_call_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionObject::Name(x) => format!(
                "NullsafeMemberCallExpressionObject::name({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionObject::Nowdoc(x) => format!(
                "NullsafeMemberCallExpressionObject::nowdoc({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionObject::NullsafeMemberAccessExpression(x) => format!(
                "NullsafeMemberCallExpressionObject::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionObject::NullsafeMemberCallExpression(x) => format!(
                "NullsafeMemberCallExpressionObject::nullsafe_member_call_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionObject::ParenthesizedExpression(x) => format!(
                "NullsafeMemberCallExpressionObject::parenthesized_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionObject::QualifiedName(x) => format!(
                "NullsafeMemberCallExpressionObject::qualified_name({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionObject::ScopedCallExpression(x) => format!(
                "NullsafeMemberCallExpressionObject::scoped_call_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionObject::ScopedPropertyAccessExpression(x) => format!(
                "NullsafeMemberCallExpressionObject::scoped_property_access_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionObject::String(x) => format!(
                "NullsafeMemberCallExpressionObject::string({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionObject::SubscriptExpression(x) => format!(
                "NullsafeMemberCallExpressionObject::subscript_expression({})",
                x.brief_desc()
            ),
            NullsafeMemberCallExpressionObject::VariableName(x) => format!(
                "NullsafeMemberCallExpressionObject::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            NullsafeMemberCallExpressionObject::Extra(x) => x.as_any(),
            NullsafeMemberCallExpressionObject::ArrayCreationExpression(x) => x.as_any(),
            NullsafeMemberCallExpressionObject::CastExpression(x) => x.as_any(),
            NullsafeMemberCallExpressionObject::ClassConstantAccessExpression(x) => x.as_any(),
            NullsafeMemberCallExpressionObject::DynamicVariableName(x) => x.as_any(),
            NullsafeMemberCallExpressionObject::EncapsedString(x) => x.as_any(),
            NullsafeMemberCallExpressionObject::FunctionCallExpression(x) => x.as_any(),
            NullsafeMemberCallExpressionObject::Heredoc(x) => x.as_any(),
            NullsafeMemberCallExpressionObject::MemberAccessExpression(x) => x.as_any(),
            NullsafeMemberCallExpressionObject::MemberCallExpression(x) => x.as_any(),
            NullsafeMemberCallExpressionObject::Name(x) => x.as_any(),
            NullsafeMemberCallExpressionObject::Nowdoc(x) => x.as_any(),
            NullsafeMemberCallExpressionObject::NullsafeMemberAccessExpression(x) => x.as_any(),
            NullsafeMemberCallExpressionObject::NullsafeMemberCallExpression(x) => x.as_any(),
            NullsafeMemberCallExpressionObject::ParenthesizedExpression(x) => x.as_any(),
            NullsafeMemberCallExpressionObject::QualifiedName(x) => x.as_any(),
            NullsafeMemberCallExpressionObject::ScopedCallExpression(x) => x.as_any(),
            NullsafeMemberCallExpressionObject::ScopedPropertyAccessExpression(x) => x.as_any(),
            NullsafeMemberCallExpressionObject::String(x) => x.as_any(),
            NullsafeMemberCallExpressionObject::SubscriptExpression(x) => x.as_any(),
            NullsafeMemberCallExpressionObject::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            NullsafeMemberCallExpressionObject::Extra(x) => x.children_any(),
            NullsafeMemberCallExpressionObject::ArrayCreationExpression(x) => x.children_any(),
            NullsafeMemberCallExpressionObject::CastExpression(x) => x.children_any(),
            NullsafeMemberCallExpressionObject::ClassConstantAccessExpression(x) => {
                x.children_any()
            }
            NullsafeMemberCallExpressionObject::DynamicVariableName(x) => x.children_any(),
            NullsafeMemberCallExpressionObject::EncapsedString(x) => x.children_any(),
            NullsafeMemberCallExpressionObject::FunctionCallExpression(x) => x.children_any(),
            NullsafeMemberCallExpressionObject::Heredoc(x) => x.children_any(),
            NullsafeMemberCallExpressionObject::MemberAccessExpression(x) => x.children_any(),
            NullsafeMemberCallExpressionObject::MemberCallExpression(x) => x.children_any(),
            NullsafeMemberCallExpressionObject::Name(x) => x.children_any(),
            NullsafeMemberCallExpressionObject::Nowdoc(x) => x.children_any(),
            NullsafeMemberCallExpressionObject::NullsafeMemberAccessExpression(x) => {
                x.children_any()
            }
            NullsafeMemberCallExpressionObject::NullsafeMemberCallExpression(x) => x.children_any(),
            NullsafeMemberCallExpressionObject::ParenthesizedExpression(x) => x.children_any(),
            NullsafeMemberCallExpressionObject::QualifiedName(x) => x.children_any(),
            NullsafeMemberCallExpressionObject::ScopedCallExpression(x) => x.children_any(),
            NullsafeMemberCallExpressionObject::ScopedPropertyAccessExpression(x) => {
                x.children_any()
            }
            NullsafeMemberCallExpressionObject::String(x) => x.children_any(),
            NullsafeMemberCallExpressionObject::SubscriptExpression(x) => x.children_any(),
            NullsafeMemberCallExpressionObject::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            NullsafeMemberCallExpressionObject::Extra(x) => x.range(),
            NullsafeMemberCallExpressionObject::ArrayCreationExpression(x) => x.range(),
            NullsafeMemberCallExpressionObject::CastExpression(x) => x.range(),
            NullsafeMemberCallExpressionObject::ClassConstantAccessExpression(x) => x.range(),
            NullsafeMemberCallExpressionObject::DynamicVariableName(x) => x.range(),
            NullsafeMemberCallExpressionObject::EncapsedString(x) => x.range(),
            NullsafeMemberCallExpressionObject::FunctionCallExpression(x) => x.range(),
            NullsafeMemberCallExpressionObject::Heredoc(x) => x.range(),
            NullsafeMemberCallExpressionObject::MemberAccessExpression(x) => x.range(),
            NullsafeMemberCallExpressionObject::MemberCallExpression(x) => x.range(),
            NullsafeMemberCallExpressionObject::Name(x) => x.range(),
            NullsafeMemberCallExpressionObject::Nowdoc(x) => x.range(),
            NullsafeMemberCallExpressionObject::NullsafeMemberAccessExpression(x) => x.range(),
            NullsafeMemberCallExpressionObject::NullsafeMemberCallExpression(x) => x.range(),
            NullsafeMemberCallExpressionObject::ParenthesizedExpression(x) => x.range(),
            NullsafeMemberCallExpressionObject::QualifiedName(x) => x.range(),
            NullsafeMemberCallExpressionObject::ScopedCallExpression(x) => x.range(),
            NullsafeMemberCallExpressionObject::ScopedPropertyAccessExpression(x) => x.range(),
            NullsafeMemberCallExpressionObject::String(x) => x.range(),
            NullsafeMemberCallExpressionObject::SubscriptExpression(x) => x.range(),
            NullsafeMemberCallExpressionObject::VariableName(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct NullsafeMemberCallExpressionNode {
    pub range: Range,
    pub arguments: ArgumentsNode,
    pub name: Box<NullsafeMemberCallExpressionName>,
    pub object: Box<NullsafeMemberCallExpressionObject>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NullsafeMemberCallExpressionNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "nullsafe_member_call_expression" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [nullsafe_member_call_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let arguments: ArgumentsNode = node
            .children_by_field_name("arguments", &mut node.walk())
            .map(|chnode1| ArgumentsNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field arguments should exist");
        let name: Box<NullsafeMemberCallExpressionName> = node
            .children_by_field_name("name", &mut node.walk())
            .map(|chnode2| NullsafeMemberCallExpressionName::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .expect("Field name should exist")
            .into();
        let object: Box<NullsafeMemberCallExpressionObject> = node
            .children_by_field_name("object", &mut node.walk())
            .map(|chnode2| NullsafeMemberCallExpressionObject::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .expect("Field object should exist")
            .into();
        Ok(Self {
            range,
            arguments,
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
        "nullsafe_member_call_expression"
    }
}

impl NodeAccess for NullsafeMemberCallExpressionNode {
    fn brief_desc(&self) -> String {
        "NullsafeMemberCallExpressionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::NullsafeMemberCallExpression(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.arguments.as_any());
        child_vec.push(self.name.as_any());
        child_vec.push(self.object.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
