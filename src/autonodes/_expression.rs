use crate::analysis::state::AnalysisState;
use crate::autonodes::_primary_expression::_PrimaryExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::assignment_expression::AssignmentExpressionNode;
use crate::autonodes::augmented_assignment_expression::AugmentedAssignmentExpressionNode;
use crate::autonodes::binary_expression::BinaryExpressionNode;
use crate::autonodes::cast_expression::CastExpressionNode;
use crate::autonodes::clone_expression::CloneExpressionNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::conditional_expression::ConditionalExpressionNode;
use crate::autonodes::error_suppression_expression::ErrorSuppressionExpressionNode;
use crate::autonodes::include_expression::IncludeExpressionNode;
use crate::autonodes::include_once_expression::IncludeOnceExpressionNode;
use crate::autonodes::match_expression::MatchExpressionNode;
use crate::autonodes::reference_assignment_expression::ReferenceAssignmentExpressionNode;
use crate::autonodes::require_expression::RequireExpressionNode;
use crate::autonodes::require_once_expression::RequireOnceExpressionNode;
use crate::autonodes::unary_op_expression::UnaryOpExpressionNode;
use crate::autonodes::yield_expression::YieldExpressionNode;
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
pub enum _ExpressionNode {
    _PrimaryExpression(Box<_PrimaryExpressionNode>),
    AssignmentExpression(Box<AssignmentExpressionNode>),
    AugmentedAssignmentExpression(Box<AugmentedAssignmentExpressionNode>),
    BinaryExpression(Box<BinaryExpressionNode>),
    CastExpression(Box<CastExpressionNode>),
    CloneExpression(Box<CloneExpressionNode>),
    ConditionalExpression(Box<ConditionalExpressionNode>),
    ErrorSuppressionExpression(Box<ErrorSuppressionExpressionNode>),
    IncludeExpression(Box<IncludeExpressionNode>),
    IncludeOnceExpression(Box<IncludeOnceExpressionNode>),
    MatchExpression(Box<MatchExpressionNode>),
    ReferenceAssignmentExpression(Box<ReferenceAssignmentExpressionNode>),
    RequireExpression(Box<RequireExpressionNode>),
    RequireOnceExpression(Box<RequireOnceExpressionNode>),
    UnaryOpExpression(Box<UnaryOpExpressionNode>),
    YieldExpression(Box<YieldExpressionNode>),
    Extra(ExtraChild),
}

