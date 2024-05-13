use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::namespace_aliasing_clause::NamespaceAliasingClauseNode;
use crate::autonodes::qualified_name::QualifiedNameNode;
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
pub enum NamespaceUseClauseChildren {
    Name(Box<NameNode>),
    NamespaceAliasingClause(Box<NamespaceAliasingClauseNode>),
    QualifiedName(Box<QualifiedNameNode>),
    Extra(ExtraChild),
}

impl NodeParser for NamespaceUseClauseChildren {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => NamespaceUseClauseChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => NamespaceUseClauseChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "name" => NamespaceUseClauseChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "namespace_aliasing_clause" => NamespaceUseClauseChildren::NamespaceAliasingClause(
                Box::new(NamespaceAliasingClauseNode::parse(node, source)?),
            ),
            "qualified_name" => NamespaceUseClauseChildren::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
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

impl NamespaceUseClauseChildren {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => NamespaceUseClauseChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => NamespaceUseClauseChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "name" => NamespaceUseClauseChildren::Name(Box::new(NameNode::parse(node, source)?)),
            "namespace_aliasing_clause" => NamespaceUseClauseChildren::NamespaceAliasingClause(
                Box::new(NamespaceAliasingClauseNode::parse(node, source)?),
            ),
            "qualified_name" => NamespaceUseClauseChildren::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            NamespaceUseClauseChildren::Extra(y) => y.kind(),
            NamespaceUseClauseChildren::Name(y) => y.kind(),
            NamespaceUseClauseChildren::NamespaceAliasingClause(y) => y.kind(),
            NamespaceUseClauseChildren::QualifiedName(y) => y.kind(),
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
            NamespaceUseClauseChildren::Extra(x) => x.get_utype(state, emitter),
            NamespaceUseClauseChildren::Name(x) => x.get_utype(state, emitter),
            NamespaceUseClauseChildren::NamespaceAliasingClause(x) => x.get_utype(state, emitter),
            NamespaceUseClauseChildren::QualifiedName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            NamespaceUseClauseChildren::Extra(x) => x.get_php_value(state, emitter),
            NamespaceUseClauseChildren::Name(x) => x.get_php_value(state, emitter),
            NamespaceUseClauseChildren::NamespaceAliasingClause(x) => {
                x.get_php_value(state, emitter)
            }
            NamespaceUseClauseChildren::QualifiedName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            NamespaceUseClauseChildren::Extra(x) => x.read_from(state, emitter),
            NamespaceUseClauseChildren::Name(x) => x.read_from(state, emitter),
            NamespaceUseClauseChildren::NamespaceAliasingClause(x) => x.read_from(state, emitter),
            NamespaceUseClauseChildren::QualifiedName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for NamespaceUseClauseChildren {
    fn brief_desc(&self) -> String {
        match self {
            NamespaceUseClauseChildren::Extra(x) => {
                format!("NamespaceUseClauseChildren::extra({})", x.brief_desc())
            }
            NamespaceUseClauseChildren::Name(x) => {
                format!("NamespaceUseClauseChildren::name({})", x.brief_desc())
            }
            NamespaceUseClauseChildren::NamespaceAliasingClause(x) => format!(
                "NamespaceUseClauseChildren::namespace_aliasing_clause({})",
                x.brief_desc()
            ),
            NamespaceUseClauseChildren::QualifiedName(x) => format!(
                "NamespaceUseClauseChildren::qualified_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            NamespaceUseClauseChildren::Extra(x) => x.as_any(),
            NamespaceUseClauseChildren::Name(x) => x.as_any(),
            NamespaceUseClauseChildren::NamespaceAliasingClause(x) => x.as_any(),
            NamespaceUseClauseChildren::QualifiedName(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            NamespaceUseClauseChildren::Extra(x) => x.children_any(),
            NamespaceUseClauseChildren::Name(x) => x.children_any(),
            NamespaceUseClauseChildren::NamespaceAliasingClause(x) => x.children_any(),
            NamespaceUseClauseChildren::QualifiedName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            NamespaceUseClauseChildren::Extra(x) => x.range(),
            NamespaceUseClauseChildren::Name(x) => x.range(),
            NamespaceUseClauseChildren::NamespaceAliasingClause(x) => x.range(),
            NamespaceUseClauseChildren::QualifiedName(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NamespaceUseClauseNode {
    pub range: Range,
    pub children: Vec<Box<NamespaceUseClauseChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for NamespaceUseClauseNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "namespace_use_clause" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [namespace_use_clause] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            children: NamespaceUseClauseChildren::parse_vec(
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

impl NamespaceUseClauseNode {
    pub fn kind(&self) -> &'static str {
        "namespace_use_clause"
    }
}

impl NodeAccess for NamespaceUseClauseNode {
    fn brief_desc(&self) -> String {
        "NamespaceUseClauseNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::NamespaceUseClause(self)
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
