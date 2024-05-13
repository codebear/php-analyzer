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
use crate::autonodes::variable_name::VariableNameNode;
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
pub enum EncapsedStringChildren {
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

impl NodeParser for EncapsedStringChildren {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => EncapsedStringChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => EncapsedStringChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "dynamic_variable_name" => EncapsedStringChildren::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "escape_sequence" => EncapsedStringChildren::EscapeSequence(Box::new(
                EscapeSequenceNode::parse(node, source)?,
            )),
            "integer" => {
                EncapsedStringChildren::Integer(Box::new(IntegerNode::parse(node, source)?))
            }
            "member_access_expression" => EncapsedStringChildren::MemberAccessExpression(Box::new(
                MemberAccessExpressionNode::parse(node, source)?,
            )),
            "name" => EncapsedStringChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "string_value" => {
                EncapsedStringChildren::StringValue(Box::new(StringValueNode::parse(node, source)?))
            }
            "variable_name" => EncapsedStringChildren::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(EncapsedStringChildren::_Expression)
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

impl EncapsedStringChildren {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => EncapsedStringChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => EncapsedStringChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "dynamic_variable_name" => EncapsedStringChildren::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "escape_sequence" => EncapsedStringChildren::EscapeSequence(Box::new(
                EscapeSequenceNode::parse(node, source)?,
            )),
            "integer" => {
                EncapsedStringChildren::Integer(Box::new(IntegerNode::parse(node, source)?))
            }
            "member_access_expression" => EncapsedStringChildren::MemberAccessExpression(Box::new(
                MemberAccessExpressionNode::parse(node, source)?,
            )),
            "name" => EncapsedStringChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "string_value" => {
                EncapsedStringChildren::StringValue(Box::new(StringValueNode::parse(node, source)?))
            }
            "variable_name" => EncapsedStringChildren::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                return Ok(_ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(EncapsedStringChildren::_Expression))
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            EncapsedStringChildren::Extra(y) => y.kind(),
            EncapsedStringChildren::_Expression(y) => y.kind(),
            EncapsedStringChildren::DynamicVariableName(y) => y.kind(),
            EncapsedStringChildren::EscapeSequence(y) => y.kind(),
            EncapsedStringChildren::Integer(y) => y.kind(),
            EncapsedStringChildren::MemberAccessExpression(y) => y.kind(),
            EncapsedStringChildren::Name(y) => y.kind(),
            EncapsedStringChildren::StringValue(y) => y.kind(),
            EncapsedStringChildren::VariableName(y) => y.kind(),
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
            EncapsedStringChildren::Extra(x) => x.get_utype(state, emitter),
            EncapsedStringChildren::_Expression(x) => x.get_utype(state, emitter),
            EncapsedStringChildren::DynamicVariableName(x) => x.get_utype(state, emitter),
            EncapsedStringChildren::EscapeSequence(x) => x.get_utype(state, emitter),
            EncapsedStringChildren::Integer(x) => x.get_utype(state, emitter),
            EncapsedStringChildren::MemberAccessExpression(x) => x.get_utype(state, emitter),
            EncapsedStringChildren::Name(x) => x.get_utype(state, emitter),
            EncapsedStringChildren::StringValue(x) => x.get_utype(state, emitter),
            EncapsedStringChildren::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            EncapsedStringChildren::Extra(x) => x.get_php_value(state, emitter),
            EncapsedStringChildren::_Expression(x) => x.get_php_value(state, emitter),
            EncapsedStringChildren::DynamicVariableName(x) => x.get_php_value(state, emitter),
            EncapsedStringChildren::EscapeSequence(x) => x.get_php_value(state, emitter),
            EncapsedStringChildren::Integer(x) => x.get_php_value(state, emitter),
            EncapsedStringChildren::MemberAccessExpression(x) => x.get_php_value(state, emitter),
            EncapsedStringChildren::Name(x) => x.get_php_value(state, emitter),
            EncapsedStringChildren::StringValue(x) => x.get_php_value(state, emitter),
            EncapsedStringChildren::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            EncapsedStringChildren::Extra(x) => x.read_from(state, emitter),
            EncapsedStringChildren::_Expression(x) => x.read_from(state, emitter),
            EncapsedStringChildren::DynamicVariableName(x) => x.read_from(state, emitter),
            EncapsedStringChildren::EscapeSequence(x) => x.read_from(state, emitter),
            EncapsedStringChildren::Integer(x) => x.read_from(state, emitter),
            EncapsedStringChildren::MemberAccessExpression(x) => x.read_from(state, emitter),
            EncapsedStringChildren::Name(x) => x.read_from(state, emitter),
            EncapsedStringChildren::StringValue(x) => x.read_from(state, emitter),
            EncapsedStringChildren::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for EncapsedStringChildren {
    fn brief_desc(&self) -> String {
        match self {
            EncapsedStringChildren::Extra(x) => {
                format!("EncapsedStringChildren::extra({})", x.brief_desc())
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
            EncapsedStringChildren::Integer(x) => {
                format!("EncapsedStringChildren::integer({})", x.brief_desc())
            }
            EncapsedStringChildren::MemberAccessExpression(x) => format!(
                "EncapsedStringChildren::member_access_expression({})",
                x.brief_desc()
            ),
            EncapsedStringChildren::Name(x) => {
                format!("EncapsedStringChildren::name({})", x.brief_desc())
            }
            EncapsedStringChildren::StringValue(x) => {
                format!("EncapsedStringChildren::string_value({})", x.brief_desc())
            }
            EncapsedStringChildren::VariableName(x) => {
                format!("EncapsedStringChildren::variable_name({})", x.brief_desc())
            }
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            EncapsedStringChildren::Extra(x) => x.as_any(),
            EncapsedStringChildren::_Expression(x) => x.as_any(),
            EncapsedStringChildren::DynamicVariableName(x) => x.as_any(),
            EncapsedStringChildren::EscapeSequence(x) => x.as_any(),
            EncapsedStringChildren::Integer(x) => x.as_any(),
            EncapsedStringChildren::MemberAccessExpression(x) => x.as_any(),
            EncapsedStringChildren::Name(x) => x.as_any(),
            EncapsedStringChildren::StringValue(x) => x.as_any(),
            EncapsedStringChildren::VariableName(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            EncapsedStringChildren::Extra(x) => x.children_any(),
            EncapsedStringChildren::_Expression(x) => x.children_any(),
            EncapsedStringChildren::DynamicVariableName(x) => x.children_any(),
            EncapsedStringChildren::EscapeSequence(x) => x.children_any(),
            EncapsedStringChildren::Integer(x) => x.children_any(),
            EncapsedStringChildren::MemberAccessExpression(x) => x.children_any(),
            EncapsedStringChildren::Name(x) => x.children_any(),
            EncapsedStringChildren::StringValue(x) => x.children_any(),
            EncapsedStringChildren::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            EncapsedStringChildren::Extra(x) => x.range(),
            EncapsedStringChildren::_Expression(x) => x.range(),
            EncapsedStringChildren::DynamicVariableName(x) => x.range(),
            EncapsedStringChildren::EscapeSequence(x) => x.range(),
            EncapsedStringChildren::Integer(x) => x.range(),
            EncapsedStringChildren::MemberAccessExpression(x) => x.range(),
            EncapsedStringChildren::Name(x) => x.range(),
            EncapsedStringChildren::StringValue(x) => x.range(),
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

impl NodeParser for EncapsedStringNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
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
}

impl EncapsedStringNode {
    pub fn kind(&self) -> &'static str {
        "encapsed_string"
    }
}

impl NodeAccess for EncapsedStringNode {
    fn brief_desc(&self) -> String {
        "EncapsedStringNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
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