impl NodeParser for _ExpressionNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => _ExpressionNode::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "ERROR" => {
                _ExpressionNode::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "assignment_expression" => _ExpressionNode::AssignmentExpression(Box::new(
                AssignmentExpressionNode::parse(node, source)?,
            )),
            "augmented_assignment_expression" => _ExpressionNode::AugmentedAssignmentExpression(
                Box::new(AugmentedAssignmentExpressionNode::parse(node, source)?),
            ),
            "binary_expression" => _ExpressionNode::BinaryExpression(Box::new(
                BinaryExpressionNode::parse(node, source)?,
            )),
            "cast_expression" => {
                _ExpressionNode::CastExpression(Box::new(CastExpressionNode::parse(node, source)?))
            }
            "clone_expression" => _ExpressionNode::CloneExpression(Box::new(
                CloneExpressionNode::parse(node, source)?,
            )),
            "conditional_expression" => _ExpressionNode::ConditionalExpression(Box::new(
                ConditionalExpressionNode::parse(node, source)?,
            )),
            "error_suppression_expression" => _ExpressionNode::ErrorSuppressionExpression(
                Box::new(ErrorSuppressionExpressionNode::parse(node, source)?),
            ),
            "include_expression" => _ExpressionNode::IncludeExpression(Box::new(
                IncludeExpressionNode::parse(node, source)?,
            )),
            "include_once_expression" => _ExpressionNode::IncludeOnceExpression(Box::new(
                IncludeOnceExpressionNode::parse(node, source)?,
            )),
            "match_expression" => _ExpressionNode::MatchExpression(Box::new(
                MatchExpressionNode::parse(node, source)?,
            )),
            "reference_assignment_expression" => _ExpressionNode::ReferenceAssignmentExpression(
                Box::new(ReferenceAssignmentExpressionNode::parse(node, source)?),
            ),
            "require_expression" => _ExpressionNode::RequireExpression(Box::new(
                RequireExpressionNode::parse(node, source)?,
            )),
            "require_once_expression" => _ExpressionNode::RequireOnceExpression(Box::new(
                RequireOnceExpressionNode::parse(node, source)?,
            )),
            "unary_op_expression" => _ExpressionNode::UnaryOpExpression(Box::new(
                UnaryOpExpressionNode::parse(node, source)?,
            )),
            "yield_expression" => _ExpressionNode::YieldExpression(Box::new(
                YieldExpressionNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _PrimaryExpressionNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| _ExpressionNode::_PrimaryExpression(y))
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

impl _ExpressionNode {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => _ExpressionNode::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "ERROR" => {
                _ExpressionNode::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "assignment_expression" => _ExpressionNode::AssignmentExpression(Box::new(
                AssignmentExpressionNode::parse(node, source)?,
            )),
            "augmented_assignment_expression" => _ExpressionNode::AugmentedAssignmentExpression(
                Box::new(AugmentedAssignmentExpressionNode::parse(node, source)?),
            ),
            "binary_expression" => _ExpressionNode::BinaryExpression(Box::new(
                BinaryExpressionNode::parse(node, source)?,
            )),
            "cast_expression" => {
                _ExpressionNode::CastExpression(Box::new(CastExpressionNode::parse(node, source)?))
            }
            "clone_expression" => _ExpressionNode::CloneExpression(Box::new(
                CloneExpressionNode::parse(node, source)?,
            )),
            "conditional_expression" => _ExpressionNode::ConditionalExpression(Box::new(
                ConditionalExpressionNode::parse(node, source)?,
            )),
            "error_suppression_expression" => _ExpressionNode::ErrorSuppressionExpression(
                Box::new(ErrorSuppressionExpressionNode::parse(node, source)?),
            ),
            "include_expression" => _ExpressionNode::IncludeExpression(Box::new(
                IncludeExpressionNode::parse(node, source)?,
            )),
            "include_once_expression" => _ExpressionNode::IncludeOnceExpression(Box::new(
                IncludeOnceExpressionNode::parse(node, source)?,
            )),
            "match_expression" => _ExpressionNode::MatchExpression(Box::new(
                MatchExpressionNode::parse(node, source)?,
            )),
            "reference_assignment_expression" => _ExpressionNode::ReferenceAssignmentExpression(
                Box::new(ReferenceAssignmentExpressionNode::parse(node, source)?),
            ),
            "require_expression" => _ExpressionNode::RequireExpression(Box::new(
                RequireExpressionNode::parse(node, source)?,
            )),
            "require_once_expression" => _ExpressionNode::RequireOnceExpression(Box::new(
                RequireOnceExpressionNode::parse(node, source)?,
            )),
            "unary_op_expression" => _ExpressionNode::UnaryOpExpression(Box::new(
                UnaryOpExpressionNode::parse(node, source)?,
            )),
            "yield_expression" => _ExpressionNode::YieldExpression(Box::new(
                YieldExpressionNode::parse(node, source)?,
            )),

            _ => {
                return Ok(
                    if let Some(x) = _PrimaryExpressionNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| _ExpressionNode::_PrimaryExpression(y))
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
            _ExpressionNode::Extra(y) => y.kind(),
            _ExpressionNode::_PrimaryExpression(y) => y.kind(),
            _ExpressionNode::AssignmentExpression(y) => y.kind(),
            _ExpressionNode::AugmentedAssignmentExpression(y) => y.kind(),
            _ExpressionNode::BinaryExpression(y) => y.kind(),
            _ExpressionNode::CastExpression(y) => y.kind(),
            _ExpressionNode::CloneExpression(y) => y.kind(),
            _ExpressionNode::ConditionalExpression(y) => y.kind(),
            _ExpressionNode::ErrorSuppressionExpression(y) => y.kind(),
            _ExpressionNode::IncludeExpression(y) => y.kind(),
            _ExpressionNode::IncludeOnceExpression(y) => y.kind(),
            _ExpressionNode::MatchExpression(y) => y.kind(),
            _ExpressionNode::ReferenceAssignmentExpression(y) => y.kind(),
            _ExpressionNode::RequireExpression(y) => y.kind(),
            _ExpressionNode::RequireOnceExpression(y) => y.kind(),
            _ExpressionNode::UnaryOpExpression(y) => y.kind(),
            _ExpressionNode::YieldExpression(y) => y.kind(),
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
            _ExpressionNode::Extra(x) => x.get_utype(state, emitter),
            _ExpressionNode::_PrimaryExpression(x) => x.get_utype(state, emitter),
            _ExpressionNode::AssignmentExpression(x) => x.get_utype(state, emitter),
            _ExpressionNode::AugmentedAssignmentExpression(x) => x.get_utype(state, emitter),
            _ExpressionNode::BinaryExpression(x) => x.get_utype(state, emitter),
            _ExpressionNode::CastExpression(x) => x.get_utype(state, emitter),
            _ExpressionNode::CloneExpression(x) => x.get_utype(state, emitter),
            _ExpressionNode::ConditionalExpression(x) => x.get_utype(state, emitter),
            _ExpressionNode::ErrorSuppressionExpression(x) => x.get_utype(state, emitter),
            _ExpressionNode::IncludeExpression(x) => x.get_utype(state, emitter),
            _ExpressionNode::IncludeOnceExpression(x) => x.get_utype(state, emitter),
            _ExpressionNode::MatchExpression(x) => x.get_utype(state, emitter),
            _ExpressionNode::ReferenceAssignmentExpression(x) => x.get_utype(state, emitter),
            _ExpressionNode::RequireExpression(x) => x.get_utype(state, emitter),
            _ExpressionNode::RequireOnceExpression(x) => x.get_utype(state, emitter),
            _ExpressionNode::UnaryOpExpression(x) => x.get_utype(state, emitter),
            _ExpressionNode::YieldExpression(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            _ExpressionNode::Extra(x) => x.get_php_value(state, emitter),
            _ExpressionNode::_PrimaryExpression(x) => x.get_php_value(state, emitter),
            _ExpressionNode::AssignmentExpression(x) => x.get_php_value(state, emitter),
            _ExpressionNode::AugmentedAssignmentExpression(x) => x.get_php_value(state, emitter),
            _ExpressionNode::BinaryExpression(x) => x.get_php_value(state, emitter),
            _ExpressionNode::CastExpression(x) => x.get_php_value(state, emitter),
            _ExpressionNode::CloneExpression(x) => x.get_php_value(state, emitter),
            _ExpressionNode::ConditionalExpression(x) => x.get_php_value(state, emitter),
            _ExpressionNode::ErrorSuppressionExpression(x) => x.get_php_value(state, emitter),
            _ExpressionNode::IncludeExpression(x) => x.get_php_value(state, emitter),
            _ExpressionNode::IncludeOnceExpression(x) => x.get_php_value(state, emitter),
            _ExpressionNode::MatchExpression(x) => x.get_php_value(state, emitter),
            _ExpressionNode::ReferenceAssignmentExpression(x) => x.get_php_value(state, emitter),
            _ExpressionNode::RequireExpression(x) => x.get_php_value(state, emitter),
            _ExpressionNode::RequireOnceExpression(x) => x.get_php_value(state, emitter),
            _ExpressionNode::UnaryOpExpression(x) => x.get_php_value(state, emitter),
            _ExpressionNode::YieldExpression(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            _ExpressionNode::Extra(x) => x.read_from(state, emitter),
            _ExpressionNode::_PrimaryExpression(x) => x.read_from(state, emitter),
            _ExpressionNode::AssignmentExpression(x) => x.read_from(state, emitter),
            _ExpressionNode::AugmentedAssignmentExpression(x) => x.read_from(state, emitter),
            _ExpressionNode::BinaryExpression(x) => x.read_from(state, emitter),
            _ExpressionNode::CastExpression(x) => x.read_from(state, emitter),
            _ExpressionNode::CloneExpression(x) => x.read_from(state, emitter),
            _ExpressionNode::ConditionalExpression(x) => x.read_from(state, emitter),
            _ExpressionNode::ErrorSuppressionExpression(x) => x.read_from(state, emitter),
            _ExpressionNode::IncludeExpression(x) => x.read_from(state, emitter),
            _ExpressionNode::IncludeOnceExpression(x) => x.read_from(state, emitter),
            _ExpressionNode::MatchExpression(x) => x.read_from(state, emitter),
            _ExpressionNode::ReferenceAssignmentExpression(x) => x.read_from(state, emitter),
            _ExpressionNode::RequireExpression(x) => x.read_from(state, emitter),
            _ExpressionNode::RequireOnceExpression(x) => x.read_from(state, emitter),
            _ExpressionNode::UnaryOpExpression(x) => x.read_from(state, emitter),
            _ExpressionNode::YieldExpression(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for _ExpressionNode {
    fn brief_desc(&self) -> String {
        match self {
            _ExpressionNode::Extra(x) => format!("_ExpressionNode::extra({})", x.brief_desc()),
            _ExpressionNode::_PrimaryExpression(x) => {
                format!("_ExpressionNode::_primary_expression({})", x.brief_desc())
            }
            _ExpressionNode::AssignmentExpression(x) => {
                format!("_ExpressionNode::assignment_expression({})", x.brief_desc())
            }
            _ExpressionNode::AugmentedAssignmentExpression(x) => format!(
                "_ExpressionNode::augmented_assignment_expression({})",
                x.brief_desc()
            ),
            _ExpressionNode::BinaryExpression(x) => {
                format!("_ExpressionNode::binary_expression({})", x.brief_desc())
            }
            _ExpressionNode::CastExpression(x) => {
                format!("_ExpressionNode::cast_expression({})", x.brief_desc())
            }
            _ExpressionNode::CloneExpression(x) => {
                format!("_ExpressionNode::clone_expression({})", x.brief_desc())
            }
            _ExpressionNode::ConditionalExpression(x) => format!(
                "_ExpressionNode::conditional_expression({})",
                x.brief_desc()
            ),
            _ExpressionNode::ErrorSuppressionExpression(x) => format!(
                "_ExpressionNode::error_suppression_expression({})",
                x.brief_desc()
            ),
            _ExpressionNode::IncludeExpression(x) => {
                format!("_ExpressionNode::include_expression({})", x.brief_desc())
            }
            _ExpressionNode::IncludeOnceExpression(x) => format!(
                "_ExpressionNode::include_once_expression({})",
                x.brief_desc()
            ),
            _ExpressionNode::MatchExpression(x) => {
                format!("_ExpressionNode::match_expression({})", x.brief_desc())
            }
            _ExpressionNode::ReferenceAssignmentExpression(x) => format!(
                "_ExpressionNode::reference_assignment_expression({})",
                x.brief_desc()
            ),
            _ExpressionNode::RequireExpression(x) => {
                format!("_ExpressionNode::require_expression({})", x.brief_desc())
            }
            _ExpressionNode::RequireOnceExpression(x) => format!(
                "_ExpressionNode::require_once_expression({})",
                x.brief_desc()
            ),
            _ExpressionNode::UnaryOpExpression(x) => {
                format!("_ExpressionNode::unary_op_expression({})", x.brief_desc())
            }
            _ExpressionNode::YieldExpression(x) => {
                format!("_ExpressionNode::yield_expression({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            _ExpressionNode::Extra(x) => x.as_any(),
            _ExpressionNode::_PrimaryExpression(x) => x.as_any(),
            _ExpressionNode::AssignmentExpression(x) => x.as_any(),
            _ExpressionNode::AugmentedAssignmentExpression(x) => x.as_any(),
            _ExpressionNode::BinaryExpression(x) => x.as_any(),
            _ExpressionNode::CastExpression(x) => x.as_any(),
            _ExpressionNode::CloneExpression(x) => x.as_any(),
            _ExpressionNode::ConditionalExpression(x) => x.as_any(),
            _ExpressionNode::ErrorSuppressionExpression(x) => x.as_any(),
            _ExpressionNode::IncludeExpression(x) => x.as_any(),
            _ExpressionNode::IncludeOnceExpression(x) => x.as_any(),
            _ExpressionNode::MatchExpression(x) => x.as_any(),
            _ExpressionNode::ReferenceAssignmentExpression(x) => x.as_any(),
            _ExpressionNode::RequireExpression(x) => x.as_any(),
            _ExpressionNode::RequireOnceExpression(x) => x.as_any(),
            _ExpressionNode::UnaryOpExpression(x) => x.as_any(),
            _ExpressionNode::YieldExpression(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            _ExpressionNode::Extra(x) => x.children_any(),
            _ExpressionNode::_PrimaryExpression(x) => x.children_any(),
            _ExpressionNode::AssignmentExpression(x) => x.children_any(),
            _ExpressionNode::AugmentedAssignmentExpression(x) => x.children_any(),
            _ExpressionNode::BinaryExpression(x) => x.children_any(),
            _ExpressionNode::CastExpression(x) => x.children_any(),
            _ExpressionNode::CloneExpression(x) => x.children_any(),
            _ExpressionNode::ConditionalExpression(x) => x.children_any(),
            _ExpressionNode::ErrorSuppressionExpression(x) => x.children_any(),
            _ExpressionNode::IncludeExpression(x) => x.children_any(),
            _ExpressionNode::IncludeOnceExpression(x) => x.children_any(),
            _ExpressionNode::MatchExpression(x) => x.children_any(),
            _ExpressionNode::ReferenceAssignmentExpression(x) => x.children_any(),
            _ExpressionNode::RequireExpression(x) => x.children_any(),
            _ExpressionNode::RequireOnceExpression(x) => x.children_any(),
            _ExpressionNode::UnaryOpExpression(x) => x.children_any(),
            _ExpressionNode::YieldExpression(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            _ExpressionNode::Extra(x) => x.range(),
            _ExpressionNode::_PrimaryExpression(x) => x.range(),
            _ExpressionNode::AssignmentExpression(x) => x.range(),
            _ExpressionNode::AugmentedAssignmentExpression(x) => x.range(),
            _ExpressionNode::BinaryExpression(x) => x.range(),
            _ExpressionNode::CastExpression(x) => x.range(),
            _ExpressionNode::CloneExpression(x) => x.range(),
            _ExpressionNode::ConditionalExpression(x) => x.range(),
            _ExpressionNode::ErrorSuppressionExpression(x) => x.range(),
            _ExpressionNode::IncludeExpression(x) => x.range(),
            _ExpressionNode::IncludeOnceExpression(x) => x.range(),
            _ExpressionNode::MatchExpression(x) => x.range(),
            _ExpressionNode::ReferenceAssignmentExpression(x) => x.range(),
            _ExpressionNode::RequireExpression(x) => x.range(),
            _ExpressionNode::RequireOnceExpression(x) => x.range(),
            _ExpressionNode::UnaryOpExpression(x) => x.range(),
            _ExpressionNode::YieldExpression(x) => x.range(),
        }
    }
}
