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
pub enum UnsetStatementChildren {
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

impl UnsetStatementChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => UnsetStatementChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => UnsetStatementChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => UnsetStatementChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "cast_expression" => UnsetStatementChildren::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "dynamic_variable_name" => UnsetStatementChildren::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "function_call_expression" => UnsetStatementChildren::FunctionCallExpression(Box::new(
                FunctionCallExpressionNode::parse(node, source)?,
            )),
            "member_access_expression" => UnsetStatementChildren::MemberAccessExpression(Box::new(
                MemberAccessExpressionNode::parse(node, source)?,
            )),
            "member_call_expression" => UnsetStatementChildren::MemberCallExpression(Box::new(
                MemberCallExpressionNode::parse(node, source)?,
            )),
            "nullsafe_member_access_expression" => {
                UnsetStatementChildren::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                UnsetStatementChildren::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "scoped_call_expression" => UnsetStatementChildren::ScopedCallExpression(Box::new(
                ScopedCallExpressionNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                UnsetStatementChildren::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "subscript_expression" => UnsetStatementChildren::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => UnsetStatementChildren::VariableName(Box::new(
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
            "comment" => UnsetStatementChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => UnsetStatementChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => UnsetStatementChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "cast_expression" => UnsetStatementChildren::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "dynamic_variable_name" => UnsetStatementChildren::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "function_call_expression" => UnsetStatementChildren::FunctionCallExpression(Box::new(
                FunctionCallExpressionNode::parse(node, source)?,
            )),
            "member_access_expression" => UnsetStatementChildren::MemberAccessExpression(Box::new(
                MemberAccessExpressionNode::parse(node, source)?,
            )),
            "member_call_expression" => UnsetStatementChildren::MemberCallExpression(Box::new(
                MemberCallExpressionNode::parse(node, source)?,
            )),
            "nullsafe_member_access_expression" => {
                UnsetStatementChildren::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                UnsetStatementChildren::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "scoped_call_expression" => UnsetStatementChildren::ScopedCallExpression(Box::new(
                ScopedCallExpressionNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                UnsetStatementChildren::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "subscript_expression" => UnsetStatementChildren::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => UnsetStatementChildren::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            UnsetStatementChildren::Extra(y) => y.kind(),
            UnsetStatementChildren::CastExpression(y) => y.kind(),
            UnsetStatementChildren::DynamicVariableName(y) => y.kind(),
            UnsetStatementChildren::FunctionCallExpression(y) => y.kind(),
            UnsetStatementChildren::MemberAccessExpression(y) => y.kind(),
            UnsetStatementChildren::MemberCallExpression(y) => y.kind(),
            UnsetStatementChildren::NullsafeMemberAccessExpression(y) => y.kind(),
            UnsetStatementChildren::NullsafeMemberCallExpression(y) => y.kind(),
            UnsetStatementChildren::ScopedCallExpression(y) => y.kind(),
            UnsetStatementChildren::ScopedPropertyAccessExpression(y) => y.kind(),
            UnsetStatementChildren::SubscriptExpression(y) => y.kind(),
            UnsetStatementChildren::VariableName(y) => y.kind(),
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
            UnsetStatementChildren::Extra(x) => x.get_utype(state, emitter),
            UnsetStatementChildren::CastExpression(x) => x.get_utype(state, emitter),
            UnsetStatementChildren::DynamicVariableName(x) => x.get_utype(state, emitter),
            UnsetStatementChildren::FunctionCallExpression(x) => x.get_utype(state, emitter),
            UnsetStatementChildren::MemberAccessExpression(x) => x.get_utype(state, emitter),
            UnsetStatementChildren::MemberCallExpression(x) => x.get_utype(state, emitter),
            UnsetStatementChildren::NullsafeMemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            UnsetStatementChildren::NullsafeMemberCallExpression(x) => x.get_utype(state, emitter),
            UnsetStatementChildren::ScopedCallExpression(x) => x.get_utype(state, emitter),
            UnsetStatementChildren::ScopedPropertyAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            UnsetStatementChildren::SubscriptExpression(x) => x.get_utype(state, emitter),
            UnsetStatementChildren::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            UnsetStatementChildren::Extra(x) => x.get_php_value(state, emitter),
            UnsetStatementChildren::CastExpression(x) => x.get_php_value(state, emitter),
            UnsetStatementChildren::DynamicVariableName(x) => x.get_php_value(state, emitter),
            UnsetStatementChildren::FunctionCallExpression(x) => x.get_php_value(state, emitter),
            UnsetStatementChildren::MemberAccessExpression(x) => x.get_php_value(state, emitter),
            UnsetStatementChildren::MemberCallExpression(x) => x.get_php_value(state, emitter),
            UnsetStatementChildren::NullsafeMemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            UnsetStatementChildren::NullsafeMemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            UnsetStatementChildren::ScopedCallExpression(x) => x.get_php_value(state, emitter),
            UnsetStatementChildren::ScopedPropertyAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            UnsetStatementChildren::SubscriptExpression(x) => x.get_php_value(state, emitter),
            UnsetStatementChildren::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            UnsetStatementChildren::Extra(x) => x.read_from(state, emitter),
            UnsetStatementChildren::CastExpression(x) => x.read_from(state, emitter),
            UnsetStatementChildren::DynamicVariableName(x) => x.read_from(state, emitter),
            UnsetStatementChildren::FunctionCallExpression(x) => x.read_from(state, emitter),
            UnsetStatementChildren::MemberAccessExpression(x) => x.read_from(state, emitter),
            UnsetStatementChildren::MemberCallExpression(x) => x.read_from(state, emitter),
            UnsetStatementChildren::NullsafeMemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            UnsetStatementChildren::NullsafeMemberCallExpression(x) => x.read_from(state, emitter),
            UnsetStatementChildren::ScopedCallExpression(x) => x.read_from(state, emitter),
            UnsetStatementChildren::ScopedPropertyAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            UnsetStatementChildren::SubscriptExpression(x) => x.read_from(state, emitter),
            UnsetStatementChildren::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for UnsetStatementChildren {
    fn brief_desc(&self) -> String {
        match self {
            UnsetStatementChildren::Extra(x) => {
                format!("UnsetStatementChildren::extra({})", x.brief_desc())
            }
            UnsetStatementChildren::CastExpression(x) => format!(
                "UnsetStatementChildren::cast_expression({})",
                x.brief_desc()
            ),
            UnsetStatementChildren::DynamicVariableName(x) => format!(
                "UnsetStatementChildren::dynamic_variable_name({})",
                x.brief_desc()
            ),
            UnsetStatementChildren::FunctionCallExpression(x) => format!(
                "UnsetStatementChildren::function_call_expression({})",
                x.brief_desc()
            ),
            UnsetStatementChildren::MemberAccessExpression(x) => format!(
                "UnsetStatementChildren::member_access_expression({})",
                x.brief_desc()
            ),
            UnsetStatementChildren::MemberCallExpression(x) => format!(
                "UnsetStatementChildren::member_call_expression({})",
                x.brief_desc()
            ),
            UnsetStatementChildren::NullsafeMemberAccessExpression(x) => format!(
                "UnsetStatementChildren::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            UnsetStatementChildren::NullsafeMemberCallExpression(x) => format!(
                "UnsetStatementChildren::nullsafe_member_call_expression({})",
                x.brief_desc()
            ),
            UnsetStatementChildren::ScopedCallExpression(x) => format!(
                "UnsetStatementChildren::scoped_call_expression({})",
                x.brief_desc()
            ),
            UnsetStatementChildren::ScopedPropertyAccessExpression(x) => format!(
                "UnsetStatementChildren::scoped_property_access_expression({})",
                x.brief_desc()
            ),
            UnsetStatementChildren::SubscriptExpression(x) => format!(
                "UnsetStatementChildren::subscript_expression({})",
                x.brief_desc()
            ),
            UnsetStatementChildren::VariableName(x) => {
                format!("UnsetStatementChildren::variable_name({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            UnsetStatementChildren::Extra(x) => x.as_any(),
            UnsetStatementChildren::CastExpression(x) => x.as_any(),
            UnsetStatementChildren::DynamicVariableName(x) => x.as_any(),
            UnsetStatementChildren::FunctionCallExpression(x) => x.as_any(),
            UnsetStatementChildren::MemberAccessExpression(x) => x.as_any(),
            UnsetStatementChildren::MemberCallExpression(x) => x.as_any(),
            UnsetStatementChildren::NullsafeMemberAccessExpression(x) => x.as_any(),
            UnsetStatementChildren::NullsafeMemberCallExpression(x) => x.as_any(),
            UnsetStatementChildren::ScopedCallExpression(x) => x.as_any(),
            UnsetStatementChildren::ScopedPropertyAccessExpression(x) => x.as_any(),
            UnsetStatementChildren::SubscriptExpression(x) => x.as_any(),
            UnsetStatementChildren::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            UnsetStatementChildren::Extra(x) => x.children_any(),
            UnsetStatementChildren::CastExpression(x) => x.children_any(),
            UnsetStatementChildren::DynamicVariableName(x) => x.children_any(),
            UnsetStatementChildren::FunctionCallExpression(x) => x.children_any(),
            UnsetStatementChildren::MemberAccessExpression(x) => x.children_any(),
            UnsetStatementChildren::MemberCallExpression(x) => x.children_any(),
            UnsetStatementChildren::NullsafeMemberAccessExpression(x) => x.children_any(),
            UnsetStatementChildren::NullsafeMemberCallExpression(x) => x.children_any(),
            UnsetStatementChildren::ScopedCallExpression(x) => x.children_any(),
            UnsetStatementChildren::ScopedPropertyAccessExpression(x) => x.children_any(),
            UnsetStatementChildren::SubscriptExpression(x) => x.children_any(),
            UnsetStatementChildren::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            UnsetStatementChildren::Extra(x) => x.range(),
            UnsetStatementChildren::CastExpression(x) => x.range(),
            UnsetStatementChildren::DynamicVariableName(x) => x.range(),
            UnsetStatementChildren::FunctionCallExpression(x) => x.range(),
            UnsetStatementChildren::MemberAccessExpression(x) => x.range(),
            UnsetStatementChildren::MemberCallExpression(x) => x.range(),
            UnsetStatementChildren::NullsafeMemberAccessExpression(x) => x.range(),
            UnsetStatementChildren::NullsafeMemberCallExpression(x) => x.range(),
            UnsetStatementChildren::ScopedCallExpression(x) => x.range(),
            UnsetStatementChildren::ScopedPropertyAccessExpression(x) => x.range(),
            UnsetStatementChildren::SubscriptExpression(x) => x.range(),
            UnsetStatementChildren::VariableName(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct UnsetStatementNode {
    pub range: Range,
    pub children: Vec<Box<UnsetStatementChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl UnsetStatementNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "unset_statement" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [unset_statement] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            children: UnsetStatementChildren::parse_vec(
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
        "unset_statement"
    }
}

impl NodeAccess for UnsetStatementNode {
    fn brief_desc(&self) -> String {
        "UnsetStatementNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::UnsetStatement(self)
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
