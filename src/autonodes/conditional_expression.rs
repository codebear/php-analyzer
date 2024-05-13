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
pub struct ConditionalExpressionNode {
    pub range: Range,
    pub alternative: _ExpressionNode,
    pub body: Option<_ExpressionNode>,
    pub condition: _ExpressionNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for ConditionalExpressionNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "conditional_expression" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [conditional_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let alternative: _ExpressionNode =
            Result::from(node.parse_child("alternative", source).into())?;
        let body: Option<_ExpressionNode> = Result::from(node.parse_child("body", source).into())?;
        let condition: _ExpressionNode =
            Result::from(node.parse_child("condition", source).into())?;
        Ok(Self {
            range,
            alternative,
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

impl ConditionalExpressionNode {
    pub fn kind(&self) -> &'static str {
        "conditional_expression"
    }
}

impl NodeAccess for ConditionalExpressionNode {
    fn brief_desc(&self) -> String {
        "ConditionalExpressionNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::ConditionalExpression(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.alternative.as_any());
        if let Some(x) = &self.body {
            child_vec.push(x.as_any());
        }
        child_vec.push(self.condition.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
