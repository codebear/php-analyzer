use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use crate::parser::Range;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct PropertyInitializerNode {
    pub range: Range,
    pub initializer: _ExpressionNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for PropertyInitializerNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "property_initializer" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [property_initializer] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let initializer: _ExpressionNode =
            Into::<Result<_, _>>::into(node.parse_child("initializer", source))?;
        Ok(Self {
            range,
            initializer,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl PropertyInitializerNode {
    pub fn kind(&self) -> &'static str {
        "property_initializer"
    }
}

impl NodeAccess for PropertyInitializerNode {
    fn brief_desc(&self) -> String {
        "PropertyInitializerNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::PropertyInitializer(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.initializer.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
