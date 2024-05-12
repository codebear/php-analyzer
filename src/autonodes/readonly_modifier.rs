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
pub struct ReadonlyModifierNode {
    pub range: Range,
    pub raw: Vec<u8>,
}

impl NodeParser for ReadonlyModifierNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "readonly_modifier" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [readonly_modifier] on pos {}:{}",
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

impl ReadonlyModifierNode {
    pub fn kind(&self) -> &'static str {
        "readonly_modifier"
    }

    pub fn get_raw(&self) -> OsString {
        OsStr::from_bytes(&self.raw).to_os_string()
    }
}

impl NodeAccess for ReadonlyModifierNode {
    fn brief_desc(&self) -> String {
        "ReadonlyModifierNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::ReadonlyModifier(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        vec![]
    }

    fn range(&self) -> Range {
        self.range
    }
}
