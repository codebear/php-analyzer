use crate::autotree::{AutoTree, ParseError};
// use crate::wrapped_tree::WrappedTree;
use tree_sitter::Tree;
use tree_sitter::{Language, Parser};

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
        parser.set_language(language).unwrap();
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
