use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub struct SilenceExpressionNode {
    pub range: Range,
    pub expr: _ExpressionNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for SilenceExpressionNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "silence_expression" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [silence_expression] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let expr: _ExpressionNode = Result::from(node.parse_child("expr", source).into())?;
        Ok(Self {
            range,
            expr,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl SilenceExpressionNode {
    pub fn kind(&self) -> &'static str {
        "silence_expression"
    }
}

impl NodeAccess for SilenceExpressionNode {
    fn brief_desc(&self) -> String {
        "SilenceExpressionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::SilenceExpression(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.expr.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
