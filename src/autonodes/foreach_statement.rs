use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::_statement::_StatementNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::by_ref::ByRefNode;
use crate::autonodes::colon_block::ColonBlockNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::list_literal::ListLiteralNode;
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
pub enum ForeachStatementBody {
    _Statement(Box<_StatementNode>),
    ColonBlock(Box<ColonBlockNode>),
    Extra(ExtraChild),
}

impl ForeachStatementBody {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ForeachStatementBody::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ForeachStatementBody::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => ForeachStatementBody::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "colon_block" => {
                ForeachStatementBody::ColonBlock(Box::new(ColonBlockNode::parse(node, source)?))
            }

            _ => {
                if let Some(x) = _StatementNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| ForeachStatementBody::_Statement(y))
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
            "comment" => ForeachStatementBody::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ForeachStatementBody::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => ForeachStatementBody::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "colon_block" => {
                ForeachStatementBody::ColonBlock(Box::new(ColonBlockNode::parse(node, source)?))
            }

            _ => {
                return Ok(
                    if let Some(x) = _StatementNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| ForeachStatementBody::_Statement(y))
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

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ForeachStatementBody::Extra(x) => x.as_any(),
            ForeachStatementBody::_Statement(x) => x.as_any(),
            ForeachStatementBody::ColonBlock(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
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
pub enum ForeachStatementValue {
    _Expression(Box<_ExpressionNode>),
    ByRef(Box<ByRefNode>),
    ListLiteral(Box<ListLiteralNode>),
    Extra(ExtraChild),
}

impl ForeachStatementValue {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ForeachStatementValue::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ForeachStatementValue::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => ForeachStatementValue::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "by_ref" => ForeachStatementValue::ByRef(Box::new(ByRefNode::parse(node, source)?)),
            "list_literal" => {
                ForeachStatementValue::ListLiteral(Box::new(ListLiteralNode::parse(node, source)?))
            }

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| ForeachStatementValue::_Expression(y))
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
            "comment" => ForeachStatementValue::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ForeachStatementValue::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => ForeachStatementValue::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "by_ref" => ForeachStatementValue::ByRef(Box::new(ByRefNode::parse(node, source)?)),
            "list_literal" => {
                ForeachStatementValue::ListLiteral(Box::new(ListLiteralNode::parse(node, source)?))
            }

            _ => {
                return Ok(
                    if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| ForeachStatementValue::_Expression(y))
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
            ForeachStatementValue::Extra(y) => y.kind(),
            ForeachStatementValue::_Expression(y) => y.kind(),
            ForeachStatementValue::ByRef(y) => y.kind(),
            ForeachStatementValue::ListLiteral(y) => y.kind(),
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
            ForeachStatementValue::Extra(x) => x.get_utype(state, emitter),
            ForeachStatementValue::_Expression(x) => x.get_utype(state, emitter),
            ForeachStatementValue::ByRef(x) => x.get_utype(state, emitter),
            ForeachStatementValue::ListLiteral(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ForeachStatementValue::Extra(x) => x.get_php_value(state, emitter),
            ForeachStatementValue::_Expression(x) => x.get_php_value(state, emitter),
            ForeachStatementValue::ByRef(x) => x.get_php_value(state, emitter),
            ForeachStatementValue::ListLiteral(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ForeachStatementValue::Extra(x) => x.read_from(state, emitter),
            ForeachStatementValue::_Expression(x) => x.read_from(state, emitter),
            ForeachStatementValue::ByRef(x) => x.read_from(state, emitter),
            ForeachStatementValue::ListLiteral(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ForeachStatementValue {
    fn brief_desc(&self) -> String {
        match self {
            ForeachStatementValue::Extra(x) => {
                format!("ForeachStatementValue::extra({})", x.brief_desc())
            }
            ForeachStatementValue::_Expression(x) => {
                format!("ForeachStatementValue::_expression({})", x.brief_desc())
            }
            ForeachStatementValue::ByRef(x) => {
                format!("ForeachStatementValue::by_ref({})", x.brief_desc())
            }
            ForeachStatementValue::ListLiteral(x) => {
                format!("ForeachStatementValue::list_literal({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ForeachStatementValue::Extra(x) => x.as_any(),
            ForeachStatementValue::_Expression(x) => x.as_any(),
            ForeachStatementValue::ByRef(x) => x.as_any(),
            ForeachStatementValue::ListLiteral(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ForeachStatementValue::Extra(x) => x.children_any(),
            ForeachStatementValue::_Expression(x) => x.children_any(),
            ForeachStatementValue::ByRef(x) => x.children_any(),
            ForeachStatementValue::ListLiteral(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ForeachStatementValue::Extra(x) => x.range(),
            ForeachStatementValue::_Expression(x) => x.range(),
            ForeachStatementValue::ByRef(x) => x.range(),
            ForeachStatementValue::ListLiteral(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ForeachStatementNode {
    pub range: Range,
    pub body: Option<Box<ForeachStatementBody>>,
    pub key: Option<_ExpressionNode>,
    pub traversable: _ExpressionNode,
    pub value: Box<ForeachStatementValue>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl ForeachStatementNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
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
        let body: Option<Box<ForeachStatementBody>> = node
            .children_by_field_name("body", &mut node.walk())
            .map(|chnode2| ForeachStatementBody::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .into();
        let key: Option<_ExpressionNode> = node
            .children_by_field_name("key", &mut node.walk())
            .map(|chnode1| _ExpressionNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        let traversable: _ExpressionNode = node
            .children_by_field_name("traversable", &mut node.walk())
            .map(|chnode1| _ExpressionNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field traversable should exist");
        let value: Box<ForeachStatementValue> = node
            .children_by_field_name("value", &mut node.walk())
            .map(|chnode2| ForeachStatementValue::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .expect("Field value should exist")
            .into();
        Ok(Self {
            range,
            body,
            key,
            traversable,
            value,
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
        "foreach_statement"
    }
}

impl NodeAccess for ForeachStatementNode {
    fn brief_desc(&self) -> String {
        "ForeachStatementNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::ForeachStatement(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.body {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.key {
            child_vec.push(x.as_any());
        }
        child_vec.push(self.traversable.as_any());
        child_vec.push(self.value.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
