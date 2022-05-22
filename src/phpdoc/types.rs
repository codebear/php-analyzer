use std::{
    ffi::{OsStr, OsString},
    os::unix::prelude::OsStrExt,
};

use nom::Finish;
use tree_sitter::Range;

use crate::{types::{parse_types::UnionOfTypes, union::UnionType}, analysis::state::AnalysisState, issue::IssueEmitter};

use super::phpdoc::parse_phpdoc;
use super::position::PHPDocInput;

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum PHPDocEntry {
    /// https://docs.phpdoc.org/guide/references/phpdoc/tags/var.html
    /// *  .0 type
    /// *  .1 Name
    /// *  .2 Description (The first word of descripton might be misinterpreted as name)
    Var(Range, UnionOfTypes, Option<OsString>, Option<OsString>),

    /// https://docs.phpdoc.org/guide/references/phpdoc/tags/param.html
    /// *  .0 type
    /// *  .1 Name Not actually optional, but declared as such to allow to parse badly declared params
    /// *  .2 Description  
    Param(Range, UnionOfTypes, Option<OsString>, Option<OsString>),

    /// *  .0 type
    /// *  .2 Description (The first word of descripton might be misinterpreted as name)
    Return(Range, UnionOfTypes, Option<OsString>),

    /// https://docs.phpdoc.org/guide/references/phpdoc/tags/desc.html
    Description(Range, OsString),

    /// https://docs.phpdoc.org/guide/references/phpdoc/tags/deprecated.html
    Deprecated(Range, Option<OsString>),

    /// https://docs.phpdoc.org/guide/references/phpdoc/tags/see.html
    See(Range, OsString, Option<OsString>),

    /// https://docs.phpdoc.org/guide/references/phpdoc/tags/template.html
    Template(Range, OsString, Option<OsString>),

    /// https://docs.phpdoc.org/guide/references/phpdoc/tags/author.html
    Author(Range, OsString),

    /// https://docs.phpdoc.org/guide/references/phpdoc/tags/version.html
    Version(Range, OsString),

    /// https://docs.phpdoc.org/guide/references/phpdoc/tags/todo.html
    Todo(Range, Option<OsString>),

    /// https://docs.phpdoc.org/guide/references/phpdoc/tags/abstract.html
    Abstract(Range),

    /// https://docs.phpdoc.org/guide/references/phpdoc/tags/copyright.html
    Copyright(Range, OsString),

    General(Range, OsString),
    GeneralWithParam(Range, OsString, OsString),

    Anything(Range, OsString),
    EmptyLine(Range),
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct PHPDocComment {
    pub raw: OsString,
    pub entries: Vec<PHPDocEntry>,
}

impl PHPDocComment {
    pub fn parse(input: &OsString, range: &Range) -> Result<Self, OsString> {
        // -> IResult<&[u8], Vec<PHPDocEntry>> {
        let parse_result = parse_phpdoc(PHPDocInput(input.as_bytes(), range.clone()))
            .map_err(|e| e.map_input(|i| OsStr::from_bytes(i.0)))
            .finish();
        match parse_result {
            Ok((_remainder, entries)) => Ok(Self {
                // FIXME assert that remainder is empty?
                raw: input.clone(),
                entries,
            }),
            Err(parse_err) => {
                let kind = parse_err.code.description();
                let mut error = OsString::new();
                error.push(kind);
                error.push(": ");
                error.push(parse_err.input);
                Err(error)
                //                todo!("ERR: {:?}", parse_err)
            }
        }
    }

    pub fn parse_inline_phpdoc_entry(buffer: &OsString, range: &Range) -> Option<(OsString, Range)> {
        let mut phpdoc_entries = Self::parse(buffer, range).ok()?;
        if phpdoc_entries.entries.len() != 1 {
            return None;
        }

        let entry = phpdoc_entries.entries.drain(..).next()?;

        if let PHPDocEntry::Anything(range, content) = entry {
            Some((content, range))
        } else {
            None
        }
    }

    pub fn parse_inline_generic(buffer: &OsString, range: &Range, state: &mut AnalysisState, emitter: &dyn IssueEmitter) -> Option<(Vec<Option<UnionType>>, Range)> {
        
        let (content, range) = Self::parse_inline_phpdoc_entry(buffer, range)?;

        let utype = UnionType::parse_generics(content.clone(), range.clone(), state, emitter)?;

        Some((utype, range.clone()))
    }

    pub fn parse_inline_type(buffer: &OsString, range: &Range, state: &mut AnalysisState, emitter: &dyn IssueEmitter) -> Option<(UnionType, Range)> {
        
        let (content, range) = Self::parse_inline_phpdoc_entry(buffer, range)?;

        let utype = UnionType::parse(content.clone(), range.clone(), state, emitter)?;

        Some((utype, range.clone()))
    }

    pub fn parse_inline_return_type(buffer: &OsString, range: &Range, state: &mut AnalysisState, emitter: &dyn IssueEmitter) -> Option<(UnionType, Range)> {
        
        let (content, range) = Self::parse_inline_phpdoc_entry(buffer, range)?;

        let utype = UnionType::parse_with_colon(content.clone(), range.clone(), state, emitter)?;

        Some((utype, range.clone()))
    }
}
