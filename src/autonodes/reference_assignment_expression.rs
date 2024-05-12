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
pub enum ReferenceAssignmentExpressionLeft {
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

impl NodeParser for ReferenceAssignmentExpressionLeft {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ReferenceAssignmentExpressionLeft::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => ReferenceAssignmentExpressionLeft::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "cast_expression" => ReferenceAssignmentExpressionLeft::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "dynamic_variable_name" => ReferenceAssignmentExpressionLeft::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "function_call_expression" => {
                ReferenceAssignmentExpressionLeft::FunctionCallExpression(Box::new(
                    FunctionCallExpressionNode::parse(node, source)?,
                ))
            }
            "list_literal" => ReferenceAssignmentExpressionLeft::ListLiteral(Box::new(
                ListLiteralNode::parse(node, source)?,
            )),
            "member_access_expression" => {
                ReferenceAssignmentExpressionLeft::MemberAccessExpression(Box::new(
                    MemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "member_call_expression" => ReferenceAssignmentExpressionLeft::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "nullsafe_member_access_expression" => {
                ReferenceAssignmentExpressionLeft::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                ReferenceAssignmentExpressionLeft::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "scoped_call_expression" => ReferenceAssignmentExpressionLeft::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "scoped_property_access_expression" => {
                ReferenceAssignmentExpressionLeft::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "subscript_expression" => ReferenceAssignmentExpressionLeft::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => ReferenceAssignmentExpressionLeft::VariableName(Box::new(
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

impl ReferenceAssignmentExpressionLeft {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ReferenceAssignmentExpressionLeft::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => ReferenceAssignmentExpressionLeft::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "cast_expression" => ReferenceAssignmentExpressionLeft::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "dynamic_variable_name" => ReferenceAssignmentExpressionLeft::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "function_call_expression" => {
                ReferenceAssignmentExpressionLeft::FunctionCallExpression(Box::new(
                    FunctionCallExpressionNode::parse(node, source)?,
                ))
            }
            "list_literal" => ReferenceAssignmentExpressionLeft::ListLiteral(Box::new(
                ListLiteralNode::parse(node, source)?,
            )),
            "member_access_expression" => {
                ReferenceAssignmentExpressionLeft::MemberAccessExpression(Box::new(
                    MemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "member_call_expression" => ReferenceAssignmentExpressionLeft::MemberCallExpression(
                Box::new(MemberCallExpressionNode::parse(node, source)?),
            ),
            "nullsafe_member_access_expression" => {
                ReferenceAssignmentExpressionLeft::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                ReferenceAssignmentExpressionLeft::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "scoped_call_expression" => ReferenceAssignmentExpressionLeft::ScopedCallExpression(
                Box::new(ScopedCallExpressionNode::parse(node, source)?),
            ),
            "scoped_property_access_expression" => {
                ReferenceAssignmentExpressionLeft::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "subscript_expression" => ReferenceAssignmentExpressionLeft::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => ReferenceAssignmentExpressionLeft::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            ReferenceAssignmentExpressionLeft::Extra(y) => y.kind(),
            ReferenceAssignmentExpressionLeft::CastExpression(y) => y.kind(),
            ReferenceAssignmentExpressionLeft::DynamicVariableName(y) => y.kind(),
            ReferenceAssignmentExpressionLeft::FunctionCallExpression(y) => y.kind(),
            ReferenceAssignmentExpressionLeft::ListLiteral(y) => y.kind(),
            ReferenceAssignmentExpressionLeft::MemberAccessExpression(y) => y.kind(),
            ReferenceAssignmentExpressionLeft::MemberCallExpression(y) => y.kind(),
            ReferenceAssignmentExpressionLeft::NullsafeMemberAccessExpression(y) => y.kind(),
            ReferenceAssignmentExpressionLeft::NullsafeMemberCallExpression(y) => y.kind(),
            ReferenceAssignmentExpressionLeft::ScopedCallExpression(y) => y.kind(),
            ReferenceAssignmentExpressionLeft::ScopedPropertyAccessExpression(y) => y.kind(),
            ReferenceAssignmentExpressionLeft::SubscriptExpression(y) => y.kind(),
            ReferenceAssignmentExpressionLeft::VariableName(y) => y.kind(),
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
            ReferenceAssignmentExpressionLeft::Extra(x) => x.get_utype(state, emitter),
            ReferenceAssignmentExpressionLeft::CastExpression(x) => x.get_utype(state, emitter),
            ReferenceAssignmentExpressionLeft::DynamicVariableName(x) => {
                x.get_utype(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::FunctionCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::ListLiteral(x) => x.get_utype(state, emitter),
            ReferenceAssignmentExpressionLeft::MemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::MemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::NullsafeMemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::NullsafeMemberCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::ScopedCallExpression(x) => {
                x.get_utype(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::ScopedPropertyAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::SubscriptExpression(x) => {
                x.get_utype(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ReferenceAssignmentExpressionLeft::Extra(x) => x.get_php_value(state, emitter),
            ReferenceAssignmentExpressionLeft::CastExpression(x) => x.get_php_value(state, emitter),
            ReferenceAssignmentExpressionLeft::DynamicVariableName(x) => {
                x.get_php_value(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::FunctionCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::ListLiteral(x) => x.get_php_value(state, emitter),
            ReferenceAssignmentExpressionLeft::MemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::MemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::NullsafeMemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::NullsafeMemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::ScopedCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::ScopedPropertyAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::SubscriptExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ReferenceAssignmentExpressionLeft::Extra(x) => x.read_from(state, emitter),
            ReferenceAssignmentExpressionLeft::CastExpression(x) => x.read_from(state, emitter),
            ReferenceAssignmentExpressionLeft::DynamicVariableName(x) => {
                x.read_from(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::FunctionCallExpression(x) => {
                x.read_from(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::ListLiteral(x) => x.read_from(state, emitter),
            ReferenceAssignmentExpressionLeft::MemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::MemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::NullsafeMemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::NullsafeMemberCallExpression(x) => {
                x.read_from(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::ScopedCallExpression(x) => {
                x.read_from(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::ScopedPropertyAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::SubscriptExpression(x) => {
                x.read_from(state, emitter)
            }
            ReferenceAssignmentExpressionLeft::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ReferenceAssignmentExpressionLeft {
    fn brief_desc(&self) -> String {
        match self {
            ReferenceAssignmentExpressionLeft::Extra(x) => format!(
                "ReferenceAssignmentExpressionLeft::extra({})",
                x.brief_desc()
            ),
            ReferenceAssignmentExpressionLeft::CastExpression(x) => format!(
                "ReferenceAssignmentExpressionLeft::cast_expression({})",
                x.brief_desc()
            ),
            ReferenceAssignmentExpressionLeft::DynamicVariableName(x) => format!(
                "ReferenceAssignmentExpressionLeft::dynamic_variable_name({})",
                x.brief_desc()
            ),
            ReferenceAssignmentExpressionLeft::FunctionCallExpression(x) => format!(
                "ReferenceAssignmentExpressionLeft::function_call_expression({})",
                x.brief_desc()
            ),
            ReferenceAssignmentExpressionLeft::ListLiteral(x) => format!(
                "ReferenceAssignmentExpressionLeft::list_literal({})",
                x.brief_desc()
            ),
            ReferenceAssignmentExpressionLeft::MemberAccessExpression(x) => format!(
                "ReferenceAssignmentExpressionLeft::member_access_expression({})",
                x.brief_desc()
            ),
            ReferenceAssignmentExpressionLeft::MemberCallExpression(x) => format!(
                "ReferenceAssignmentExpressionLeft::member_call_expression({})",
                x.brief_desc()
            ),
            ReferenceAssignmentExpressionLeft::NullsafeMemberAccessExpression(x) => format!(
                "ReferenceAssignmentExpressionLeft::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            ReferenceAssignmentExpressionLeft::NullsafeMemberCallExpression(x) => format!(
                "ReferenceAssignmentExpressionLeft::nullsafe_member_call_expression({})",
                x.brief_desc()
            ),
            ReferenceAssignmentExpressionLeft::ScopedCallExpression(x) => format!(
                "ReferenceAssignmentExpressionLeft::scoped_call_expression({})",
                x.brief_desc()
            ),
            ReferenceAssignmentExpressionLeft::ScopedPropertyAccessExpression(x) => format!(
                "ReferenceAssignmentExpressionLeft::scoped_property_access_expression({})",
                x.brief_desc()
            ),
            ReferenceAssignmentExpressionLeft::SubscriptExpression(x) => format!(
                "ReferenceAssignmentExpressionLeft::subscript_expression({})",
                x.brief_desc()
            ),
            ReferenceAssignmentExpressionLeft::VariableName(x) => format!(
                "ReferenceAssignmentExpressionLeft::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ReferenceAssignmentExpressionLeft::Extra(x) => x.as_any(),
            ReferenceAssignmentExpressionLeft::CastExpression(x) => x.as_any(),
            ReferenceAssignmentExpressionLeft::DynamicVariableName(x) => x.as_any(),
            ReferenceAssignmentExpressionLeft::FunctionCallExpression(x) => x.as_any(),
            ReferenceAssignmentExpressionLeft::ListLiteral(x) => x.as_any(),
            ReferenceAssignmentExpressionLeft::MemberAccessExpression(x) => x.as_any(),
            ReferenceAssignmentExpressionLeft::MemberCallExpression(x) => x.as_any(),
            ReferenceAssignmentExpressionLeft::NullsafeMemberAccessExpression(x) => x.as_any(),
            ReferenceAssignmentExpressionLeft::NullsafeMemberCallExpression(x) => x.as_any(),
            ReferenceAssignmentExpressionLeft::ScopedCallExpression(x) => x.as_any(),
            ReferenceAssignmentExpressionLeft::ScopedPropertyAccessExpression(x) => x.as_any(),
            ReferenceAssignmentExpressionLeft::SubscriptExpression(x) => x.as_any(),
            ReferenceAssignmentExpressionLeft::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ReferenceAssignmentExpressionLeft::Extra(x) => x.children_any(),
            ReferenceAssignmentExpressionLeft::CastExpression(x) => x.children_any(),
            ReferenceAssignmentExpressionLeft::DynamicVariableName(x) => x.children_any(),
            ReferenceAssignmentExpressionLeft::FunctionCallExpression(x) => x.children_any(),
            ReferenceAssignmentExpressionLeft::ListLiteral(x) => x.children_any(),
            ReferenceAssignmentExpressionLeft::MemberAccessExpression(x) => x.children_any(),
            ReferenceAssignmentExpressionLeft::MemberCallExpression(x) => x.children_any(),
            ReferenceAssignmentExpressionLeft::NullsafeMemberAccessExpression(x) => {
                x.children_any()
            }
            ReferenceAssignmentExpressionLeft::NullsafeMemberCallExpression(x) => x.children_any(),
            ReferenceAssignmentExpressionLeft::ScopedCallExpression(x) => x.children_any(),
            ReferenceAssignmentExpressionLeft::ScopedPropertyAccessExpression(x) => {
                x.children_any()
            }
            ReferenceAssignmentExpressionLeft::SubscriptExpression(x) => x.children_any(),
            ReferenceAssignmentExpressionLeft::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ReferenceAssignmentExpressionLeft::Extra(x) => x.range(),
            ReferenceAssignmentExpressionLeft::CastExpression(x) => x.range(),
            ReferenceAssignmentExpressionLeft::DynamicVariableName(x) => x.range(),
            ReferenceAssignmentExpressionLeft::FunctionCallExpression(x) => x.range(),
            ReferenceAssignmentExpressionLeft::ListLiteral(x) => x.range(),
            ReferenceAssignmentExpressionLeft::MemberAccessExpression(x) => x.range(),
            ReferenceAssignmentExpressionLeft::MemberCallExpression(x) => x.range(),
            ReferenceAssignmentExpressionLeft::NullsafeMemberAccessExpression(x) => x.range(),
            ReferenceAssignmentExpressionLeft::NullsafeMemberCallExpression(x) => x.range(),
            ReferenceAssignmentExpressionLeft::ScopedCallExpression(x) => x.range(),
            ReferenceAssignmentExpressionLeft::ScopedPropertyAccessExpression(x) => x.range(),
            ReferenceAssignmentExpressionLeft::SubscriptExpression(x) => x.range(),
            ReferenceAssignmentExpressionLeft::VariableName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReferenceAssignmentExpressionNode {
    pub range: Range,
    pub left: Box<ReferenceAssignmentExpressionLeft>,
    pub right: _ExpressionNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for ReferenceAssignmentExpressionNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "reference_assignment_expression" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [reference_assignment_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let left: Box<ReferenceAssignmentExpressionLeft> =
            Result::from(node.parse_child("left", source).into())?;
        let right: _ExpressionNode = Result::from(node.parse_child("right", source).into())?;
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

impl ReferenceAssignmentExpressionNode {
    pub fn kind(&self) -> &'static str {
        "reference_assignment_expression"
    }
}

impl NodeAccess for ReferenceAssignmentExpressionNode {
    fn brief_desc(&self) -> String {
        "ReferenceAssignmentExpressionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::ReferenceAssignmentExpression(self)
    }

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
