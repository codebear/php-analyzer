use crate::autonodes::_type::_TypeNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::attribute_list::AttributeListNode;
use crate::autonodes::reference_modifier::ReferenceModifierNode;
use crate::autonodes::variable_name::VariableNameNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use crate::parser::Range;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct VariadicParameterNode {
    pub range: Range,
    pub attributes: Option<AttributeListNode>,
    pub name: VariableNameNode,
    pub reference_modifier: Option<ReferenceModifierNode>,
    pub type_: Option<_TypeNode>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for VariadicParameterNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "variadic_parameter" {
            return Err(ParseError::new(range, format!("VariadicParameterNode: Node is of the wrong kind [{}] vs expected [variadic_parameter] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let attributes: Option<AttributeListNode> =
            Into::<Result<_, _>>::into(node.parse_child("attributes", source))?;
        let name: VariableNameNode = Into::<Result<_, _>>::into(node.parse_child("name", source))?;
        let reference_modifier: Option<ReferenceModifierNode> =
            Into::<Result<_, _>>::into(node.parse_child("reference_modifier", source))?;
        let type_: Option<_TypeNode> =
            Into::<Result<_, _>>::into(node.parse_child("type", source))?;
        Ok(Self {
            range,
            attributes,
            name,
            reference_modifier,
            type_,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl VariadicParameterNode {
    pub fn kind(&self) -> &'static str {
        "variadic_parameter"
    }
}

impl NodeAccess for VariadicParameterNode {
    fn brief_desc(&self) -> String {
        "VariadicParameterNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::VariadicParameter(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.attributes {
            child_vec.push(x.as_any());
        }
        child_vec.push(self.name.as_any());
        if let Some(x) = &self.reference_modifier {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.type_ {
            child_vec.push(x.as_any());
        }

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
