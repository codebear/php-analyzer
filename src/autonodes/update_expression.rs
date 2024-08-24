use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::cast_expression::CastExpressionNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::dynamic_variable_name::DynamicVariableNameNode;
use crate::autonodes::function_call_expression::FunctionCallExpressionNode;
use crate::autonodes::member_access_expression::MemberAccessExpressionNode;
use crate::autonodes::member_call_expression::MemberCallExpressionNode;
use crate::autonodes::nullsafe_member_access_expression::NullsafeMemberAccessExpressionNode;
use crate::autonodes::nullsafe_member_call_expression::NullsafeMemberCallExpressionNode;
use crate::autonodes::scoped_call_expression::ScopedCallExpressionNode;
use crate::autonodes::scoped_property_access_expression::ScopedPropertyAccessExpressionNode;
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
use crate::operators::decrement::DecrementOperator;
use crate::operators::increment::IncrementOperator;
use crate::operators::operator::Operator;
use crate::parser::Range;
use crate::types::union::PHPType;
use crate::value::PHPValue;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub enum UpdateExpressionArgument {
    CastExpression(Box<CastExpressionNode>),
    DynamicVariableName(Box<DynamicVariableNameNode>),
    FunctionCallExpression(Box<FunctionCallExpressionNode>),
    MemberAccessExpression(Box<MemberAccessExpressionNode>),
    MemberCallExpression(Box<MemberCallExpressionNode>),
    NullsafeMemberAccessExpression(Box<NullsafeMemberAccessExpressionNode>),
    NullsafeMemberCallExpression(Box<NullsafeMemberCallExpressionNode>),
    ScopedCallExpression(Box<ScopedCallExpressionNode>),
    ScopedPropertyAccessExpression(Box<ScopedPropertyAccessExpressionNode>),
    SubscriptExpression(Box<SubscriptExpressionNode>),
    VariableName(Box<VariableNameNode>),
    Extra(ExtraChild),
}

impl NodeParser for UpdateExpressionArgument {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => UpdateExpressionArgument::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => UpdateExpressionArgument::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => UpdateExpressionArgument::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "cast_expression" => UpdateExpressionArgument::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "dynamic_variable_name" => UpdateExpressionArgument::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "function_call_expression" => UpdateExpressionArgument::FunctionCallExpression(
                Box::new(FunctionCallExpressionNode::parse(node, source)?),
            ),
            "member_access_expression" => UpdateExpressionArgument::MemberAccessExpression(
                Box::new(MemberAccessExpressionNode::parse(node, source)?),
            ),
            "member_call_expression" => UpdateExpressionArgument::MemberCallExpression(Box::new(
                MemberCallExpressionNode::parse(node, source)?,
            )),
            "nullsafe_member_access_expression" => {
                UpdateExpressionArgument::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                UpdateExpressionArgument::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "scoped_call_expression" => UpdateExpressionArgument::ScopedCallExpression(Box::new(
                ScopedCallExpressionNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                UpdateExpressionArgument::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "subscript_expression" => UpdateExpressionArgument::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => UpdateExpressionArgument::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!(
                        "UpdateExpressionArgument: Parse error, unexpected node-type: {}",
                        node.kind()
                    ),
                ))
            }
        })
    }
}

