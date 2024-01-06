use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::dynamic_variable_name::DynamicVariableNameNode;
use crate::autonodes::escape_sequence::EscapeSequenceNode;
use crate::autonodes::member_access_expression::MemberAccessExpressionNode;
use crate::autonodes::string_value::StringValueNode;
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
pub enum HeredocBodyChildren {
    _Expression(Box<_ExpressionNode>),
    DynamicVariableName(Box<DynamicVariableNameNode>),
    EscapeSequence(Box<EscapeSequenceNode>),
    MemberAccessExpression(Box<MemberAccessExpressionNode>),
    StringValue(Box<StringValueNode>),
    SubscriptExpression(Box<SubscriptExpressionNode>),
    VariableName(Box<VariableNameNode>),
    Extra(ExtraChild),
}

impl HeredocBodyChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => HeredocBodyChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => HeredocBodyChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => HeredocBodyChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "dynamic_variable_name" => HeredocBodyChildren::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "escape_sequence" => HeredocBodyChildren::EscapeSequence(Box::new(
                EscapeSequenceNode::parse(node, source)?,
            )),
            "member_access_expression" => HeredocBodyChildren::MemberAccessExpression(Box::new(
                MemberAccessExpressionNode::parse(node, source)?,
            )),
            "string_value" => {
                HeredocBodyChildren::StringValue(Box::new(StringValueNode::parse(node, source)?))
            }
            "subscript_expression" => HeredocBodyChildren::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => {
                HeredocBodyChildren::VariableName(Box::new(VariableNameNode::parse(node, source)?))
            }

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| HeredocBodyChildren::_Expression(y))
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
            "comment" => HeredocBodyChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => HeredocBodyChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => HeredocBodyChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "dynamic_variable_name" => HeredocBodyChildren::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "escape_sequence" => HeredocBodyChildren::EscapeSequence(Box::new(
                EscapeSequenceNode::parse(node, source)?,
            )),
            "member_access_expression" => HeredocBodyChildren::MemberAccessExpression(Box::new(
                MemberAccessExpressionNode::parse(node, source)?,
            )),
            "string_value" => {
                HeredocBodyChildren::StringValue(Box::new(StringValueNode::parse(node, source)?))
            }
            "subscript_expression" => HeredocBodyChildren::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => {
                HeredocBodyChildren::VariableName(Box::new(VariableNameNode::parse(node, source)?))
            }

            _ => {
                return Ok(
                    if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| HeredocBodyChildren::_Expression(y))
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
            HeredocBodyChildren::Extra(y) => y.kind(),
            HeredocBodyChildren::_Expression(y) => y.kind(),
            HeredocBodyChildren::DynamicVariableName(y) => y.kind(),
            HeredocBodyChildren::EscapeSequence(y) => y.kind(),
            HeredocBodyChildren::MemberAccessExpression(y) => y.kind(),
            HeredocBodyChildren::StringValue(y) => y.kind(),
            HeredocBodyChildren::SubscriptExpression(y) => y.kind(),
            HeredocBodyChildren::VariableName(y) => y.kind(),
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
            HeredocBodyChildren::Extra(x) => x.get_utype(state, emitter),
            HeredocBodyChildren::_Expression(x) => x.get_utype(state, emitter),
            HeredocBodyChildren::DynamicVariableName(x) => x.get_utype(state, emitter),
            HeredocBodyChildren::EscapeSequence(x) => x.get_utype(state, emitter),
            HeredocBodyChildren::MemberAccessExpression(x) => x.get_utype(state, emitter),
            HeredocBodyChildren::StringValue(x) => x.get_utype(state, emitter),
            HeredocBodyChildren::SubscriptExpression(x) => x.get_utype(state, emitter),
            HeredocBodyChildren::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            HeredocBodyChildren::Extra(x) => x.get_php_value(state, emitter),
            HeredocBodyChildren::_Expression(x) => x.get_php_value(state, emitter),
            HeredocBodyChildren::DynamicVariableName(x) => x.get_php_value(state, emitter),
            HeredocBodyChildren::EscapeSequence(x) => x.get_php_value(state, emitter),
            HeredocBodyChildren::MemberAccessExpression(x) => x.get_php_value(state, emitter),
            HeredocBodyChildren::StringValue(x) => x.get_php_value(state, emitter),
            HeredocBodyChildren::SubscriptExpression(x) => x.get_php_value(state, emitter),
            HeredocBodyChildren::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            HeredocBodyChildren::Extra(x) => x.read_from(state, emitter),
            HeredocBodyChildren::_Expression(x) => x.read_from(state, emitter),
            HeredocBodyChildren::DynamicVariableName(x) => x.read_from(state, emitter),
            HeredocBodyChildren::EscapeSequence(x) => x.read_from(state, emitter),
            HeredocBodyChildren::MemberAccessExpression(x) => x.read_from(state, emitter),
            HeredocBodyChildren::StringValue(x) => x.read_from(state, emitter),
            HeredocBodyChildren::SubscriptExpression(x) => x.read_from(state, emitter),
            HeredocBodyChildren::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for HeredocBodyChildren {
    fn brief_desc(&self) -> String {
        match self {
            HeredocBodyChildren::Extra(x) => {
                format!("HeredocBodyChildren::extra({})", x.brief_desc())
            }
            HeredocBodyChildren::_Expression(x) => {
                format!("HeredocBodyChildren::_expression({})", x.brief_desc())
            }
            HeredocBodyChildren::DynamicVariableName(x) => format!(
                "HeredocBodyChildren::dynamic_variable_name({})",
                x.brief_desc()
            ),
            HeredocBodyChildren::EscapeSequence(x) => {
                format!("HeredocBodyChildren::escape_sequence({})", x.brief_desc())
            }
            HeredocBodyChildren::MemberAccessExpression(x) => format!(
                "HeredocBodyChildren::member_access_expression({})",
                x.brief_desc()
            ),
            HeredocBodyChildren::StringValue(x) => {
                format!("HeredocBodyChildren::string_value({})", x.brief_desc())
            }
            HeredocBodyChildren::SubscriptExpression(x) => format!(
                "HeredocBodyChildren::subscript_expression({})",
                x.brief_desc()
            ),
            HeredocBodyChildren::VariableName(x) => {
                format!("HeredocBodyChildren::variable_name({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            HeredocBodyChildren::Extra(x) => x.as_any(),
            HeredocBodyChildren::_Expression(x) => x.as_any(),
            HeredocBodyChildren::DynamicVariableName(x) => x.as_any(),
            HeredocBodyChildren::EscapeSequence(x) => x.as_any(),
            HeredocBodyChildren::MemberAccessExpression(x) => x.as_any(),
            HeredocBodyChildren::StringValue(x) => x.as_any(),
            HeredocBodyChildren::SubscriptExpression(x) => x.as_any(),
            HeredocBodyChildren::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            HeredocBodyChildren::Extra(x) => x.children_any(),
            HeredocBodyChildren::_Expression(x) => x.children_any(),
            HeredocBodyChildren::DynamicVariableName(x) => x.children_any(),
            HeredocBodyChildren::EscapeSequence(x) => x.children_any(),
            HeredocBodyChildren::MemberAccessExpression(x) => x.children_any(),
            HeredocBodyChildren::StringValue(x) => x.children_any(),
            HeredocBodyChildren::SubscriptExpression(x) => x.children_any(),
            HeredocBodyChildren::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            HeredocBodyChildren::Extra(x) => x.range(),
            HeredocBodyChildren::_Expression(x) => x.range(),
            HeredocBodyChildren::DynamicVariableName(x) => x.range(),
            HeredocBodyChildren::EscapeSequence(x) => x.range(),
            HeredocBodyChildren::MemberAccessExpression(x) => x.range(),
            HeredocBodyChildren::StringValue(x) => x.range(),
            HeredocBodyChildren::SubscriptExpression(x) => x.range(),
            HeredocBodyChildren::VariableName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HeredocBodyNode {
    pub range: Range,
    pub children: Vec<Box<HeredocBodyChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl HeredocBodyNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "heredoc_body" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [heredoc_body] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            children: HeredocBodyChildren::parse_vec(
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
        "heredoc_body"
    }
}

impl NodeAccess for HeredocBodyNode {
    fn brief_desc(&self) -> String {
        "HeredocBodyNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::HeredocBody(self)
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
