use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::match_conditional_expression::MatchConditionalExpressionNode;
use crate::autonodes::match_default_expression::MatchDefaultExpressionNode;
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
pub enum MatchBlockChildren {
    MatchConditionalExpression(Box<MatchConditionalExpressionNode>),
    MatchDefaultExpression(Box<MatchDefaultExpressionNode>),
    Extra(ExtraChild),
}

impl NodeParser for MatchBlockChildren {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => MatchBlockChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => MatchBlockChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "match_conditional_expression" => MatchBlockChildren::MatchConditionalExpression(
                Box::new(MatchConditionalExpressionNode::parse(node, source)?),
            ),
            "match_default_expression" => MatchBlockChildren::MatchDefaultExpression(Box::new(
                MatchDefaultExpressionNode::parse(node, source)?,
            )),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }
}

impl MatchBlockChildren {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => MatchBlockChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => MatchBlockChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "match_conditional_expression" => MatchBlockChildren::MatchConditionalExpression(
                Box::new(MatchConditionalExpressionNode::parse(node, source)?),
            ),
            "match_default_expression" => MatchBlockChildren::MatchDefaultExpression(Box::new(
                MatchDefaultExpressionNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            MatchBlockChildren::Extra(y) => y.kind(),
            MatchBlockChildren::MatchConditionalExpression(y) => y.kind(),
            MatchBlockChildren::MatchDefaultExpression(y) => y.kind(),
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
            MatchBlockChildren::Extra(x) => x.get_utype(state, emitter),
            MatchBlockChildren::MatchConditionalExpression(x) => x.get_utype(state, emitter),
            MatchBlockChildren::MatchDefaultExpression(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            MatchBlockChildren::Extra(x) => x.get_php_value(state, emitter),
            MatchBlockChildren::MatchConditionalExpression(x) => x.get_php_value(state, emitter),
            MatchBlockChildren::MatchDefaultExpression(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            MatchBlockChildren::Extra(x) => x.read_from(state, emitter),
            MatchBlockChildren::MatchConditionalExpression(x) => x.read_from(state, emitter),
            MatchBlockChildren::MatchDefaultExpression(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for MatchBlockChildren {
    fn brief_desc(&self) -> String {
        match self {
            MatchBlockChildren::Extra(x) => {
                format!("MatchBlockChildren::extra({})", x.brief_desc())
            }
            MatchBlockChildren::MatchConditionalExpression(x) => format!(
                "MatchBlockChildren::match_conditional_expression({})",
                x.brief_desc()
            ),
            MatchBlockChildren::MatchDefaultExpression(x) => format!(
                "MatchBlockChildren::match_default_expression({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            MatchBlockChildren::Extra(x) => x.as_any(),
            MatchBlockChildren::MatchConditionalExpression(x) => x.as_any(),
            MatchBlockChildren::MatchDefaultExpression(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            MatchBlockChildren::Extra(x) => x.children_any(),
            MatchBlockChildren::MatchConditionalExpression(x) => x.children_any(),
            MatchBlockChildren::MatchDefaultExpression(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            MatchBlockChildren::Extra(x) => x.range(),
            MatchBlockChildren::MatchConditionalExpression(x) => x.range(),
            MatchBlockChildren::MatchDefaultExpression(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatchBlockNode {
    pub range: Range,
    pub children: Vec<Box<MatchBlockChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for MatchBlockNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "match_block" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [match_block] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            children: MatchBlockChildren::parse_vec(
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
}

impl MatchBlockNode {
    pub fn kind(&self) -> &'static str {
        "match_block"
    }
}

impl NodeAccess for MatchBlockNode {
    fn brief_desc(&self) -> String {
        "MatchBlockNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::MatchBlock(self)
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
