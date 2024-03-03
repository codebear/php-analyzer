use crate::autonodes::any::AnyNodeRef;

use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::os::unix::ffi::OsStrExt;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub struct FinalModifierNode {
    pub range: Range,
    pub raw: Vec<u8>,
}

impl NodeParser for FinalModifierNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "final_modifier" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [final_modifier] on pos {}:{}",
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

impl FinalModifierNode {
    pub fn kind(&self) -> &'static str {
        "final_modifier"
    }

    pub fn get_raw(&self) -> OsString {
        OsStr::from_bytes(&self.raw).to_os_string()
    }
}

impl NodeAccess for FinalModifierNode {
    fn brief_desc(&self) -> String {
        "FinalModifierNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::FinalModifier(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        vec![]
    }

    fn range(&self) -> Range {
        self.range
    }
}
