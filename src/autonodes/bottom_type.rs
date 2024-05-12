use crate::autonodes::any::AnyNodeRef;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::parser::Range;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::os::unix::ffi::OsStrExt;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct BottomTypeNode {
    pub range: Range,
    pub raw: Vec<u8>,
}

impl NodeParser for BottomTypeNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "bottom_type" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [bottom_type] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            raw: source[range.start_byte..range.end_byte].to_vec(),
        })
    }
}

impl BottomTypeNode {
    pub fn kind(&self) -> &'static str {
        "bottom_type"
    }

    pub fn get_raw(&self) -> OsString {
        OsStr::from_bytes(&self.raw).to_os_string()
    }
}

impl NodeAccess for BottomTypeNode {
    fn brief_desc(&self) -> String {
        "BottomTypeNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::BottomType(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        vec![]
    }

    fn range(&self) -> Range {
        self.range
    }
}
