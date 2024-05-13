use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::use_as_clause::UseAsClauseNode;
use crate::autonodes::use_instead_of_clause::UseInsteadOfClauseNode;
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
pub enum UseListChildren {
    UseAsClause(Box<UseAsClauseNode>),
    UseInsteadOfClause(Box<UseInsteadOfClauseNode>),
    Extra(ExtraChild),
}

impl NodeParser for UseListChildren {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => UseListChildren::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "ERROR" => {
                UseListChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "use_as_clause" => {
                UseListChildren::UseAsClause(Box::new(UseAsClauseNode::parse(node, source)?))
            }
            "use_instead_of_clause" => UseListChildren::UseInsteadOfClause(Box::new(
                UseInsteadOfClauseNode::parse(node, source)?,
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

impl UseListChildren {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => UseListChildren::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "ERROR" => {
                UseListChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "use_as_clause" => {
                UseListChildren::UseAsClause(Box::new(UseAsClauseNode::parse(node, source)?))
            }
            "use_instead_of_clause" => UseListChildren::UseInsteadOfClause(Box::new(
                UseInsteadOfClauseNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            UseListChildren::Extra(y) => y.kind(),
            UseListChildren::UseAsClause(y) => y.kind(),
            UseListChildren::UseInsteadOfClause(y) => y.kind(),
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
            UseListChildren::Extra(x) => x.get_utype(state, emitter),
            UseListChildren::UseAsClause(x) => x.get_utype(state, emitter),
            UseListChildren::UseInsteadOfClause(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            UseListChildren::Extra(x) => x.get_php_value(state, emitter),
            UseListChildren::UseAsClause(x) => x.get_php_value(state, emitter),
            UseListChildren::UseInsteadOfClause(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            UseListChildren::Extra(x) => x.read_from(state, emitter),
            UseListChildren::UseAsClause(x) => x.read_from(state, emitter),
            UseListChildren::UseInsteadOfClause(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for UseListChildren {
    fn brief_desc(&self) -> String {
        match self {
            UseListChildren::Extra(x) => format!("UseListChildren::extra({})", x.brief_desc()),
            UseListChildren::UseAsClause(x) => {
                format!("UseListChildren::use_as_clause({})", x.brief_desc())
            }
            UseListChildren::UseInsteadOfClause(x) => {
                format!("UseListChildren::use_instead_of_clause({})", x.brief_desc())
            }
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            UseListChildren::Extra(x) => x.as_any(),
            UseListChildren::UseAsClause(x) => x.as_any(),
            UseListChildren::UseInsteadOfClause(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            UseListChildren::Extra(x) => x.children_any(),
            UseListChildren::UseAsClause(x) => x.children_any(),
            UseListChildren::UseInsteadOfClause(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            UseListChildren::Extra(x) => x.range(),
            UseListChildren::UseAsClause(x) => x.range(),
            UseListChildren::UseInsteadOfClause(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UseListNode {
    pub range: Range,
    pub children: Vec<Box<UseListChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for UseListNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "use_list" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [use_list] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            children: UseListChildren::parse_vec(
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

impl UseListNode {
    pub fn kind(&self) -> &'static str {
        "use_list"
    }
}

impl NodeAccess for UseListNode {
    fn brief_desc(&self) -> String {
        "UseListNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::UseList(self)
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
