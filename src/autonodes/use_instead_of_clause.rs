use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::class_constant_access_expression::ClassConstantAccessExpressionNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;

use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub enum UseInsteadOfClauseChildren {
    ClassConstantAccessExpression(Box<ClassConstantAccessExpressionNode>),
    Name(Box<NameNode>),
    Extra(ExtraChild),
}

impl NodeParser for UseInsteadOfClauseChildren {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => UseInsteadOfClauseChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                UseInsteadOfClauseChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => UseInsteadOfClauseChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "class_constant_access_expression" => {
                UseInsteadOfClauseChildren::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "name" => UseInsteadOfClauseChildren::Name(Box::new(NameNode::parse(node, source)?)),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }
}

impl UseInsteadOfClauseChildren {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => UseInsteadOfClauseChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                UseInsteadOfClauseChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => UseInsteadOfClauseChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "class_constant_access_expression" => {
                UseInsteadOfClauseChildren::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "name" => UseInsteadOfClauseChildren::Name(Box::new(NameNode::parse(node, source)?)),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            UseInsteadOfClauseChildren::Extra(y) => y.kind(),
            UseInsteadOfClauseChildren::ClassConstantAccessExpression(y) => y.kind(),
            UseInsteadOfClauseChildren::Name(y) => y.kind(),
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
            UseInsteadOfClauseChildren::Extra(x) => x.get_utype(state, emitter),
            UseInsteadOfClauseChildren::ClassConstantAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            UseInsteadOfClauseChildren::Name(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            UseInsteadOfClauseChildren::Extra(x) => x.get_php_value(state, emitter),
            UseInsteadOfClauseChildren::ClassConstantAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            UseInsteadOfClauseChildren::Name(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            UseInsteadOfClauseChildren::Extra(x) => x.read_from(state, emitter),
            UseInsteadOfClauseChildren::ClassConstantAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            UseInsteadOfClauseChildren::Name(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for UseInsteadOfClauseChildren {
    fn brief_desc(&self) -> String {
        match self {
            UseInsteadOfClauseChildren::Extra(x) => {
                format!("UseInsteadOfClauseChildren::extra({})", x.brief_desc())
            }
            UseInsteadOfClauseChildren::ClassConstantAccessExpression(x) => format!(
                "UseInsteadOfClauseChildren::class_constant_access_expression({})",
                x.brief_desc()
            ),
            UseInsteadOfClauseChildren::Name(x) => {
                format!("UseInsteadOfClauseChildren::name({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            UseInsteadOfClauseChildren::Extra(x) => x.as_any(),
            UseInsteadOfClauseChildren::ClassConstantAccessExpression(x) => x.as_any(),
            UseInsteadOfClauseChildren::Name(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            UseInsteadOfClauseChildren::Extra(x) => x.children_any(),
            UseInsteadOfClauseChildren::ClassConstantAccessExpression(x) => x.children_any(),
            UseInsteadOfClauseChildren::Name(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            UseInsteadOfClauseChildren::Extra(x) => x.range(),
            UseInsteadOfClauseChildren::ClassConstantAccessExpression(x) => x.range(),
            UseInsteadOfClauseChildren::Name(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UseInsteadOfClauseNode {
    pub range: Range,
    pub children: Vec<Box<UseInsteadOfClauseChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for UseInsteadOfClauseNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "use_instead_of_clause" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [use_instead_of_clause] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            children: UseInsteadOfClauseChildren::parse_vec(
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

impl UseInsteadOfClauseNode {
    pub fn kind(&self) -> &'static str {
        "use_instead_of_clause"
    }
}

impl NodeAccess for UseInsteadOfClauseNode {
    fn brief_desc(&self) -> String {
        "UseInsteadOfClauseNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::UseInsteadOfClause(self)
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
