use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::_type::_TypeNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::readonly_modifier::ReadonlyModifierNode;
use crate::autonodes::variable_name::VariableNameNode;
use crate::autonodes::visibility_modifier::VisibilityModifierNode;
use crate::autotree::NodeAccess;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub struct PropertyPromotionParameterNode {
    pub range: Range,
    pub default_value: Option<_ExpressionNode>,
    pub name: VariableNameNode,
    pub readonly: Option<ReadonlyModifierNode>,
    pub type_: Option<_TypeNode>,
    pub visibility: VisibilityModifierNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl PropertyPromotionParameterNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "property_promotion_parameter" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [property_promotion_parameter] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let default_value: Option<_ExpressionNode> = node
            .children_by_field_name("default_value", &mut node.walk())
            .map(|chnode1| _ExpressionNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        let name: VariableNameNode = node
            .children_by_field_name("name", &mut node.walk())
            .map(|chnode1| VariableNameNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field name should exist");
        let readonly: Option<ReadonlyModifierNode> = node
            .children_by_field_name("readonly", &mut node.walk())
            .map(|chnode1| ReadonlyModifierNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        let type_: Option<_TypeNode> = node
            .children_by_field_name("type", &mut node.walk())
            .map(|chnode1| _TypeNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        let visibility: VisibilityModifierNode = node
            .children_by_field_name("visibility", &mut node.walk())
            .map(|chnode1| VisibilityModifierNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field visibility should exist");
        Ok(Self {
            range,
            default_value,
            name,
            readonly,
            type_,
            visibility,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
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
        "property_promotion_parameter"
    }
}

impl NodeAccess for PropertyPromotionParameterNode {
    fn brief_desc(&self) -> String {
        "PropertyPromotionParameterNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::PropertyPromotionParameter(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.default_value {
            child_vec.push(x.as_any());
        }
        child_vec.push(self.name.as_any());
        if let Some(x) = &self.readonly {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.type_ {
            child_vec.push(x.as_any());
        }
        child_vec.push(self.visibility.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
