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
use crate::operators::decrement::DecrementOperator;
use crate::operators::increment::IncrementOperator;
use crate::operators::operator::Operator;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub enum UpdateExpressionExpr {
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

impl UpdateExpressionExpr {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => UpdateExpressionExpr::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => UpdateExpressionExpr::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => UpdateExpressionExpr::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "cast_expression" => UpdateExpressionExpr::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "dynamic_variable_name" => UpdateExpressionExpr::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "function_call_expression" => UpdateExpressionExpr::FunctionCallExpression(Box::new(
                FunctionCallExpressionNode::parse(node, source)?,
            )),
            "member_access_expression" => UpdateExpressionExpr::MemberAccessExpression(Box::new(
                MemberAccessExpressionNode::parse(node, source)?,
            )),
            "member_call_expression" => UpdateExpressionExpr::MemberCallExpression(Box::new(
                MemberCallExpressionNode::parse(node, source)?,
            )),
            "nullsafe_member_access_expression" => {
                UpdateExpressionExpr::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                UpdateExpressionExpr::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "scoped_call_expression" => UpdateExpressionExpr::ScopedCallExpression(Box::new(
                ScopedCallExpressionNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                UpdateExpressionExpr::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "subscript_expression" => UpdateExpressionExpr::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => {
                UpdateExpressionExpr::VariableName(Box::new(VariableNameNode::parse(node, source)?))
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
            "comment" => UpdateExpressionExpr::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => UpdateExpressionExpr::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => UpdateExpressionExpr::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "cast_expression" => UpdateExpressionExpr::CastExpression(Box::new(
                CastExpressionNode::parse(node, source)?,
            )),
            "dynamic_variable_name" => UpdateExpressionExpr::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "function_call_expression" => UpdateExpressionExpr::FunctionCallExpression(Box::new(
                FunctionCallExpressionNode::parse(node, source)?,
            )),
            "member_access_expression" => UpdateExpressionExpr::MemberAccessExpression(Box::new(
                MemberAccessExpressionNode::parse(node, source)?,
            )),
            "member_call_expression" => UpdateExpressionExpr::MemberCallExpression(Box::new(
                MemberCallExpressionNode::parse(node, source)?,
            )),
            "nullsafe_member_access_expression" => {
                UpdateExpressionExpr::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "nullsafe_member_call_expression" => {
                UpdateExpressionExpr::NullsafeMemberCallExpression(Box::new(
                    NullsafeMemberCallExpressionNode::parse(node, source)?,
                ))
            }
            "scoped_call_expression" => UpdateExpressionExpr::ScopedCallExpression(Box::new(
                ScopedCallExpressionNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                UpdateExpressionExpr::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "subscript_expression" => UpdateExpressionExpr::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => {
                UpdateExpressionExpr::VariableName(Box::new(VariableNameNode::parse(node, source)?))
            }

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            UpdateExpressionExpr::Extra(y) => y.kind(),
            UpdateExpressionExpr::CastExpression(y) => y.kind(),
            UpdateExpressionExpr::DynamicVariableName(y) => y.kind(),
            UpdateExpressionExpr::FunctionCallExpression(y) => y.kind(),
            UpdateExpressionExpr::MemberAccessExpression(y) => y.kind(),
            UpdateExpressionExpr::MemberCallExpression(y) => y.kind(),
            UpdateExpressionExpr::NullsafeMemberAccessExpression(y) => y.kind(),
            UpdateExpressionExpr::NullsafeMemberCallExpression(y) => y.kind(),
            UpdateExpressionExpr::ScopedCallExpression(y) => y.kind(),
            UpdateExpressionExpr::ScopedPropertyAccessExpression(y) => y.kind(),
            UpdateExpressionExpr::SubscriptExpression(y) => y.kind(),
            UpdateExpressionExpr::VariableName(y) => y.kind(),
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
            UpdateExpressionExpr::Extra(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::CastExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::DynamicVariableName(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::FunctionCallExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::MemberAccessExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::MemberCallExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::NullsafeMemberAccessExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::NullsafeMemberCallExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::ScopedCallExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::ScopedPropertyAccessExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::SubscriptExpression(x) => x.get_utype(state, emitter),
            UpdateExpressionExpr::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            UpdateExpressionExpr::Extra(x) => x.get_php_value(state, emitter),
            UpdateExpressionExpr::CastExpression(x) => x.get_php_value(state, emitter),
            UpdateExpressionExpr::DynamicVariableName(x) => x.get_php_value(state, emitter),
            UpdateExpressionExpr::FunctionCallExpression(x) => x.get_php_value(state, emitter),
            UpdateExpressionExpr::MemberAccessExpression(x) => x.get_php_value(state, emitter),
            UpdateExpressionExpr::MemberCallExpression(x) => x.get_php_value(state, emitter),
            UpdateExpressionExpr::NullsafeMemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            UpdateExpressionExpr::NullsafeMemberCallExpression(x) => {
                x.get_php_value(state, emitter)
            }
            UpdateExpressionExpr::ScopedCallExpression(x) => x.get_php_value(state, emitter),
            UpdateExpressionExpr::ScopedPropertyAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            UpdateExpressionExpr::SubscriptExpression(x) => x.get_php_value(state, emitter),
            UpdateExpressionExpr::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            UpdateExpressionExpr::Extra(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::CastExpression(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::DynamicVariableName(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::FunctionCallExpression(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::MemberAccessExpression(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::MemberCallExpression(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::NullsafeMemberAccessExpression(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::NullsafeMemberCallExpression(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::ScopedCallExpression(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::ScopedPropertyAccessExpression(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::SubscriptExpression(x) => x.read_from(state, emitter),
            UpdateExpressionExpr::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for UpdateExpressionExpr {
    fn brief_desc(&self) -> String {
        match self {
            UpdateExpressionExpr::Extra(x) => {
                format!("UpdateExpressionExpr::extra({})", x.brief_desc())
            }
            UpdateExpressionExpr::CastExpression(x) => {
                format!("UpdateExpressionExpr::cast_expression({})", x.brief_desc())
            }
            UpdateExpressionExpr::DynamicVariableName(x) => format!(
                "UpdateExpressionExpr::dynamic_variable_name({})",
                x.brief_desc()
            ),
            UpdateExpressionExpr::FunctionCallExpression(x) => format!(
                "UpdateExpressionExpr::function_call_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionExpr::MemberAccessExpression(x) => format!(
                "UpdateExpressionExpr::member_access_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionExpr::MemberCallExpression(x) => format!(
                "UpdateExpressionExpr::member_call_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionExpr::NullsafeMemberAccessExpression(x) => format!(
                "UpdateExpressionExpr::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionExpr::NullsafeMemberCallExpression(x) => format!(
                "UpdateExpressionExpr::nullsafe_member_call_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionExpr::ScopedCallExpression(x) => format!(
                "UpdateExpressionExpr::scoped_call_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionExpr::ScopedPropertyAccessExpression(x) => format!(
                "UpdateExpressionExpr::scoped_property_access_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionExpr::SubscriptExpression(x) => format!(
                "UpdateExpressionExpr::subscript_expression({})",
                x.brief_desc()
            ),
            UpdateExpressionExpr::VariableName(x) => {
                format!("UpdateExpressionExpr::variable_name({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            UpdateExpressionExpr::Extra(x) => x.as_any(),
            UpdateExpressionExpr::CastExpression(x) => x.as_any(),
            UpdateExpressionExpr::DynamicVariableName(x) => x.as_any(),
            UpdateExpressionExpr::FunctionCallExpression(x) => x.as_any(),
            UpdateExpressionExpr::MemberAccessExpression(x) => x.as_any(),
            UpdateExpressionExpr::MemberCallExpression(x) => x.as_any(),
            UpdateExpressionExpr::NullsafeMemberAccessExpression(x) => x.as_any(),
            UpdateExpressionExpr::NullsafeMemberCallExpression(x) => x.as_any(),
            UpdateExpressionExpr::ScopedCallExpression(x) => x.as_any(),
            UpdateExpressionExpr::ScopedPropertyAccessExpression(x) => x.as_any(),
            UpdateExpressionExpr::SubscriptExpression(x) => x.as_any(),
            UpdateExpressionExpr::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            UpdateExpressionExpr::Extra(x) => x.children_any(),
            UpdateExpressionExpr::CastExpression(x) => x.children_any(),
            UpdateExpressionExpr::DynamicVariableName(x) => x.children_any(),
            UpdateExpressionExpr::FunctionCallExpression(x) => x.children_any(),
            UpdateExpressionExpr::MemberAccessExpression(x) => x.children_any(),
            UpdateExpressionExpr::MemberCallExpression(x) => x.children_any(),
            UpdateExpressionExpr::NullsafeMemberAccessExpression(x) => x.children_any(),
            UpdateExpressionExpr::NullsafeMemberCallExpression(x) => x.children_any(),
            UpdateExpressionExpr::ScopedCallExpression(x) => x.children_any(),
            UpdateExpressionExpr::ScopedPropertyAccessExpression(x) => x.children_any(),
            UpdateExpressionExpr::SubscriptExpression(x) => x.children_any(),
            UpdateExpressionExpr::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            UpdateExpressionExpr::Extra(x) => x.range(),
            UpdateExpressionExpr::CastExpression(x) => x.range(),
            UpdateExpressionExpr::DynamicVariableName(x) => x.range(),
            UpdateExpressionExpr::FunctionCallExpression(x) => x.range(),
            UpdateExpressionExpr::MemberAccessExpression(x) => x.range(),
            UpdateExpressionExpr::MemberCallExpression(x) => x.range(),
            UpdateExpressionExpr::NullsafeMemberAccessExpression(x) => x.range(),
            UpdateExpressionExpr::NullsafeMemberCallExpression(x) => x.range(),
            UpdateExpressionExpr::ScopedCallExpression(x) => x.range(),
            UpdateExpressionExpr::ScopedPropertyAccessExpression(x) => x.range(),
            UpdateExpressionExpr::SubscriptExpression(x) => x.range(),
            UpdateExpressionExpr::VariableName(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UpdateExpressionPostfix {
    Increment(IncrementOperator),
    Decrement(DecrementOperator),
    Extra(ExtraChild),
}

impl UpdateExpressionPostfix {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
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
            "++" => UpdateExpressionPostfix::Increment(IncrementOperator(node.range())),
            "--" => UpdateExpressionPostfix::Decrement(DecrementOperator(node.range())),

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
            "comment" => UpdateExpressionPostfix::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => UpdateExpressionPostfix::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => UpdateExpressionPostfix::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "++" => UpdateExpressionPostfix::Increment(IncrementOperator(node.range())),
            "--" => UpdateExpressionPostfix::Decrement(DecrementOperator(node.range())),

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
}

#[derive(Debug, Clone)]
pub enum UpdateExpressionPrefix {
    Increment(IncrementOperator),
    Decrement(DecrementOperator),
    Extra(ExtraChild),
}

impl UpdateExpressionPrefix {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
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
            "++" => UpdateExpressionPrefix::Increment(IncrementOperator(node.range())),
            "--" => UpdateExpressionPrefix::Decrement(DecrementOperator(node.range())),

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
            "comment" => UpdateExpressionPrefix::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => UpdateExpressionPrefix::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => UpdateExpressionPrefix::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "++" => UpdateExpressionPrefix::Increment(IncrementOperator(node.range())),
            "--" => UpdateExpressionPrefix::Decrement(DecrementOperator(node.range())),

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
}

#[derive(Debug, Clone)]
pub struct UpdateExpressionNode {
    pub range: Range,
    pub expr: Box<UpdateExpressionExpr>,
    pub postfix: Option<Box<UpdateExpressionPostfix>>,
    pub prefix: Option<Box<UpdateExpressionPrefix>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl UpdateExpressionNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "update_expression" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [update_expression] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let expr: Box<UpdateExpressionExpr> = node
            .children_by_field_name("expr", &mut node.walk())
            .map(|chnode2| UpdateExpressionExpr::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .expect("Field expr should exist")
            .into();
        let postfix: Option<Box<UpdateExpressionPostfix>> = node
            .children_by_field_name("postfix", &mut node.walk())
            .map(|chnode2| UpdateExpressionPostfix::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .into();
        let prefix: Option<Box<UpdateExpressionPrefix>> = node
            .children_by_field_name("prefix", &mut node.walk())
            .map(|chnode2| UpdateExpressionPrefix::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .into();
        Ok(Self {
            range,
            expr,
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
        "update_expression"
    }
}

impl NodeAccess for UpdateExpressionNode {
    fn brief_desc(&self) -> String {
        "UpdateExpressionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::UpdateExpression(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.expr.as_any());
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
