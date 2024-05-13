use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::parenthesized_expression::ParenthesizedExpressionNode;
use crate::autonodes::switch_block::SwitchBlockNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use crate::parser::Range;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct SwitchStatementNode {
    pub range: Range,
    pub body: SwitchBlockNode,
    pub condition: ParenthesizedExpressionNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for SwitchStatementNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "switch_statement" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [switch_statement] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let body: SwitchBlockNode = Into::<Result<_, _>>::into(node.parse_child("body", source))?;
        let condition: ParenthesizedExpressionNode =
            Into::<Result<_, _>>::into(node.parse_child("condition", source))?;
        Ok(Self {
            range,
            body,
            condition,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl SwitchStatementNode {
    pub fn kind(&self) -> &'static str {
        "switch_statement"
    }
}

impl NodeAccess for SwitchStatementNode {
    fn brief_desc(&self) -> String {
        "SwitchStatementNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::SwitchStatement(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.body.as_any());
        child_vec.push(self.condition.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
