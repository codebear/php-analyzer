use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::arguments::ArgumentsNode;
use crate::autonodes::array_creation_expression::ArrayCreationExpressionNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::dynamic_variable_name::DynamicVariableNameNode;
use crate::autonodes::encapsed_string::EncapsedStringNode;
use crate::autonodes::heredoc::HeredocNode;
use crate::autonodes::member_call_expression::MemberCallExpressionNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::nowdoc::NowdocNode;
use crate::autonodes::nullsafe_member_call_expression::NullsafeMemberCallExpressionNode;
use crate::autonodes::parenthesized_expression::ParenthesizedExpressionNode;
use crate::autonodes::qualified_name::QualifiedNameNode;
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
pub enum FunctionCallExpressionFunction {
    ArrayCreationExpression(Box<ArrayCreationExpressionNode>),
    DynamicVariableName(Box<DynamicVariableNameNode>),
    EncapsedString(Box<EncapsedStringNode>),
    FunctionCallExpression(Box<FunctionCallExpressionNode>),
    Heredoc(Box<HeredocNode>),
    MemberCallExpression(Box<MemberCallExpressionNode>),
    Name(Box<NameNode>),
    Nowdoc(Box<NowdocNode>),
    NullsafeMemberCallExpression(Box<NullsafeMemberCallExpressionNode>),
    ParenthesizedExpression(Box<ParenthesizedExpressionNode>),
    QualifiedName(Box<QualifiedNameNode>),
    ScopedCallExpression(Box<ScopedCallExpressionNode>),
    String(Box<StringNode>),
    SubscriptExpression(Box<SubscriptExpressionNode>),
    VariableName(Box<VariableNameNode>),
    Extra(ExtraChild),
}

