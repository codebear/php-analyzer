use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::class_constant_access_expression::ClassConstantAccessExpressionNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::visibility_modifier::VisibilityModifierNode;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::parser::Range;
use crate::types::union::PHPType;
use crate::value::PHPValue;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub enum UseAsClauseChildren {
    ClassConstantAccessExpression(Box<ClassConstantAccessExpressionNode>),
    Name(Box<NameNode>),
    VisibilityModifier(Box<VisibilityModifierNode>),
    Extra(ExtraChild),
}

impl NodeParser for UseAsClauseChildren {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => UseAsClauseChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => UseAsClauseChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => UseAsClauseChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "class_constant_access_expression" => {
                UseAsClauseChildren::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "name" => UseAsClauseChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "visibility_modifier" => UseAsClauseChildren::VisibilityModifier(Box::new(
                VisibilityModifierNode::parse(node, source)?,
            )),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!(
                        "UseAsClauseChildren: Parse error, unexpected node-type: {}",
                        node.kind()
                    ),
                ))
            }
        })
    }
}

impl UseAsClauseChildren {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => UseAsClauseChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => UseAsClauseChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => UseAsClauseChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "class_constant_access_expression" => {
                UseAsClauseChildren::ClassConstantAccessExpression(Box::new(
                    ClassConstantAccessExpressionNode::parse(node, source)?,
                ))
            }
            "name" => UseAsClauseChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "visibility_modifier" => UseAsClauseChildren::VisibilityModifier(Box::new(
                VisibilityModifierNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            UseAsClauseChildren::Extra(y) => y.kind(),
            UseAsClauseChildren::ClassConstantAccessExpression(y) => y.kind(),
            UseAsClauseChildren::Name(y) => y.kind(),
            UseAsClauseChildren::VisibilityModifier(y) => y.kind(),
        }
    }

    pub fn parse_vec<'a, I>(children: I, source: &[u8]) -> Result<Vec<Box<Self>>, ParseError>
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
    ) -> Option<PHPType> {
        match self {
            UseAsClauseChildren::Extra(x) => x.get_utype(state, emitter),
            UseAsClauseChildren::ClassConstantAccessExpression(x) => x.get_utype(state, emitter),
            UseAsClauseChildren::Name(x) => x.get_utype(state, emitter),
            UseAsClauseChildren::VisibilityModifier(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            UseAsClauseChildren::Extra(x) => x.get_php_value(state, emitter),
            UseAsClauseChildren::ClassConstantAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            UseAsClauseChildren::Name(x) => x.get_php_value(state, emitter),
            UseAsClauseChildren::VisibilityModifier(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            UseAsClauseChildren::Extra(x) => x.read_from(state, emitter),
            UseAsClauseChildren::ClassConstantAccessExpression(x) => x.read_from(state, emitter),
            UseAsClauseChildren::Name(x) => x.read_from(state, emitter),
            UseAsClauseChildren::VisibilityModifier(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for UseAsClauseChildren {
    fn brief_desc(&self) -> String {
        match self {
            UseAsClauseChildren::Extra(x) => {
                format!("UseAsClauseChildren::extra({})", x.brief_desc())
            }
            UseAsClauseChildren::ClassConstantAccessExpression(x) => format!(
                "UseAsClauseChildren::class_constant_access_expression({})",
                x.brief_desc()
            ),
            UseAsClauseChildren::Name(x) => {
                format!("UseAsClauseChildren::name({})", x.brief_desc())
            }
            UseAsClauseChildren::VisibilityModifier(x) => format!(
                "UseAsClauseChildren::visibility_modifier({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            UseAsClauseChildren::Extra(x) => x.as_any(),
            UseAsClauseChildren::ClassConstantAccessExpression(x) => x.as_any(),
            UseAsClauseChildren::Name(x) => x.as_any(),
            UseAsClauseChildren::VisibilityModifier(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            UseAsClauseChildren::Extra(x) => x.children_any(),
            UseAsClauseChildren::ClassConstantAccessExpression(x) => x.children_any(),
            UseAsClauseChildren::Name(x) => x.children_any(),
            UseAsClauseChildren::VisibilityModifier(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            UseAsClauseChildren::Extra(x) => x.range(),
            UseAsClauseChildren::ClassConstantAccessExpression(x) => x.range(),
            UseAsClauseChildren::Name(x) => x.range(),
            UseAsClauseChildren::VisibilityModifier(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UseAsClauseNode {
    pub range: Range,
    pub children: Vec<Box<UseAsClauseChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for UseAsClauseNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "use_as_clause" {
            return Err(ParseError::new(range, format!("UseAsClauseNode: Node is of the wrong kind [{}] vs expected [use_as_clause] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            children: UseAsClauseChildren::parse_vec(
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

impl UseAsClauseNode {
    pub fn kind(&self) -> &'static str {
        "use_as_clause"
    }
}

impl NodeAccess for UseAsClauseNode {
    fn brief_desc(&self) -> String {
        "UseAsClauseNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::UseAsClause(self)
    }

    #[allow(clippy::vec_init_then_push)]
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
