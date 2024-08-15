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
pub struct VariadicPlaceholderNode {
    pub range: Range,
    pub raw: Vec<u8>,
}

impl NodeParser for VariadicPlaceholderNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "variadic_placeholder" {
            return Err(ParseError::new(range, format!("VariadicPlaceholderNode: Node is of the wrong kind [{}] vs expected [variadic_placeholder] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            raw: source[range.start_byte..range.end_byte].to_vec(),
        })
    }
}

impl VariadicPlaceholderNode {
    pub fn kind(&self) -> &'static str {
        "variadic_placeholder"
    }

    pub fn get_raw(&self) -> OsString {
        OsStr::from_bytes(&self.raw).to_os_string()
    }
}

impl NodeAccess for VariadicPlaceholderNode {
    fn brief_desc(&self) -> String {
        "VariadicPlaceholderNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::VariadicPlaceholder(self)
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        vec![]
    }

    fn range(&self) -> Range {
        self.range
    }
}