impl NodeParser for FunctionCallExpressionFunction {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => FunctionCallExpressionFunction::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => FunctionCallExpressionFunction::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "array_creation_expression" => FunctionCallExpressionFunction::ArrayCreationExpression(
                Box::new(ArrayCreationExpressionNode::parse(node, source)?),
            ),
            "dynamic_variable_name" => FunctionCallExpressionFunction::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "encapsed_string" => FunctionCallExpressionFunction::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => FunctionCallExpressionFunction::FunctionCallExpression(
                Box::new(FunctionCallExpressionNode::parse(node, source)?),
            ),
            "heredoc" => {
                FunctionCallExpressionFunction::Heredoc(Box::new(HeredocNode::parse(node, source)?))
            }
            "member_call_expression" => FunctionCallExpressionFunction::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "name" => {
                FunctionCallExpressionFunction::Name(Box::new(NameNode::parse(node, source)?))
            }
            "nowdoc" => {
                FunctionCallExpressionFunction::Nowdoc(Box::new(NowdocNode::parse(node, source)?))
            }
            "nullsafe_member_call_expression" => {
                FunctionCallExpressionFunction::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => FunctionCallExpressionFunction::ParenthesizedExpression(
                Box::new(ParenthesizedExpressionNode::parse(node, source)?),
            ),
            "qualified_name" => FunctionCallExpressionFunction::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_call_expression" => FunctionCallExpressionFunction::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "string" => {
                FunctionCallExpressionFunction::String(Box::new(StringNode::parse(node, source)?))
            }
            "subscript_expression" => FunctionCallExpressionFunction::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => FunctionCallExpressionFunction::VariableName(Box::new(
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

impl FunctionCallExpressionFunction {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => FunctionCallExpressionFunction::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => FunctionCallExpressionFunction::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "array_creation_expression" => FunctionCallExpressionFunction::ArrayCreationExpression(
                Box::new(ArrayCreationExpressionNode::parse(node, source)?),
            ),
            "dynamic_variable_name" => FunctionCallExpressionFunction::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "encapsed_string" => FunctionCallExpressionFunction::EncapsedString(Box::new(
                EncapsedStringNode::parse(node, source)?,
            )),
            "function_call_expression" => FunctionCallExpressionFunction::FunctionCallExpression(
                Box::new(FunctionCallExpressionNode::parse(node, source)?),
            ),
            "heredoc" => {
                FunctionCallExpressionFunction::Heredoc(Box::new(HeredocNode::parse(node, source)?))
            }
            "member_call_expression" => FunctionCallExpressionFunction::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "name" => {
                FunctionCallExpressionFunction::Name(Box::new(NameNode::parse(node, source)?))
            }
            "nowdoc" => {
                FunctionCallExpressionFunction::Nowdoc(Box::new(NowdocNode::parse(node, source)?))
            }
            "nullsafe_member_call_expression" => {
                FunctionCallExpressionFunction::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "parenthesized_expression" => FunctionCallExpressionFunction::ParenthesizedExpression(
                Box::new(ParenthesizedExpressionNode::parse(node, source)?),
            ),
            "qualified_name" => FunctionCallExpressionFunction::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_call_expression" => FunctionCallExpressionFunction::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "string" => {
                FunctionCallExpressionFunction::String(Box::new(StringNode::parse(node, source)?))
            }
            "subscript_expression" => FunctionCallExpressionFunction::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => FunctionCallExpressionFunction::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            FunctionCallExpressionFunction::Extra(y) => y.kind(),
            FunctionCallExpressionFunction::ArrayCreationExpression(y) => y.kind(),
            FunctionCallExpressionFunction::DynamicVariableName(y) => y.kind(),
            FunctionCallExpressionFunction::EncapsedString(y) => y.kind(),
            FunctionCallExpressionFunction::FunctionCallExpression(y) => y.kind(),
            FunctionCallExpressionFunction::Heredoc(y) => y.kind(),
            FunctionCallExpressionFunction::MemberCallExpression(y) => y.kind(),
            FunctionCallExpressionFunction::Name(y) => y.kind(),
            FunctionCallExpressionFunction::Nowdoc(y) => y.kind(),
            FunctionCallExpressionFunction::NullsafeMemberCallExpression(y) => y.kind(),
            FunctionCallExpressionFunction::ParenthesizedExpression(y) => y.kind(),
            FunctionCallExpressionFunction::QualifiedName(y) => y.kind(),
            FunctionCallExpressionFunction::ScopedCallExpression(y) => y.kind(),
            FunctionCallExpressionFunction::String(y) => y.kind(),
            FunctionCallExpressionFunction::SubscriptExpression(y) => y.kind(),
            FunctionCallExpressionFunction::VariableName(y) => y.kind(),
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
            FunctionCallExpressionFunction::Extra(x) => x.get_utype(state, emitter),
            FunctionCallExpressionFunction::ArrayCreationExpression(x) => {
                x.get_utype(state, emitter)
            }
            FunctionCallExpressionFunction::DynamicVariableName(x) => x.get_utype(state, emitter),
            FunctionCallExpressionFunction::EncapsedString(x) => x.get_utype(state, emitter),
            FunctionCallExpressionFunction::FunctionCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            FunctionCallExpressionFunction::Heredoc(x) => x.get_utype(state, emitter),
            FunctionCallExpressionFunction::MemberCallExpression(x) => x.get_utype(state, emitter),
            FunctionCallExpressionFunction::Name(x) => x.get_utype(state, emitter),
            FunctionCallExpressionFunction::Nowdoc(x) => x.get_utype(state, emitter),
            FunctionCallExpressionFunction::NullsafeMemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            FunctionCallExpressionFunction::ParenthesizedExpression(x) => {
                x.get_utype(state, emitter)
            }
            FunctionCallExpressionFunction::QualifiedName(x) => x.get_utype(state, emitter),
            FunctionCallExpressionFunction::ScopedCallExpression(x) => x.get_utype(state, emitter),
            FunctionCallExpressionFunction::String(x) => x.get_utype(state, emitter),
            FunctionCallExpressionFunction::SubscriptExpression(x) => x.get_utype(state, emitter),
            FunctionCallExpressionFunction::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            FunctionCallExpressionFunction::Extra(x) => x.get_php_value(state, emitter),
            FunctionCallExpressionFunction::ArrayCreationExpression(x) => {
                x.get_php_value(state, emitter)
            }
            FunctionCallExpressionFunction::DynamicVariableName(x) => {
                x.get_php_value(state, emitter)
            }
            FunctionCallExpressionFunction::EncapsedString(x) => x.get_php_value(state, emitter),
            FunctionCallExpressionFunction::FunctionCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            FunctionCallExpressionFunction::Heredoc(x) => x.get_php_value(state, emitter),
            FunctionCallExpressionFunction::MemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            FunctionCallExpressionFunction::Name(x) => x.get_php_value(state, emitter),
            FunctionCallExpressionFunction::Nowdoc(x) => x.get_php_value(state, emitter),
            FunctionCallExpressionFunction::NullsafeMemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            FunctionCallExpressionFunction::ParenthesizedExpression(x) => {
                x.get_php_value(state, emitter)
            }
            FunctionCallExpressionFunction::QualifiedName(x) => x.get_php_value(state, emitter),
            FunctionCallExpressionFunction::ScopedCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            FunctionCallExpressionFunction::String(x) => x.get_php_value(state, emitter),
            FunctionCallExpressionFunction::SubscriptExpression(x) => {
                x.get_php_value(state, emitter)
            }
            FunctionCallExpressionFunction::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            FunctionCallExpressionFunction::Extra(x) => x.read_from(state, emitter),
            FunctionCallExpressionFunction::ArrayCreationExpression(x) => {
                x.read_from(state, emitter)
            }
            FunctionCallExpressionFunction::DynamicVariableName(x) => x.read_from(state, emitter),
            FunctionCallExpressionFunction::EncapsedString(x) => x.read_from(state, emitter),
            FunctionCallExpressionFunction::FunctionCallExpression(x) => {
                x.read_from(state, emitter)
            }
            FunctionCallExpressionFunction::Heredoc(x) => x.read_from(state, emitter),
            FunctionCallExpressionFunction::MemberCallExpression(x) => x.read_from(state, emitter),
            FunctionCallExpressionFunction::Name(x) => x.read_from(state, emitter),
            FunctionCallExpressionFunction::Nowdoc(x) => x.read_from(state, emitter),
            FunctionCallExpressionFunction::NullsafeMemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            FunctionCallExpressionFunction::ParenthesizedExpression(x) => {
                x.read_from(state, emitter)
            }
            FunctionCallExpressionFunction::QualifiedName(x) => x.read_from(state, emitter),
            FunctionCallExpressionFunction::ScopedCallExpression(x) => x.read_from(state, emitter),
            FunctionCallExpressionFunction::String(x) => x.read_from(state, emitter),
            FunctionCallExpressionFunction::SubscriptExpression(x) => x.read_from(state, emitter),
            FunctionCallExpressionFunction::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for FunctionCallExpressionFunction {
    fn brief_desc(&self) -> String {
        match self {
            FunctionCallExpressionFunction::Extra(x) => {
                format!("FunctionCallExpressionFunction::extra({})", x.brief_desc())
            }
            FunctionCallExpressionFunction::ArrayCreationExpression(x) => format!(
                "FunctionCallExpressionFunction::array_creation_expression({})",
                x.brief_desc()
            ),
            FunctionCallExpressionFunction::DynamicVariableName(x) => format!(
                "FunctionCallExpressionFunction::dynamic_variable_name({})",
                x.brief_desc()
            ),
            FunctionCallExpressionFunction::EncapsedString(x) => format!(
                "FunctionCallExpressionFunction::encapsed_string({})",
                x.brief_desc()
            ),
            FunctionCallExpressionFunction::FunctionCallExpression(x) => format!(
                "FunctionCallExpressionFunction::function_call_expression({})",
                x.brief_desc()
            ),
            FunctionCallExpressionFunction::Heredoc(x) => format!(
                "FunctionCallExpressionFunction::heredoc({})",
                x.brief_desc()
            ),
            FunctionCallExpressionFunction::MemberCallExpression(x) => format!(
                "FunctionCallExpressionFunction::member_call_expression({})",
                x.brief_desc()
            ),
            FunctionCallExpressionFunction::Name(x) => {
                format!("FunctionCallExpressionFunction::name({})", x.brief_desc())
            }
            FunctionCallExpressionFunction::Nowdoc(x) => {
                format!("FunctionCallExpressionFunction::nowdoc({})", x.brief_desc())
            }
            FunctionCallExpressionFunction::NullsafeMemberCallExpression(x) => format!(
                "FunctionCallExpressionFunction::nullsafe_member_call_expression({})",
                x.brief_desc()
            ),
            FunctionCallExpressionFunction::ParenthesizedExpression(x) => format!(
                "FunctionCallExpressionFunction::parenthesized_expression({})",
                x.brief_desc()
            ),
            FunctionCallExpressionFunction::QualifiedName(x) => format!(
                "FunctionCallExpressionFunction::qualified_name({})",
                x.brief_desc()
            ),
            FunctionCallExpressionFunction::ScopedCallExpression(x) => format!(
                "FunctionCallExpressionFunction::scoped_call_expression({})",
                x.brief_desc()
            ),
            FunctionCallExpressionFunction::String(x) => {
                format!("FunctionCallExpressionFunction::string({})", x.brief_desc())
            }
            FunctionCallExpressionFunction::SubscriptExpression(x) => format!(
                "FunctionCallExpressionFunction::subscript_expression({})",
                x.brief_desc()
            ),
            FunctionCallExpressionFunction::VariableName(x) => format!(
                "FunctionCallExpressionFunction::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            FunctionCallExpressionFunction::Extra(x) => x.as_any(),
            FunctionCallExpressionFunction::ArrayCreationExpression(x) => x.as_any(),
            FunctionCallExpressionFunction::DynamicVariableName(x) => x.as_any(),
            FunctionCallExpressionFunction::EncapsedString(x) => x.as_any(),
            FunctionCallExpressionFunction::FunctionCallExpression(x) => x.as_any(),
            FunctionCallExpressionFunction::Heredoc(x) => x.as_any(),
            FunctionCallExpressionFunction::MemberCallExpression(x) => x.as_any(),
            FunctionCallExpressionFunction::Name(x) => x.as_any(),
            FunctionCallExpressionFunction::Nowdoc(x) => x.as_any(),
            FunctionCallExpressionFunction::NullsafeMemberCallExpression(x) => x.as_any(),
            FunctionCallExpressionFunction::ParenthesizedExpression(x) => x.as_any(),
            FunctionCallExpressionFunction::QualifiedName(x) => x.as_any(),
            FunctionCallExpressionFunction::ScopedCallExpression(x) => x.as_any(),
            FunctionCallExpressionFunction::String(x) => x.as_any(),
            FunctionCallExpressionFunction::SubscriptExpression(x) => x.as_any(),
            FunctionCallExpressionFunction::VariableName(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            FunctionCallExpressionFunction::Extra(x) => x.children_any(),
            FunctionCallExpressionFunction::ArrayCreationExpression(x) => x.children_any(),
            FunctionCallExpressionFunction::DynamicVariableName(x) => x.children_any(),
            FunctionCallExpressionFunction::EncapsedString(x) => x.children_any(),
            FunctionCallExpressionFunction::FunctionCallExpression(x) => x.children_any(),
            FunctionCallExpressionFunction::Heredoc(x) => x.children_any(),
            FunctionCallExpressionFunction::MemberCallExpression(x) => x.children_any(),
            FunctionCallExpressionFunction::Name(x) => x.children_any(),
            FunctionCallExpressionFunction::Nowdoc(x) => x.children_any(),
            FunctionCallExpressionFunction::NullsafeMemberCallExpression(x) => x.children_any(),
            FunctionCallExpressionFunction::ParenthesizedExpression(x) => x.children_any(),
            FunctionCallExpressionFunction::QualifiedName(x) => x.children_any(),
            FunctionCallExpressionFunction::ScopedCallExpression(x) => x.children_any(),
            FunctionCallExpressionFunction::String(x) => x.children_any(),
            FunctionCallExpressionFunction::SubscriptExpression(x) => x.children_any(),
            FunctionCallExpressionFunction::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            FunctionCallExpressionFunction::Extra(x) => x.range(),
            FunctionCallExpressionFunction::ArrayCreationExpression(x) => x.range(),
            FunctionCallExpressionFunction::DynamicVariableName(x) => x.range(),
            FunctionCallExpressionFunction::EncapsedString(x) => x.range(),
            FunctionCallExpressionFunction::FunctionCallExpression(x) => x.range(),
            FunctionCallExpressionFunction::Heredoc(x) => x.range(),
            FunctionCallExpressionFunction::MemberCallExpression(x) => x.range(),
            FunctionCallExpressionFunction::Name(x) => x.range(),
            FunctionCallExpressionFunction::Nowdoc(x) => x.range(),
            FunctionCallExpressionFunction::NullsafeMemberCallExpression(x) => x.range(),
            FunctionCallExpressionFunction::ParenthesizedExpression(x) => x.range(),
            FunctionCallExpressionFunction::QualifiedName(x) => x.range(),
            FunctionCallExpressionFunction::ScopedCallExpression(x) => x.range(),
            FunctionCallExpressionFunction::String(x) => x.range(),
            FunctionCallExpressionFunction::SubscriptExpression(x) => x.range(),
            FunctionCallExpressionFunction::VariableName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionCallExpressionNode {
    pub range: Range,
    pub arguments: ArgumentsNode,
    pub function: Box<FunctionCallExpressionFunction>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for FunctionCallExpressionNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "function_call_expression" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [function_call_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let arguments: ArgumentsNode =
            Into::<Result<_, _>>::into(node.parse_child("arguments", source))?;
        let function: Box<FunctionCallExpressionFunction> =
            Into::<Result<_, _>>::into(node.parse_child("function", source))?;
        Ok(Self {
            range,
            arguments,
            function,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl FunctionCallExpressionNode {
    pub fn kind(&self) -> &'static str {
        "function_call_expression"
    }
}

impl NodeAccess for FunctionCallExpressionNode {
    fn brief_desc(&self) -> String {
        "FunctionCallExpressionNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::FunctionCallExpression(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.arguments.as_any());
        child_vec.push(self.function.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
