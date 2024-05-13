use crate::analysis::state::AnalysisState;
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
use crate::autonodes::relative_scope::RelativeScopeNode;
use crate::autonodes::scoped_call_expression::ScopedCallExpressionNode;
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
pub enum ScopedPropertyAccessExpressionName {
    DynamicVariableName(Box<DynamicVariableNameNode>),
    VariableName(Box<VariableNameNode>),
    Extra(ExtraChild),
}

impl NodeParser for ScopedPropertyAccessExpressionName {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ScopedPropertyAccessExpressionName::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => ScopedPropertyAccessExpressionName::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "dynamic_variable_name" => ScopedPropertyAccessExpressionName::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "variable_name" => ScopedPropertyAccessExpressionName::VariableName(Box::new(
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

impl ScopedPropertyAccessExpressionName {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ScopedPropertyAccessExpressionName::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => ScopedPropertyAccessExpressionName::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "dynamic_variable_name" => ScopedPropertyAccessExpressionName::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "variable_name" => ScopedPropertyAccessExpressionName::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            ScopedPropertyAccessExpressionName::Extra(y) => y.kind(),
            ScopedPropertyAccessExpressionName::DynamicVariableName(y) => y.kind(),
            ScopedPropertyAccessExpressionName::VariableName(y) => y.kind(),
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
            ScopedPropertyAccessExpressionName::Extra(x) => x.get_utype(state, emitter),
            ScopedPropertyAccessExpressionName::DynamicVariableName(x) => {
                x.get_utype(state, emitter)
            }
            ScopedPropertyAccessExpressionName::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ScopedPropertyAccessExpressionName::Extra(x) => x.get_php_value(state, emitter),
            ScopedPropertyAccessExpressionName::DynamicVariableName(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedPropertyAccessExpressionName::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ScopedPropertyAccessExpressionName::Extra(x) => x.read_from(state, emitter),
            ScopedPropertyAccessExpressionName::DynamicVariableName(x) => {
                x.read_from(state, emitter)
            }
            ScopedPropertyAccessExpressionName::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ScopedPropertyAccessExpressionName {
    fn brief_desc(&self) -> String {
        match self {
            ScopedPropertyAccessExpressionName::Extra(x) => format!(
                "ScopedPropertyAccessExpressionName::extra({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionName::DynamicVariableName(x) => format!(
                "ScopedPropertyAccessExpressionName::dynamic_variable_name({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionName::VariableName(x) => format!(
                "ScopedPropertyAccessExpressionName::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            ScopedPropertyAccessExpressionName::Extra(x) => x.as_any(),
            ScopedPropertyAccessExpressionName::DynamicVariableName(x) => x.as_any(),
            ScopedPropertyAccessExpressionName::VariableName(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            ScopedPropertyAccessExpressionName::Extra(x) => x.children_any(),
            ScopedPropertyAccessExpressionName::DynamicVariableName(x) => x.children_any(),
            ScopedPropertyAccessExpressionName::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ScopedPropertyAccessExpressionName::Extra(x) => x.range(),
            ScopedPropertyAccessExpressionName::DynamicVariableName(x) => x.range(),
            ScopedPropertyAccessExpressionName::VariableName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ScopedPropertyAccessExpressionScope {
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

impl NodeParser for ScopedPropertyAccessExpressionScope {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ScopedPropertyAccessExpressionScope::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => ScopedPropertyAccessExpressionScope::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "array_creation_expression" => {
                ScopedPropertyAccessExpressionScope::ArrayCreationExpression(Box::new(
                    ArrayCreationExpressionNode::parse(node, source)?,
                ))
            }
            "cast_expression" => ScopedPropertyAccessExpressionScope::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                ScopedPropertyAccessExpressionScope::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => ScopedPropertyAccessExpressionScope::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "encapsed_string" => ScopedPropertyAccessExpressionScope::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => {
                ScopedPropertyAccessExpressionScope::FunctionCallExpression(Box::new(
                    FunctionCallExpressionNode::parse(node, source)?,
                ))
            }
            "heredoc" => ScopedPropertyAccessExpressionScope::Heredoc(Box::new(
                HeredocNode::parse(node, source)?,
            )),
            "member_access_expression" => {
                ScopedPropertyAccessExpressionScope::MemberAccessExpression(Box::new(
                    MemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "member_call_expression" => ScopedPropertyAccessExpressionScope::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "name" => {
                ScopedPropertyAccessExpressionScope::Name(Box::new(NameNode::parse(node, source)?))
            }
            "nowdoc" => ScopedPropertyAccessExpressionScope::Nowdoc(Box::new(NowdocNode::parse(
                node, source,
            )?)),
            "nullsafe_member_access_expression" => {
                ScopedPropertyAccessExpressionScope::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                ScopedPropertyAccessExpressionScope::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => {
                ScopedPropertyAccessExpressionScope::ParenthesizedExpression(Box::new(
                    ParenthesizedExpressionNode::parse(node, source)?,
                ))
            }
            "qualified_name" => ScopedPropertyAccessExpressionScope::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "relative_scope" => ScopedPropertyAccessExpressionScope::RelativeScope(Box::new(
                RelativeScopeNode::parse(node, source)?,
            )),
            "scoped_call_expression" => ScopedPropertyAccessExpressionScope::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "scoped_property_access_expression" => {
                ScopedPropertyAccessExpressionScope::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "string" => ScopedPropertyAccessExpressionScope::String(Box::new(StringNode::parse(
                node, source,
            )?)),
            "subscript_expression" => ScopedPropertyAccessExpressionScope::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => ScopedPropertyAccessExpressionScope::VariableName(Box::new(
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

impl ScopedPropertyAccessExpressionScope {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ScopedPropertyAccessExpressionScope::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => ScopedPropertyAccessExpressionScope::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "array_creation_expression" => {
                ScopedPropertyAccessExpressionScope::ArrayCreationExpression(Box::new(
                    ArrayCreationExpressionNode::parse(node, source)?,
                ))
            }
            "cast_expression" => ScopedPropertyAccessExpressionScope::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "class_constant_access_expression" => {
                ScopedPropertyAccessExpressionScope::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "dynamic_variable_name" => ScopedPropertyAccessExpressionScope::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "encapsed_string" => ScopedPropertyAccessExpressionScope::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => {
                ScopedPropertyAccessExpressionScope::FunctionCallExpression(Box::new(
                    FunctionCallExpressionNode::parse(node, source)?,
                ))
            }
            "heredoc" => ScopedPropertyAccessExpressionScope::Heredoc(Box::new(
                HeredocNode::parse(node, source)?,
            )),
            "member_access_expression" => {
                ScopedPropertyAccessExpressionScope::MemberAccessExpression(Box::new(
                    MemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "member_call_expression" => ScopedPropertyAccessExpressionScope::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "name" => {
                ScopedPropertyAccessExpressionScope::Name(Box::new(NameNode::parse(node, source)?))
            }
            "nowdoc" => ScopedPropertyAccessExpressionScope::Nowdoc(Box::new(NowdocNode::parse(
                node, source,
            )?)),
            "nullsafe_member_access_expression" => {
                ScopedPropertyAccessExpressionScope::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                ScopedPropertyAccessExpressionScope::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => {
                ScopedPropertyAccessExpressionScope::ParenthesizedExpression(Box::new(
                    ParenthesizedExpressionNode::parse(node, source)?,
                ))
            }
            "qualified_name" => ScopedPropertyAccessExpressionScope::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "relative_scope" => ScopedPropertyAccessExpressionScope::RelativeScope(Box::new(
                RelativeScopeNode::parse(node, source)?,
            )),
            "scoped_call_expression" => ScopedPropertyAccessExpressionScope::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "scoped_property_access_expression" => {
                ScopedPropertyAccessExpressionScope::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "string" => ScopedPropertyAccessExpressionScope::String(Box::new(StringNode::parse(
                node, source,
            )?)),
            "subscript_expression" => ScopedPropertyAccessExpressionScope::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => ScopedPropertyAccessExpressionScope::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            ScopedPropertyAccessExpressionScope::Extra(y) => y.kind(),
            ScopedPropertyAccessExpressionScope::ArrayCreationExpression(y) => y.kind(),
            ScopedPropertyAccessExpressionScope::CastExpression(y) => y.kind(),
            ScopedPropertyAccessExpressionScope::ClassConstantAccessExpression(y) => y.kind(),
            ScopedPropertyAccessExpressionScope::DynamicVariableName(y) => y.kind(),
            ScopedPropertyAccessExpressionScope::EncapsedString(y) => y.kind(),
            ScopedPropertyAccessExpressionScope::FunctionCallExpression(y) => y.kind(),
            ScopedPropertyAccessExpressionScope::Heredoc(y) => y.kind(),
            ScopedPropertyAccessExpressionScope::MemberAccessExpression(y) => y.kind(),
            ScopedPropertyAccessExpressionScope::MemberCallExpression(y) => y.kind(),
            ScopedPropertyAccessExpressionScope::Name(y) => y.kind(),
            ScopedPropertyAccessExpressionScope::Nowdoc(y) => y.kind(),
            ScopedPropertyAccessExpressionScope::NullsafeMemberAccessExpression(y) => y.kind(),
            ScopedPropertyAccessExpressionScope::NullsafeMemberCallExpression(y) => y.kind(),
            ScopedPropertyAccessExpressionScope::ParenthesizedExpression(y) => y.kind(),
            ScopedPropertyAccessExpressionScope::QualifiedName(y) => y.kind(),
            ScopedPropertyAccessExpressionScope::RelativeScope(y) => y.kind(),
            ScopedPropertyAccessExpressionScope::ScopedCallExpression(y) => y.kind(),
            ScopedPropertyAccessExpressionScope::ScopedPropertyAccessExpression(y) => y.kind(),
            ScopedPropertyAccessExpressionScope::String(y) => y.kind(),
            ScopedPropertyAccessExpressionScope::SubscriptExpression(y) => y.kind(),
            ScopedPropertyAccessExpressionScope::VariableName(y) => y.kind(),
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
            ScopedPropertyAccessExpressionScope::Extra(x) => x.get_utype(state, emitter),
            ScopedPropertyAccessExpressionScope::ArrayCreationExpression(x) => {
                x.get_utype(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::CastExpression(x) => x.get_utype(state, emitter),
            ScopedPropertyAccessExpressionScope::ClassConstantAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::DynamicVariableName(x) => {
                x.get_utype(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::EncapsedString(x) => x.get_utype(state, emitter),
            ScopedPropertyAccessExpressionScope::FunctionCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::Heredoc(x) => x.get_utype(state, emitter),
            ScopedPropertyAccessExpressionScope::MemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::MemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::Name(x) => x.get_utype(state, emitter),
            ScopedPropertyAccessExpressionScope::Nowdoc(x) => x.get_utype(state, emitter),
            ScopedPropertyAccessExpressionScope::NullsafeMemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::NullsafeMemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::ParenthesizedExpression(x) => {
                x.get_utype(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::QualifiedName(x) => x.get_utype(state, emitter),
            ScopedPropertyAccessExpressionScope::RelativeScope(x) => x.get_utype(state, emitter),
            ScopedPropertyAccessExpressionScope::ScopedCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::ScopedPropertyAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::String(x) => x.get_utype(state, emitter),
            ScopedPropertyAccessExpressionScope::SubscriptExpression(x) => {
                x.get_utype(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ScopedPropertyAccessExpressionScope::Extra(x) => x.get_php_value(state, emitter),
            ScopedPropertyAccessExpressionScope::ArrayCreationExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::CastExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::ClassConstantAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::DynamicVariableName(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::EncapsedString(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::FunctionCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::Heredoc(x) => x.get_php_value(state, emitter),
            ScopedPropertyAccessExpressionScope::MemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::MemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::Name(x) => x.get_php_value(state, emitter),
            ScopedPropertyAccessExpressionScope::Nowdoc(x) => x.get_php_value(state, emitter),
            ScopedPropertyAccessExpressionScope::NullsafeMemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::NullsafeMemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::ParenthesizedExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::QualifiedName(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::RelativeScope(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::ScopedCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::ScopedPropertyAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::String(x) => x.get_php_value(state, emitter),
            ScopedPropertyAccessExpressionScope::SubscriptExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ScopedPropertyAccessExpressionScope::Extra(x) => x.read_from(state, emitter),
            ScopedPropertyAccessExpressionScope::ArrayCreationExpression(x) => {
                x.read_from(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::CastExpression(x) => x.read_from(state, emitter),
            ScopedPropertyAccessExpressionScope::ClassConstantAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::DynamicVariableName(x) => {
                x.read_from(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::EncapsedString(x) => x.read_from(state, emitter),
            ScopedPropertyAccessExpressionScope::FunctionCallExpression(x) => {
                x.read_from(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::Heredoc(x) => x.read_from(state, emitter),
            ScopedPropertyAccessExpressionScope::MemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::MemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::Name(x) => x.read_from(state, emitter),
            ScopedPropertyAccessExpressionScope::Nowdoc(x) => x.read_from(state, emitter),
            ScopedPropertyAccessExpressionScope::NullsafeMemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::NullsafeMemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::ParenthesizedExpression(x) => {
                x.read_from(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::QualifiedName(x) => x.read_from(state, emitter),
            ScopedPropertyAccessExpressionScope::RelativeScope(x) => x.read_from(state, emitter),
            ScopedPropertyAccessExpressionScope::ScopedCallExpression(x) => {
                x.read_from(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::ScopedPropertyAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::String(x) => x.read_from(state, emitter),
            ScopedPropertyAccessExpressionScope::SubscriptExpression(x) => {
                x.read_from(state, emitter)
            }
            ScopedPropertyAccessExpressionScope::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ScopedPropertyAccessExpressionScope {
    fn brief_desc(&self) -> String {
        match self {
            ScopedPropertyAccessExpressionScope::Extra(x) => format!(
                "ScopedPropertyAccessExpressionScope::extra({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionScope::ArrayCreationExpression(x) => format!(
                "ScopedPropertyAccessExpressionScope::array_creation_expression({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionScope::CastExpression(x) => format!(
                "ScopedPropertyAccessExpressionScope::cast_expression({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionScope::ClassConstantAccessExpression(x) => format!(
                "ScopedPropertyAccessExpressionScope::class_constant_access_expression({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionScope::DynamicVariableName(x) => format!(
                "ScopedPropertyAccessExpressionScope::dynamic_variable_name({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionScope::EncapsedString(x) => format!(
                "ScopedPropertyAccessExpressionScope::encapsed_string({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionScope::FunctionCallExpression(x) => format!(
                "ScopedPropertyAccessExpressionScope::function_call_expression({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionScope::Heredoc(x) => format!(
                "ScopedPropertyAccessExpressionScope::heredoc({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionScope::MemberAccessExpression(x) => format!(
                "ScopedPropertyAccessExpressionScope::member_access_expression({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionScope::MemberCallExpression(x) => format!(
                "ScopedPropertyAccessExpressionScope::member_call_expression({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionScope::Name(x) => format!(
                "ScopedPropertyAccessExpressionScope::name({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionScope::Nowdoc(x) => format!(
                "ScopedPropertyAccessExpressionScope::nowdoc({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionScope::NullsafeMemberAccessExpression(x) => format!(
                "ScopedPropertyAccessExpressionScope::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionScope::NullsafeMemberCallExpression(x) => format!(
                "ScopedPropertyAccessExpressionScope::nullsafe_member_call_expression({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionScope::ParenthesizedExpression(x) => format!(
                "ScopedPropertyAccessExpressionScope::parenthesized_expression({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionScope::QualifiedName(x) => format!(
                "ScopedPropertyAccessExpressionScope::qualified_name({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionScope::RelativeScope(x) => format!(
                "ScopedPropertyAccessExpressionScope::relative_scope({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionScope::ScopedCallExpression(x) => format!(
                "ScopedPropertyAccessExpressionScope::scoped_call_expression({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionScope::ScopedPropertyAccessExpression(x) => format!(
                "ScopedPropertyAccessExpressionScope::scoped_property_access_expression({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionScope::String(x) => format!(
                "ScopedPropertyAccessExpressionScope::string({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionScope::SubscriptExpression(x) => format!(
                "ScopedPropertyAccessExpressionScope::subscript_expression({})",
                x.brief_desc()
            ),
            ScopedPropertyAccessExpressionScope::VariableName(x) => format!(
                "ScopedPropertyAccessExpressionScope::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            ScopedPropertyAccessExpressionScope::Extra(x) => x.as_any(),
            ScopedPropertyAccessExpressionScope::ArrayCreationExpression(x) => x.as_any(),
            ScopedPropertyAccessExpressionScope::CastExpression(x) => x.as_any(),
            ScopedPropertyAccessExpressionScope::ClassConstantAccessExpression(x) => x.as_any(),
            ScopedPropertyAccessExpressionScope::DynamicVariableName(x) => x.as_any(),
            ScopedPropertyAccessExpressionScope::EncapsedString(x) => x.as_any(),
            ScopedPropertyAccessExpressionScope::FunctionCallExpression(x) => x.as_any(),
            ScopedPropertyAccessExpressionScope::Heredoc(x) => x.as_any(),
            ScopedPropertyAccessExpressionScope::MemberAccessExpression(x) => x.as_any(),
            ScopedPropertyAccessExpressionScope::MemberCallExpression(x) => x.as_any(),
            ScopedPropertyAccessExpressionScope::Name(x) => x.as_any(),
            ScopedPropertyAccessExpressionScope::Nowdoc(x) => x.as_any(),
            ScopedPropertyAccessExpressionScope::NullsafeMemberAccessExpression(x) => x.as_any(),
            ScopedPropertyAccessExpressionScope::NullsafeMemberCallExpression(x) => x.as_any(),
            ScopedPropertyAccessExpressionScope::ParenthesizedExpression(x) => x.as_any(),
            ScopedPropertyAccessExpressionScope::QualifiedName(x) => x.as_any(),
            ScopedPropertyAccessExpressionScope::RelativeScope(x) => x.as_any(),
            ScopedPropertyAccessExpressionScope::ScopedCallExpression(x) => x.as_any(),
            ScopedPropertyAccessExpressionScope::ScopedPropertyAccessExpression(x) => x.as_any(),
            ScopedPropertyAccessExpressionScope::String(x) => x.as_any(),
            ScopedPropertyAccessExpressionScope::SubscriptExpression(x) => x.as_any(),
            ScopedPropertyAccessExpressionScope::VariableName(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            ScopedPropertyAccessExpressionScope::Extra(x) => x.children_any(),
            ScopedPropertyAccessExpressionScope::ArrayCreationExpression(x) => x.children_any(),
            ScopedPropertyAccessExpressionScope::CastExpression(x) => x.children_any(),
            ScopedPropertyAccessExpressionScope::ClassConstantAccessExpression(x) => {
                x.children_any()
            }
            ScopedPropertyAccessExpressionScope::DynamicVariableName(x) => x.children_any(),
            ScopedPropertyAccessExpressionScope::EncapsedString(x) => x.children_any(),
            ScopedPropertyAccessExpressionScope::FunctionCallExpression(x) => x.children_any(),
            ScopedPropertyAccessExpressionScope::Heredoc(x) => x.children_any(),
            ScopedPropertyAccessExpressionScope::MemberAccessExpression(x) => x.children_any(),
            ScopedPropertyAccessExpressionScope::MemberCallExpression(x) => x.children_any(),
            ScopedPropertyAccessExpressionScope::Name(x) => x.children_any(),
            ScopedPropertyAccessExpressionScope::Nowdoc(x) => x.children_any(),
            ScopedPropertyAccessExpressionScope::NullsafeMemberAccessExpression(x) => {
                x.children_any()
            }
            ScopedPropertyAccessExpressionScope::NullsafeMemberCallExpression(x) => {
                x.children_any()
            }
            ScopedPropertyAccessExpressionScope::ParenthesizedExpression(x) => x.children_any(),
            ScopedPropertyAccessExpressionScope::QualifiedName(x) => x.children_any(),
            ScopedPropertyAccessExpressionScope::RelativeScope(x) => x.children_any(),
            ScopedPropertyAccessExpressionScope::ScopedCallExpression(x) => x.children_any(),
            ScopedPropertyAccessExpressionScope::ScopedPropertyAccessExpression(x) => {
                x.children_any()
            }
            ScopedPropertyAccessExpressionScope::String(x) => x.children_any(),
            ScopedPropertyAccessExpressionScope::SubscriptExpression(x) => x.children_any(),
            ScopedPropertyAccessExpressionScope::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ScopedPropertyAccessExpressionScope::Extra(x) => x.range(),
            ScopedPropertyAccessExpressionScope::ArrayCreationExpression(x) => x.range(),
            ScopedPropertyAccessExpressionScope::CastExpression(x) => x.range(),
            ScopedPropertyAccessExpressionScope::ClassConstantAccessExpression(x) => x.range(),
            ScopedPropertyAccessExpressionScope::DynamicVariableName(x) => x.range(),
            ScopedPropertyAccessExpressionScope::EncapsedString(x) => x.range(),
            ScopedPropertyAccessExpressionScope::FunctionCallExpression(x) => x.range(),
            ScopedPropertyAccessExpressionScope::Heredoc(x) => x.range(),
            ScopedPropertyAccessExpressionScope::MemberAccessExpression(x) => x.range(),
            ScopedPropertyAccessExpressionScope::MemberCallExpression(x) => x.range(),
            ScopedPropertyAccessExpressionScope::Name(x) => x.range(),
            ScopedPropertyAccessExpressionScope::Nowdoc(x) => x.range(),
            ScopedPropertyAccessExpressionScope::NullsafeMemberAccessExpression(x) => x.range(),
            ScopedPropertyAccessExpressionScope::NullsafeMemberCallExpression(x) => x.range(),
            ScopedPropertyAccessExpressionScope::ParenthesizedExpression(x) => x.range(),
            ScopedPropertyAccessExpressionScope::QualifiedName(x) => x.range(),
            ScopedPropertyAccessExpressionScope::RelativeScope(x) => x.range(),
            ScopedPropertyAccessExpressionScope::ScopedCallExpression(x) => x.range(),
            ScopedPropertyAccessExpressionScope::ScopedPropertyAccessExpression(x) => x.range(),
            ScopedPropertyAccessExpressionScope::String(x) => x.range(),
            ScopedPropertyAccessExpressionScope::SubscriptExpression(x) => x.range(),
            ScopedPropertyAccessExpressionScope::VariableName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScopedPropertyAccessExpressionNode {
    pub range: Range,
    pub name: Box<ScopedPropertyAccessExpressionName>,
    pub scope: Box<ScopedPropertyAccessExpressionScope>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for ScopedPropertyAccessExpressionNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "scoped_property_access_expression" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [scoped_property_access_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let name: Box<ScopedPropertyAccessExpressionName> =
            Into::<Result<_, _>>::into(node.parse_child("name", source))?;
        let scope: Box<ScopedPropertyAccessExpressionScope> =
            Into::<Result<_, _>>::into(node.parse_child("scope", source))?;
        Ok(Self {
            range,
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

impl ScopedPropertyAccessExpressionNode {
    pub fn kind(&self) -> &'static str {
        "scoped_property_access_expression"
    }
}

impl NodeAccess for ScopedPropertyAccessExpressionNode {
    fn brief_desc(&self) -> String {
        "ScopedPropertyAccessExpressionNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::ScopedPropertyAccessExpression(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.name.as_any());
        child_vec.push(self.scope.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
