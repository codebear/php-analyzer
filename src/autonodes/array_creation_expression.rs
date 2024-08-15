use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::array_element_initializer::ArrayElementInitializerNode;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use crate::parser::Range;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct ArrayCreationExpressionNode {
    pub range: Range,
    pub children: Vec<Box<ArrayElementInitializerNode>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for ArrayCreationExpressionNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "array_creation_expression" {
            return Err(ParseError::new(range, format!("ArrayCreationExpressionNode: Node is of the wrong kind [{}] vs expected [array_creation_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            children: ArrayElementInitializerNode::parse_vec(
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

impl ArrayCreationExpressionNode {
    pub fn kind(&self) -> &'static str {
        "array_creation_expression"
    }
}

impl NodeAccess for ArrayCreationExpressionNode {
    fn brief_desc(&self) -> String {
        "ArrayCreationExpressionNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::ArrayCreationExpression(self)
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
