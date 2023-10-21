use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
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
pub enum SequenceExpressionChildren {
    _Expression(Box<_ExpressionNode>),
    SequenceExpression(Box<SequenceExpressionNode>),
    Extra(ExtraChild),
}

impl SequenceExpressionChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => SequenceExpressionChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                SequenceExpressionChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => SequenceExpressionChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "sequence_expression" => SequenceExpressionChildren::SequenceExpression(Box::new(
                SequenceExpressionNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| SequenceExpressionChildren::_Expression(y))
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
            "comment" => SequenceExpressionChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                SequenceExpressionChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => SequenceExpressionChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "sequence_expression" => SequenceExpressionChildren::SequenceExpression(Box::new(
                SequenceExpressionNode::parse(node, source)?,
            )),

            _ => {
                return Ok(
                    if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| SequenceExpressionChildren::_Expression(y))
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
            SequenceExpressionChildren::Extra(y) => y.kind(),
            SequenceExpressionChildren::_Expression(y) => y.kind(),
            SequenceExpressionChildren::SequenceExpression(y) => y.kind(),
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
            SequenceExpressionChildren::Extra(x) => x.get_utype(state, emitter),
            SequenceExpressionChildren::_Expression(x) => x.get_utype(state, emitter),
            SequenceExpressionChildren::SequenceExpression(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            SequenceExpressionChildren::Extra(x) => x.get_php_value(state, emitter),
            SequenceExpressionChildren::_Expression(x) => x.get_php_value(state, emitter),
            SequenceExpressionChildren::SequenceExpression(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            SequenceExpressionChildren::Extra(x) => x.read_from(state, emitter),
            SequenceExpressionChildren::_Expression(x) => x.read_from(state, emitter),
            SequenceExpressionChildren::SequenceExpression(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for SequenceExpressionChildren {
    fn brief_desc(&self) -> String {
        match self {
            SequenceExpressionChildren::Extra(x) => {
                format!("SequenceExpressionChildren::extra({})", x.brief_desc())
            }
            SequenceExpressionChildren::_Expression(x) => format!(
                "SequenceExpressionChildren::_expression({})",
                x.brief_desc()
            ),
            SequenceExpressionChildren::SequenceExpression(x) => format!(
                "SequenceExpressionChildren::sequence_expression({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            SequenceExpressionChildren::Extra(x) => x.as_any(),
            SequenceExpressionChildren::_Expression(x) => x.as_any(),
            SequenceExpressionChildren::SequenceExpression(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            SequenceExpressionChildren::Extra(x) => x.children_any(),
            SequenceExpressionChildren::_Expression(x) => x.children_any(),
            SequenceExpressionChildren::SequenceExpression(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            SequenceExpressionChildren::Extra(x) => x.range(),
            SequenceExpressionChildren::_Expression(x) => x.range(),
            SequenceExpressionChildren::SequenceExpression(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct SequenceExpressionNode {
    pub range: Range,
    pub children: Vec<Box<SequenceExpressionChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl SequenceExpressionNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "sequence_expression" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [sequence_expression] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            children: SequenceExpressionChildren::parse_vec(
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
        "sequence_expression"
    }
}

impl NodeAccess for SequenceExpressionNode {
    fn brief_desc(&self) -> String {
        "SequenceExpressionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::SequenceExpression(self)
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
