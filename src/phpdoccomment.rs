use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    os::unix::prelude::OsStrExt,
};

use tree_sitter::Range;

#[derive(Debug, Clone)]
pub struct PHPDocComment {
    pub raw: OsString,
    pub params: HashMap<OsString, Option<(OsString, Range)>>,
}

impl PHPDocComment {
    pub fn new(raw: OsString, params: HashMap<OsString, Option<(OsString, Range)>>) -> Self {
        Self { raw, params }
    }

    pub fn parse(doc_comment: &OsString, range: Range) -> Option<Self> {
        let dc_regex = regex::bytes::Regex::new(r"(?s)^/\*\*+(.*)\*+/$").unwrap();

        let dc_bytes = doc_comment.as_bytes();

        let dc_bytes = if let Some(m) = dc_regex.captures(&dc_bytes) {
            if let Some(x) = m.get(1) {
                &dc_bytes[x.start()..x.end()]
            } else {
                return None;
            }
        } else {
            return None;
        };

        let params_rx = regex::bytes::Regex::new(r"^[ \t]*\**[ \t]*(@\w+)([ \t]+(.*))?").unwrap();
        let mut params = HashMap::new();

        let trim = regex::bytes::Regex::new(r"^[ \t]*\**[ \t]*(.*?)[ \t]*$").unwrap();
        for line in dc_bytes.split(|x| *x == b'\n').filter_map(|line| {
            let captures = trim.captures(line)?;
            let m = captures.get(1)?;
            let line_match = &line[m.start()..m.end()];
            if line_match.len() > 0 {
                Some(line_match)
            } else {
                None
            }
        }) {
            let mtch = if let Some(m) = params_rx.captures(line) {
                m
            } else {
                continue;
            };
            let mut match_range = range.clone();
            match (mtch.get(1), mtch.get(3)) {
                (Some(prop), Some(val)) => {
                    match_range.start_byte = range.start_byte + val.start();
                    match_range.end_byte = range.start_byte + val.end();
                    let value = OsString::from(OsStr::from_bytes(val.as_bytes()));
                    // FIXME reallign start_point and end_point
                    params.insert(
                        OsString::from(OsStr::from_bytes(prop.as_bytes())).to_ascii_lowercase(),
                        Some((value, match_range)),
                    );
                }
                (Some(prop), None) => {
                    params.insert(
                        OsString::from(OsStr::from_bytes(prop.as_bytes().into()))
                            .to_ascii_lowercase(),
                        None,
                    );
                }
                _ => (),
            }
        }
        Some(Self::new(doc_comment.clone(), params))
    }

    pub fn get_param<T>(&self, key: T) -> Option<(OsString, Range)>
    where
        T: Into<OsString>,
    {
        let str: OsString = key.into();
        self.params.get(&str).cloned()?
    }
}
