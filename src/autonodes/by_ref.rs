use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::dynamic_variable_name::DynamicVariableNameNode;
use crate::autonodes::function_call_expression::FunctionCallExpressionNode;
use crate::autonodes::member_access_expression::MemberAccessExpressionNode;
use crate::autonodes::member_call_expression::MemberCallExpressionNode;
use crate::autonodes::nullsafe_member_access_expression::NullsafeMemberAccessExpressionNode;
use crate::autonodes::nullsafe_member_call_expression::NullsafeMemberCallExpressionNode;
use crate::autonodes::scoped_call_expression::ScopedCallExpressionNode;
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
pub enum ByRefChildren {
    DynamicVariableName(Box<DynamicVariableNameNode>),
    FunctionCallExpression(Box<FunctionCallExpressionNode>),
    MemberAccessExpression(Box<MemberAccessExpressionNode>),
    MemberCallExpression(Box<MemberCallExpressionNode>),
    NullsafeMemberAccessExpression(Box<NullsafeMemberAccessExpressionNode>),
    NullsafeMemberCallExpression(Box<NullsafeMemberCallExpressionNode>),
    ScopedCallExpression(Box<ScopedCallExpressionNode>),
    SubscriptExpression(Box<SubscriptExpressionNode>),
    VariableName(Box<VariableNameNode>),
    Extra(ExtraChild),
}

