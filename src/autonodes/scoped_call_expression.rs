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
use crate::autonodes::nullsafe_member_access_expression::NullsafeMemberAccessExpressionNode;
use crate::autonodes::nullsafe_member_call_expression::NullsafeMemberCallExpressionNode;
use crate::autonodes::parenthesized_expression::ParenthesizedExpressionNode;
use crate::autonodes::qualified_name::QualifiedNameNode;
use crate::autonodes::relative_scope::RelativeScopeNode;
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
pub enum ScopedCallExpressionName {
    _Expression(Box<_ExpressionNode>),
    DynamicVariableName(Box<DynamicVariableNameNode>),
    Name(Box<NameNode>),
    VariableName(Box<VariableNameNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl ScopedCallExpressionName {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => {
                ScopedCallExpressionName::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => ScopedCallExpressionName::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => ScopedCallExpressionName::Error(Box::new(ErrorNode::parse(node, source)?)),
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

    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => {
                ScopedCallExpressionName::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => ScopedCallExpressionName::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => ScopedCallExpressionName::Error(Box::new(ErrorNode::parse(node, source)?)),
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
            ScopedCallExpressionName::Comment(x) => x.get_utype(state, emitter),
            ScopedCallExpressionName::TextInterpolation(x) => x.get_utype(state, emitter),
            ScopedCallExpressionName::Error(x) => x.get_utype(state, emitter),
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
            ScopedCallExpressionName::Comment(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionName::TextInterpolation(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionName::Error(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionName::_Expression(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionName::DynamicVariableName(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionName::Name(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionName::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ScopedCallExpressionName::Comment(x) => x.read_from(state, emitter),
            ScopedCallExpressionName::TextInterpolation(x) => x.read_from(state, emitter),
            ScopedCallExpressionName::Error(x) => x.read_from(state, emitter),
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
            ScopedCallExpressionName::Comment(x) => {
                format!("ScopedCallExpressionName::comment({})", x.brief_desc())
            }
            ScopedCallExpressionName::TextInterpolation(x) => format!(
                "ScopedCallExpressionName::text_interpolation({})",
                x.brief_desc()
            ),
            ScopedCallExpressionName::Error(x) => {
                format!("ScopedCallExpressionName::ERROR({})", x.brief_desc())
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
            ScopedCallExpressionName::Comment(x) => x.as_any(),
            ScopedCallExpressionName::TextInterpolation(x) => x.as_any(),
            ScopedCallExpressionName::Error(x) => x.as_any(),
            ScopedCallExpressionName::_Expression(x) => x.as_any(),
            ScopedCallExpressionName::DynamicVariableName(x) => x.as_any(),
            ScopedCallExpressionName::Name(x) => x.as_any(),
            ScopedCallExpressionName::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ScopedCallExpressionName::Comment(x) => x.children_any(),
            ScopedCallExpressionName::TextInterpolation(x) => x.children_any(),
            ScopedCallExpressionName::Error(x) => x.children_any(),
            ScopedCallExpressionName::_Expression(x) => x.children_any(),
            ScopedCallExpressionName::DynamicVariableName(x) => x.children_any(),
            ScopedCallExpressionName::Name(x) => x.children_any(),
            ScopedCallExpressionName::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ScopedCallExpressionName::Comment(x) => x.range(),
            ScopedCallExpressionName::TextInterpolation(x) => x.range(),
            ScopedCallExpressionName::Error(x) => x.range(),
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
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl ScopedCallExpressionScope {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => {
                ScopedCallExpressionScope::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => ScopedCallExpressionScope::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => ScopedCallExpressionScope::Error(Box::new(ErrorNode::parse(node, source)?)),
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

    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => {
                ScopedCallExpressionScope::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => ScopedCallExpressionScope::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => ScopedCallExpressionScope::Error(Box::new(ErrorNode::parse(node, source)?)),
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
            ScopedCallExpressionScope::Comment(x) => x.get_utype(state, emitter),
            ScopedCallExpressionScope::TextInterpolation(x) => x.get_utype(state, emitter),
            ScopedCallExpressionScope::Error(x) => x.get_utype(state, emitter),
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
            ScopedCallExpressionScope::Comment(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionScope::TextInterpolation(x) => x.get_php_value(state, emitter),
            ScopedCallExpressionScope::Error(x) => x.get_php_value(state, emitter),
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
            ScopedCallExpressionScope::Comment(x) => x.read_from(state, emitter),
            ScopedCallExpressionScope::TextInterpolation(x) => x.read_from(state, emitter),
            ScopedCallExpressionScope::Error(x) => x.read_from(state, emitter),
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
            ScopedCallExpressionScope::Comment(x) => {
                format!("ScopedCallExpressionScope::comment({})", x.brief_desc())
            }
            ScopedCallExpressionScope::TextInterpolation(x) => format!(
                "ScopedCallExpressionScope::text_interpolation({})",
                x.brief_desc()
            ),
            ScopedCallExpressionScope::Error(x) => {
                format!("ScopedCallExpressionScope::ERROR({})", x.brief_desc())
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
            ScopedCallExpressionScope::Comment(x) => x.as_any(),
            ScopedCallExpressionScope::TextInterpolation(x) => x.as_any(),
            ScopedCallExpressionScope::Error(x) => x.as_any(),
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
            ScopedCallExpressionScope::Comment(x) => x.children_any(),
            ScopedCallExpressionScope::TextInterpolation(x) => x.children_any(),
            ScopedCallExpressionScope::Error(x) => x.children_any(),
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
            ScopedCallExpressionScope::Comment(x) => x.range(),
            ScopedCallExpressionScope::TextInterpolation(x) => x.range(),
            ScopedCallExpressionScope::Error(x) => x.range(),
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

impl ScopedCallExpressionNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "scoped_call_expression" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [scoped_call_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let arguments: ArgumentsNode = node
            .children_by_field_name("arguments", &mut node.walk())
            .map(|chnode1| ArgumentsNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field arguments should exist");
        let name: Box<ScopedCallExpressionName> = node
            .children_by_field_name("name", &mut node.walk())
            .map(|chnode2| ScopedCallExpressionName::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .expect("Field name should exist")
            .into();
        let scope: Box<ScopedCallExpressionScope> = node
            .children_by_field_name("scope", &mut node.walk())
            .map(|chnode2| ScopedCallExpressionScope::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .expect("Field scope should exist")
            .into();
        Ok(Self {
            range,
            arguments,
            name,
            scope,
            extras: vec![], // todo lookup unused nodes
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
