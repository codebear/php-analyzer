use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
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
pub enum DynamicVariableNameChildren {
    _Expression(Box<_ExpressionNode>),
    DynamicVariableName(Box<DynamicVariableNameNode>),
    VariableName(Box<VariableNameNode>),
    Extra(ExtraChild),
}

impl NodeParser for DynamicVariableNameChildren {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => DynamicVariableNameChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                DynamicVariableNameChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => DynamicVariableNameChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "dynamic_variable_name" => DynamicVariableNameChildren::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "variable_name" => DynamicVariableNameChildren::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| DynamicVariableNameChildren::_Expression(y))
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

impl DynamicVariableNameChildren {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => DynamicVariableNameChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                DynamicVariableNameChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => DynamicVariableNameChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "dynamic_variable_name" => DynamicVariableNameChildren::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "variable_name" => DynamicVariableNameChildren::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                return Ok(
                    if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| DynamicVariableNameChildren::_Expression(y))
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
            DynamicVariableNameChildren::Extra(y) => y.kind(),
            DynamicVariableNameChildren::_Expression(y) => y.kind(),
            DynamicVariableNameChildren::DynamicVariableName(y) => y.kind(),
            DynamicVariableNameChildren::VariableName(y) => y.kind(),
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
            DynamicVariableNameChildren::Extra(x) => x.get_utype(state, emitter),
            DynamicVariableNameChildren::_Expression(x) => x.get_utype(state, emitter),
            DynamicVariableNameChildren::DynamicVariableName(x) => x.get_utype(state, emitter),
            DynamicVariableNameChildren::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            DynamicVariableNameChildren::Extra(x) => x.get_php_value(state, emitter),
            DynamicVariableNameChildren::_Expression(x) => x.get_php_value(state, emitter),
            DynamicVariableNameChildren::DynamicVariableName(x) => x.get_php_value(state, emitter),
            DynamicVariableNameChildren::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            DynamicVariableNameChildren::Extra(x) => x.read_from(state, emitter),
            DynamicVariableNameChildren::_Expression(x) => x.read_from(state, emitter),
            DynamicVariableNameChildren::DynamicVariableName(x) => x.read_from(state, emitter),
            DynamicVariableNameChildren::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for DynamicVariableNameChildren {
    fn brief_desc(&self) -> String {
        match self {
            DynamicVariableNameChildren::Extra(x) => {
                format!("DynamicVariableNameChildren::extra({})", x.brief_desc())
            }
            DynamicVariableNameChildren::_Expression(x) => format!(
                "DynamicVariableNameChildren::_expression({})",
                x.brief_desc()
            ),
            DynamicVariableNameChildren::DynamicVariableName(x) => format!(
                "DynamicVariableNameChildren::dynamic_variable_name({})",
                x.brief_desc()
            ),
            DynamicVariableNameChildren::VariableName(x) => format!(
                "DynamicVariableNameChildren::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            DynamicVariableNameChildren::Extra(x) => x.as_any(),
            DynamicVariableNameChildren::_Expression(x) => x.as_any(),
            DynamicVariableNameChildren::DynamicVariableName(x) => x.as_any(),
            DynamicVariableNameChildren::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            DynamicVariableNameChildren::Extra(x) => x.children_any(),
            DynamicVariableNameChildren::_Expression(x) => x.children_any(),
            DynamicVariableNameChildren::DynamicVariableName(x) => x.children_any(),
            DynamicVariableNameChildren::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            DynamicVariableNameChildren::Extra(x) => x.range(),
            DynamicVariableNameChildren::_Expression(x) => x.range(),
            DynamicVariableNameChildren::DynamicVariableName(x) => x.range(),
            DynamicVariableNameChildren::VariableName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DynamicVariableNameNode {
    pub range: Range,
    pub child: Box<DynamicVariableNameChildren>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for DynamicVariableNameNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "dynamic_variable_name" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [dynamic_variable_name] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            child: node
                .named_children(&mut node.walk())
                .filter(|node| node.kind() != "comment")
                .map(|k| DynamicVariableNameChildren::parse(k, source))
                .collect::<Result<Vec<DynamicVariableNameChildren>, ParseError>>()?
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
}

impl DynamicVariableNameNode {
    pub fn kind(&self) -> &'static str {
        "dynamic_variable_name"
    }
}

impl NodeAccess for DynamicVariableNameNode {
    fn brief_desc(&self) -> String {
        "DynamicVariableNameNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::DynamicVariableName(self)
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
