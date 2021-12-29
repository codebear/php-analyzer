use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autotree::NodeAccess;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub struct ConditionalExpressionNode {
    pub range: Range,
    pub alternative: _ExpressionNode,
    pub body: Option<_ExpressionNode>,
    pub condition: _ExpressionNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl ConditionalExpressionNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "conditional_expression" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [conditional_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let alternative: _ExpressionNode = node
            .children_by_field_name("alternative", &mut node.walk())
            .map(|chnode1| _ExpressionNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field alternative should exist");
        let body: Option<_ExpressionNode> = node
            .children_by_field_name("body", &mut node.walk())
            .map(|chnode1| _ExpressionNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        let condition: _ExpressionNode = node
            .children_by_field_name("condition", &mut node.walk())
            .map(|chnode1| _ExpressionNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field condition should exist");
        Ok(Self {
            range,
            alternative,
            body,
            condition,
            extras: vec![], // todo lookup unused nodes
        })
    }

    pub fn parse_vec<'a, I>(children: I, source: &Vec<u8>) -> Result<Vec<Box<Self>>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
    {
        let mut res: Vec<Box<Self>> = vec![];
        for child in children {
            if child.kind() == "comment" {
                continue;
            }
            res.push(Box::new(Self::parse(child, source)?));
        }
        Ok(res)
    }

    pub fn kind(&self) -> &'static str {
        "conditional_expression"
    }
}

impl NodeAccess for ConditionalExpressionNode {
    fn brief_desc(&self) -> String {
        "ConditionalExpressionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
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
