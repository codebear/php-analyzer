use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::property_initializer::PropertyInitializerNode;
use crate::autonodes::variable_name::VariableNameNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use crate::parser::Range;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct PropertyElementNode {
    pub range: Range,
    pub name: VariableNameNode,
    pub child: Option<Box<PropertyInitializerNode>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for PropertyElementNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "property_element" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [property_element] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let mut skip_nodes: Vec<usize> = vec![];
        let name: VariableNameNode = Result::from(
            node.parse_child("name", source)
                .mark_skipped_node(&mut skip_nodes)
                .into(),
        )?;
        Ok(Self {
            range,
            name,
            child: node
                .named_children(&mut node.walk())
                .filter(|node| !skip_nodes.contains(&node.id()))
                .filter(|node| node.kind() != "comment")
                .map(|k| PropertyInitializerNode::parse(k, source))
                .collect::<Result<Vec<PropertyInitializerNode>, ParseError>>()?
                .drain(..)
                .map(|j| Box::new(j))
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

impl PropertyElementNode {
    pub fn kind(&self) -> &'static str {
        "property_element"
    }
}

impl NodeAccess for PropertyElementNode {
    fn brief_desc(&self) -> String {
        "PropertyElementNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::PropertyElement(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
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
