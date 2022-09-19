use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::heredoc_end::HeredocEndNode;
use crate::autonodes::heredoc_start::HeredocStartNode;
use crate::autonodes::nowdoc_body::NowdocBodyNode;
use crate::autotree::NodeAccess;
use crate::autotree::ParseError;
use crate::extra::ExtraChild;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub struct NowdocNode {
    pub range: Range,
    pub end_tag: HeredocEndNode,
    pub identifier: HeredocStartNode,
    pub value: Option<NowdocBodyNode>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NowdocNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
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
        let end_tag: HeredocEndNode = node
            .children_by_field_name("end_tag", &mut node.walk())
            .map(|chnode1| HeredocEndNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field end_tag should exist");
        let identifier: HeredocStartNode = node
            .children_by_field_name("identifier", &mut node.walk())
            .map(|chnode1| HeredocStartNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next()
            .expect("Field identifier should exist");
        let value: Option<NowdocBodyNode> = node
            .children_by_field_name("value", &mut node.walk())
            .map(|chnode1| NowdocBodyNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
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
