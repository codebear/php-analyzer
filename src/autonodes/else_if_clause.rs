use crate::analysis::state::AnalysisState;
use crate::autonodes::_statement::_StatementNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::colon_block::ColonBlockNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::parenthesized_expression::ParenthesizedExpressionNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autotree::ChildNodeParser;
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
pub enum ElseIfClauseBody {
    _Statement(Box<_StatementNode>),
    ColonBlock(Box<ColonBlockNode>),
    Extra(ExtraChild),
}

impl NodeParser for ElseIfClauseBody {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ElseIfClauseBody::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ElseIfClauseBody::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => ElseIfClauseBody::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "colon_block" => {
                ElseIfClauseBody::ColonBlock(Box::new(ColonBlockNode::parse(node, source)?))
            }

            _ => {
                if let Some(x) = _StatementNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(ElseIfClauseBody::_Statement)
                {
                    x
                } else {
                    return Err(ParseError::new(
                        node.range(),
                        format!(
                            "ElseIfClauseBody: Parse error, unexpected node-type: {}",
                            node.kind()
                        ),
                    ));
                }
            }
        })
    }
}

impl ElseIfClauseBody {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ElseIfClauseBody::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ElseIfClauseBody::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => ElseIfClauseBody::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "colon_block" => {
                ElseIfClauseBody::ColonBlock(Box::new(ColonBlockNode::parse(node, source)?))
            }

            _ => {
                return Ok(_StatementNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(ElseIfClauseBody::_Statement))
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            ElseIfClauseBody::Extra(y) => y.kind(),
            ElseIfClauseBody::_Statement(y) => y.kind(),
            ElseIfClauseBody::ColonBlock(y) => y.kind(),
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
    ) -> Option<UnionType> {
        match self {
            ElseIfClauseBody::Extra(x) => x.get_utype(state, emitter),
            ElseIfClauseBody::_Statement(x) => x.get_utype(state, emitter),
            ElseIfClauseBody::ColonBlock(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ElseIfClauseBody::Extra(x) => x.get_php_value(state, emitter),
            ElseIfClauseBody::_Statement(x) => x.get_php_value(state, emitter),
            ElseIfClauseBody::ColonBlock(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ElseIfClauseBody::Extra(x) => x.read_from(state, emitter),
            ElseIfClauseBody::_Statement(x) => x.read_from(state, emitter),
            ElseIfClauseBody::ColonBlock(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ElseIfClauseBody {
    fn brief_desc(&self) -> String {
        match self {
            ElseIfClauseBody::Extra(x) => format!("ElseIfClauseBody::extra({})", x.brief_desc()),
            ElseIfClauseBody::_Statement(x) => {
                format!("ElseIfClauseBody::_statement({})", x.brief_desc())
            }
            ElseIfClauseBody::ColonBlock(x) => {
                format!("ElseIfClauseBody::colon_block({})", x.brief_desc())
            }
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            ElseIfClauseBody::Extra(x) => x.as_any(),
            ElseIfClauseBody::_Statement(x) => x.as_any(),
            ElseIfClauseBody::ColonBlock(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            ElseIfClauseBody::Extra(x) => x.children_any(),
            ElseIfClauseBody::_Statement(x) => x.children_any(),
            ElseIfClauseBody::ColonBlock(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ElseIfClauseBody::Extra(x) => x.range(),
            ElseIfClauseBody::_Statement(x) => x.range(),
            ElseIfClauseBody::ColonBlock(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ElseIfClauseNode {
    pub range: Range,
    pub body: Box<ElseIfClauseBody>,
    pub condition: ParenthesizedExpressionNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for ElseIfClauseNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "else_if_clause" {
            return Err(ParseError::new(range, format!("ElseIfClauseNode: Node is of the wrong kind [{}] vs expected [else_if_clause] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let body: Box<ElseIfClauseBody> =
            Into::<Result<_, _>>::into(node.parse_child("body", source))?;
        let condition: ParenthesizedExpressionNode =
            Into::<Result<_, _>>::into(node.parse_child("condition", source))?;
        Ok(Self {
            range,
            body,
            condition,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl ElseIfClauseNode {
    pub fn kind(&self) -> &'static str {
        "else_if_clause"
    }
}

impl NodeAccess for ElseIfClauseNode {
    fn brief_desc(&self) -> String {
        "ElseIfClauseNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::ElseIfClause(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.body.as_any());
        child_vec.push(self.condition.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
