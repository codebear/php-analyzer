use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use crate::parser::Range;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct ErrorSuppressionExpressionNode {
    pub range: Range,
    pub child: Box<_ExpressionNode>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for ErrorSuppressionExpressionNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "error_suppression_expression" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [error_suppression_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            child: node
                .named_children(&mut node.walk())
                .filter(|node| node.kind() != "comment")
                .map(|k| _ExpressionNode::parse(k, source))
                .collect::<Result<Vec<_ExpressionNode>, ParseError>>()?
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

impl ErrorSuppressionExpressionNode {
    pub fn kind(&self) -> &'static str {
        "error_suppression_expression"
    }
}

impl NodeAccess for ErrorSuppressionExpressionNode {
    fn brief_desc(&self) -> String {
        "ErrorSuppressionExpressionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::ErrorSuppressionExpression(self)
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
