use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::variable_name::VariableNameNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use crate::parser::Range;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct StaticVariableDeclarationNode {
    pub range: Range,
    pub name: VariableNameNode,
    pub value: Option<_ExpressionNode>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for StaticVariableDeclarationNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "static_variable_declaration" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [static_variable_declaration] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let name: VariableNameNode = Into::<Result<_, _>>::into(node.parse_child("name", source))?;
        let value: Option<_ExpressionNode> =
            Into::<Result<_, _>>::into(node.parse_child("value", source))?;
        Ok(Self {
            range,
            name,
            value,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl StaticVariableDeclarationNode {
    pub fn kind(&self) -> &'static str {
        "static_variable_declaration"
    }
}

impl NodeAccess for StaticVariableDeclarationNode {
    fn brief_desc(&self) -> String {
        "StaticVariableDeclarationNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::StaticVariableDeclaration(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.name.as_any());
        if let Some(x) = &self.value {
            child_vec.push(x.as_any());
        }

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
