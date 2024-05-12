use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::heredoc_end::HeredocEndNode;
use crate::autonodes::heredoc_start::HeredocStartNode;
use crate::autonodes::nowdoc_body::NowdocBodyNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use crate::parser::Range;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct NowdocNode {
    pub range: Range,
    pub end_tag: HeredocEndNode,
    pub identifier: HeredocStartNode,
    pub value: Option<NowdocBodyNode>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for NowdocNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "nowdoc" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [nowdoc] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let end_tag: HeredocEndNode = Result::from(node.parse_child("end_tag", source).into())?;
        let identifier: HeredocStartNode =
            Result::from(node.parse_child("identifier", source).into())?;
        let value: Option<NowdocBodyNode> = Result::from(node.parse_child("value", source).into())?;
        Ok(Self {
            range,
            end_tag,
            identifier,
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

impl NowdocNode {
    pub fn kind(&self) -> &'static str {
        "nowdoc"
    }
}

impl NodeAccess for NowdocNode {
    fn brief_desc(&self) -> String {
        "NowdocNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::Nowdoc(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.end_tag.as_any());
        child_vec.push(self.identifier.as_any());
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
