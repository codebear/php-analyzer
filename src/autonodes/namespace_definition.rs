use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::compound_statement::CompoundStatementNode;
use crate::autonodes::namespace_name::NamespaceNameNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use crate::parser::Range;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct NamespaceDefinitionNode {
    pub range: Range,
    pub body: Option<CompoundStatementNode>,
    pub name: Option<NamespaceNameNode>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for NamespaceDefinitionNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "namespace_definition" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [namespace_definition] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let body: Option<CompoundStatementNode> =
            Result::from(node.parse_child("body", source).into())?;
        let name: Option<NamespaceNameNode> =
            Result::from(node.parse_child("name", source).into())?;
        Ok(Self {
            range,
            body,
            name,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl NamespaceDefinitionNode {
    pub fn kind(&self) -> &'static str {
        "namespace_definition"
    }
}

impl NodeAccess for NamespaceDefinitionNode {
    fn brief_desc(&self) -> String {
        "NamespaceDefinitionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::NamespaceDefinition(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.body {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.name {
            child_vec.push(x.as_any());
        }

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