impl UpdateExpressionArgument {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => UpdateExpressionArgument::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => UpdateExpressionArgument::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => UpdateExpressionArgument::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "cast_expression" => UpdateExpressionArgument::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "dynamic_variable_name" => UpdateExpressionArgument::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "function_call_expression" => UpdateExpressionArgument::FunctionCallExpression(
                Box::new(FunctionCallExpressionNode::parse(node, source)?),
            ),
            "member_access_expression" => UpdateExpressionArgument::MemberAccessExpression(
                Box::new(MemberAccessExpressionNode::parse(node, source)?),
            ),
            "member_call_expression" => UpdateExpressionArgument::MemberCallExpression(Box::new(
                MemberCallExpressionNode::parse(node, source)?,
            )),
            "nullsafe_member_access_expression" => {
                UpdateExpressionArgument::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                UpdateExpressionArgument::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "scoped_call_expression" => UpdateExpressionArgument::ScopedCallExpression(Box::new(
                ScopedCallExpressionNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                UpdateExpressionArgument::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "subscript_expression" => UpdateExpressionArgument::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => UpdateExpressionArgument::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            UpdateExpressionArgument::Extra(y) => y.kind(),
            UpdateExpressionArgument::CastExpression(y) => y.kind(),
            UpdateExpressionArgument::DynamicVariableName(y) => y.kind(),
            UpdateExpressionArgument::FunctionCallExpression(y) => y.kind(),
            UpdateExpressionArgument::MemberAccessExpression(y) => y.kind(),
            UpdateExpressionArgument::MemberCallExpression(y) => y.kind(),
            UpdateExpressionArgument::NullsafeMemberAccessExpression(y) => y.kind(),
            UpdateExpressionArgument::NullsafeMemberCallExpression(y) => y.kind(),
            UpdateExpressionArgument::ScopedCallExpression(y) => y.kind(),
            UpdateExpressionArgument::ScopedPropertyAccessExpression(y) => y.kind(),
            UpdateExpressionArgument::SubscriptExpression(y) => y.kind(),
            UpdateExpressionArgument::VariableName(y) => y.kind(),
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
            UpdateExpressionArgument::Extra(x) => x.get_utype(state, emitter),
            UpdateExpressionArgument::CastExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionArgument::DynamicVariableName(x) => x.get_utype(state, emitter),
            UpdateExpressionArgument::FunctionCallExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionArgument::MemberAccessExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionArgument::MemberCallExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionArgument::NullsafeMemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            UpdateExpressionArgument::NullsafeMemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            UpdateExpressionArgument::ScopedCallExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionArgument::ScopedPropertyAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            UpdateExpressionArgument::SubscriptExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionArgument::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            UpdateExpressionArgument::Extra(x) => x.get_php_value(state, emitter),
            UpdateExpressionArgument::CastExpression(x) => x.get_php_value(state, emitter),
            UpdateExpressionArgument::DynamicVariableName(x) => x.get_php_value(state, emitter),
            UpdateExpressionArgument::FunctionCallExpression(x) => x.get_php_value(state, emitter),
            UpdateExpressionArgument::MemberAccessExpression(x) => x.get_php_value(state, emitter),
            UpdateExpressionArgument::MemberCallExpression(x) => x.get_php_value(state, emitter),
            UpdateExpressionArgument::NullsafeMemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            UpdateExpressionArgument::NullsafeMemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            UpdateExpressionArgument::ScopedCallExpression(x) => x.get_php_value(state, emitter),
            UpdateExpressionArgument::ScopedPropertyAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            UpdateExpressionArgument::SubscriptExpression(x) => x.get_php_value(state, emitter),
            UpdateExpressionArgument::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            UpdateExpressionArgument::Extra(x) => x.read_from(state, emitter),
            UpdateExpressionArgument::CastExpression(x) => x.read_from(state, emitter),
            UpdateExpressionArgument::DynamicVariableName(x) => x.read_from(state, emitter),
            UpdateExpressionArgument::FunctionCallExpression(x) => x.read_from(state, emitter),
            UpdateExpressionArgument::MemberAccessExpression(x) => x.read_from(state, emitter),
            UpdateExpressionArgument::MemberCallExpression(x) => x.read_from(state, emitter),
            UpdateExpressionArgument::NullsafeMemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            UpdateExpressionArgument::NullsafeMemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            UpdateExpressionArgument::ScopedCallExpression(x) => x.read_from(state, emitter),
            UpdateExpressionArgument::ScopedPropertyAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            UpdateExpressionArgument::SubscriptExpression(x) => x.read_from(state, emitter),
            UpdateExpressionArgument::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for UpdateExpressionArgument {
    fn brief_desc(&self) -> String {
        match self {
            UpdateExpressionArgument::Extra(x) => {
                format!("UpdateExpressionArgument::extra({})", x.brief_desc())
            }
            UpdateExpressionArgument::CastExpression(x) => format!(
                "UpdateExpressionArgument::cast_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionArgument::DynamicVariableName(x) => format!(
                "UpdateExpressionArgument::dynamic_variable_name({})",
                x.brief_desc()
            ),
            UpdateExpressionArgument::FunctionCallExpression(x) => format!(
                "UpdateExpressionArgument::function_call_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionArgument::MemberAccessExpression(x) => format!(
                "UpdateExpressionArgument::member_access_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionArgument::MemberCallExpression(x) => format!(
                "UpdateExpressionArgument::member_call_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionArgument::NullsafeMemberAccessExpression(x) => format!(
                "UpdateExpressionArgument::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionArgument::NullsafeMemberCallExpression(x) => format!(
                "UpdateExpressionArgument::nullsafe_member_call_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionArgument::ScopedCallExpression(x) => format!(
                "UpdateExpressionArgument::scoped_call_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionArgument::ScopedPropertyAccessExpression(x) => format!(
                "UpdateExpressionArgument::scoped_property_access_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionArgument::SubscriptExpression(x) => format!(
                "UpdateExpressionArgument::subscript_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionArgument::VariableName(x) => format!(
                "UpdateExpressionArgument::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            UpdateExpressionArgument::Extra(x) => x.as_any(),
            UpdateExpressionArgument::CastExpression(x) => x.as_any(),
            UpdateExpressionArgument::DynamicVariableName(x) => x.as_any(),
            UpdateExpressionArgument::FunctionCallExpression(x) => x.as_any(),
            UpdateExpressionArgument::MemberAccessExpression(x) => x.as_any(),
            UpdateExpressionArgument::MemberCallExpression(x) => x.as_any(),
            UpdateExpressionArgument::NullsafeMemberAccessExpression(x) => x.as_any(),
            UpdateExpressionArgument::NullsafeMemberCallExpression(x) => x.as_any(),
            UpdateExpressionArgument::ScopedCallExpression(x) => x.as_any(),
            UpdateExpressionArgument::ScopedPropertyAccessExpression(x) => x.as_any(),
            UpdateExpressionArgument::SubscriptExpression(x) => x.as_any(),
            UpdateExpressionArgument::VariableName(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            UpdateExpressionArgument::Extra(x) => x.children_any(),
            UpdateExpressionArgument::CastExpression(x) => x.children_any(),
            UpdateExpressionArgument::DynamicVariableName(x) => x.children_any(),
            UpdateExpressionArgument::FunctionCallExpression(x) => x.children_any(),
            UpdateExpressionArgument::MemberAccessExpression(x) => x.children_any(),
            UpdateExpressionArgument::MemberCallExpression(x) => x.children_any(),
            UpdateExpressionArgument::NullsafeMemberAccessExpression(x) => x.children_any(),
            UpdateExpressionArgument::NullsafeMemberCallExpression(x) => x.children_any(),
            UpdateExpressionArgument::ScopedCallExpression(x) => x.children_any(),
            UpdateExpressionArgument::ScopedPropertyAccessExpression(x) => x.children_any(),
            UpdateExpressionArgument::SubscriptExpression(x) => x.children_any(),
            UpdateExpressionArgument::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            UpdateExpressionArgument::Extra(x) => x.range(),
            UpdateExpressionArgument::CastExpression(x) => x.range(),
            UpdateExpressionArgument::DynamicVariableName(x) => x.range(),
            UpdateExpressionArgument::FunctionCallExpression(x) => x.range(),
            UpdateExpressionArgument::MemberAccessExpression(x) => x.range(),
            UpdateExpressionArgument::MemberCallExpression(x) => x.range(),
            UpdateExpressionArgument::NullsafeMemberAccessExpression(x) => x.range(),
            UpdateExpressionArgument::NullsafeMemberCallExpression(x) => x.range(),
            UpdateExpressionArgument::ScopedCallExpression(x) => x.range(),
            UpdateExpressionArgument::ScopedPropertyAccessExpression(x) => x.range(),
            UpdateExpressionArgument::SubscriptExpression(x) => x.range(),
            UpdateExpressionArgument::VariableName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum UpdateExpressionPostfix {
    Increment(IncrementOperator),
    Decrement(DecrementOperator),
    Extra(ExtraChild),
}

impl NodeParser for UpdateExpressionPostfix {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => UpdateExpressionPostfix::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => UpdateExpressionPostfix::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => UpdateExpressionPostfix::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "++" => UpdateExpressionPostfix::Increment(IncrementOperator(node.range().into())),
            "--" => UpdateExpressionPostfix::Decrement(DecrementOperator(node.range().into())),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!(
                        "UpdateExpressionPostfix: Parse error, unexpected node-type: {}",
                        node.kind()
                    ),
                ))
            }
        })
    }
}

impl UpdateExpressionPostfix {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => UpdateExpressionPostfix::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => UpdateExpressionPostfix::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => UpdateExpressionPostfix::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "++" => UpdateExpressionPostfix::Increment(IncrementOperator(node.range().into())),
            "--" => UpdateExpressionPostfix::Decrement(DecrementOperator(node.range().into())),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            UpdateExpressionPostfix::Extra(y) => y.kind(),
            UpdateExpressionPostfix::Increment(y) => y.kind(),
            UpdateExpressionPostfix::Decrement(y) => y.kind(),
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
}

#[derive(Debug, Clone)]
pub enum UpdateExpressionPrefix {
    Increment(IncrementOperator),
    Decrement(DecrementOperator),
    Extra(ExtraChild),
}

impl NodeParser for UpdateExpressionPrefix {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => UpdateExpressionPrefix::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => UpdateExpressionPrefix::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => UpdateExpressionPrefix::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "++" => UpdateExpressionPrefix::Increment(IncrementOperator(node.range().into())),
            "--" => UpdateExpressionPrefix::Decrement(DecrementOperator(node.range().into())),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!(
                        "UpdateExpressionPrefix: Parse error, unexpected node-type: {}",
                        node.kind()
                    ),
                ))
            }
        })
    }
}

