use crate::analysis::state::AnalysisState;
use crate::autonodes::_statement::_StatementNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::colon_block::ColonBlockNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::parenthesized_expression::ParenthesizedExpressionNode;
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
pub enum WhileStatementBody {
    _Statement(Box<_StatementNode>),
    ColonBlock(Box<ColonBlockNode>),
    Extra(ExtraChild),
}

impl NodeParser for WhileStatementBody {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => WhileStatementBody::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => WhileStatementBody::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "colon_block" => {
                WhileStatementBody::ColonBlock(Box::new(ColonBlockNode::parse(node, source)?))
            }

            _ => {
                if let Some(x) = _StatementNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(WhileStatementBody::_Statement)
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

impl WhileStatementBody {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => WhileStatementBody::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => WhileStatementBody::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "colon_block" => {
                WhileStatementBody::ColonBlock(Box::new(ColonBlockNode::parse(node, source)?))
            }

            _ => {
                return Ok(_StatementNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(WhileStatementBody::_Statement))
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            WhileStatementBody::Extra(y) => y.kind(),
            WhileStatementBody::_Statement(y) => y.kind(),
            WhileStatementBody::ColonBlock(y) => y.kind(),
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
            WhileStatementBody::Extra(x) => x.get_utype(state, emitter),
            WhileStatementBody::_Statement(x) => x.get_utype(state, emitter),
            WhileStatementBody::ColonBlock(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            WhileStatementBody::Extra(x) => x.get_php_value(state, emitter),
            WhileStatementBody::_Statement(x) => x.get_php_value(state, emitter),
            WhileStatementBody::ColonBlock(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            WhileStatementBody::Extra(x) => x.read_from(state, emitter),
            WhileStatementBody::_Statement(x) => x.read_from(state, emitter),
            WhileStatementBody::ColonBlock(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for WhileStatementBody {
    fn brief_desc(&self) -> String {
        match self {
            WhileStatementBody::Extra(x) => {
                format!("WhileStatementBody::extra({})", x.brief_desc())
            }
            WhileStatementBody::_Statement(x) => {
                format!("WhileStatementBody::_statement({})", x.brief_desc())
            }
            WhileStatementBody::ColonBlock(x) => {
                format!("WhileStatementBody::colon_block({})", x.brief_desc())
            }
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            WhileStatementBody::Extra(x) => x.as_any(),
            WhileStatementBody::_Statement(x) => x.as_any(),
            WhileStatementBody::ColonBlock(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            WhileStatementBody::Extra(x) => x.children_any(),
            WhileStatementBody::_Statement(x) => x.children_any(),
            WhileStatementBody::ColonBlock(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            WhileStatementBody::Extra(x) => x.range(),
            WhileStatementBody::_Statement(x) => x.range(),
            WhileStatementBody::ColonBlock(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WhileStatementNode {
    pub range: Range,
    pub body: Box<WhileStatementBody>,
    pub condition: ParenthesizedExpressionNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for WhileStatementNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "while_statement" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [while_statement] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let body: Box<WhileStatementBody> =
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

impl WhileStatementNode {
    pub fn kind(&self) -> &'static str {
        "while_statement"
    }
}

impl NodeAccess for WhileStatementNode {
    fn brief_desc(&self) -> String {
        "WhileStatementNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::WhileStatement(self)
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
