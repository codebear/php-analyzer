use crate::analysis::state::AnalysisState;
use crate::autonodes::_statement::_StatementNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::colon_block::ColonBlockNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::else_clause::ElseClauseNode;
use crate::autonodes::else_if_clause::ElseIfClauseNode;
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
pub enum IfStatementAlternative {
    ElseClause(Box<ElseClauseNode>),
    ElseIfClause(Box<ElseIfClauseNode>),
    Extra(ExtraChild),
}

impl NodeParser for IfStatementAlternative {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => IfStatementAlternative::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => IfStatementAlternative::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => IfStatementAlternative::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "else_clause" => {
                IfStatementAlternative::ElseClause(Box::new(ElseClauseNode::parse(node, source)?))
            }
            "else_if_clause" => IfStatementAlternative::ElseIfClause(Box::new(
                ElseIfClauseNode::parse(node, source)?,
            )),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!(
                        "IfStatementAlternative: Parse error, unexpected node-type: {}",
                        node.kind()
                    ),
                ))
            }
        })
    }
}

impl IfStatementAlternative {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => IfStatementAlternative::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => IfStatementAlternative::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => IfStatementAlternative::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "else_clause" => {
                IfStatementAlternative::ElseClause(Box::new(ElseClauseNode::parse(node, source)?))
            }
            "else_if_clause" => IfStatementAlternative::ElseIfClause(Box::new(
                ElseIfClauseNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            IfStatementAlternative::Extra(y) => y.kind(),
            IfStatementAlternative::ElseClause(y) => y.kind(),
            IfStatementAlternative::ElseIfClause(y) => y.kind(),
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
            IfStatementAlternative::Extra(x) => x.get_utype(state, emitter),
            IfStatementAlternative::ElseClause(x) => x.get_utype(state, emitter),
            IfStatementAlternative::ElseIfClause(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            IfStatementAlternative::Extra(x) => x.get_php_value(state, emitter),
            IfStatementAlternative::ElseClause(x) => x.get_php_value(state, emitter),
            IfStatementAlternative::ElseIfClause(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            IfStatementAlternative::Extra(x) => x.read_from(state, emitter),
            IfStatementAlternative::ElseClause(x) => x.read_from(state, emitter),
            IfStatementAlternative::ElseIfClause(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for IfStatementAlternative {
    fn brief_desc(&self) -> String {
        match self {
            IfStatementAlternative::Extra(x) => {
                format!("IfStatementAlternative::extra({})", x.brief_desc())
            }
            IfStatementAlternative::ElseClause(x) => {
                format!("IfStatementAlternative::else_clause({})", x.brief_desc())
            }
            IfStatementAlternative::ElseIfClause(x) => {
                format!("IfStatementAlternative::else_if_clause({})", x.brief_desc())
            }
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            IfStatementAlternative::Extra(x) => x.as_any(),
            IfStatementAlternative::ElseClause(x) => x.as_any(),
            IfStatementAlternative::ElseIfClause(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            IfStatementAlternative::Extra(x) => x.children_any(),
            IfStatementAlternative::ElseClause(x) => x.children_any(),
            IfStatementAlternative::ElseIfClause(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            IfStatementAlternative::Extra(x) => x.range(),
            IfStatementAlternative::ElseClause(x) => x.range(),
            IfStatementAlternative::ElseIfClause(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IfStatementBody {
    _Statement(Box<_StatementNode>),
    ColonBlock(Box<ColonBlockNode>),
    Extra(ExtraChild),
}

impl NodeParser for IfStatementBody {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => IfStatementBody::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "text_interpolation" => IfStatementBody::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => {
                IfStatementBody::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "colon_block" => {
                IfStatementBody::ColonBlock(Box::new(ColonBlockNode::parse(node, source)?))
            }

            _ => {
                if let Some(x) = _StatementNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(IfStatementBody::_Statement)
                {
                    x
                } else {
                    return Err(ParseError::new(
                        node.range(),
                        format!(
                            "IfStatementBody: Parse error, unexpected node-type: {}",
                            node.kind()
                        ),
                    ));
                }
            }
        })
    }
}

impl IfStatementBody {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => IfStatementBody::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "text_interpolation" => IfStatementBody::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => {
                IfStatementBody::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "colon_block" => {
                IfStatementBody::ColonBlock(Box::new(ColonBlockNode::parse(node, source)?))
            }

            _ => {
                return Ok(_StatementNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(IfStatementBody::_Statement))
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            IfStatementBody::Extra(y) => y.kind(),
            IfStatementBody::_Statement(y) => y.kind(),
            IfStatementBody::ColonBlock(y) => y.kind(),
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
            IfStatementBody::Extra(x) => x.get_utype(state, emitter),
            IfStatementBody::_Statement(x) => x.get_utype(state, emitter),
            IfStatementBody::ColonBlock(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            IfStatementBody::Extra(x) => x.get_php_value(state, emitter),
            IfStatementBody::_Statement(x) => x.get_php_value(state, emitter),
            IfStatementBody::ColonBlock(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            IfStatementBody::Extra(x) => x.read_from(state, emitter),
            IfStatementBody::_Statement(x) => x.read_from(state, emitter),
            IfStatementBody::ColonBlock(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for IfStatementBody {
    fn brief_desc(&self) -> String {
        match self {
            IfStatementBody::Extra(x) => format!("IfStatementBody::extra({})", x.brief_desc()),
            IfStatementBody::_Statement(x) => {
                format!("IfStatementBody::_statement({})", x.brief_desc())
            }
            IfStatementBody::ColonBlock(x) => {
                format!("IfStatementBody::colon_block({})", x.brief_desc())
            }
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            IfStatementBody::Extra(x) => x.as_any(),
            IfStatementBody::_Statement(x) => x.as_any(),
            IfStatementBody::ColonBlock(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            IfStatementBody::Extra(x) => x.children_any(),
            IfStatementBody::_Statement(x) => x.children_any(),
            IfStatementBody::ColonBlock(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            IfStatementBody::Extra(x) => x.range(),
            IfStatementBody::_Statement(x) => x.range(),
            IfStatementBody::ColonBlock(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IfStatementNode {
    pub range: Range,
    pub alternative: Option<Vec<Box<IfStatementAlternative>>>,
    pub body: Box<IfStatementBody>,
    pub condition: ParenthesizedExpressionNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for IfStatementNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "if_statement" {
            return Err(ParseError::new(range, format!("IfStatementNode: Node is of the wrong kind [{}] vs expected [if_statement] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let alternative: Option<Vec<Box<IfStatementAlternative>>> =
            Into::<Result<_, _>>::into(node.parse_child("alternative", source))?;
        let body: Box<IfStatementBody> =
            Into::<Result<_, _>>::into(node.parse_child("body", source))?;
        let condition: ParenthesizedExpressionNode =
            Into::<Result<_, _>>::into(node.parse_child("condition", source))?;
        Ok(Self {
            range,
            alternative,
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

impl IfStatementNode {
    pub fn kind(&self) -> &'static str {
        "if_statement"
    }
}

impl NodeAccess for IfStatementNode {
    fn brief_desc(&self) -> String {
        "IfStatementNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::IfStatement(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.alternative {
            child_vec.extend(x.iter().map(|z| z.as_any()));
        }
        child_vec.push(self.body.as_any());
        child_vec.push(self.condition.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
