use crate::analysis::state::AnalysisState;
use crate::autonodes::_primary_expression::_PrimaryExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::assignment_expression::AssignmentExpressionNode;
use crate::autonodes::augmented_assignment_expression::AugmentedAssignmentExpressionNode;
use crate::autonodes::clone_expression::CloneExpressionNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::match_expression::MatchExpressionNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::unary_op_expression::UnaryOpExpressionNode;
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
pub enum ExponentiationExpressionLeft {
    _PrimaryExpression(Box<_PrimaryExpressionNode>),
    CloneExpression(Box<CloneExpressionNode>),
    MatchExpression(Box<MatchExpressionNode>),
    UnaryOpExpression(Box<UnaryOpExpressionNode>),
    Extra(ExtraChild),
}

impl ExponentiationExpressionLeft {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ExponentiationExpressionLeft::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                ExponentiationExpressionLeft::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => ExponentiationExpressionLeft::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "clone_expression" => ExponentiationExpressionLeft::CloneExpression(Box::new(
                CloneExpressionNode::parse(node, source)?,
            )),
            "match_expression" => ExponentiationExpressionLeft::MatchExpression(Box::new(
                MatchExpressionNode::parse(node, source)?,
            )),
            "unary_op_expression" => ExponentiationExpressionLeft::UnaryOpExpression(Box::new(
                UnaryOpExpressionNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _PrimaryExpressionNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| ExponentiationExpressionLeft::_PrimaryExpression(y))
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
            "comment" => ExponentiationExpressionLeft::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                ExponentiationExpressionLeft::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => ExponentiationExpressionLeft::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "clone_expression" => ExponentiationExpressionLeft::CloneExpression(Box::new(
                CloneExpressionNode::parse(node, source)?,
            )),
            "match_expression" => ExponentiationExpressionLeft::MatchExpression(Box::new(
                MatchExpressionNode::parse(node, source)?,
            )),
            "unary_op_expression" => ExponentiationExpressionLeft::UnaryOpExpression(Box::new(
                UnaryOpExpressionNode::parse(node, source)?,
            )),

            _ => {
                return Ok(
                    if let Some(x) = _PrimaryExpressionNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| ExponentiationExpressionLeft::_PrimaryExpression(y))
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
            ExponentiationExpressionLeft::Extra(y) => y.kind(),
            ExponentiationExpressionLeft::_PrimaryExpression(y) => y.kind(),
            ExponentiationExpressionLeft::CloneExpression(y) => y.kind(),
            ExponentiationExpressionLeft::MatchExpression(y) => y.kind(),
            ExponentiationExpressionLeft::UnaryOpExpression(y) => y.kind(),
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
            ExponentiationExpressionLeft::Extra(x) => x.get_utype(state, emitter),
            ExponentiationExpressionLeft::_PrimaryExpression(x) => x.get_utype(state, emitter),
            ExponentiationExpressionLeft::CloneExpression(x) => x.get_utype(state, emitter),
            ExponentiationExpressionLeft::MatchExpression(x) => x.get_utype(state, emitter),
            ExponentiationExpressionLeft::UnaryOpExpression(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ExponentiationExpressionLeft::Extra(x) => x.get_php_value(state, emitter),
            ExponentiationExpressionLeft::_PrimaryExpression(x) => x.get_php_value(state, emitter),
            ExponentiationExpressionLeft::CloneExpression(x) => x.get_php_value(state, emitter),
            ExponentiationExpressionLeft::MatchExpression(x) => x.get_php_value(state, emitter),
            ExponentiationExpressionLeft::UnaryOpExpression(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ExponentiationExpressionLeft::Extra(x) => x.read_from(state, emitter),
            ExponentiationExpressionLeft::_PrimaryExpression(x) => x.read_from(state, emitter),
            ExponentiationExpressionLeft::CloneExpression(x) => x.read_from(state, emitter),
            ExponentiationExpressionLeft::MatchExpression(x) => x.read_from(state, emitter),
            ExponentiationExpressionLeft::UnaryOpExpression(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ExponentiationExpressionLeft {
    fn brief_desc(&self) -> String {
        match self {
            ExponentiationExpressionLeft::Extra(x) => {
                format!("ExponentiationExpressionLeft::extra({})", x.brief_desc())
            }
            ExponentiationExpressionLeft::_PrimaryExpression(x) => format!(
                "ExponentiationExpressionLeft::_primary_expression({})",
                x.brief_desc()
            ),
            ExponentiationExpressionLeft::CloneExpression(x) => format!(
                "ExponentiationExpressionLeft::clone_expression({})",
                x.brief_desc()
            ),
            ExponentiationExpressionLeft::MatchExpression(x) => format!(
                "ExponentiationExpressionLeft::match_expression({})",
                x.brief_desc()
            ),
            ExponentiationExpressionLeft::UnaryOpExpression(x) => format!(
                "ExponentiationExpressionLeft::unary_op_expression({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ExponentiationExpressionLeft::Extra(x) => x.as_any(),
            ExponentiationExpressionLeft::_PrimaryExpression(x) => x.as_any(),
            ExponentiationExpressionLeft::CloneExpression(x) => x.as_any(),
            ExponentiationExpressionLeft::MatchExpression(x) => x.as_any(),
            ExponentiationExpressionLeft::UnaryOpExpression(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ExponentiationExpressionLeft::Extra(x) => x.children_any(),
            ExponentiationExpressionLeft::_PrimaryExpression(x) => x.children_any(),
            ExponentiationExpressionLeft::CloneExpression(x) => x.children_any(),
            ExponentiationExpressionLeft::MatchExpression(x) => x.children_any(),
            ExponentiationExpressionLeft::UnaryOpExpression(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ExponentiationExpressionLeft::Extra(x) => x.range(),
            ExponentiationExpressionLeft::_PrimaryExpression(x) => x.range(),
            ExponentiationExpressionLeft::CloneExpression(x) => x.range(),
            ExponentiationExpressionLeft::MatchExpression(x) => x.range(),
            ExponentiationExpressionLeft::UnaryOpExpression(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExponentiationExpressionRight {
    _PrimaryExpression(Box<_PrimaryExpressionNode>),
    AssignmentExpression(Box<AssignmentExpressionNode>),
    AugmentedAssignmentExpression(Box<AugmentedAssignmentExpressionNode>),
    CloneExpression(Box<CloneExpressionNode>),
    ExponentiationExpression(Box<ExponentiationExpressionNode>),
    MatchExpression(Box<MatchExpressionNode>),
    UnaryOpExpression(Box<UnaryOpExpressionNode>),
    Extra(ExtraChild),
}

impl ExponentiationExpressionRight {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ExponentiationExpressionRight::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                ExponentiationExpressionRight::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => ExponentiationExpressionRight::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "assignment_expression" => ExponentiationExpressionRight::AssignmentExpression(
                Box::new(AssignmentExpressionNode::parse(node, source)?),
            ),
            "augmented_assignment_expression" => {
                ExponentiationExpressionRight::AugmentedAssignmentExpression(Box::new(
                    AugmentedAssignmentExpressionNode::parse(node, source)?,
                ))
            }
            "clone_expression" => ExponentiationExpressionRight::CloneExpression(Box::new(
                CloneExpressionNode::parse(node, source)?,
            )),
            "exponentiation_expression" => ExponentiationExpressionRight::ExponentiationExpression(
                Box::new(ExponentiationExpressionNode::parse(node, source)?),
            ),
            "match_expression" => ExponentiationExpressionRight::MatchExpression(Box::new(
                MatchExpressionNode::parse(node, source)?,
            )),
            "unary_op_expression" => ExponentiationExpressionRight::UnaryOpExpression(Box::new(
                UnaryOpExpressionNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _PrimaryExpressionNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| ExponentiationExpressionRight::_PrimaryExpression(y))
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
            "comment" => ExponentiationExpressionRight::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                ExponentiationExpressionRight::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => ExponentiationExpressionRight::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "assignment_expression" => ExponentiationExpressionRight::AssignmentExpression(
                Box::new(AssignmentExpressionNode::parse(node, source)?),
            ),
            "augmented_assignment_expression" => {
                ExponentiationExpressionRight::AugmentedAssignmentExpression(Box::new(
                    AugmentedAssignmentExpressionNode::parse(node, source)?,
                ))
            }
            "clone_expression" => ExponentiationExpressionRight::CloneExpression(Box::new(
                CloneExpressionNode::parse(node, source)?,
            )),
            "exponentiation_expression" => ExponentiationExpressionRight::ExponentiationExpression(
                Box::new(ExponentiationExpressionNode::parse(node, source)?),
            ),
            "match_expression" => ExponentiationExpressionRight::MatchExpression(Box::new(
                MatchExpressionNode::parse(node, source)?,
            )),
            "unary_op_expression" => ExponentiationExpressionRight::UnaryOpExpression(Box::new(
                UnaryOpExpressionNode::parse(node, source)?,
            )),

            _ => {
                return Ok(
                    if let Some(x) = _PrimaryExpressionNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| ExponentiationExpressionRight::_PrimaryExpression(y))
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
            ExponentiationExpressionRight::Extra(y) => y.kind(),
            ExponentiationExpressionRight::_PrimaryExpression(y) => y.kind(),
            ExponentiationExpressionRight::AssignmentExpression(y) => y.kind(),
            ExponentiationExpressionRight::AugmentedAssignmentExpression(y) => y.kind(),
            ExponentiationExpressionRight::CloneExpression(y) => y.kind(),
            ExponentiationExpressionRight::ExponentiationExpression(y) => y.kind(),
            ExponentiationExpressionRight::MatchExpression(y) => y.kind(),
            ExponentiationExpressionRight::UnaryOpExpression(y) => y.kind(),
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
            ExponentiationExpressionRight::Extra(x) => x.get_utype(state, emitter),
            ExponentiationExpressionRight::_PrimaryExpression(x) => x.get_utype(state, emitter),
            ExponentiationExpressionRight::AssignmentExpression(x) => x.get_utype(state, emitter),
            ExponentiationExpressionRight::AugmentedAssignmentExpression(x) => {
                x.get_utype(state, emitter)
            }
            ExponentiationExpressionRight::CloneExpression(x) => x.get_utype(state, emitter),
            ExponentiationExpressionRight::ExponentiationExpression(x) => {
                x.get_utype(state, emitter)
            }
            ExponentiationExpressionRight::MatchExpression(x) => x.get_utype(state, emitter),
            ExponentiationExpressionRight::UnaryOpExpression(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ExponentiationExpressionRight::Extra(x) => x.get_php_value(state, emitter),
            ExponentiationExpressionRight::_PrimaryExpression(x) => x.get_php_value(state, emitter),
            ExponentiationExpressionRight::AssignmentExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ExponentiationExpressionRight::AugmentedAssignmentExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ExponentiationExpressionRight::CloneExpression(x) => x.get_php_value(state, emitter),
            ExponentiationExpressionRight::ExponentiationExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ExponentiationExpressionRight::MatchExpression(x) => x.get_php_value(state, emitter),
            ExponentiationExpressionRight::UnaryOpExpression(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ExponentiationExpressionRight::Extra(x) => x.read_from(state, emitter),
            ExponentiationExpressionRight::_PrimaryExpression(x) => x.read_from(state, emitter),
            ExponentiationExpressionRight::AssignmentExpression(x) => x.read_from(state, emitter),
            ExponentiationExpressionRight::AugmentedAssignmentExpression(x) => {
                x.read_from(state, emitter)
            }
            ExponentiationExpressionRight::CloneExpression(x) => x.read_from(state, emitter),
            ExponentiationExpressionRight::ExponentiationExpression(x) => {
                x.read_from(state, emitter)
            }
            ExponentiationExpressionRight::MatchExpression(x) => x.read_from(state, emitter),
            ExponentiationExpressionRight::UnaryOpExpression(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ExponentiationExpressionRight {
    fn brief_desc(&self) -> String {
        match self {
            ExponentiationExpressionRight::Extra(x) => {
                format!("ExponentiationExpressionRight::extra({})", x.brief_desc())
            }
            ExponentiationExpressionRight::_PrimaryExpression(x) => format!(
                "ExponentiationExpressionRight::_primary_expression({})",
                x.brief_desc()
            ),
            ExponentiationExpressionRight::AssignmentExpression(x) => format!(
                "ExponentiationExpressionRight::assignment_expression({})",
                x.brief_desc()
            ),
            ExponentiationExpressionRight::AugmentedAssignmentExpression(x) => format!(
                "ExponentiationExpressionRight::augmented_assignment_expression({})",
                x.brief_desc()
            ),
            ExponentiationExpressionRight::CloneExpression(x) => format!(
                "ExponentiationExpressionRight::clone_expression({})",
                x.brief_desc()
            ),
            ExponentiationExpressionRight::ExponentiationExpression(x) => format!(
                "ExponentiationExpressionRight::exponentiation_expression({})",
                x.brief_desc()
            ),
            ExponentiationExpressionRight::MatchExpression(x) => format!(
                "ExponentiationExpressionRight::match_expression({})",
                x.brief_desc()
            ),
            ExponentiationExpressionRight::UnaryOpExpression(x) => format!(
                "ExponentiationExpressionRight::unary_op_expression({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ExponentiationExpressionRight::Extra(x) => x.as_any(),
            ExponentiationExpressionRight::_PrimaryExpression(x) => x.as_any(),
            ExponentiationExpressionRight::AssignmentExpression(x) => x.as_any(),
            ExponentiationExpressionRight::AugmentedAssignmentExpression(x) => x.as_any(),
            ExponentiationExpressionRight::CloneExpression(x) => x.as_any(),
            ExponentiationExpressionRight::ExponentiationExpression(x) => x.as_any(),
            ExponentiationExpressionRight::MatchExpression(x) => x.as_any(),
            ExponentiationExpressionRight::UnaryOpExpression(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ExponentiationExpressionRight::Extra(x) => x.children_any(),
            ExponentiationExpressionRight::_PrimaryExpression(x) => x.children_any(),
            ExponentiationExpressionRight::AssignmentExpression(x) => x.children_any(),
            ExponentiationExpressionRight::AugmentedAssignmentExpression(x) => x.children_any(),
            ExponentiationExpressionRight::CloneExpression(x) => x.children_any(),
            ExponentiationExpressionRight::ExponentiationExpression(x) => x.children_any(),
            ExponentiationExpressionRight::MatchExpression(x) => x.children_any(),
            ExponentiationExpressionRight::UnaryOpExpression(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ExponentiationExpressionRight::Extra(x) => x.range(),
            ExponentiationExpressionRight::_PrimaryExpression(x) => x.range(),
            ExponentiationExpressionRight::AssignmentExpression(x) => x.range(),
            ExponentiationExpressionRight::AugmentedAssignmentExpression(x) => x.range(),
            ExponentiationExpressionRight::CloneExpression(x) => x.range(),
            ExponentiationExpressionRight::ExponentiationExpression(x) => x.range(),
            ExponentiationExpressionRight::MatchExpression(x) => x.range(),
            ExponentiationExpressionRight::UnaryOpExpression(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ExponentiationExpressionNode {
    pub range: Range,
    pub left: Box<ExponentiationExpressionLeft>,
    pub right: Box<ExponentiationExpressionRight>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl ExponentiationExpressionNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "exponentiation_expression" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [exponentiation_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let left: Box<ExponentiationExpressionLeft> = node
            .children_by_field_name("left", &mut node.walk())
            .map(|chnode2| ExponentiationExpressionLeft::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .expect("Field left should exist")
            .into();
        let right: Box<ExponentiationExpressionRight> = node
            .children_by_field_name("right", &mut node.walk())
            .map(|chnode2| ExponentiationExpressionRight::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .expect("Field right should exist")
            .into();
        Ok(Self {
            range,
            left,
            right,
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
        "exponentiation_expression"
    }
}

impl NodeAccess for ExponentiationExpressionNode {
    fn brief_desc(&self) -> String {
        "ExponentiationExpressionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::ExponentiationExpression(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.left.as_any());
        child_vec.push(self.right.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
