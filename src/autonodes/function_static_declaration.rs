use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::static_variable_declaration::StaticVariableDeclarationNode;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use crate::parser::Range;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct FunctionStaticDeclarationNode {
    pub range: Range,
    pub children: Vec<Box<StaticVariableDeclarationNode>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for FunctionStaticDeclarationNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "function_static_declaration" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [function_static_declaration] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            children: StaticVariableDeclarationNode::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() != "comment"),
                source,
            )?,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )?,
        })
    }
}

impl FunctionStaticDeclarationNode {
    pub fn kind(&self) -> &'static str {
        "function_static_declaration"
    }
}

impl NodeAccess for FunctionStaticDeclarationNode {
    fn brief_desc(&self) -> String {
        "FunctionStaticDeclarationNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::FunctionStaticDeclaration(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.extend(self.children.iter().map(|n| n.as_any()));
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
