use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::cast_expression::CastExpressionNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::dynamic_variable_name::DynamicVariableNameNode;
use crate::autonodes::function_call_expression::FunctionCallExpressionNode;
use crate::autonodes::list_literal::ListLiteralNode;
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
use crate::parser::Range;
use crate::types::union::PHPType;
use crate::value::PHPValue;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub enum AssignmentExpressionLeft {
    CastExpression(Box<CastExpressionNode>),
    DynamicVariableName(Box<DynamicVariableNameNode>),
    FunctionCallExpression(Box<FunctionCallExpressionNode>),
    ListLiteral(Box<ListLiteralNode>),
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

impl NodeParser for AssignmentExpressionLeft {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => AssignmentExpressionLeft::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => AssignmentExpressionLeft::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => AssignmentExpressionLeft::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "cast_expression" => AssignmentExpressionLeft::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "dynamic_variable_name" => AssignmentExpressionLeft::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "function_call_expression" => AssignmentExpressionLeft::FunctionCallExpression(
                Box::new(FunctionCallExpressionNode::parse(node, source)?),
            ),
            "list_literal" => AssignmentExpressionLeft::ListLiteral(Box::new(
                ListLiteralNode::parse(node, source)?,
            )),
            "member_access_expression" => AssignmentExpressionLeft::MemberAccessExpression(
                Box::new(MemberAccessExpressionNode::parse(node, source)?),
            ),
            "member_call_expression" => AssignmentExpressionLeft::MemberCallExpression(Box::new(
                MemberCallExpressionNode::parse(node, source)?,
            )),
            "nullsafe_member_access_expression" => {
                AssignmentExpressionLeft::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                AssignmentExpressionLeft::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "scoped_call_expression" => AssignmentExpressionLeft::ScopedCallExpression(Box::new(
                ScopedCallExpressionNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                AssignmentExpressionLeft::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "subscript_expression" => AssignmentExpressionLeft::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => AssignmentExpressionLeft::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!(
                        "AssignmentExpressionLeft: Parse error, unexpected node-type: {}",
                        node.kind()
                    ),
                ))
            }
        })
    }
}

