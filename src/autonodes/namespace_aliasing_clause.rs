use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::name::NameNode;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use crate::parser::Range;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct NamespaceAliasingClauseNode {
    pub range: Range,
    pub child: Box<NameNode>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for NamespaceAliasingClauseNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "namespace_aliasing_clause" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [namespace_aliasing_clause] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            child: node
                .named_children(&mut node.walk())
                .filter(|node| node.kind() != "comment")
                .map(|k| NameNode::parse(k, source))
                .collect::<Result<Vec<NameNode>, ParseError>>()?
                .drain(..)
                .map(Box::new)
                .next()
                .expect("Should be a child"),
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )?,
        })
    }
}

impl NamespaceAliasingClauseNode {
    pub fn kind(&self) -> &'static str {
        "namespace_aliasing_clause"
    }
}

impl NodeAccess for NamespaceAliasingClauseNode {
    fn brief_desc(&self) -> String {
        "NamespaceAliasingClauseNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::NamespaceAliasingClause(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.child.as_any());
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
