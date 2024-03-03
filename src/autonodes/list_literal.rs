use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::by_ref::ByRefNode;
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

use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub enum ListLiteralChildren {
    _Expression(Box<_ExpressionNode>),
    ByRef(Box<ByRefNode>),
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

impl NodeParser for ListLiteralChildren {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ListLiteralChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ListLiteralChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => ListLiteralChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "by_ref" => ListLiteralChildren::ByRef(Box::new(ByRefNode::parse(node, source)?)),
            "dynamic_variable_name" => ListLiteralChildren::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "function_call_expression" => ListLiteralChildren::FunctionCallExpression(Box::new(
                FunctionCallExpressionNode::parse(node, source)?,
            )),
            "list_literal" => {
                ListLiteralChildren::ListLiteral(Box::new(ListLiteralNode::parse(node, source)?))
            }
            "member_access_expression" => ListLiteralChildren::MemberAccessExpression(Box::new(
                MemberAccessExpressionNode::parse(node, source)?,
            )),
            "member_call_expression" => ListLiteralChildren::MemberCallExpression(Box::new(
                MemberCallExpressionNode::parse(node, source)?,
            )),
            "nullsafe_member_access_expression" => {
                ListLiteralChildren::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => ListLiteralChildren::NullsafeMemberCallExpression(
                Box::new(NullsafeMemberCallExpressionNode::parse(node, source)?),
            ),
            "scoped_call_expression" => ListLiteralChildren::ScopedCallExpression(Box::new(
                ScopedCallExpressionNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                ListLiteralChildren::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "subscript_expression" => ListLiteralChildren::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => {
                ListLiteralChildren::VariableName(Box::new(VariableNameNode::parse(node, source)?))
            }

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| ListLiteralChildren::_Expression(y))
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

impl ListLiteralChildren {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ListLiteralChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ListLiteralChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => ListLiteralChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "by_ref" => ListLiteralChildren::ByRef(Box::new(ByRefNode::parse(node, source)?)),
            "dynamic_variable_name" => ListLiteralChildren::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "function_call_expression" => ListLiteralChildren::FunctionCallExpression(Box::new(
                FunctionCallExpressionNode::parse(node, source)?,
            )),
            "list_literal" => {
                ListLiteralChildren::ListLiteral(Box::new(ListLiteralNode::parse(node, source)?))
            }
            "member_access_expression" => ListLiteralChildren::MemberAccessExpression(Box::new(
                MemberAccessExpressionNode::parse(node, source)?,
            )),
            "member_call_expression" => ListLiteralChildren::MemberCallExpression(Box::new(
                MemberCallExpressionNode::parse(node, source)?,
            )),
            "nullsafe_member_access_expression" => {
                ListLiteralChildren::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => ListLiteralChildren::NullsafeMemberCallExpression(
                Box::new(NullsafeMemberCallExpressionNode::parse(node, source)?),
            ),
            "scoped_call_expression" => ListLiteralChildren::ScopedCallExpression(Box::new(
                ScopedCallExpressionNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                ListLiteralChildren::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "subscript_expression" => ListLiteralChildren::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => {
                ListLiteralChildren::VariableName(Box::new(VariableNameNode::parse(node, source)?))
            }

            _ => {
                return Ok(
                    if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| ListLiteralChildren::_Expression(y))
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
            ListLiteralChildren::Extra(y) => y.kind(),
            ListLiteralChildren::_Expression(y) => y.kind(),
            ListLiteralChildren::ByRef(y) => y.kind(),
            ListLiteralChildren::DynamicVariableName(y) => y.kind(),
            ListLiteralChildren::FunctionCallExpression(y) => y.kind(),
            ListLiteralChildren::ListLiteral(y) => y.kind(),
            ListLiteralChildren::MemberAccessExpression(y) => y.kind(),
            ListLiteralChildren::MemberCallExpression(y) => y.kind(),
            ListLiteralChildren::NullsafeMemberAccessExpression(y) => y.kind(),
            ListLiteralChildren::NullsafeMemberCallExpression(y) => y.kind(),
            ListLiteralChildren::ScopedCallExpression(y) => y.kind(),
            ListLiteralChildren::ScopedPropertyAccessExpression(y) => y.kind(),
            ListLiteralChildren::SubscriptExpression(y) => y.kind(),
            ListLiteralChildren::VariableName(y) => y.kind(),
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
            ListLiteralChildren::Extra(x) => x.get_utype(state, emitter),
            ListLiteralChildren::_Expression(x) => x.get_utype(state, emitter),
            ListLiteralChildren::ByRef(x) => x.get_utype(state, emitter),
            ListLiteralChildren::DynamicVariableName(x) => x.get_utype(state, emitter),
            ListLiteralChildren::FunctionCallExpression(x) => x.get_utype(state, emitter),
            ListLiteralChildren::ListLiteral(x) => x.get_utype(state, emitter),
            ListLiteralChildren::MemberAccessExpression(x) => x.get_utype(state, emitter),
            ListLiteralChildren::MemberCallExpression(x) => x.get_utype(state, emitter),
            ListLiteralChildren::NullsafeMemberAccessExpression(x) => x.get_utype(state, emitter),
            ListLiteralChildren::NullsafeMemberCallExpression(x) => x.get_utype(state, emitter),
            ListLiteralChildren::ScopedCallExpression(x) => x.get_utype(state, emitter),
            ListLiteralChildren::ScopedPropertyAccessExpression(x) => x.get_utype(state, emitter),
            ListLiteralChildren::SubscriptExpression(x) => x.get_utype(state, emitter),
            ListLiteralChildren::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ListLiteralChildren::Extra(x) => x.get_php_value(state, emitter),
            ListLiteralChildren::_Expression(x) => x.get_php_value(state, emitter),
            ListLiteralChildren::ByRef(x) => x.get_php_value(state, emitter),
            ListLiteralChildren::DynamicVariableName(x) => x.get_php_value(state, emitter),
            ListLiteralChildren::FunctionCallExpression(x) => x.get_php_value(state, emitter),
            ListLiteralChildren::ListLiteral(x) => x.get_php_value(state, emitter),
            ListLiteralChildren::MemberAccessExpression(x) => x.get_php_value(state, emitter),
            ListLiteralChildren::MemberCallExpression(x) => x.get_php_value(state, emitter),
            ListLiteralChildren::NullsafeMemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ListLiteralChildren::NullsafeMemberCallExpression(x) => x.get_php_value(state, emitter),
            ListLiteralChildren::ScopedCallExpression(x) => x.get_php_value(state, emitter),
            ListLiteralChildren::ScopedPropertyAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ListLiteralChildren::SubscriptExpression(x) => x.get_php_value(state, emitter),
            ListLiteralChildren::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ListLiteralChildren::Extra(x) => x.read_from(state, emitter),
            ListLiteralChildren::_Expression(x) => x.read_from(state, emitter),
            ListLiteralChildren::ByRef(x) => x.read_from(state, emitter),
            ListLiteralChildren::DynamicVariableName(x) => x.read_from(state, emitter),
            ListLiteralChildren::FunctionCallExpression(x) => x.read_from(state, emitter),
            ListLiteralChildren::ListLiteral(x) => x.read_from(state, emitter),
            ListLiteralChildren::MemberAccessExpression(x) => x.read_from(state, emitter),
            ListLiteralChildren::MemberCallExpression(x) => x.read_from(state, emitter),
            ListLiteralChildren::NullsafeMemberAccessExpression(x) => x.read_from(state, emitter),
            ListLiteralChildren::NullsafeMemberCallExpression(x) => x.read_from(state, emitter),
            ListLiteralChildren::ScopedCallExpression(x) => x.read_from(state, emitter),
            ListLiteralChildren::ScopedPropertyAccessExpression(x) => x.read_from(state, emitter),
            ListLiteralChildren::SubscriptExpression(x) => x.read_from(state, emitter),
            ListLiteralChildren::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ListLiteralChildren {
    fn brief_desc(&self) -> String {
        match self {
            ListLiteralChildren::Extra(x) => {
                format!("ListLiteralChildren::extra({})", x.brief_desc())
            }
            ListLiteralChildren::_Expression(x) => {
                format!("ListLiteralChildren::_expression({})", x.brief_desc())
            }
            ListLiteralChildren::ByRef(x) => {
                format!("ListLiteralChildren::by_ref({})", x.brief_desc())
            }
            ListLiteralChildren::DynamicVariableName(x) => format!(
                "ListLiteralChildren::dynamic_variable_name({})",
                x.brief_desc()
            ),
            ListLiteralChildren::FunctionCallExpression(x) => format!(
                "ListLiteralChildren::function_call_expression({})",
                x.brief_desc()
            ),
            ListLiteralChildren::ListLiteral(x) => {
                format!("ListLiteralChildren::list_literal({})", x.brief_desc())
            }
            ListLiteralChildren::MemberAccessExpression(x) => format!(
                "ListLiteralChildren::member_access_expression({})",
                x.brief_desc()
            ),
            ListLiteralChildren::MemberCallExpression(x) => format!(
                "ListLiteralChildren::member_call_expression({})",
                x.brief_desc()
            ),
            ListLiteralChildren::NullsafeMemberAccessExpression(x) => format!(
                "ListLiteralChildren::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            ListLiteralChildren::NullsafeMemberCallExpression(x) => format!(
                "ListLiteralChildren::nullsafe_member_call_expression({})",
                x.brief_desc()
            ),
            ListLiteralChildren::ScopedCallExpression(x) => format!(
                "ListLiteralChildren::scoped_call_expression({})",
                x.brief_desc()
            ),
            ListLiteralChildren::ScopedPropertyAccessExpression(x) => format!(
                "ListLiteralChildren::scoped_property_access_expression({})",
                x.brief_desc()
            ),
            ListLiteralChildren::SubscriptExpression(x) => format!(
                "ListLiteralChildren::subscript_expression({})",
                x.brief_desc()
            ),
            ListLiteralChildren::VariableName(x) => {
                format!("ListLiteralChildren::variable_name({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ListLiteralChildren::Extra(x) => x.as_any(),
            ListLiteralChildren::_Expression(x) => x.as_any(),
            ListLiteralChildren::ByRef(x) => x.as_any(),
            ListLiteralChildren::DynamicVariableName(x) => x.as_any(),
            ListLiteralChildren::FunctionCallExpression(x) => x.as_any(),
            ListLiteralChildren::ListLiteral(x) => x.as_any(),
            ListLiteralChildren::MemberAccessExpression(x) => x.as_any(),
            ListLiteralChildren::MemberCallExpression(x) => x.as_any(),
            ListLiteralChildren::NullsafeMemberAccessExpression(x) => x.as_any(),
            ListLiteralChildren::NullsafeMemberCallExpression(x) => x.as_any(),
            ListLiteralChildren::ScopedCallExpression(x) => x.as_any(),
            ListLiteralChildren::ScopedPropertyAccessExpression(x) => x.as_any(),
            ListLiteralChildren::SubscriptExpression(x) => x.as_any(),
            ListLiteralChildren::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ListLiteralChildren::Extra(x) => x.children_any(),
            ListLiteralChildren::_Expression(x) => x.children_any(),
            ListLiteralChildren::ByRef(x) => x.children_any(),
            ListLiteralChildren::DynamicVariableName(x) => x.children_any(),
            ListLiteralChildren::FunctionCallExpression(x) => x.children_any(),
            ListLiteralChildren::ListLiteral(x) => x.children_any(),
            ListLiteralChildren::MemberAccessExpression(x) => x.children_any(),
            ListLiteralChildren::MemberCallExpression(x) => x.children_any(),
            ListLiteralChildren::NullsafeMemberAccessExpression(x) => x.children_any(),
            ListLiteralChildren::NullsafeMemberCallExpression(x) => x.children_any(),
            ListLiteralChildren::ScopedCallExpression(x) => x.children_any(),
            ListLiteralChildren::ScopedPropertyAccessExpression(x) => x.children_any(),
            ListLiteralChildren::SubscriptExpression(x) => x.children_any(),
            ListLiteralChildren::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ListLiteralChildren::Extra(x) => x.range(),
            ListLiteralChildren::_Expression(x) => x.range(),
            ListLiteralChildren::ByRef(x) => x.range(),
            ListLiteralChildren::DynamicVariableName(x) => x.range(),
            ListLiteralChildren::FunctionCallExpression(x) => x.range(),
            ListLiteralChildren::ListLiteral(x) => x.range(),
            ListLiteralChildren::MemberAccessExpression(x) => x.range(),
            ListLiteralChildren::MemberCallExpression(x) => x.range(),
            ListLiteralChildren::NullsafeMemberAccessExpression(x) => x.range(),
            ListLiteralChildren::NullsafeMemberCallExpression(x) => x.range(),
            ListLiteralChildren::ScopedCallExpression(x) => x.range(),
            ListLiteralChildren::ScopedPropertyAccessExpression(x) => x.range(),
            ListLiteralChildren::SubscriptExpression(x) => x.range(),
            ListLiteralChildren::VariableName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListLiteralNode {
    pub range: Range,
    pub children: Vec<Box<ListLiteralChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for ListLiteralNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "list_literal" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [list_literal] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            children: ListLiteralChildren::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() != "comment"),
                source,
            )?,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )?,
        })
    }
}

impl ListLiteralNode {
    pub fn kind(&self) -> &'static str {
        "list_literal"
    }
}

impl NodeAccess for ListLiteralNode {
    fn brief_desc(&self) -> String {
        "ListLiteralNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::ListLiteral(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.extend(self.children.iter().map(|n| n.as_any()));
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
