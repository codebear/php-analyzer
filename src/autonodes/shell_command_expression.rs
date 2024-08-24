use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::dynamic_variable_name::DynamicVariableNameNode;
use crate::autonodes::escape_sequence::EscapeSequenceNode;
use crate::autonodes::integer::IntegerNode;
use crate::autonodes::member_access_expression::MemberAccessExpressionNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::string_value::StringValueNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::variable_name::VariableNameNode;
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
pub enum ShellCommandExpressionChildren {
    _Expression(Box<_ExpressionNode>),
    DynamicVariableName(Box<DynamicVariableNameNode>),
    EscapeSequence(Box<EscapeSequenceNode>),
    Integer(Box<IntegerNode>),
    MemberAccessExpression(Box<MemberAccessExpressionNode>),
    Name(Box<NameNode>),
    StringValue(Box<StringValueNode>),
    VariableName(Box<VariableNameNode>),
    Extra(ExtraChild),
}

impl NodeParser for ShellCommandExpressionChildren {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ShellCommandExpressionChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                ShellCommandExpressionChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => ShellCommandExpressionChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "dynamic_variable_name" => ShellCommandExpressionChildren::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "escape_sequence" => ShellCommandExpressionChildren::EscapeSequence(Box::new(
                EscapeSequenceNode::parse(node, source)?,
            )),
            "integer" => {
                ShellCommandExpressionChildren::Integer(Box::new(IntegerNode::parse(node, source)?))
            }
            "member_access_expression" => ShellCommandExpressionChildren::MemberAccessExpression(
                Box::new(MemberAccessExpressionNode::parse(node, source)?),
            ),
            "name" => {
                ShellCommandExpressionChildren::Name(Box::new(NameNode::parse(node, source)?))
            }
            "string_value" => ShellCommandExpressionChildren::StringValue(Box::new(
                StringValueNode::parse(node, source)?,
            )),
            "variable_name" => ShellCommandExpressionChildren::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(ShellCommandExpressionChildren::_Expression)
                {
                    x
                } else {
                    return Err(ParseError::new(
                        node.range(),
                        format!(
                            "ShellCommandExpressionChildren: Parse error, unexpected node-type: {}",
                            node.kind()
                        ),
                    ));
                }
            }
        })
    }
}

