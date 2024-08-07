use std::ffi::OsString;
use std::os::unix::prelude::OsStrExt;

use crate::autonodes::any::AnyNodeRef;
use crate::autotree::AutoTree;
use crate::autotree::NodeAccess;
use crate::config::PHPAnalyzeConfig;
use crate::issue::IssueEmitter;
use crate::nodeanalysis::analysis::ErrorPassAnalyzableNode;
use crate::nodeanalysis::analysis::FirstPassAnalyzeableNode;
use crate::nodeanalysis::analysis::SecondPassAnalyzeableNode;
use crate::nodeanalysis::analysis::ThirdPassAnalyzeableNode;

use super::state::AnalysisState;

pub struct Analyzer {
    config: PHPAnalyzeConfig,
    get_content: Box<dyn FnMut() -> std::io::Result<Vec<u8>> + Send + Sync>,
    content_id: OsString,
    // file: PHPFile,
    tree: Option<AutoTree>,
}

impl Analyzer {
    pub fn new(
        config: PHPAnalyzeConfig,
        content_provider: Box<dyn FnMut() -> std::io::Result<Vec<u8>> + Send + Sync>,
        content_id: OsString,
    ) -> Analyzer {
        Self {
            config,
            get_content: content_provider,
            content_id,
            tree: None,
        }
    }

    pub fn new_from_buffer(
        config: PHPAnalyzeConfig,
        buffer: OsString,
        ident: Option<OsString>,
    ) -> Self {
        Self {
            config,
            get_content: Box::new(move || Ok(Vec::from(buffer.as_bytes()))),
            content_id: ident.unwrap_or_default(),
            tree: None,
        }
    }

    pub fn parse(&mut self) -> Result<(), &'static str> {
        use crate::parser::PHPParser;
        let mut parser = PHPParser::new();

        let contents = (self.get_content)().expect("Something went wrong reading the file content");

        let stru = match parser.parse_struct(contents.clone()) {
            Ok(Some(stru)) => stru,
            Err(err) => {
                eprintln!(
                    "ERROR: {}:{}:{}: {}",
                    self.content_id.to_string_lossy(),
                    err.range.start_point.row,
                    err.range.start_point.column,
                    err.error
                );
                return Err("TODO Trouble with something");
            }
            Ok(None) => return Err("TODO Trouble with something else"),
        };

        self.tree = Some(stru);

        Ok(())
    }

    pub fn with_node_ref_at_position<T, CB>(
        &self,
        line: usize,
        character: usize,
        cb: &mut CB,
    ) -> Option<T>
    where
        CB: FnMut(AnyNodeRef) -> T,
    {
        let tree = match &self.tree {
            Some(t) => t,
            None => return None,
        };
        tree.root.with_node_ref_at_position(line, character, cb)
    }

    pub fn with_node_ref_path_at_position<T, CB>(
        &self,
        line: usize,
        character: usize,
        cb: &mut CB,
    ) -> Option<T>
    where
        CB: FnMut(&Vec<AnyNodeRef>) -> T,
    {
        let tree = match &self.tree {
            Some(t) => t,
            None => return None,
        };
        tree.root
            .with_node_ref_path_at_position(line, character, cb)
    }

    pub fn first_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        if let Some(tree) = &self.tree {
            let any_root = tree.root.as_any();
            any_root.analyze_errors(&any_root, state, emitter);
            any_root.analyze_first_pass(state, emitter);
        }
    }

    pub fn second_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        if let Some(tree) = &self.tree {
            tree.root.as_any().analyze_second_pass(state, emitter);
        }
    }

    pub fn third_pass(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        if let Some(tree) = &self.tree {
            let path = vec![];
            tree.root.as_any().analyze_third_pass(state, emitter, &path);
        }
    }

    pub fn dump(&self) {
        if let Some(tree) = &self.tree {
            tree.debug_dump();
        } else {
            eprintln!("FAILED PARSING BUFFER DATA");
        }
    }
}
