use crate::autotree::{AutoTree, ParseError};
// use crate::wrapped_tree::WrappedTree;
use tree_sitter::{Language, Parser};
use tree_sitter::{Point, Tree};

#[link(name = "c++")]
extern "C" {
    fn tree_sitter_php() -> Language;
}

pub struct PHPParser {
    parser: Parser,
}

impl PHPParser {
    pub fn new() -> Self {
        let mut parser = Parser::new();

        let language = unsafe { tree_sitter_php() };
        parser.set_language(&language).unwrap();
        PHPParser { parser: parser }
    }

    pub fn parse(&mut self, text: impl AsRef<[u8]>, old_tree: Option<&Tree>) -> Option<Tree> {
        self.parser.parse(text, old_tree)
    }

    pub fn parse_struct(&mut self, text: Vec<u8>) -> Result<Option<AutoTree>, ParseError> {
        Ok(if let Some(tree) = self.parser.parse(&text, None) {
            Some(AutoTree::new(tree, text)?)
        } else {
            None
        })
    }
}

use tree_sitter::Range as TSRange;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, Hash)]
pub struct Range {
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_point: Point,
    pub end_point: Point,
}

impl From<TSRange> for Range {
    fn from(r: TSRange) -> Self {
        Self {
            start_byte: r.start_byte,
            end_byte: r.end_byte,
            start_point: r.start_point,
            end_point: r.end_point,
        }
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.start_byte.cmp(&other.start_byte))
    }
}