impl ShellCommandExpressionChildren {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ShellCommandExpressionChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                ShellCommandExpressionChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => ShellCommandExpressionChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "dynamic_variable_name" => ShellCommandExpressionChildren::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "escape_sequence" => ShellCommandExpressionChildren::EscapeSequence(Box::new(
                EscapeSequenceNode::parse(node, source)?,
            )),
            "integer" => {
                ShellCommandExpressionChildren::Integer(Box::new(IntegerNode::parse(node, source)?))
            }
            "member_access_expression" => ShellCommandExpressionChildren::MemberAccessExpression(
                Box::new(MemberAccessExpressionNode::parse(node, source)?),
            ),
            "name" => {
                ShellCommandExpressionChildren::Name(Box::new(NameNode::parse(node, source)?))
            }
            "string_value" => ShellCommandExpressionChildren::StringValue(Box::new(
                StringValueNode::parse(node, source)?,
            )),
            "variable_name" => ShellCommandExpressionChildren::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                return Ok(_ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(ShellCommandExpressionChildren::_Expression))
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            ShellCommandExpressionChildren::Extra(y) => y.kind(),
            ShellCommandExpressionChildren::_Expression(y) => y.kind(),
            ShellCommandExpressionChildren::DynamicVariableName(y) => y.kind(),
            ShellCommandExpressionChildren::EscapeSequence(y) => y.kind(),
            ShellCommandExpressionChildren::Integer(y) => y.kind(),
            ShellCommandExpressionChildren::MemberAccessExpression(y) => y.kind(),
            ShellCommandExpressionChildren::Name(y) => y.kind(),
            ShellCommandExpressionChildren::StringValue(y) => y.kind(),
            ShellCommandExpressionChildren::VariableName(y) => y.kind(),
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
            ShellCommandExpressionChildren::Extra(x) => x.get_utype(state, emitter),
            ShellCommandExpressionChildren::_Expression(x) => x.get_utype(state, emitter),
            ShellCommandExpressionChildren::DynamicVariableName(x) => x.get_utype(state, emitter),
            ShellCommandExpressionChildren::EscapeSequence(x) => x.get_utype(state, emitter),
            ShellCommandExpressionChildren::Integer(x) => x.get_utype(state, emitter),
            ShellCommandExpressionChildren::MemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            ShellCommandExpressionChildren::Name(x) => x.get_utype(state, emitter),
            ShellCommandExpressionChildren::StringValue(x) => x.get_utype(state, emitter),
            ShellCommandExpressionChildren::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ShellCommandExpressionChildren::Extra(x) => x.get_php_value(state, emitter),
            ShellCommandExpressionChildren::_Expression(x) => x.get_php_value(state, emitter),
            ShellCommandExpressionChildren::DynamicVariableName(x) => {
                x.get_php_value(state, emitter)
            }
            ShellCommandExpressionChildren::EscapeSequence(x) => x.get_php_value(state, emitter),
            ShellCommandExpressionChildren::Integer(x) => x.get_php_value(state, emitter),
            ShellCommandExpressionChildren::MemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ShellCommandExpressionChildren::Name(x) => x.get_php_value(state, emitter),
            ShellCommandExpressionChildren::StringValue(x) => x.get_php_value(state, emitter),
            ShellCommandExpressionChildren::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ShellCommandExpressionChildren::Extra(x) => x.read_from(state, emitter),
            ShellCommandExpressionChildren::_Expression(x) => x.read_from(state, emitter),
            ShellCommandExpressionChildren::DynamicVariableName(x) => x.read_from(state, emitter),
            ShellCommandExpressionChildren::EscapeSequence(x) => x.read_from(state, emitter),
            ShellCommandExpressionChildren::Integer(x) => x.read_from(state, emitter),
            ShellCommandExpressionChildren::MemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            ShellCommandExpressionChildren::Name(x) => x.read_from(state, emitter),
            ShellCommandExpressionChildren::StringValue(x) => x.read_from(state, emitter),
            ShellCommandExpressionChildren::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ShellCommandExpressionChildren {
    fn brief_desc(&self) -> String {
        match self {
            ShellCommandExpressionChildren::Extra(x) => {
                format!("ShellCommandExpressionChildren::extra({})", x.brief_desc())
            }
            ShellCommandExpressionChildren::_Expression(x) => format!(
                "ShellCommandExpressionChildren::_expression({})",
                x.brief_desc()
            ),
            ShellCommandExpressionChildren::DynamicVariableName(x) => format!(
                "ShellCommandExpressionChildren::dynamic_variable_name({})",
                x.brief_desc()
            ),
            ShellCommandExpressionChildren::EscapeSequence(x) => format!(
                "ShellCommandExpressionChildren::escape_sequence({})",
                x.brief_desc()
            ),
            ShellCommandExpressionChildren::Integer(x) => format!(
                "ShellCommandExpressionChildren::integer({})",
                x.brief_desc()
            ),
            ShellCommandExpressionChildren::MemberAccessExpression(x) => format!(
                "ShellCommandExpressionChildren::member_access_expression({})",
                x.brief_desc()
            ),
            ShellCommandExpressionChildren::Name(x) => {
                format!("ShellCommandExpressionChildren::name({})", x.brief_desc())
            }
            ShellCommandExpressionChildren::StringValue(x) => format!(
                "ShellCommandExpressionChildren::string_value({})",
                x.brief_desc()
            ),
            ShellCommandExpressionChildren::VariableName(x) => format!(
                "ShellCommandExpressionChildren::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            ShellCommandExpressionChildren::Extra(x) => x.as_any(),
            ShellCommandExpressionChildren::_Expression(x) => x.as_any(),
            ShellCommandExpressionChildren::DynamicVariableName(x) => x.as_any(),
            ShellCommandExpressionChildren::EscapeSequence(x) => x.as_any(),
            ShellCommandExpressionChildren::Integer(x) => x.as_any(),
            ShellCommandExpressionChildren::MemberAccessExpression(x) => x.as_any(),
            ShellCommandExpressionChildren::Name(x) => x.as_any(),
            ShellCommandExpressionChildren::StringValue(x) => x.as_any(),
            ShellCommandExpressionChildren::VariableName(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            ShellCommandExpressionChildren::Extra(x) => x.children_any(),
            ShellCommandExpressionChildren::_Expression(x) => x.children_any(),
            ShellCommandExpressionChildren::DynamicVariableName(x) => x.children_any(),
            ShellCommandExpressionChildren::EscapeSequence(x) => x.children_any(),
            ShellCommandExpressionChildren::Integer(x) => x.children_any(),
            ShellCommandExpressionChildren::MemberAccessExpression(x) => x.children_any(),
            ShellCommandExpressionChildren::Name(x) => x.children_any(),
            ShellCommandExpressionChildren::StringValue(x) => x.children_any(),
            ShellCommandExpressionChildren::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ShellCommandExpressionChildren::Extra(x) => x.range(),
            ShellCommandExpressionChildren::_Expression(x) => x.range(),
            ShellCommandExpressionChildren::DynamicVariableName(x) => x.range(),
            ShellCommandExpressionChildren::EscapeSequence(x) => x.range(),
            ShellCommandExpressionChildren::Integer(x) => x.range(),
            ShellCommandExpressionChildren::MemberAccessExpression(x) => x.range(),
            ShellCommandExpressionChildren::Name(x) => x.range(),
            ShellCommandExpressionChildren::StringValue(x) => x.range(),
            ShellCommandExpressionChildren::VariableName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShellCommandExpressionNode {
    pub range: Range,
    pub children: Vec<Box<ShellCommandExpressionChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for ShellCommandExpressionNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "shell_command_expression" {
            return Err(ParseError::new(range, format!("ShellCommandExpressionNode: Node is of the wrong kind [{}] vs expected [shell_command_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            children: ShellCommandExpressionChildren::parse_vec(
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

impl ShellCommandExpressionNode {
    pub fn kind(&self) -> &'static str {
        "shell_command_expression"
    }
}

impl NodeAccess for ShellCommandExpressionNode {
    fn brief_desc(&self) -> String {
        "ShellCommandExpressionNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::ShellCommandExpression(self)
    }

    #[allow(clippy::vec_init_then_push)]
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
