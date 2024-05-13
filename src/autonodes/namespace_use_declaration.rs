use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::namespace_name::NamespaceNameNode;
use crate::autonodes::namespace_use_clause::NamespaceUseClauseNode;
use crate::autonodes::namespace_use_group::NamespaceUseGroupNode;
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
pub enum NamespaceUseDeclarationChildren {
    NamespaceName(Box<NamespaceNameNode>),
    NamespaceUseClause(Box<NamespaceUseClauseNode>),
    NamespaceUseGroup(Box<NamespaceUseGroupNode>),
    Extra(ExtraChild),
}

impl NodeParser for NamespaceUseDeclarationChildren {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => NamespaceUseDeclarationChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => NamespaceUseDeclarationChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "namespace_name" => NamespaceUseDeclarationChildren::NamespaceName(Box::new(
                NamespaceNameNode::parse(node, source)?,
            )),
            "namespace_use_clause" => NamespaceUseDeclarationChildren::NamespaceUseClause(
                Box::new(NamespaceUseClauseNode::parse(node, source)?),
            ),
            "namespace_use_group" => NamespaceUseDeclarationChildren::NamespaceUseGroup(Box::new(
                NamespaceUseGroupNode::parse(node, source)?,
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

impl NamespaceUseDeclarationChildren {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => NamespaceUseDeclarationChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => NamespaceUseDeclarationChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "namespace_name" => NamespaceUseDeclarationChildren::NamespaceName(Box::new(
                NamespaceNameNode::parse(node, source)?,
            )),
            "namespace_use_clause" => NamespaceUseDeclarationChildren::NamespaceUseClause(
                Box::new(NamespaceUseClauseNode::parse(node, source)?),
            ),
            "namespace_use_group" => NamespaceUseDeclarationChildren::NamespaceUseGroup(Box::new(
                NamespaceUseGroupNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            NamespaceUseDeclarationChildren::Extra(y) => y.kind(),
            NamespaceUseDeclarationChildren::NamespaceName(y) => y.kind(),
            NamespaceUseDeclarationChildren::NamespaceUseClause(y) => y.kind(),
            NamespaceUseDeclarationChildren::NamespaceUseGroup(y) => y.kind(),
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
            NamespaceUseDeclarationChildren::Extra(x) => x.get_utype(state, emitter),
            NamespaceUseDeclarationChildren::NamespaceName(x) => x.get_utype(state, emitter),
            NamespaceUseDeclarationChildren::NamespaceUseClause(x) => x.get_utype(state, emitter),
            NamespaceUseDeclarationChildren::NamespaceUseGroup(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            NamespaceUseDeclarationChildren::Extra(x) => x.get_php_value(state, emitter),
            NamespaceUseDeclarationChildren::NamespaceName(x) => x.get_php_value(state, emitter),
            NamespaceUseDeclarationChildren::NamespaceUseClause(x) => {
                x.get_php_value(state, emitter)
            }
            NamespaceUseDeclarationChildren::NamespaceUseGroup(x) => {
                x.get_php_value(state, emitter)
            }
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            NamespaceUseDeclarationChildren::Extra(x) => x.read_from(state, emitter),
            NamespaceUseDeclarationChildren::NamespaceName(x) => x.read_from(state, emitter),
            NamespaceUseDeclarationChildren::NamespaceUseClause(x) => x.read_from(state, emitter),
            NamespaceUseDeclarationChildren::NamespaceUseGroup(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for NamespaceUseDeclarationChildren {
    fn brief_desc(&self) -> String {
        match self {
            NamespaceUseDeclarationChildren::Extra(x) => {
                format!("NamespaceUseDeclarationChildren::extra({})", x.brief_desc())
            }
            NamespaceUseDeclarationChildren::NamespaceName(x) => format!(
                "NamespaceUseDeclarationChildren::namespace_name({})",
                x.brief_desc()
            ),
            NamespaceUseDeclarationChildren::NamespaceUseClause(x) => format!(
                "NamespaceUseDeclarationChildren::namespace_use_clause({})",
                x.brief_desc()
            ),
            NamespaceUseDeclarationChildren::NamespaceUseGroup(x) => format!(
                "NamespaceUseDeclarationChildren::namespace_use_group({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            NamespaceUseDeclarationChildren::Extra(x) => x.as_any(),
            NamespaceUseDeclarationChildren::NamespaceName(x) => x.as_any(),
            NamespaceUseDeclarationChildren::NamespaceUseClause(x) => x.as_any(),
            NamespaceUseDeclarationChildren::NamespaceUseGroup(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            NamespaceUseDeclarationChildren::Extra(x) => x.children_any(),
            NamespaceUseDeclarationChildren::NamespaceName(x) => x.children_any(),
            NamespaceUseDeclarationChildren::NamespaceUseClause(x) => x.children_any(),
            NamespaceUseDeclarationChildren::NamespaceUseGroup(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            NamespaceUseDeclarationChildren::Extra(x) => x.range(),
            NamespaceUseDeclarationChildren::NamespaceName(x) => x.range(),
            NamespaceUseDeclarationChildren::NamespaceUseClause(x) => x.range(),
            NamespaceUseDeclarationChildren::NamespaceUseGroup(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NamespaceUseDeclarationNode {
    pub range: Range,
    pub children: Vec<Box<NamespaceUseDeclarationChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for NamespaceUseDeclarationNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "namespace_use_declaration" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [namespace_use_declaration] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            children: NamespaceUseDeclarationChildren::parse_vec(
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

impl NamespaceUseDeclarationNode {
    pub fn kind(&self) -> &'static str {
        "namespace_use_declaration"
    }
}

impl NodeAccess for NamespaceUseDeclarationNode {
    fn brief_desc(&self) -> String {
        "NamespaceUseDeclarationNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::NamespaceUseDeclaration(self)
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
