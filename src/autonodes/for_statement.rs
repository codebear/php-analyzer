use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::_statement::_StatementNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::sequence_expression::SequenceExpressionNode;
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
pub enum ForStatementCondition {
    _Expression(Box<_ExpressionNode>),
    SequenceExpression(Box<SequenceExpressionNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl ForStatementCondition {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => {
                ForStatementCondition::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => ForStatementCondition::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => ForStatementCondition::Error(Box::new(ErrorNode::parse(node, source)?)),
            "sequence_expression" => ForStatementCondition::SequenceExpression(Box::new(
                SequenceExpressionNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| ForStatementCondition::_Expression(y))
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
            "comment" => {
                ForStatementCondition::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => ForStatementCondition::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => ForStatementCondition::Error(Box::new(ErrorNode::parse(node, source)?)),
            "sequence_expression" => ForStatementCondition::SequenceExpression(Box::new(
                SequenceExpressionNode::parse(node, source)?,
            )),

            _ => {
                return Ok(
                    if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| ForStatementCondition::_Expression(y))
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
            ForStatementCondition::Comment(x) => x.get_utype(state, emitter),
            ForStatementCondition::TextInterpolation(x) => x.get_utype(state, emitter),
            ForStatementCondition::Error(x) => x.get_utype(state, emitter),
            ForStatementCondition::_Expression(x) => x.get_utype(state, emitter),
            ForStatementCondition::SequenceExpression(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ForStatementCondition::Comment(x) => x.get_php_value(state, emitter),
            ForStatementCondition::TextInterpolation(x) => x.get_php_value(state, emitter),
            ForStatementCondition::Error(x) => x.get_php_value(state, emitter),
            ForStatementCondition::_Expression(x) => x.get_php_value(state, emitter),
            ForStatementCondition::SequenceExpression(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ForStatementCondition::Comment(x) => x.read_from(state, emitter),
            ForStatementCondition::TextInterpolation(x) => x.read_from(state, emitter),
            ForStatementCondition::Error(x) => x.read_from(state, emitter),
            ForStatementCondition::_Expression(x) => x.read_from(state, emitter),
            ForStatementCondition::SequenceExpression(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ForStatementCondition {
    fn brief_desc(&self) -> String {
        match self {
            ForStatementCondition::Comment(x) => {
                format!("ForStatementCondition::comment({})", x.brief_desc())
            }
            ForStatementCondition::TextInterpolation(x) => format!(
                "ForStatementCondition::text_interpolation({})",
                x.brief_desc()
            ),
            ForStatementCondition::Error(x) => {
                format!("ForStatementCondition::ERROR({})", x.brief_desc())
            }
            ForStatementCondition::_Expression(x) => {
                format!("ForStatementCondition::_expression({})", x.brief_desc())
            }
            ForStatementCondition::SequenceExpression(x) => format!(
                "ForStatementCondition::sequence_expression({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ForStatementCondition::Comment(x) => x.as_any(),
            ForStatementCondition::TextInterpolation(x) => x.as_any(),
            ForStatementCondition::Error(x) => x.as_any(),
            ForStatementCondition::_Expression(x) => x.as_any(),
            ForStatementCondition::SequenceExpression(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ForStatementCondition::Comment(x) => x.children_any(),
            ForStatementCondition::TextInterpolation(x) => x.children_any(),
            ForStatementCondition::Error(x) => x.children_any(),
            ForStatementCondition::_Expression(x) => x.children_any(),
            ForStatementCondition::SequenceExpression(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ForStatementCondition::Comment(x) => x.range(),
            ForStatementCondition::TextInterpolation(x) => x.range(),
            ForStatementCondition::Error(x) => x.range(),
            ForStatementCondition::_Expression(x) => x.range(),
            ForStatementCondition::SequenceExpression(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForStatementIncrement {
    _Expression(Box<_ExpressionNode>),
    SequenceExpression(Box<SequenceExpressionNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl ForStatementIncrement {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => {
                ForStatementIncrement::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => ForStatementIncrement::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => ForStatementIncrement::Error(Box::new(ErrorNode::parse(node, source)?)),
            "sequence_expression" => ForStatementIncrement::SequenceExpression(Box::new(
                SequenceExpressionNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| ForStatementIncrement::_Expression(y))
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
            "comment" => {
                ForStatementIncrement::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => ForStatementIncrement::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => ForStatementIncrement::Error(Box::new(ErrorNode::parse(node, source)?)),
            "sequence_expression" => ForStatementIncrement::SequenceExpression(Box::new(
                SequenceExpressionNode::parse(node, source)?,
            )),

            _ => {
                return Ok(
                    if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| ForStatementIncrement::_Expression(y))
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
            ForStatementIncrement::Comment(x) => x.get_utype(state, emitter),
            ForStatementIncrement::TextInterpolation(x) => x.get_utype(state, emitter),
            ForStatementIncrement::Error(x) => x.get_utype(state, emitter),
            ForStatementIncrement::_Expression(x) => x.get_utype(state, emitter),
            ForStatementIncrement::SequenceExpression(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ForStatementIncrement::Comment(x) => x.get_php_value(state, emitter),
            ForStatementIncrement::TextInterpolation(x) => x.get_php_value(state, emitter),
            ForStatementIncrement::Error(x) => x.get_php_value(state, emitter),
            ForStatementIncrement::_Expression(x) => x.get_php_value(state, emitter),
            ForStatementIncrement::SequenceExpression(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ForStatementIncrement::Comment(x) => x.read_from(state, emitter),
            ForStatementIncrement::TextInterpolation(x) => x.read_from(state, emitter),
            ForStatementIncrement::Error(x) => x.read_from(state, emitter),
            ForStatementIncrement::_Expression(x) => x.read_from(state, emitter),
            ForStatementIncrement::SequenceExpression(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ForStatementIncrement {
    fn brief_desc(&self) -> String {
        match self {
            ForStatementIncrement::Comment(x) => {
                format!("ForStatementIncrement::comment({})", x.brief_desc())
            }
            ForStatementIncrement::TextInterpolation(x) => format!(
                "ForStatementIncrement::text_interpolation({})",
                x.brief_desc()
            ),
            ForStatementIncrement::Error(x) => {
                format!("ForStatementIncrement::ERROR({})", x.brief_desc())
            }
            ForStatementIncrement::_Expression(x) => {
                format!("ForStatementIncrement::_expression({})", x.brief_desc())
            }
            ForStatementIncrement::SequenceExpression(x) => format!(
                "ForStatementIncrement::sequence_expression({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ForStatementIncrement::Comment(x) => x.as_any(),
            ForStatementIncrement::TextInterpolation(x) => x.as_any(),
            ForStatementIncrement::Error(x) => x.as_any(),
            ForStatementIncrement::_Expression(x) => x.as_any(),
            ForStatementIncrement::SequenceExpression(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ForStatementIncrement::Comment(x) => x.children_any(),
            ForStatementIncrement::TextInterpolation(x) => x.children_any(),
            ForStatementIncrement::Error(x) => x.children_any(),
            ForStatementIncrement::_Expression(x) => x.children_any(),
            ForStatementIncrement::SequenceExpression(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ForStatementIncrement::Comment(x) => x.range(),
            ForStatementIncrement::TextInterpolation(x) => x.range(),
            ForStatementIncrement::Error(x) => x.range(),
            ForStatementIncrement::_Expression(x) => x.range(),
            ForStatementIncrement::SequenceExpression(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForStatementInitialize {
    _Expression(Box<_ExpressionNode>),
    SequenceExpression(Box<SequenceExpressionNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl ForStatementInitialize {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => {
                ForStatementInitialize::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => ForStatementInitialize::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => ForStatementInitialize::Error(Box::new(ErrorNode::parse(node, source)?)),
            "sequence_expression" => ForStatementInitialize::SequenceExpression(Box::new(
                SequenceExpressionNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| ForStatementInitialize::_Expression(y))
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
            "comment" => {
                ForStatementInitialize::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => ForStatementInitialize::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => ForStatementInitialize::Error(Box::new(ErrorNode::parse(node, source)?)),
            "sequence_expression" => ForStatementInitialize::SequenceExpression(Box::new(
                SequenceExpressionNode::parse(node, source)?,
            )),

            _ => {
                return Ok(
                    if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| ForStatementInitialize::_Expression(y))
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
            ForStatementInitialize::Comment(x) => x.get_utype(state, emitter),
            ForStatementInitialize::TextInterpolation(x) => x.get_utype(state, emitter),
            ForStatementInitialize::Error(x) => x.get_utype(state, emitter),
            ForStatementInitialize::_Expression(x) => x.get_utype(state, emitter),
            ForStatementInitialize::SequenceExpression(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ForStatementInitialize::Comment(x) => x.get_php_value(state, emitter),
            ForStatementInitialize::TextInterpolation(x) => x.get_php_value(state, emitter),
            ForStatementInitialize::Error(x) => x.get_php_value(state, emitter),
            ForStatementInitialize::_Expression(x) => x.get_php_value(state, emitter),
            ForStatementInitialize::SequenceExpression(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ForStatementInitialize::Comment(x) => x.read_from(state, emitter),
            ForStatementInitialize::TextInterpolation(x) => x.read_from(state, emitter),
            ForStatementInitialize::Error(x) => x.read_from(state, emitter),
            ForStatementInitialize::_Expression(x) => x.read_from(state, emitter),
            ForStatementInitialize::SequenceExpression(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ForStatementInitialize {
    fn brief_desc(&self) -> String {
        match self {
            ForStatementInitialize::Comment(x) => {
                format!("ForStatementInitialize::comment({})", x.brief_desc())
            }
            ForStatementInitialize::TextInterpolation(x) => format!(
                "ForStatementInitialize::text_interpolation({})",
                x.brief_desc()
            ),
            ForStatementInitialize::Error(x) => {
                format!("ForStatementInitialize::ERROR({})", x.brief_desc())
            }
            ForStatementInitialize::_Expression(x) => {
                format!("ForStatementInitialize::_expression({})", x.brief_desc())
            }
            ForStatementInitialize::SequenceExpression(x) => format!(
                "ForStatementInitialize::sequence_expression({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ForStatementInitialize::Comment(x) => x.as_any(),
            ForStatementInitialize::TextInterpolation(x) => x.as_any(),
            ForStatementInitialize::Error(x) => x.as_any(),
            ForStatementInitialize::_Expression(x) => x.as_any(),
            ForStatementInitialize::SequenceExpression(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ForStatementInitialize::Comment(x) => x.children_any(),
            ForStatementInitialize::TextInterpolation(x) => x.children_any(),
            ForStatementInitialize::Error(x) => x.children_any(),
            ForStatementInitialize::_Expression(x) => x.children_any(),
            ForStatementInitialize::SequenceExpression(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ForStatementInitialize::Comment(x) => x.range(),
            ForStatementInitialize::TextInterpolation(x) => x.range(),
            ForStatementInitialize::Error(x) => x.range(),
            ForStatementInitialize::_Expression(x) => x.range(),
            ForStatementInitialize::SequenceExpression(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ForStatementNode {
    pub range: Range,
    pub condition: Option<Box<ForStatementCondition>>,
    pub increment: Option<Box<ForStatementIncrement>>,
    pub initialize: Option<Box<ForStatementInitialize>>,
    pub children: Vec<Box<_StatementNode>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl ForStatementNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "for_statement" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [for_statement] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let mut skip_nodes: Vec<usize> = vec![];
        let condition: Option<Box<ForStatementCondition>> = node
            .children_by_field_name("condition", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode2| ForStatementCondition::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .into();
        let increment: Option<Box<ForStatementIncrement>> = node
            .children_by_field_name("increment", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode2| ForStatementIncrement::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .into();
        let initialize: Option<Box<ForStatementInitialize>> = node
            .children_by_field_name("initialize", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode2| ForStatementInitialize::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .into();
        Ok(Self {
            range,
            condition,
            increment,
            initialize,
            children: _StatementNode::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| !skip_nodes.contains(&node.id()))
                    .filter(|node| node.kind() != "comment"),
                source,
            )?,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment")
                    .filter(|node| !skip_nodes.contains(&node.id())),
                source,
            )?,
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
        "for_statement"
    }
}

impl NodeAccess for ForStatementNode {
    fn brief_desc(&self) -> String {
        "ForStatementNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::ForStatement(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.condition {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.increment {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.initialize {
            child_vec.push(x.as_any());
        }
        child_vec.extend(self.children.iter().map(|n| n.as_any()));
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
