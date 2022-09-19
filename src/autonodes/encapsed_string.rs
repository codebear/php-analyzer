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
pub enum EncapsedStringChildren {
    _Expression(Box<_ExpressionNode>),
    DynamicVariableName(Box<DynamicVariableNameNode>),
    EscapeSequence(Box<EscapeSequenceNode>),
    MemberAccessExpression(Box<MemberAccessExpressionNode>),
    StringValue(Box<StringValueNode>),
    SubscriptExpression(Box<SubscriptExpressionNode>),
    VariableName(Box<VariableNameNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl EncapsedStringChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => {
                EncapsedStringChildren::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => EncapsedStringChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => EncapsedStringChildren::Error(Box::new(ErrorNode::parse(node, source)?)),
            "dynamic_variable_name" => EncapsedStringChildren::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "escape_sequence" => EncapsedStringChildren::EscapeSequence(Box::new(
                EscapeSequenceNode::parse(node, source)?,
            )),
            "member_access_expression" => EncapsedStringChildren::MemberAccessExpression(Box::new(
                MemberAccessExpressionNode::parse(node, source)?,
            )),
            "string_value" => {
                EncapsedStringChildren::StringValue(Box::new(StringValueNode::parse(node, source)?))
            }
            "subscript_expression" => EncapsedStringChildren::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => EncapsedStringChildren::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| EncapsedStringChildren::_Expression(y))
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
            "comment" => {
                EncapsedStringChildren::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => EncapsedStringChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => EncapsedStringChildren::Error(Box::new(ErrorNode::parse(node, source)?)),
            "dynamic_variable_name" => EncapsedStringChildren::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "escape_sequence" => EncapsedStringChildren::EscapeSequence(Box::new(
                EscapeSequenceNode::parse(node, source)?,
            )),
            "member_access_expression" => EncapsedStringChildren::MemberAccessExpression(Box::new(
                MemberAccessExpressionNode::parse(node, source)?,
            )),
            "string_value" => {
                EncapsedStringChildren::StringValue(Box::new(StringValueNode::parse(node, source)?))
            }
            "subscript_expression" => EncapsedStringChildren::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => EncapsedStringChildren::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                return Ok(
                    if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| EncapsedStringChildren::_Expression(y))
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
            EncapsedStringChildren::Comment(x) => x.get_utype(state, emitter),
            EncapsedStringChildren::TextInterpolation(x) => x.get_utype(state, emitter),
            EncapsedStringChildren::Error(x) => x.get_utype(state, emitter),
            EncapsedStringChildren::_Expression(x) => x.get_utype(state, emitter),
            EncapsedStringChildren::DynamicVariableName(x) => x.get_utype(state, emitter),
            EncapsedStringChildren::EscapeSequence(x) => x.get_utype(state, emitter),
            EncapsedStringChildren::MemberAccessExpression(x) => x.get_utype(state, emitter),
            EncapsedStringChildren::StringValue(x) => x.get_utype(state, emitter),
            EncapsedStringChildren::SubscriptExpression(x) => x.get_utype(state, emitter),
            EncapsedStringChildren::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            EncapsedStringChildren::Comment(x) => x.get_php_value(state, emitter),
            EncapsedStringChildren::TextInterpolation(x) => x.get_php_value(state, emitter),
            EncapsedStringChildren::Error(x) => x.get_php_value(state, emitter),
            EncapsedStringChildren::_Expression(x) => x.get_php_value(state, emitter),
            EncapsedStringChildren::DynamicVariableName(x) => x.get_php_value(state, emitter),
            EncapsedStringChildren::EscapeSequence(x) => x.get_php_value(state, emitter),
            EncapsedStringChildren::MemberAccessExpression(x) => x.get_php_value(state, emitter),
            EncapsedStringChildren::StringValue(x) => x.get_php_value(state, emitter),
            EncapsedStringChildren::SubscriptExpression(x) => x.get_php_value(state, emitter),
            EncapsedStringChildren::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            EncapsedStringChildren::Comment(x) => x.read_from(state, emitter),
            EncapsedStringChildren::TextInterpolation(x) => x.read_from(state, emitter),
            EncapsedStringChildren::Error(x) => x.read_from(state, emitter),
            EncapsedStringChildren::_Expression(x) => x.read_from(state, emitter),
            EncapsedStringChildren::DynamicVariableName(x) => x.read_from(state, emitter),
            EncapsedStringChildren::EscapeSequence(x) => x.read_from(state, emitter),
            EncapsedStringChildren::MemberAccessExpression(x) => x.read_from(state, emitter),
            EncapsedStringChildren::StringValue(x) => x.read_from(state, emitter),
            EncapsedStringChildren::SubscriptExpression(x) => x.read_from(state, emitter),
            EncapsedStringChildren::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for EncapsedStringChildren {
    fn brief_desc(&self) -> String {
        match self {
            EncapsedStringChildren::Comment(x) => {
                format!("EncapsedStringChildren::comment({})", x.brief_desc())
            }
            EncapsedStringChildren::TextInterpolation(x) => format!(
                "EncapsedStringChildren::text_interpolation({})",
                x.brief_desc()
            ),
            EncapsedStringChildren::Error(x) => {
                format!("EncapsedStringChildren::ERROR({})", x.brief_desc())
            }
            EncapsedStringChildren::_Expression(x) => {
                format!("EncapsedStringChildren::_expression({})", x.brief_desc())
            }
            EncapsedStringChildren::DynamicVariableName(x) => format!(
                "EncapsedStringChildren::dynamic_variable_name({})",
                x.brief_desc()
            ),
            EncapsedStringChildren::EscapeSequence(x) => format!(
                "EncapsedStringChildren::escape_sequence({})",
                x.brief_desc()
            ),
            EncapsedStringChildren::MemberAccessExpression(x) => format!(
                "EncapsedStringChildren::member_access_expression({})",
                x.brief_desc()
            ),
            EncapsedStringChildren::StringValue(x) => {
                format!("EncapsedStringChildren::string_value({})", x.brief_desc())
            }
            EncapsedStringChildren::SubscriptExpression(x) => format!(
                "EncapsedStringChildren::subscript_expression({})",
                x.brief_desc()
            ),
            EncapsedStringChildren::VariableName(x) => {
                format!("EncapsedStringChildren::variable_name({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            EncapsedStringChildren::Comment(x) => x.as_any(),
            EncapsedStringChildren::TextInterpolation(x) => x.as_any(),
            EncapsedStringChildren::Error(x) => x.as_any(),
            EncapsedStringChildren::_Expression(x) => x.as_any(),
            EncapsedStringChildren::DynamicVariableName(x) => x.as_any(),
            EncapsedStringChildren::EscapeSequence(x) => x.as_any(),
            EncapsedStringChildren::MemberAccessExpression(x) => x.as_any(),
            EncapsedStringChildren::StringValue(x) => x.as_any(),
            EncapsedStringChildren::SubscriptExpression(x) => x.as_any(),
            EncapsedStringChildren::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            EncapsedStringChildren::Comment(x) => x.children_any(),
            EncapsedStringChildren::TextInterpolation(x) => x.children_any(),
            EncapsedStringChildren::Error(x) => x.children_any(),
            EncapsedStringChildren::_Expression(x) => x.children_any(),
            EncapsedStringChildren::DynamicVariableName(x) => x.children_any(),
            EncapsedStringChildren::EscapeSequence(x) => x.children_any(),
            EncapsedStringChildren::MemberAccessExpression(x) => x.children_any(),
            EncapsedStringChildren::StringValue(x) => x.children_any(),
            EncapsedStringChildren::SubscriptExpression(x) => x.children_any(),
            EncapsedStringChildren::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            EncapsedStringChildren::Comment(x) => x.range(),
            EncapsedStringChildren::TextInterpolation(x) => x.range(),
            EncapsedStringChildren::Error(x) => x.range(),
            EncapsedStringChildren::_Expression(x) => x.range(),
            EncapsedStringChildren::DynamicVariableName(x) => x.range(),
            EncapsedStringChildren::EscapeSequence(x) => x.range(),
            EncapsedStringChildren::MemberAccessExpression(x) => x.range(),
            EncapsedStringChildren::StringValue(x) => x.range(),
            EncapsedStringChildren::SubscriptExpression(x) => x.range(),
            EncapsedStringChildren::VariableName(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct EncapsedStringNode {
    pub range: Range,
    pub children: Vec<Box<EncapsedStringChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl EncapsedStringNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "encapsed_string" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [encapsed_string] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            children: EncapsedStringChildren::parse_vec(
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
        "encapsed_string"
    }
}

impl NodeAccess for EncapsedStringNode {
    fn brief_desc(&self) -> String {
        "EncapsedStringNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::EncapsedString(self)
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
