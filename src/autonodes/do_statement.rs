use crate::autonodes::_statement::_StatementNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::parenthesized_expression::ParenthesizedExpressionNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub struct DoStatementNode {
    pub range: Range,
    pub body: _StatementNode,
    pub condition: ParenthesizedExpressionNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for DoStatementNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "do_statement" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [do_statement] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let body: _StatementNode = Result::from(node.parse_child("body", source).into())?;
        let condition: ParenthesizedExpressionNode =
            Result::from(node.parse_child("condition", source).into())?;
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

impl DoStatementNode {
    pub fn kind(&self) -> &'static str {
        "do_statement"
    }
}

impl NodeAccess for DoStatementNode {
    fn brief_desc(&self) -> String {
        "DoStatementNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::DoStatement(self)
    }

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
