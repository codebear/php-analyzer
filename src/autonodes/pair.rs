use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::by_ref::ByRefNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::list_literal::ListLiteralNode;
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
pub enum PairValue {
    _Expression(Box<_ExpressionNode>),
    ByRef(Box<ByRefNode>),
    ListLiteral(Box<ListLiteralNode>),
    Extra(ExtraChild),
}

impl NodeParser for PairValue {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => PairValue::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "ERROR" => {
                PairValue::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "by_ref" => PairValue::ByRef(Box::new(ByRefNode::parse(node, source)?)),
            "list_literal" => {
                PairValue::ListLiteral(Box::new(ListLiteralNode::parse(node, source)?))
            }

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| PairValue::_Expression(y))
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

impl PairValue {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => PairValue::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "ERROR" => {
                PairValue::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "by_ref" => PairValue::ByRef(Box::new(ByRefNode::parse(node, source)?)),
            "list_literal" => {
                PairValue::ListLiteral(Box::new(ListLiteralNode::parse(node, source)?))
            }

            _ => {
                return Ok(
                    if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| PairValue::_Expression(y))
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
            PairValue::Extra(y) => y.kind(),
            PairValue::_Expression(y) => y.kind(),
            PairValue::ByRef(y) => y.kind(),
            PairValue::ListLiteral(y) => y.kind(),
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
            PairValue::Extra(x) => x.get_utype(state, emitter),
            PairValue::_Expression(x) => x.get_utype(state, emitter),
            PairValue::ByRef(x) => x.get_utype(state, emitter),
            PairValue::ListLiteral(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            PairValue::Extra(x) => x.get_php_value(state, emitter),
            PairValue::_Expression(x) => x.get_php_value(state, emitter),
            PairValue::ByRef(x) => x.get_php_value(state, emitter),
            PairValue::ListLiteral(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            PairValue::Extra(x) => x.read_from(state, emitter),
            PairValue::_Expression(x) => x.read_from(state, emitter),
            PairValue::ByRef(x) => x.read_from(state, emitter),
            PairValue::ListLiteral(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for PairValue {
    fn brief_desc(&self) -> String {
        match self {
            PairValue::Extra(x) => format!("PairValue::extra({})", x.brief_desc()),
            PairValue::_Expression(x) => format!("PairValue::_expression({})", x.brief_desc()),
            PairValue::ByRef(x) => format!("PairValue::by_ref({})", x.brief_desc()),
            PairValue::ListLiteral(x) => format!("PairValue::list_literal({})", x.brief_desc()),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            PairValue::Extra(x) => x.as_any(),
            PairValue::_Expression(x) => x.as_any(),
            PairValue::ByRef(x) => x.as_any(),
            PairValue::ListLiteral(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            PairValue::Extra(x) => x.children_any(),
            PairValue::_Expression(x) => x.children_any(),
            PairValue::ByRef(x) => x.children_any(),
            PairValue::ListLiteral(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            PairValue::Extra(x) => x.range(),
            PairValue::_Expression(x) => x.range(),
            PairValue::ByRef(x) => x.range(),
            PairValue::ListLiteral(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PairNode {
    pub range: Range,
    pub key: _ExpressionNode,
    pub value: Box<PairValue>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for PairNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "pair" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [pair] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let key: _ExpressionNode = Result::from(node.parse_child("key", source).into())?;
        let value: Box<PairValue> = Result::from(node.parse_child("value", source).into())?;
        Ok(Self {
            range,
            key,
            value,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl PairNode {
    pub fn kind(&self) -> &'static str {
        "pair"
    }
}

impl NodeAccess for PairNode {
    fn brief_desc(&self) -> String {
        "PairNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::Pair(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.key.as_any());
        child_vec.push(self.value.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
