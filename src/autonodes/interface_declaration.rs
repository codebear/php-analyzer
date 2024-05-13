use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::base_clause::BaseClauseNode;
use crate::autonodes::declaration_list::DeclarationListNode;
use crate::autonodes::name::NameNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use crate::parser::Range;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct InterfaceDeclarationNode {
    pub range: Range,
    pub body: DeclarationListNode,
    pub name: NameNode,
    pub child: Option<Box<BaseClauseNode>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for InterfaceDeclarationNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "interface_declaration" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [interface_declaration] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let mut skip_nodes: Vec<usize> = vec![];
        let body: DeclarationListNode = Into::<Result<_, _>>::into(
            node.parse_child("body", source)
                .mark_skipped_node(&mut skip_nodes),
        )?;
        let name: NameNode = Into::<Result<_, _>>::into(
            node.parse_child("name", source)
                .mark_skipped_node(&mut skip_nodes),
        )?;
        Ok(Self {
            range,
            body,
            name,
            child: node
                .named_children(&mut node.walk())
                .filter(|node| !skip_nodes.contains(&node.id()))
                .filter(|node| node.kind() != "comment")
                .map(|k| BaseClauseNode::parse(k, source))
                .collect::<Result<Vec<BaseClauseNode>, ParseError>>()?
                .drain(..)
                .map(Box::new)
                .next(),
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment")
                    .filter(|node| !skip_nodes.contains(&node.id())),
                source,
            )?,
        })
    }
}

impl InterfaceDeclarationNode {
    pub fn kind(&self) -> &'static str {
        "interface_declaration"
    }
}

impl NodeAccess for InterfaceDeclarationNode {
    fn brief_desc(&self) -> String {
        "InterfaceDeclarationNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::InterfaceDeclaration(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.body.as_any());
        child_vec.push(self.name.as_any());
        if let Some(x) = &self.child {
            child_vec.push(x.as_any());
        }
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
