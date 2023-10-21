use crate::analysis::state::AnalysisState;
use crate::autonodes::_statement::_StatementNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::colon_block::ColonBlockNode;
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
pub enum ElseClauseBody {
    _Statement(Box<_StatementNode>),
    ColonBlock(Box<ColonBlockNode>),
    Extra(ExtraChild),
}

impl ElseClauseBody {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ElseClauseBody::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "text_interpolation" => ElseClauseBody::Extra(ExtraChild::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            ))),
            "ERROR" => {
                ElseClauseBody::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "colon_block" => {
                ElseClauseBody::ColonBlock(Box::new(ColonBlockNode::parse(node, source)?))
            }

            _ => {
                if let Some(x) = _StatementNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| ElseClauseBody::_Statement(y))
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
            "comment" => ElseClauseBody::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "text_interpolation" => ElseClauseBody::Extra(ExtraChild::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            ))),
            "ERROR" => {
                ElseClauseBody::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "colon_block" => {
                ElseClauseBody::ColonBlock(Box::new(ColonBlockNode::parse(node, source)?))
            }

            _ => {
                return Ok(
                    if let Some(x) = _StatementNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| ElseClauseBody::_Statement(y))
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
            ElseClauseBody::Extra(y) => y.kind(),
            ElseClauseBody::_Statement(y) => y.kind(),
            ElseClauseBody::ColonBlock(y) => y.kind(),
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
            ElseClauseBody::Extra(x) => x.get_utype(state, emitter),
            ElseClauseBody::_Statement(x) => x.get_utype(state, emitter),
            ElseClauseBody::ColonBlock(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ElseClauseBody::Extra(x) => x.get_php_value(state, emitter),
            ElseClauseBody::_Statement(x) => x.get_php_value(state, emitter),
            ElseClauseBody::ColonBlock(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ElseClauseBody::Extra(x) => x.read_from(state, emitter),
            ElseClauseBody::_Statement(x) => x.read_from(state, emitter),
            ElseClauseBody::ColonBlock(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ElseClauseBody {
    fn brief_desc(&self) -> String {
        match self {
            ElseClauseBody::Extra(x) => format!("ElseClauseBody::extra({})", x.brief_desc()),
            ElseClauseBody::_Statement(x) => {
                format!("ElseClauseBody::_statement({})", x.brief_desc())
            }
            ElseClauseBody::ColonBlock(x) => {
                format!("ElseClauseBody::colon_block({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ElseClauseBody::Extra(x) => x.as_any(),
            ElseClauseBody::_Statement(x) => x.as_any(),
            ElseClauseBody::ColonBlock(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ElseClauseBody::Extra(x) => x.children_any(),
            ElseClauseBody::_Statement(x) => x.children_any(),
            ElseClauseBody::ColonBlock(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ElseClauseBody::Extra(x) => x.range(),
            ElseClauseBody::_Statement(x) => x.range(),
            ElseClauseBody::ColonBlock(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ElseClauseNode {
    pub range: Range,
    pub body: Box<ElseClauseBody>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl ElseClauseNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "else_clause" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [else_clause] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let body: Box<ElseClauseBody> = node
            .children_by_field_name("body", &mut node.walk())
            .map(|chnode2| ElseClauseBody::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .expect("Field body should exist")
            .into();
        Ok(Self {
            range,
            body,
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
        "else_clause"
    }
}

impl NodeAccess for ElseClauseNode {
    fn brief_desc(&self) -> String {
        "ElseClauseNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::ElseClause(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.body.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
