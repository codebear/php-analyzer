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
use crate::autonodes::nullsafe_member_call_expression::NullsafeMemberCallExpressionNode;
use crate::autonodes::parenthesized_expression::ParenthesizedExpressionNode;
use crate::autonodes::qualified_name::QualifiedNameNode;
use crate::autonodes::relative_scope::RelativeScopeNode;
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
pub enum ScopedCallExpressionName {
    _Expression(Box<_ExpressionNode>),
    DynamicVariableName(Box<DynamicVariableNameNode>),
    Name(Box<NameNode>),
    VariableName(Box<VariableNameNode>),
    Extra(ExtraChild),
}

impl NodeParser for ScopedCallExpressionName {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ScopedCallExpressionName::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => ScopedCallExpressionName::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "dynamic_variable_name" => ScopedCallExpressionName::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "name" => ScopedCallExpressionName::Name(Box::new(NameNode::parse(node, source)?)),
            "variable_name" => ScopedCallExpressionName::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| ScopedCallExpressionName::_Expression(y))
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

impl ScopedCallExpressionName {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ScopedCallExpressionName::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => ScopedCallExpressionName::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "dynamic_variable_name" => ScopedCallExpressionName::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "name" => ScopedCallExpressionName::Name(Box::new(NameNode::parse(node, source)?)),
            "variable_name" => ScopedCallExpressionName::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                return Ok(
                    if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| ScopedCallExpressionName::_Expression(y))
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
            ScopedCallExpressionName::Extra(y) => y.kind(),
            ScopedCallExpressionName::_Expression(y) => y.kind(),
            ScopedCallExpressionName::DynamicVariableName(y) => y.kind(),
            ScopedCallExpressionName::Name(y) => y.kind(),
            ScopedCallExpressionName::VariableName(y) => y.kind(),
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
            ScopedCallExpressionName::Extra(x) => x.get_utype(state, emitter),
            ScopedCallExpressionName::_Expression(x) => x.get_utype(state, emitter),
            ScopedCallExpressionName::DynamicVariableName(x) => x.get_utype(state, emitter),
            ScopedCallExpressionName::Name(x) => x.get_utype(state, emitter),
            ScopedCallExpressionName::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ScopedCallExpressionName::Extra(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionName::_Expression(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionName::DynamicVariableName(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionName::Name(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionName::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ScopedCallExpressionName::Extra(x) => x.read_from(state, emitter),
            ScopedCallExpressionName::_Expression(x) => x.read_from(state, emitter),
            ScopedCallExpressionName::DynamicVariableName(x) => x.read_from(state, emitter),
            ScopedCallExpressionName::Name(x) => x.read_from(state, emitter),
            ScopedCallExpressionName::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ScopedCallExpressionName {
    fn brief_desc(&self) -> String {
        match self {
            ScopedCallExpressionName::Extra(x) => {
                format!("ScopedCallExpressionName::extra({})", x.brief_desc())
            }
            ScopedCallExpressionName::_Expression(x) => {
                format!("ScopedCallExpressionName::_expression({})", x.brief_desc())
            }
            ScopedCallExpressionName::DynamicVariableName(x) => format!(
                "ScopedCallExpressionName::dynamic_variable_name({})",
                x.brief_desc()
            ),
            ScopedCallExpressionName::Name(x) => {
                format!("ScopedCallExpressionName::name({})", x.brief_desc())
            }
            ScopedCallExpressionName::VariableName(x) => format!(
                "ScopedCallExpressionName::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ScopedCallExpressionName::Extra(x) => x.as_any(),
            ScopedCallExpressionName::_Expression(x) => x.as_any(),
            ScopedCallExpressionName::DynamicVariableName(x) => x.as_any(),
            ScopedCallExpressionName::Name(x) => x.as_any(),
            ScopedCallExpressionName::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ScopedCallExpressionName::Extra(x) => x.children_any(),
            ScopedCallExpressionName::_Expression(x) => x.children_any(),
            ScopedCallExpressionName::DynamicVariableName(x) => x.children_any(),
            ScopedCallExpressionName::Name(x) => x.children_any(),
            ScopedCallExpressionName::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ScopedCallExpressionName::Extra(x) => x.range(),
            ScopedCallExpressionName::_Expression(x) => x.range(),
            ScopedCallExpressionName::DynamicVariableName(x) => x.range(),
            ScopedCallExpressionName::Name(x) => x.range(),
            ScopedCallExpressionName::VariableName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ScopedCallExpressionScope {
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

impl NodeParser for ScopedCallExpressionScope {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ScopedCallExpressionScope::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => ScopedCallExpressionScope::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "array_creation_expression" => ScopedCallExpressionScope::ArrayCreationExpression(
                Box::new(ArrayCreationExpressionNode::parse(node, source)?),
            ),
            "cast_expression" => ScopedCallExpressionScope::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                ScopedCallExpressionScope::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => ScopedCallExpressionScope::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "encapsed_string" => ScopedCallExpressionScope::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => ScopedCallExpressionScope::FunctionCallExpression(
                Box::new(FunctionCallExpressionNode::parse(node, source)?),
            ),
            "heredoc" => {
                ScopedCallExpressionScope::Heredoc(Box::new(HeredocNode::parse(node, source)?))
            }
            "member_access_expression" => ScopedCallExpressionScope::MemberAccessExpression(
                Box::new(MemberAccessExpressionNode::parse(node, source)?),
            ),
            "member_call_expression" => ScopedCallExpressionScope::MemberCallExpression(Box::new(
                MemberCallExpressionNode::parse(node, source)?,
            )),
            "name" => ScopedCallExpressionScope::Name(Box::new(NameNode::parse(node, source)?)),
            "nowdoc" => {
                ScopedCallExpressionScope::Nowdoc(Box::new(NowdocNode::parse(node, source)?))
            }
            "nullsafe_member_access_expression" => {
                ScopedCallExpressionScope::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                ScopedCallExpressionScope::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => ScopedCallExpressionScope::ParenthesizedExpression(
                Box::new(ParenthesizedExpressionNode::parse(node, source)?),
            ),
            "qualified_name" => ScopedCallExpressionScope::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "relative_scope" => ScopedCallExpressionScope::RelativeScope(Box::new(
                RelativeScopeNode::parse(node, source)?,
            )),
            "scoped_call_expression" => ScopedCallExpressionScope::ScopedCallExpression(Box::new(
                ScopedCallExpressionNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                ScopedCallExpressionScope::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "string" => {
                ScopedCallExpressionScope::String(Box::new(StringNode::parse(node, source)?))
            }
            "subscript_expression" => ScopedCallExpressionScope::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => ScopedCallExpressionScope::VariableName(Box::new(
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

impl ScopedCallExpressionScope {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ScopedCallExpressionScope::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => ScopedCallExpressionScope::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "array_creation_expression" => ScopedCallExpressionScope::ArrayCreationExpression(
                Box::new(ArrayCreationExpressionNode::parse(node, source)?),
            ),
            "cast_expression" => ScopedCallExpressionScope::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                ScopedCallExpressionScope::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => ScopedCallExpressionScope::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "encapsed_string" => ScopedCallExpressionScope::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => ScopedCallExpressionScope::FunctionCallExpression(
                Box::new(FunctionCallExpressionNode::parse(node, source)?),
            ),
            "heredoc" => {
                ScopedCallExpressionScope::Heredoc(Box::new(HeredocNode::parse(node, source)?))
            }
            "member_access_expression" => ScopedCallExpressionScope::MemberAccessExpression(
                Box::new(MemberAccessExpressionNode::parse(node, source)?),
            ),
            "member_call_expression" => ScopedCallExpressionScope::MemberCallExpression(Box::new(
                MemberCallExpressionNode::parse(node, source)?,
            )),
            "name" => ScopedCallExpressionScope::Name(Box::new(NameNode::parse(node, source)?)),
            "nowdoc" => {
                ScopedCallExpressionScope::Nowdoc(Box::new(NowdocNode::parse(node, source)?))
            }
            "nullsafe_member_access_expression" => {
                ScopedCallExpressionScope::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                ScopedCallExpressionScope::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => ScopedCallExpressionScope::ParenthesizedExpression(
                Box::new(ParenthesizedExpressionNode::parse(node, source)?),
            ),
            "qualified_name" => ScopedCallExpressionScope::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "relative_scope" => ScopedCallExpressionScope::RelativeScope(Box::new(
                RelativeScopeNode::parse(node, source)?,
            )),
            "scoped_call_expression" => ScopedCallExpressionScope::ScopedCallExpression(Box::new(
                ScopedCallExpressionNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                ScopedCallExpressionScope::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "string" => {
                ScopedCallExpressionScope::String(Box::new(StringNode::parse(node, source)?))
            }
            "subscript_expression" => ScopedCallExpressionScope::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => ScopedCallExpressionScope::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            ScopedCallExpressionScope::Extra(y) => y.kind(),
            ScopedCallExpressionScope::ArrayCreationExpression(y) => y.kind(),
            ScopedCallExpressionScope::CastExpression(y) => y.kind(),
            ScopedCallExpressionScope::ClassConstantAccessExpression(y) => y.kind(),
            ScopedCallExpressionScope::DynamicVariableName(y) => y.kind(),
            ScopedCallExpressionScope::EncapsedString(y) => y.kind(),
            ScopedCallExpressionScope::FunctionCallExpression(y) => y.kind(),
            ScopedCallExpressionScope::Heredoc(y) => y.kind(),
            ScopedCallExpressionScope::MemberAccessExpression(y) => y.kind(),
            ScopedCallExpressionScope::MemberCallExpression(y) => y.kind(),
            ScopedCallExpressionScope::Name(y) => y.kind(),
            ScopedCallExpressionScope::Nowdoc(y) => y.kind(),
            ScopedCallExpressionScope::NullsafeMemberAccessExpression(y) => y.kind(),
            ScopedCallExpressionScope::NullsafeMemberCallExpression(y) => y.kind(),
            ScopedCallExpressionScope::ParenthesizedExpression(y) => y.kind(),
            ScopedCallExpressionScope::QualifiedName(y) => y.kind(),
            ScopedCallExpressionScope::RelativeScope(y) => y.kind(),
            ScopedCallExpressionScope::ScopedCallExpression(y) => y.kind(),
            ScopedCallExpressionScope::ScopedPropertyAccessExpression(y) => y.kind(),
            ScopedCallExpressionScope::String(y) => y.kind(),
            ScopedCallExpressionScope::SubscriptExpression(y) => y.kind(),
            ScopedCallExpressionScope::VariableName(y) => y.kind(),
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
            ScopedCallExpressionScope::Extra(x) => x.get_utype(state, emitter),
            ScopedCallExpressionScope::ArrayCreationExpression(x) => x.get_utype(state, emitter),
            ScopedCallExpressionScope::CastExpression(x) => x.get_utype(state, emitter),
            ScopedCallExpressionScope::ClassConstantAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            ScopedCallExpressionScope::DynamicVariableName(x) => x.get_utype(state, emitter),
            ScopedCallExpressionScope::EncapsedString(x) => x.get_utype(state, emitter),
            ScopedCallExpressionScope::FunctionCallExpression(x) => x.get_utype(state, emitter),
            ScopedCallExpressionScope::Heredoc(x) => x.get_utype(state, emitter),
            ScopedCallExpressionScope::MemberAccessExpression(x) => x.get_utype(state, emitter),
            ScopedCallExpressionScope::MemberCallExpression(x) => x.get_utype(state, emitter),
            ScopedCallExpressionScope::Name(x) => x.get_utype(state, emitter),
            ScopedCallExpressionScope::Nowdoc(x) => x.get_utype(state, emitter),
            ScopedCallExpressionScope::NullsafeMemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            ScopedCallExpressionScope::NullsafeMemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            ScopedCallExpressionScope::ParenthesizedExpression(x) => x.get_utype(state, emitter),
            ScopedCallExpressionScope::QualifiedName(x) => x.get_utype(state, emitter),
            ScopedCallExpressionScope::RelativeScope(x) => x.get_utype(state, emitter),
            ScopedCallExpressionScope::ScopedCallExpression(x) => x.get_utype(state, emitter),
            ScopedCallExpressionScope::ScopedPropertyAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            ScopedCallExpressionScope::String(x) => x.get_utype(state, emitter),
            ScopedCallExpressionScope::SubscriptExpression(x) => x.get_utype(state, emitter),
            ScopedCallExpressionScope::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ScopedCallExpressionScope::Extra(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionScope::ArrayCreationExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedCallExpressionScope::CastExpression(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionScope::ClassConstantAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedCallExpressionScope::DynamicVariableName(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionScope::EncapsedString(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionScope::FunctionCallExpression(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionScope::Heredoc(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionScope::MemberAccessExpression(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionScope::MemberCallExpression(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionScope::Name(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionScope::Nowdoc(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionScope::NullsafeMemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedCallExpressionScope::NullsafeMemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedCallExpressionScope::ParenthesizedExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedCallExpressionScope::QualifiedName(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionScope::RelativeScope(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionScope::ScopedCallExpression(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionScope::ScopedPropertyAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedCallExpressionScope::String(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionScope::SubscriptExpression(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionScope::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ScopedCallExpressionScope::Extra(x) => x.read_from(state, emitter),
            ScopedCallExpressionScope::ArrayCreationExpression(x) => x.read_from(state, emitter),
            ScopedCallExpressionScope::CastExpression(x) => x.read_from(state, emitter),
            ScopedCallExpressionScope::ClassConstantAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            ScopedCallExpressionScope::DynamicVariableName(x) => x.read_from(state, emitter),
            ScopedCallExpressionScope::EncapsedString(x) => x.read_from(state, emitter),
            ScopedCallExpressionScope::FunctionCallExpression(x) => x.read_from(state, emitter),
            ScopedCallExpressionScope::Heredoc(x) => x.read_from(state, emitter),
            ScopedCallExpressionScope::MemberAccessExpression(x) => x.read_from(state, emitter),
            ScopedCallExpressionScope::MemberCallExpression(x) => x.read_from(state, emitter),
            ScopedCallExpressionScope::Name(x) => x.read_from(state, emitter),
            ScopedCallExpressionScope::Nowdoc(x) => x.read_from(state, emitter),
            ScopedCallExpressionScope::NullsafeMemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            ScopedCallExpressionScope::NullsafeMemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            ScopedCallExpressionScope::ParenthesizedExpression(x) => x.read_from(state, emitter),
            ScopedCallExpressionScope::QualifiedName(x) => x.read_from(state, emitter),
            ScopedCallExpressionScope::RelativeScope(x) => x.read_from(state, emitter),
            ScopedCallExpressionScope::ScopedCallExpression(x) => x.read_from(state, emitter),
            ScopedCallExpressionScope::ScopedPropertyAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            ScopedCallExpressionScope::String(x) => x.read_from(state, emitter),
            ScopedCallExpressionScope::SubscriptExpression(x) => x.read_from(state, emitter),
            ScopedCallExpressionScope::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ScopedCallExpressionScope {
    fn brief_desc(&self) -> String {
        match self {
            ScopedCallExpressionScope::Extra(x) => {
                format!("ScopedCallExpressionScope::extra({})", x.brief_desc())
            }
            ScopedCallExpressionScope::ArrayCreationExpression(x) => format!(
                "ScopedCallExpressionScope::array_creation_expression({})",
                x.brief_desc()
            ),
            ScopedCallExpressionScope::CastExpression(x) => format!(
                "ScopedCallExpressionScope::cast_expression({})",
                x.brief_desc()
            ),
            ScopedCallExpressionScope::ClassConstantAccessExpression(x) => format!(
                "ScopedCallExpressionScope::class_constant_access_expression({})",
                x.brief_desc()
            ),
            ScopedCallExpressionScope::DynamicVariableName(x) => format!(
                "ScopedCallExpressionScope::dynamic_variable_name({})",
                x.brief_desc()
            ),
            ScopedCallExpressionScope::EncapsedString(x) => format!(
                "ScopedCallExpressionScope::encapsed_string({})",
                x.brief_desc()
            ),
            ScopedCallExpressionScope::FunctionCallExpression(x) => format!(
                "ScopedCallExpressionScope::function_call_expression({})",
                x.brief_desc()
            ),
            ScopedCallExpressionScope::Heredoc(x) => {
                format!("ScopedCallExpressionScope::heredoc({})", x.brief_desc())
            }
            ScopedCallExpressionScope::MemberAccessExpression(x) => format!(
                "ScopedCallExpressionScope::member_access_expression({})",
                x.brief_desc()
            ),
            ScopedCallExpressionScope::MemberCallExpression(x) => format!(
                "ScopedCallExpressionScope::member_call_expression({})",
                x.brief_desc()
            ),
            ScopedCallExpressionScope::Name(x) => {
                format!("ScopedCallExpressionScope::name({})", x.brief_desc())
            }
            ScopedCallExpressionScope::Nowdoc(x) => {
                format!("ScopedCallExpressionScope::nowdoc({})", x.brief_desc())
            }
            ScopedCallExpressionScope::NullsafeMemberAccessExpression(x) => format!(
                "ScopedCallExpressionScope::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            ScopedCallExpressionScope::NullsafeMemberCallExpression(x) => format!(
                "ScopedCallExpressionScope::nullsafe_member_call_expression({})",
                x.brief_desc()
            ),
            ScopedCallExpressionScope::ParenthesizedExpression(x) => format!(
                "ScopedCallExpressionScope::parenthesized_expression({})",
                x.brief_desc()
            ),
            ScopedCallExpressionScope::QualifiedName(x) => format!(
                "ScopedCallExpressionScope::qualified_name({})",
                x.brief_desc()
            ),
            ScopedCallExpressionScope::RelativeScope(x) => format!(
                "ScopedCallExpressionScope::relative_scope({})",
                x.brief_desc()
            ),
            ScopedCallExpressionScope::ScopedCallExpression(x) => format!(
                "ScopedCallExpressionScope::scoped_call_expression({})",
                x.brief_desc()
            ),
            ScopedCallExpressionScope::ScopedPropertyAccessExpression(x) => format!(
                "ScopedCallExpressionScope::scoped_property_access_expression({})",
                x.brief_desc()
            ),
            ScopedCallExpressionScope::String(x) => {
                format!("ScopedCallExpressionScope::string({})", x.brief_desc())
            }
            ScopedCallExpressionScope::SubscriptExpression(x) => format!(
                "ScopedCallExpressionScope::subscript_expression({})",
                x.brief_desc()
            ),
            ScopedCallExpressionScope::VariableName(x) => format!(
                "ScopedCallExpressionScope::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ScopedCallExpressionScope::Extra(x) => x.as_any(),
            ScopedCallExpressionScope::ArrayCreationExpression(x) => x.as_any(),
            ScopedCallExpressionScope::CastExpression(x) => x.as_any(),
            ScopedCallExpressionScope::ClassConstantAccessExpression(x) => x.as_any(),
            ScopedCallExpressionScope::DynamicVariableName(x) => x.as_any(),
            ScopedCallExpressionScope::EncapsedString(x) => x.as_any(),
            ScopedCallExpressionScope::FunctionCallExpression(x) => x.as_any(),
            ScopedCallExpressionScope::Heredoc(x) => x.as_any(),
            ScopedCallExpressionScope::MemberAccessExpression(x) => x.as_any(),
            ScopedCallExpressionScope::MemberCallExpression(x) => x.as_any(),
            ScopedCallExpressionScope::Name(x) => x.as_any(),
            ScopedCallExpressionScope::Nowdoc(x) => x.as_any(),
            ScopedCallExpressionScope::NullsafeMemberAccessExpression(x) => x.as_any(),
            ScopedCallExpressionScope::NullsafeMemberCallExpression(x) => x.as_any(),
            ScopedCallExpressionScope::ParenthesizedExpression(x) => x.as_any(),
            ScopedCallExpressionScope::QualifiedName(x) => x.as_any(),
            ScopedCallExpressionScope::RelativeScope(x) => x.as_any(),
            ScopedCallExpressionScope::ScopedCallExpression(x) => x.as_any(),
            ScopedCallExpressionScope::ScopedPropertyAccessExpression(x) => x.as_any(),
            ScopedCallExpressionScope::String(x) => x.as_any(),
            ScopedCallExpressionScope::SubscriptExpression(x) => x.as_any(),
            ScopedCallExpressionScope::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ScopedCallExpressionScope::Extra(x) => x.children_any(),
            ScopedCallExpressionScope::ArrayCreationExpression(x) => x.children_any(),
            ScopedCallExpressionScope::CastExpression(x) => x.children_any(),
            ScopedCallExpressionScope::ClassConstantAccessExpression(x) => x.children_any(),
            ScopedCallExpressionScope::DynamicVariableName(x) => x.children_any(),
            ScopedCallExpressionScope::EncapsedString(x) => x.children_any(),
            ScopedCallExpressionScope::FunctionCallExpression(x) => x.children_any(),
            ScopedCallExpressionScope::Heredoc(x) => x.children_any(),
            ScopedCallExpressionScope::MemberAccessExpression(x) => x.children_any(),
            ScopedCallExpressionScope::MemberCallExpression(x) => x.children_any(),
            ScopedCallExpressionScope::Name(x) => x.children_any(),
            ScopedCallExpressionScope::Nowdoc(x) => x.children_any(),
            ScopedCallExpressionScope::NullsafeMemberAccessExpression(x) => x.children_any(),
            ScopedCallExpressionScope::NullsafeMemberCallExpression(x) => x.children_any(),
            ScopedCallExpressionScope::ParenthesizedExpression(x) => x.children_any(),
            ScopedCallExpressionScope::QualifiedName(x) => x.children_any(),
            ScopedCallExpressionScope::RelativeScope(x) => x.children_any(),
            ScopedCallExpressionScope::ScopedCallExpression(x) => x.children_any(),
            ScopedCallExpressionScope::ScopedPropertyAccessExpression(x) => x.children_any(),
            ScopedCallExpressionScope::String(x) => x.children_any(),
            ScopedCallExpressionScope::SubscriptExpression(x) => x.children_any(),
            ScopedCallExpressionScope::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ScopedCallExpressionScope::Extra(x) => x.range(),
            ScopedCallExpressionScope::ArrayCreationExpression(x) => x.range(),
            ScopedCallExpressionScope::CastExpression(x) => x.range(),
            ScopedCallExpressionScope::ClassConstantAccessExpression(x) => x.range(),
            ScopedCallExpressionScope::DynamicVariableName(x) => x.range(),
            ScopedCallExpressionScope::EncapsedString(x) => x.range(),
            ScopedCallExpressionScope::FunctionCallExpression(x) => x.range(),
            ScopedCallExpressionScope::Heredoc(x) => x.range(),
            ScopedCallExpressionScope::MemberAccessExpression(x) => x.range(),
            ScopedCallExpressionScope::MemberCallExpression(x) => x.range(),
            ScopedCallExpressionScope::Name(x) => x.range(),
            ScopedCallExpressionScope::Nowdoc(x) => x.range(),
            ScopedCallExpressionScope::NullsafeMemberAccessExpression(x) => x.range(),
            ScopedCallExpressionScope::NullsafeMemberCallExpression(x) => x.range(),
            ScopedCallExpressionScope::ParenthesizedExpression(x) => x.range(),
            ScopedCallExpressionScope::QualifiedName(x) => x.range(),
            ScopedCallExpressionScope::RelativeScope(x) => x.range(),
            ScopedCallExpressionScope::ScopedCallExpression(x) => x.range(),
            ScopedCallExpressionScope::ScopedPropertyAccessExpression(x) => x.range(),
            ScopedCallExpressionScope::String(x) => x.range(),
            ScopedCallExpressionScope::SubscriptExpression(x) => x.range(),
            ScopedCallExpressionScope::VariableName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScopedCallExpressionNode {
    pub range: Range,
    pub arguments: ArgumentsNode,
    pub name: Box<ScopedCallExpressionName>,
    pub scope: Box<ScopedCallExpressionScope>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for ScopedCallExpressionNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "scoped_call_expression" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [scoped_call_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let arguments: ArgumentsNode = Result::from(node.parse_child("arguments", source).into())?;
        let name: Box<ScopedCallExpressionName> =
            Result::from(node.parse_child("name", source).into())?;
        let scope: Box<ScopedCallExpressionScope> =
            Result::from(node.parse_child("scope", source).into())?;
        Ok(Self {
            range,
            arguments,
            name,
            scope,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl ScopedCallExpressionNode {
    pub fn kind(&self) -> &'static str {
        "scoped_call_expression"
    }
}

impl NodeAccess for ScopedCallExpressionNode {
    fn brief_desc(&self) -> String {
        "ScopedCallExpressionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::ScopedCallExpression(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.arguments.as_any());
        child_vec.push(self.name.as_any());
        child_vec.push(self.scope.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
