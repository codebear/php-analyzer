use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::qualified_name::QualifiedNameNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
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
pub enum BaseClauseChildren {
    Name(Box<NameNode>),
    QualifiedName(Box<QualifiedNameNode>),
    Extra(ExtraChild),
}

impl NodeParser for BaseClauseChildren {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => BaseClauseChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => BaseClauseChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => BaseClauseChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "name" => BaseClauseChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "qualified_name" => {
                BaseClauseChildren::QualifiedName(Box::new(QualifiedNameNode::parse(node, source)?))
            }

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!(
                        "BaseClauseChildren: Parse error, unexpected node-type: {}",
                        node.kind()
                    ),
                ))
            }
        })
    }
}

impl BaseClauseChildren {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => BaseClauseChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => BaseClauseChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => BaseClauseChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "name" => BaseClauseChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "qualified_name" => {
                BaseClauseChildren::QualifiedName(Box::new(QualifiedNameNode::parse(node, source)?))
            }

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            BaseClauseChildren::Extra(y) => y.kind(),
            BaseClauseChildren::Name(y) => y.kind(),
            BaseClauseChildren::QualifiedName(y) => y.kind(),
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
            BaseClauseChildren::Extra(x) => x.get_utype(state, emitter),
            BaseClauseChildren::Name(x) => x.get_utype(state, emitter),
            BaseClauseChildren::QualifiedName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            BaseClauseChildren::Extra(x) => x.get_php_value(state, emitter),
            BaseClauseChildren::Name(x) => x.get_php_value(state, emitter),
            BaseClauseChildren::QualifiedName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            BaseClauseChildren::Extra(x) => x.read_from(state, emitter),
            BaseClauseChildren::Name(x) => x.read_from(state, emitter),
            BaseClauseChildren::QualifiedName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for BaseClauseChildren {
    fn brief_desc(&self) -> String {
        match self {
            BaseClauseChildren::Extra(x) => {
                format!("BaseClauseChildren::extra({})", x.brief_desc())
            }
            BaseClauseChildren::Name(x) => format!("BaseClauseChildren::name({})", x.brief_desc()),
            BaseClauseChildren::QualifiedName(x) => {
                format!("BaseClauseChildren::qualified_name({})", x.brief_desc())
            }
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            BaseClauseChildren::Extra(x) => x.as_any(),
            BaseClauseChildren::Name(x) => x.as_any(),
            BaseClauseChildren::QualifiedName(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            BaseClauseChildren::Extra(x) => x.children_any(),
            BaseClauseChildren::Name(x) => x.children_any(),
            BaseClauseChildren::QualifiedName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            BaseClauseChildren::Extra(x) => x.range(),
            BaseClauseChildren::Name(x) => x.range(),
            BaseClauseChildren::QualifiedName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BaseClauseNode {
    pub range: Range,
    pub children: Vec<Box<BaseClauseChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for BaseClauseNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "base_clause" {
            return Err(ParseError::new(range, format!("BaseClauseNode: Node is of the wrong kind [{}] vs expected [base_clause] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            children: BaseClauseChildren::parse_vec(
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

impl BaseClauseNode {
    pub fn kind(&self) -> &'static str {
        "base_clause"
    }
}

impl NodeAccess for BaseClauseNode {
    fn brief_desc(&self) -> String {
        "BaseClauseNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::BaseClause(self)
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
