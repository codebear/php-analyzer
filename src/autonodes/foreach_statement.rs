use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::_statement::_StatementNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::by_ref::ByRefNode;
use crate::autonodes::colon_block::ColonBlockNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::list_literal::ListLiteralNode;
use crate::autonodes::pair::PairNode;
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
pub enum ForeachStatementBody {
    _Statement(Box<_StatementNode>),
    ColonBlock(Box<ColonBlockNode>),
    Extra(ExtraChild),
}

impl NodeParser for ForeachStatementBody {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ForeachStatementBody::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => ForeachStatementBody::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "colon_block" => {
                ForeachStatementBody::ColonBlock(Box::new(ColonBlockNode::parse(node, source)?))
            }

            _ => {
                if let Some(x) = _StatementNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(ForeachStatementBody::_Statement)
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

impl ForeachStatementBody {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ForeachStatementBody::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => ForeachStatementBody::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "colon_block" => {
                ForeachStatementBody::ColonBlock(Box::new(ColonBlockNode::parse(node, source)?))
            }

            _ => {
                return Ok(_StatementNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(ForeachStatementBody::_Statement))
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            ForeachStatementBody::Extra(y) => y.kind(),
            ForeachStatementBody::_Statement(y) => y.kind(),
            ForeachStatementBody::ColonBlock(y) => y.kind(),
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
            ForeachStatementBody::Extra(x) => x.get_utype(state, emitter),
            ForeachStatementBody::_Statement(x) => x.get_utype(state, emitter),
            ForeachStatementBody::ColonBlock(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ForeachStatementBody::Extra(x) => x.get_php_value(state, emitter),
            ForeachStatementBody::_Statement(x) => x.get_php_value(state, emitter),
            ForeachStatementBody::ColonBlock(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ForeachStatementBody::Extra(x) => x.read_from(state, emitter),
            ForeachStatementBody::_Statement(x) => x.read_from(state, emitter),
            ForeachStatementBody::ColonBlock(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ForeachStatementBody {
    fn brief_desc(&self) -> String {
        match self {
            ForeachStatementBody::Extra(x) => {
                format!("ForeachStatementBody::extra({})", x.brief_desc())
            }
            ForeachStatementBody::_Statement(x) => {
                format!("ForeachStatementBody::_statement({})", x.brief_desc())
            }
            ForeachStatementBody::ColonBlock(x) => {
                format!("ForeachStatementBody::colon_block({})", x.brief_desc())
            }
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            ForeachStatementBody::Extra(x) => x.as_any(),
            ForeachStatementBody::_Statement(x) => x.as_any(),
            ForeachStatementBody::ColonBlock(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            ForeachStatementBody::Extra(x) => x.children_any(),
            ForeachStatementBody::_Statement(x) => x.children_any(),
            ForeachStatementBody::ColonBlock(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ForeachStatementBody::Extra(x) => x.range(),
            ForeachStatementBody::_Statement(x) => x.range(),
            ForeachStatementBody::ColonBlock(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ForeachStatementEntry {
    _Expression(Box<_ExpressionNode>),
    ByRef(Box<ByRefNode>),
    ListLiteral(Box<ListLiteralNode>),
    Pair(Box<PairNode>),
    Extra(ExtraChild),
}

impl NodeParser for ForeachStatementEntry {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ForeachStatementEntry::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => ForeachStatementEntry::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "by_ref" => ForeachStatementEntry::ByRef(Box::new(ByRefNode::parse(node, source)?)),
            "list_literal" => {
                ForeachStatementEntry::ListLiteral(Box::new(ListLiteralNode::parse(node, source)?))
            }
            "pair" => ForeachStatementEntry::Pair(Box::new(PairNode::parse(node, source)?)),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(ForeachStatementEntry::_Expression)
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

impl ForeachStatementEntry {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ForeachStatementEntry::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => ForeachStatementEntry::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "by_ref" => ForeachStatementEntry::ByRef(Box::new(ByRefNode::parse(node, source)?)),
            "list_literal" => {
                ForeachStatementEntry::ListLiteral(Box::new(ListLiteralNode::parse(node, source)?))
            }
            "pair" => ForeachStatementEntry::Pair(Box::new(PairNode::parse(node, source)?)),

            _ => {
                return Ok(_ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(ForeachStatementEntry::_Expression))
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            ForeachStatementEntry::Extra(y) => y.kind(),
            ForeachStatementEntry::_Expression(y) => y.kind(),
            ForeachStatementEntry::ByRef(y) => y.kind(),
            ForeachStatementEntry::ListLiteral(y) => y.kind(),
            ForeachStatementEntry::Pair(y) => y.kind(),
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
            ForeachStatementEntry::Extra(x) => x.get_utype(state, emitter),
            ForeachStatementEntry::_Expression(x) => x.get_utype(state, emitter),
            ForeachStatementEntry::ByRef(x) => x.get_utype(state, emitter),
            ForeachStatementEntry::ListLiteral(x) => x.get_utype(state, emitter),
            ForeachStatementEntry::Pair(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ForeachStatementEntry::Extra(x) => x.get_php_value(state, emitter),
            ForeachStatementEntry::_Expression(x) => x.get_php_value(state, emitter),
            ForeachStatementEntry::ByRef(x) => x.get_php_value(state, emitter),
            ForeachStatementEntry::ListLiteral(x) => x.get_php_value(state, emitter),
            ForeachStatementEntry::Pair(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ForeachStatementEntry::Extra(x) => x.read_from(state, emitter),
            ForeachStatementEntry::_Expression(x) => x.read_from(state, emitter),
            ForeachStatementEntry::ByRef(x) => x.read_from(state, emitter),
            ForeachStatementEntry::ListLiteral(x) => x.read_from(state, emitter),
            ForeachStatementEntry::Pair(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ForeachStatementEntry {
    fn brief_desc(&self) -> String {
        match self {
            ForeachStatementEntry::Extra(x) => {
                format!("ForeachStatementEntry::extra({})", x.brief_desc())
            }
            ForeachStatementEntry::_Expression(x) => {
                format!("ForeachStatementEntry::_expression({})", x.brief_desc())
            }
            ForeachStatementEntry::ByRef(x) => {
                format!("ForeachStatementEntry::by_ref({})", x.brief_desc())
            }
            ForeachStatementEntry::ListLiteral(x) => {
                format!("ForeachStatementEntry::list_literal({})", x.brief_desc())
            }
            ForeachStatementEntry::Pair(x) => {
                format!("ForeachStatementEntry::pair({})", x.brief_desc())
            }
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            ForeachStatementEntry::Extra(x) => x.as_any(),
            ForeachStatementEntry::_Expression(x) => x.as_any(),
            ForeachStatementEntry::ByRef(x) => x.as_any(),
            ForeachStatementEntry::ListLiteral(x) => x.as_any(),
            ForeachStatementEntry::Pair(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            ForeachStatementEntry::Extra(x) => x.children_any(),
            ForeachStatementEntry::_Expression(x) => x.children_any(),
            ForeachStatementEntry::ByRef(x) => x.children_any(),
            ForeachStatementEntry::ListLiteral(x) => x.children_any(),
            ForeachStatementEntry::Pair(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ForeachStatementEntry::Extra(x) => x.range(),
            ForeachStatementEntry::_Expression(x) => x.range(),
            ForeachStatementEntry::ByRef(x) => x.range(),
            ForeachStatementEntry::ListLiteral(x) => x.range(),
            ForeachStatementEntry::Pair(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ForeachStatementNode {
    pub range: Range,
    pub body: Option<Box<ForeachStatementBody>>,
    pub entry: Box<ForeachStatementEntry>,
    pub traversable: _ExpressionNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for ForeachStatementNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "foreach_statement" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [foreach_statement] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let body: Option<Box<ForeachStatementBody>> =
            Into::<Result<_, _>>::into(node.parse_child("body", source))?;
        let entry: Box<ForeachStatementEntry> =
            Into::<Result<_, _>>::into(node.parse_child("entry", source))?;
        let traversable: _ExpressionNode =
            Into::<Result<_, _>>::into(node.parse_child("traversable", source))?;
        Ok(Self {
            range,
            body,
            entry,
            traversable,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl ForeachStatementNode {
    pub fn kind(&self) -> &'static str {
        "foreach_statement"
    }
}

impl NodeAccess for ForeachStatementNode {
    fn brief_desc(&self) -> String {
        "ForeachStatementNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::ForeachStatement(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.body {
            child_vec.push(x.as_any());
        }
        child_vec.push(self.entry.as_any());
        child_vec.push(self.traversable.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