impl AssignmentExpressionLeft {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => AssignmentExpressionLeft::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => AssignmentExpressionLeft::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => AssignmentExpressionLeft::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "cast_expression" => AssignmentExpressionLeft::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "dynamic_variable_name" => AssignmentExpressionLeft::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "function_call_expression" => AssignmentExpressionLeft::FunctionCallExpression(
                Box::new(FunctionCallExpressionNode::parse(node, source)?),
            ),
            "list_literal" => AssignmentExpressionLeft::ListLiteral(Box::new(
                ListLiteralNode::parse(node, source)?,
            )),
            "member_access_expression" => AssignmentExpressionLeft::MemberAccessExpression(
                Box::new(MemberAccessExpressionNode::parse(node, source)?),
            ),
            "member_call_expression" => AssignmentExpressionLeft::MemberCallExpression(Box::new(
                MemberCallExpressionNode::parse(node, source)?,
            )),
            "nullsafe_member_access_expression" => {
                AssignmentExpressionLeft::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                AssignmentExpressionLeft::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "scoped_call_expression" => AssignmentExpressionLeft::ScopedCallExpression(Box::new(
                ScopedCallExpressionNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                AssignmentExpressionLeft::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "subscript_expression" => AssignmentExpressionLeft::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => AssignmentExpressionLeft::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            AssignmentExpressionLeft::Extra(y) => y.kind(),
            AssignmentExpressionLeft::CastExpression(y) => y.kind(),
            AssignmentExpressionLeft::DynamicVariableName(y) => y.kind(),
            AssignmentExpressionLeft::FunctionCallExpression(y) => y.kind(),
            AssignmentExpressionLeft::ListLiteral(y) => y.kind(),
            AssignmentExpressionLeft::MemberAccessExpression(y) => y.kind(),
            AssignmentExpressionLeft::MemberCallExpression(y) => y.kind(),
            AssignmentExpressionLeft::NullsafeMemberAccessExpression(y) => y.kind(),
            AssignmentExpressionLeft::NullsafeMemberCallExpression(y) => y.kind(),
            AssignmentExpressionLeft::ScopedCallExpression(y) => y.kind(),
            AssignmentExpressionLeft::ScopedPropertyAccessExpression(y) => y.kind(),
            AssignmentExpressionLeft::SubscriptExpression(y) => y.kind(),
            AssignmentExpressionLeft::VariableName(y) => y.kind(),
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
            AssignmentExpressionLeft::Extra(x) => x.get_utype(state, emitter),
            AssignmentExpressionLeft::CastExpression(x) => x.get_utype(state, emitter),
            AssignmentExpressionLeft::DynamicVariableName(x) => x.get_utype(state, emitter),
            AssignmentExpressionLeft::FunctionCallExpression(x) => x.get_utype(state, emitter),
            AssignmentExpressionLeft::ListLiteral(x) => x.get_utype(state, emitter),
            AssignmentExpressionLeft::MemberAccessExpression(x) => x.get_utype(state, emitter),
            AssignmentExpressionLeft::MemberCallExpression(x) => x.get_utype(state, emitter),
            AssignmentExpressionLeft::NullsafeMemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            AssignmentExpressionLeft::NullsafeMemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            AssignmentExpressionLeft::ScopedCallExpression(x) => x.get_utype(state, emitter),
            AssignmentExpressionLeft::ScopedPropertyAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            AssignmentExpressionLeft::SubscriptExpression(x) => x.get_utype(state, emitter),
            AssignmentExpressionLeft::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            AssignmentExpressionLeft::Extra(x) => x.get_php_value(state, emitter),
            AssignmentExpressionLeft::CastExpression(x) => x.get_php_value(state, emitter),
            AssignmentExpressionLeft::DynamicVariableName(x) => x.get_php_value(state, emitter),
            AssignmentExpressionLeft::FunctionCallExpression(x) => x.get_php_value(state, emitter),
            AssignmentExpressionLeft::ListLiteral(x) => x.get_php_value(state, emitter),
            AssignmentExpressionLeft::MemberAccessExpression(x) => x.get_php_value(state, emitter),
            AssignmentExpressionLeft::MemberCallExpression(x) => x.get_php_value(state, emitter),
            AssignmentExpressionLeft::NullsafeMemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            AssignmentExpressionLeft::NullsafeMemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            AssignmentExpressionLeft::ScopedCallExpression(x) => x.get_php_value(state, emitter),
            AssignmentExpressionLeft::ScopedPropertyAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            AssignmentExpressionLeft::SubscriptExpression(x) => x.get_php_value(state, emitter),
            AssignmentExpressionLeft::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            AssignmentExpressionLeft::Extra(x) => x.read_from(state, emitter),
            AssignmentExpressionLeft::CastExpression(x) => x.read_from(state, emitter),
            AssignmentExpressionLeft::DynamicVariableName(x) => x.read_from(state, emitter),
            AssignmentExpressionLeft::FunctionCallExpression(x) => x.read_from(state, emitter),
            AssignmentExpressionLeft::ListLiteral(x) => x.read_from(state, emitter),
            AssignmentExpressionLeft::MemberAccessExpression(x) => x.read_from(state, emitter),
            AssignmentExpressionLeft::MemberCallExpression(x) => x.read_from(state, emitter),
            AssignmentExpressionLeft::NullsafeMemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            AssignmentExpressionLeft::NullsafeMemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            AssignmentExpressionLeft::ScopedCallExpression(x) => x.read_from(state, emitter),
            AssignmentExpressionLeft::ScopedPropertyAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            AssignmentExpressionLeft::SubscriptExpression(x) => x.read_from(state, emitter),
            AssignmentExpressionLeft::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for AssignmentExpressionLeft {
    fn brief_desc(&self) -> String {
        match self {
            AssignmentExpressionLeft::Extra(x) => {
                format!("AssignmentExpressionLeft::extra({})", x.brief_desc())
            }
            AssignmentExpressionLeft::CastExpression(x) => format!(
                "AssignmentExpressionLeft::cast_expression({})",
                x.brief_desc()
            ),
            AssignmentExpressionLeft::DynamicVariableName(x) => format!(
                "AssignmentExpressionLeft::dynamic_variable_name({})",
                x.brief_desc()
            ),
            AssignmentExpressionLeft::FunctionCallExpression(x) => format!(
                "AssignmentExpressionLeft::function_call_expression({})",
                x.brief_desc()
            ),
            AssignmentExpressionLeft::ListLiteral(x) => {
                format!("AssignmentExpressionLeft::list_literal({})", x.brief_desc())
            }
            AssignmentExpressionLeft::MemberAccessExpression(x) => format!(
                "AssignmentExpressionLeft::member_access_expression({})",
                x.brief_desc()
            ),
            AssignmentExpressionLeft::MemberCallExpression(x) => format!(
                "AssignmentExpressionLeft::member_call_expression({})",
                x.brief_desc()
            ),
            AssignmentExpressionLeft::NullsafeMemberAccessExpression(x) => format!(
                "AssignmentExpressionLeft::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            AssignmentExpressionLeft::NullsafeMemberCallExpression(x) => format!(
                "AssignmentExpressionLeft::nullsafe_member_call_expression({})",
                x.brief_desc()
            ),
            AssignmentExpressionLeft::ScopedCallExpression(x) => format!(
                "AssignmentExpressionLeft::scoped_call_expression({})",
                x.brief_desc()
            ),
            AssignmentExpressionLeft::ScopedPropertyAccessExpression(x) => format!(
                "AssignmentExpressionLeft::scoped_property_access_expression({})",
                x.brief_desc()
            ),
            AssignmentExpressionLeft::SubscriptExpression(x) => format!(
                "AssignmentExpressionLeft::subscript_expression({})",
                x.brief_desc()
            ),
            AssignmentExpressionLeft::VariableName(x) => format!(
                "AssignmentExpressionLeft::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            AssignmentExpressionLeft::Extra(x) => x.as_any(),
            AssignmentExpressionLeft::CastExpression(x) => x.as_any(),
            AssignmentExpressionLeft::DynamicVariableName(x) => x.as_any(),
            AssignmentExpressionLeft::FunctionCallExpression(x) => x.as_any(),
            AssignmentExpressionLeft::ListLiteral(x) => x.as_any(),
            AssignmentExpressionLeft::MemberAccessExpression(x) => x.as_any(),
            AssignmentExpressionLeft::MemberCallExpression(x) => x.as_any(),
            AssignmentExpressionLeft::NullsafeMemberAccessExpression(x) => x.as_any(),
            AssignmentExpressionLeft::NullsafeMemberCallExpression(x) => x.as_any(),
            AssignmentExpressionLeft::ScopedCallExpression(x) => x.as_any(),
            AssignmentExpressionLeft::ScopedPropertyAccessExpression(x) => x.as_any(),
            AssignmentExpressionLeft::SubscriptExpression(x) => x.as_any(),
            AssignmentExpressionLeft::VariableName(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            AssignmentExpressionLeft::Extra(x) => x.children_any(),
            AssignmentExpressionLeft::CastExpression(x) => x.children_any(),
            AssignmentExpressionLeft::DynamicVariableName(x) => x.children_any(),
            AssignmentExpressionLeft::FunctionCallExpression(x) => x.children_any(),
            AssignmentExpressionLeft::ListLiteral(x) => x.children_any(),
            AssignmentExpressionLeft::MemberAccessExpression(x) => x.children_any(),
            AssignmentExpressionLeft::MemberCallExpression(x) => x.children_any(),
            AssignmentExpressionLeft::NullsafeMemberAccessExpression(x) => x.children_any(),
            AssignmentExpressionLeft::NullsafeMemberCallExpression(x) => x.children_any(),
            AssignmentExpressionLeft::ScopedCallExpression(x) => x.children_any(),
            AssignmentExpressionLeft::ScopedPropertyAccessExpression(x) => x.children_any(),
            AssignmentExpressionLeft::SubscriptExpression(x) => x.children_any(),
            AssignmentExpressionLeft::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            AssignmentExpressionLeft::Extra(x) => x.range(),
            AssignmentExpressionLeft::CastExpression(x) => x.range(),
            AssignmentExpressionLeft::DynamicVariableName(x) => x.range(),
            AssignmentExpressionLeft::FunctionCallExpression(x) => x.range(),
            AssignmentExpressionLeft::ListLiteral(x) => x.range(),
            AssignmentExpressionLeft::MemberAccessExpression(x) => x.range(),
            AssignmentExpressionLeft::MemberCallExpression(x) => x.range(),
            AssignmentExpressionLeft::NullsafeMemberAccessExpression(x) => x.range(),
            AssignmentExpressionLeft::NullsafeMemberCallExpression(x) => x.range(),
            AssignmentExpressionLeft::ScopedCallExpression(x) => x.range(),
            AssignmentExpressionLeft::ScopedPropertyAccessExpression(x) => x.range(),
            AssignmentExpressionLeft::SubscriptExpression(x) => x.range(),
            AssignmentExpressionLeft::VariableName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AssignmentExpressionNode {
    pub range: Range,
    pub left: Box<AssignmentExpressionLeft>,
    pub right: _ExpressionNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for AssignmentExpressionNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "assignment_expression" {
            return Err(ParseError::new(range, format!("AssignmentExpressionNode: Node is of the wrong kind [{}] vs expected [assignment_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let left: Box<AssignmentExpressionLeft> =
            Into::<Result<_, _>>::into(node.parse_child("left", source))?;
        let right: _ExpressionNode = Into::<Result<_, _>>::into(node.parse_child("right", source))?;
        Ok(Self {
            range,
            left,
            right,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl AssignmentExpressionNode {
    pub fn kind(&self) -> &'static str {
        "assignment_expression"
    }
}

impl NodeAccess for AssignmentExpressionNode {
    fn brief_desc(&self) -> String {
        "AssignmentExpressionNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::AssignmentExpression(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.left.as_any());
        child_vec.push(self.right.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
