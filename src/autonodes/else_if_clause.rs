use crate::analysis::state::AnalysisState;
use crate::autonodes::_statement::_StatementNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::colon_block::ColonBlockNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::parenthesized_expression::ParenthesizedExpressionNode;
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
pub enum ElseIfClauseBody {
    _Statement(Box<_StatementNode>),
    ColonBlock(Box<ColonBlockNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl ElseIfClauseBody {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ElseIfClauseBody::Comment(Box::new(CommentNode::parse(node, source)?)),
            "text_interpolation" => ElseIfClauseBody::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => ElseIfClauseBody::Error(Box::new(ErrorNode::parse(node, source)?)),
            "colon_block" => {
                ElseIfClauseBody::ColonBlock(Box::new(ColonBlockNode::parse(node, source)?))
            }

            _ => {
                if let Some(x) = _StatementNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| ElseIfClauseBody::_Statement(y))
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
            "comment" => ElseIfClauseBody::Comment(Box::new(CommentNode::parse(node, source)?)),
            "text_interpolation" => ElseIfClauseBody::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => ElseIfClauseBody::Error(Box::new(ErrorNode::parse(node, source)?)),
            "colon_block" => {
                ElseIfClauseBody::ColonBlock(Box::new(ColonBlockNode::parse(node, source)?))
            }

            _ => {
                return Ok(
                    if let Some(x) = _StatementNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| ElseIfClauseBody::_Statement(y))
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
            ElseIfClauseBody::Comment(x) => x.get_utype(state, emitter),
            ElseIfClauseBody::TextInterpolation(x) => x.get_utype(state, emitter),
            ElseIfClauseBody::Error(x) => x.get_utype(state, emitter),
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
            ElseIfClauseBody::Comment(x) => x.get_php_value(state, emitter),
            ElseIfClauseBody::TextInterpolation(x) => x.get_php_value(state, emitter),
            ElseIfClauseBody::Error(x) => x.get_php_value(state, emitter),
            ElseIfClauseBody::_Statement(x) => x.get_php_value(state, emitter),
            ElseIfClauseBody::ColonBlock(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ElseIfClauseBody::Comment(x) => x.read_from(state, emitter),
            ElseIfClauseBody::TextInterpolation(x) => x.read_from(state, emitter),
            ElseIfClauseBody::Error(x) => x.read_from(state, emitter),
            ElseIfClauseBody::_Statement(x) => x.read_from(state, emitter),
            ElseIfClauseBody::ColonBlock(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ElseIfClauseBody {
    fn brief_desc(&self) -> String {
        match self {
            ElseIfClauseBody::Comment(x) => {
                format!("ElseIfClauseBody::comment({})", x.brief_desc())
            }
            ElseIfClauseBody::TextInterpolation(x) => {
                format!("ElseIfClauseBody::text_interpolation({})", x.brief_desc())
            }
            ElseIfClauseBody::Error(x) => format!("ElseIfClauseBody::ERROR({})", x.brief_desc()),
            ElseIfClauseBody::_Statement(x) => {
                format!("ElseIfClauseBody::_statement({})", x.brief_desc())
            }
            ElseIfClauseBody::ColonBlock(x) => {
                format!("ElseIfClauseBody::colon_block({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ElseIfClauseBody::Comment(x) => x.as_any(),
            ElseIfClauseBody::TextInterpolation(x) => x.as_any(),
            ElseIfClauseBody::Error(x) => x.as_any(),
            ElseIfClauseBody::_Statement(x) => x.as_any(),
            ElseIfClauseBody::ColonBlock(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ElseIfClauseBody::Comment(x) => x.children_any(),
            ElseIfClauseBody::TextInterpolation(x) => x.children_any(),
            ElseIfClauseBody::Error(x) => x.children_any(),
            ElseIfClauseBody::_Statement(x) => x.children_any(),
            ElseIfClauseBody::ColonBlock(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ElseIfClauseBody::Comment(x) => x.range(),
            ElseIfClauseBody::TextInterpolation(x) => x.range(),
            ElseIfClauseBody::Error(x) => x.range(),
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

impl ElseIfClauseNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "else_if_clause" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [else_if_clause] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let body: Box<ElseIfClauseBody> = node
            .children_by_field_name("body", &mut node.walk())
            .map(|chnode2| ElseIfClauseBody::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .expect("Field body should exist")
            .into();
        let condition: ParenthesizedExpressionNode = node
            .children_by_field_name("condition", &mut node.walk())
            .map(|chnode1| ParenthesizedExpressionNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field condition should exist");
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
        "else_if_clause"
    }
}

impl NodeAccess for ElseIfClauseNode {
    fn brief_desc(&self) -> String {
        "ElseIfClauseNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::ElseIfClause(self)
    }

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
