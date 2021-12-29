use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::compound_statement::CompoundStatementNode;
use crate::autonodes::namespace_name::NamespaceNameNode;
use crate::autotree::NodeAccess;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub struct NamespaceDefinitionNode {
    pub range: Range,
    pub body: Option<CompoundStatementNode>,
    pub name: Option<NamespaceNameNode>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NamespaceDefinitionNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "namespace_definition" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [namespace_definition] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let body: Option<CompoundStatementNode> = node
            .children_by_field_name("body", &mut node.walk())
            .map(|chnode1| CompoundStatementNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        let name: Option<NamespaceNameNode> = node
            .children_by_field_name("name", &mut node.walk())
            .map(|chnode1| NamespaceNameNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        Ok(Self {
            range,
            body,
            name,
            extras: vec![], // todo lookup unused nodes
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
