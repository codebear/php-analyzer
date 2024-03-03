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
pub struct PhpTagNode {
    pub range: Range,
    pub raw: Vec<u8>,
}

impl NodeParser for PhpTagNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "php_tag" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [php_tag] on pos {}:{}",
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

impl PhpTagNode {
    pub fn kind(&self) -> &'static str {
        "php_tag"
    }

    pub fn get_raw(&self) -> OsString {
        OsStr::from_bytes(&self.raw).to_os_string()
    }
}

impl NodeAccess for PhpTagNode {
    fn brief_desc(&self) -> String {
        "PhpTagNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::PhpTag(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        vec![]
    }

    fn range(&self) -> Range {
        self.range
    }
}