impl ByRefChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ByRefChildren::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "text_interpolation" => ByRefChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            ))),
            "ERROR" => {
                ByRefChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "dynamic_variable_name" => ByRefChildren::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "function_call_expression" => ByRefChildren::FunctionCallExpression(Box::new(
                FunctionCallExpressionNode::parse(node, source)?,
            )),
            "member_access_expression" => ByRefChildren::MemberAccessExpression(Box::new(
                MemberAccessExpressionNode::parse(node, source)?,
            )),
            "member_call_expression" => ByRefChildren::MemberCallExpression(Box::new(
                MemberCallExpressionNode::parse(node, source)?,
            )),
            "nullsafe_member_access_expression" => ByRefChildren::NullsafeMemberAccessExpression(
                Box::new(NullsafeMemberAccessExpressionNode::parse(node, source)?),
            ),
            "nullsafe_member_call_expression" => ByRefChildren::NullsafeMemberCallExpression(
                Box::new(NullsafeMemberCallExpressionNode::parse(node, source)?),
            ),
            "scoped_call_expression" => ByRefChildren::ScopedCallExpression(Box::new(
                ScopedCallExpressionNode::parse(node, source)?,
            )),
            "subscript_expression" => ByRefChildren::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => {
                ByRefChildren::VariableName(Box::new(VariableNameNode::parse(node, source)?))
            }

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
            "comment" => ByRefChildren::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "text_interpolation" => ByRefChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            ))),
            "ERROR" => {
                ByRefChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "dynamic_variable_name" => ByRefChildren::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "function_call_expression" => ByRefChildren::FunctionCallExpression(Box::new(
                FunctionCallExpressionNode::parse(node, source)?,
            )),
            "member_access_expression" => ByRefChildren::MemberAccessExpression(Box::new(
                MemberAccessExpressionNode::parse(node, source)?,
            )),
            "member_call_expression" => ByRefChildren::MemberCallExpression(Box::new(
                MemberCallExpressionNode::parse(node, source)?,
            )),
            "nullsafe_member_access_expression" => ByRefChildren::NullsafeMemberAccessExpression(
                Box::new(NullsafeMemberAccessExpressionNode::parse(node, source)?),
            ),
            "nullsafe_member_call_expression" => ByRefChildren::NullsafeMemberCallExpression(
                Box::new(NullsafeMemberCallExpressionNode::parse(node, source)?),
            ),
            "scoped_call_expression" => ByRefChildren::ScopedCallExpression(Box::new(
                ScopedCallExpressionNode::parse(node, source)?,
            )),
            "subscript_expression" => ByRefChildren::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => {
                ByRefChildren::VariableName(Box::new(VariableNameNode::parse(node, source)?))
            }

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
            ByRefChildren::Extra(x) => x.get_utype(state, emitter),
            ByRefChildren::DynamicVariableName(x) => x.get_utype(state, emitter),
            ByRefChildren::FunctionCallExpression(x) => x.get_utype(state, emitter),
            ByRefChildren::MemberAccessExpression(x) => x.get_utype(state, emitter),
            ByRefChildren::MemberCallExpression(x) => x.get_utype(state, emitter),
            ByRefChildren::NullsafeMemberAccessExpression(x) => x.get_utype(state, emitter),
            ByRefChildren::NullsafeMemberCallExpression(x) => x.get_utype(state, emitter),
            ByRefChildren::ScopedCallExpression(x) => x.get_utype(state, emitter),
            ByRefChildren::SubscriptExpression(x) => x.get_utype(state, emitter),
            ByRefChildren::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ByRefChildren::Extra(x) => x.get_php_value(state, emitter),
            ByRefChildren::DynamicVariableName(x) => x.get_php_value(state, emitter),
            ByRefChildren::FunctionCallExpression(x) => x.get_php_value(state, emitter),
            ByRefChildren::MemberAccessExpression(x) => x.get_php_value(state, emitter),
            ByRefChildren::MemberCallExpression(x) => x.get_php_value(state, emitter),
            ByRefChildren::NullsafeMemberAccessExpression(x) => x.get_php_value(state, emitter),
            ByRefChildren::NullsafeMemberCallExpression(x) => x.get_php_value(state, emitter),
            ByRefChildren::ScopedCallExpression(x) => x.get_php_value(state, emitter),
            ByRefChildren::SubscriptExpression(x) => x.get_php_value(state, emitter),
            ByRefChildren::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ByRefChildren::Extra(x) => x.read_from(state, emitter),
            ByRefChildren::DynamicVariableName(x) => x.read_from(state, emitter),
            ByRefChildren::FunctionCallExpression(x) => x.read_from(state, emitter),
            ByRefChildren::MemberAccessExpression(x) => x.read_from(state, emitter),
            ByRefChildren::MemberCallExpression(x) => x.read_from(state, emitter),
            ByRefChildren::NullsafeMemberAccessExpression(x) => x.read_from(state, emitter),
            ByRefChildren::NullsafeMemberCallExpression(x) => x.read_from(state, emitter),
            ByRefChildren::ScopedCallExpression(x) => x.read_from(state, emitter),
            ByRefChildren::SubscriptExpression(x) => x.read_from(state, emitter),
            ByRefChildren::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ByRefChildren {
    fn brief_desc(&self) -> String {
        match self {
            ByRefChildren::Extra(x) => format!("ByRefChildren::extra({})", x.brief_desc()),
            ByRefChildren::DynamicVariableName(x) => {
                format!("ByRefChildren::dynamic_variable_name({})", x.brief_desc())
            }
            ByRefChildren::FunctionCallExpression(x) => format!(
                "ByRefChildren::function_call_expression({})",
                x.brief_desc()
            ),
            ByRefChildren::MemberAccessExpression(x) => format!(
                "ByRefChildren::member_access_expression({})",
                x.brief_desc()
            ),
            ByRefChildren::MemberCallExpression(x) => {
                format!("ByRefChildren::member_call_expression({})", x.brief_desc())
            }
            ByRefChildren::NullsafeMemberAccessExpression(x) => format!(
                "ByRefChildren::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            ByRefChildren::NullsafeMemberCallExpression(x) => format!(
                "ByRefChildren::nullsafe_member_call_expression({})",
                x.brief_desc()
            ),
            ByRefChildren::ScopedCallExpression(x) => {
                format!("ByRefChildren::scoped_call_expression({})", x.brief_desc())
            }
            ByRefChildren::SubscriptExpression(x) => {
                format!("ByRefChildren::subscript_expression({})", x.brief_desc())
            }
            ByRefChildren::VariableName(x) => {
                format!("ByRefChildren::variable_name({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ByRefChildren::Extra(x) => x.as_any(),
            ByRefChildren::DynamicVariableName(x) => x.as_any(),
            ByRefChildren::FunctionCallExpression(x) => x.as_any(),
            ByRefChildren::MemberAccessExpression(x) => x.as_any(),
            ByRefChildren::MemberCallExpression(x) => x.as_any(),
            ByRefChildren::NullsafeMemberAccessExpression(x) => x.as_any(),
            ByRefChildren::NullsafeMemberCallExpression(x) => x.as_any(),
            ByRefChildren::ScopedCallExpression(x) => x.as_any(),
            ByRefChildren::SubscriptExpression(x) => x.as_any(),
            ByRefChildren::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ByRefChildren::Extra(x) => x.children_any(),
            ByRefChildren::DynamicVariableName(x) => x.children_any(),
            ByRefChildren::FunctionCallExpression(x) => x.children_any(),
            ByRefChildren::MemberAccessExpression(x) => x.children_any(),
            ByRefChildren::MemberCallExpression(x) => x.children_any(),
            ByRefChildren::NullsafeMemberAccessExpression(x) => x.children_any(),
            ByRefChildren::NullsafeMemberCallExpression(x) => x.children_any(),
            ByRefChildren::ScopedCallExpression(x) => x.children_any(),
            ByRefChildren::SubscriptExpression(x) => x.children_any(),
            ByRefChildren::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ByRefChildren::Extra(x) => x.range(),
            ByRefChildren::DynamicVariableName(x) => x.range(),
            ByRefChildren::FunctionCallExpression(x) => x.range(),
            ByRefChildren::MemberAccessExpression(x) => x.range(),
            ByRefChildren::MemberCallExpression(x) => x.range(),
            ByRefChildren::NullsafeMemberAccessExpression(x) => x.range(),
            ByRefChildren::NullsafeMemberCallExpression(x) => x.range(),
            ByRefChildren::ScopedCallExpression(x) => x.range(),
            ByRefChildren::SubscriptExpression(x) => x.range(),
            ByRefChildren::VariableName(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ByRefNode {
    pub range: Range,
    pub child: Box<ByRefChildren>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl ByRefNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "by_ref" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [by_ref] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            child: node
                .named_children(&mut node.walk())
                .filter(|node| node.kind() != "comment")
                .map(|k| ByRefChildren::parse(k, source))
                .collect::<Result<Vec<ByRefChildren>, ParseError>>()?
                .drain(..)
                .map(|j| Box::new(j))
                .next()
                .expect("Should be a child"),
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )?,
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
        "by_ref"
    }
}

impl NodeAccess for ByRefNode {
    fn brief_desc(&self) -> String {
        "ByRefNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::ByRef(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.child.as_any());
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
