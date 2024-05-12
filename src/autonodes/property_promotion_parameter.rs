use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::_type::_TypeNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::attribute_list::AttributeListNode;
use crate::autonodes::readonly_modifier::ReadonlyModifierNode;
use crate::autonodes::variable_name::VariableNameNode;
use crate::autonodes::visibility_modifier::VisibilityModifierNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use crate::parser::Range;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct PropertyPromotionParameterNode {
    pub range: Range,
    pub attributes: Option<AttributeListNode>,
    pub default_value: Option<_ExpressionNode>,
    pub name: VariableNameNode,
    pub readonly: Option<ReadonlyModifierNode>,
    pub type_: Option<_TypeNode>,
    pub visibility: VisibilityModifierNode,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for PropertyPromotionParameterNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "property_promotion_parameter" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [property_promotion_parameter] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let attributes: Option<AttributeListNode> =
            Result::from(node.parse_child("attributes", source).into())?;
        let default_value: Option<_ExpressionNode> =
            Result::from(node.parse_child("default_value", source).into())?;
        let name: VariableNameNode = Result::from(node.parse_child("name", source).into())?;
        let readonly: Option<ReadonlyModifierNode> =
            Result::from(node.parse_child("readonly", source).into())?;
        let type_: Option<_TypeNode> = Result::from(node.parse_child("type", source).into())?;
        let visibility: VisibilityModifierNode =
            Result::from(node.parse_child("visibility", source).into())?;
        Ok(Self {
            range,
            attributes,
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
}

impl PropertyPromotionParameterNode {
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
        if let Some(x) = &self.attributes {
            child_vec.push(x.as_any());
        }
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