impl UpdateExpressionPrefix {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => UpdateExpressionPrefix::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => UpdateExpressionPrefix::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => UpdateExpressionPrefix::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "++" => UpdateExpressionPrefix::Increment(IncrementOperator(node.range().into())),
            "--" => UpdateExpressionPrefix::Decrement(DecrementOperator(node.range().into())),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            UpdateExpressionPrefix::Extra(y) => y.kind(),
            UpdateExpressionPrefix::Increment(y) => y.kind(),
            UpdateExpressionPrefix::Decrement(y) => y.kind(),
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
}

#[derive(Debug, Clone)]
pub struct UpdateExpressionNode {
    pub range: Range,
    pub argument: Box<UpdateExpressionArgument>,
    pub postfix: Option<Box<UpdateExpressionPostfix>>,
    pub prefix: Option<Box<UpdateExpressionPrefix>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for UpdateExpressionNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "update_expression" {
            return Err(ParseError::new(range, format!("UpdateExpressionNode: Node is of the wrong kind [{}] vs expected [update_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let argument: Box<UpdateExpressionArgument> =
            Into::<Result<_, _>>::into(node.parse_child("argument", source))?;
        let postfix: Option<Box<UpdateExpressionPostfix>> =
            Into::<Result<_, _>>::into(node.parse_child("postfix", source))?;
        let prefix: Option<Box<UpdateExpressionPrefix>> =
            Into::<Result<_, _>>::into(node.parse_child("prefix", source))?;
        Ok(Self {
            range,
            argument,
            postfix,
            prefix,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl UpdateExpressionNode {
    pub fn kind(&self) -> &'static str {
        "update_expression"
    }
}

impl NodeAccess for UpdateExpressionNode {
    fn brief_desc(&self) -> String {
        "UpdateExpressionNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::UpdateExpression(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.argument.as_any());
        if let Some(x) = &self.postfix {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.prefix {
            child_vec.push(x.as_any());
        }

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
