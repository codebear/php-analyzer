use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::sequence_expression::SequenceExpressionNode;
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
pub enum EchoStatementChildren {
    _Expression(Box<_ExpressionNode>),
    SequenceExpression(Box<SequenceExpressionNode>),
    Extra(ExtraChild),
}

impl NodeParser for EchoStatementChildren {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => EchoStatementChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => EchoStatementChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "sequence_expression" => EchoStatementChildren::SequenceExpression(Box::new(
                SequenceExpressionNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(EchoStatementChildren::_Expression)
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

impl EchoStatementChildren {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => EchoStatementChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => EchoStatementChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "sequence_expression" => EchoStatementChildren::SequenceExpression(Box::new(
                SequenceExpressionNode::parse(node, source)?,
            )),

            _ => {
                return Ok(
                    _ExpressionNode::parse_opt(node, source)?
                        .map(Box::new)
                        .map(EchoStatementChildren::_Expression),
                )
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            EchoStatementChildren::Extra(y) => y.kind(),
            EchoStatementChildren::_Expression(y) => y.kind(),
            EchoStatementChildren::SequenceExpression(y) => y.kind(),
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
            EchoStatementChildren::Extra(x) => x.get_utype(state, emitter),
            EchoStatementChildren::_Expression(x) => x.get_utype(state, emitter),
            EchoStatementChildren::SequenceExpression(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            EchoStatementChildren::Extra(x) => x.get_php_value(state, emitter),
            EchoStatementChildren::_Expression(x) => x.get_php_value(state, emitter),
            EchoStatementChildren::SequenceExpression(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            EchoStatementChildren::Extra(x) => x.read_from(state, emitter),
            EchoStatementChildren::_Expression(x) => x.read_from(state, emitter),
            EchoStatementChildren::SequenceExpression(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for EchoStatementChildren {
    fn brief_desc(&self) -> String {
        match self {
            EchoStatementChildren::Extra(x) => {
                format!("EchoStatementChildren::extra({})", x.brief_desc())
            }
            EchoStatementChildren::_Expression(x) => {
                format!("EchoStatementChildren::_expression({})", x.brief_desc())
            }
            EchoStatementChildren::SequenceExpression(x) => format!(
                "EchoStatementChildren::sequence_expression({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            EchoStatementChildren::Extra(x) => x.as_any(),
            EchoStatementChildren::_Expression(x) => x.as_any(),
            EchoStatementChildren::SequenceExpression(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            EchoStatementChildren::Extra(x) => x.children_any(),
            EchoStatementChildren::_Expression(x) => x.children_any(),
            EchoStatementChildren::SequenceExpression(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            EchoStatementChildren::Extra(x) => x.range(),
            EchoStatementChildren::_Expression(x) => x.range(),
            EchoStatementChildren::SequenceExpression(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EchoStatementNode {
    pub range: Range,
    pub child: Box<EchoStatementChildren>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for EchoStatementNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "echo_statement" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [echo_statement] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            child: node
                .named_children(&mut node.walk())
                .filter(|node| node.kind() != "comment")
                .map(|k| EchoStatementChildren::parse(k, source))
                .collect::<Result<Vec<EchoStatementChildren>, ParseError>>()?
                .drain(..)
                .map(Box::new)
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

impl EchoStatementNode {
    pub fn kind(&self) -> &'static str {
        "echo_statement"
    }
}

impl NodeAccess for EchoStatementNode {
    fn brief_desc(&self) -> String {
        "EchoStatementNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::EchoStatement(self)
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
