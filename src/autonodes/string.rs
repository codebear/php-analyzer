use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::string_value::StringValueNode;

use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub struct StringNode {
    pub range: Range,
    pub child: Box<StringValueNode>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for StringNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "string" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [string] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            child: node
                .named_children(&mut node.walk())
                .filter(|node| node.kind() != "comment")
                .map(|k| StringValueNode::parse(k, source))
                .collect::<Result<Vec<StringValueNode>, ParseError>>()?
                .drain(..)
                .map(|j| Box::new(j))
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

impl StringNode {
    pub fn kind(&self) -> &'static str {
        "string"
    }
}

impl NodeAccess for StringNode {
    fn brief_desc(&self) -> String {
        "StringNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::String(self)
    }

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
