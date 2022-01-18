use std::{ffi::{OsString, OsStr}, os::unix::prelude::OsStrExt};

use nom::Finish;
use tree_sitter::Range;

use crate::types::parse_types::UnionOfTypes;

use super::phpdoc::parse_phpdoc;

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum PHPDocEntry {
    /// *  .0 type
    /// *  .1 Name
    /// *  .2 Description (The first word of descripton might be misinterpreted as name)
    Var(UnionOfTypes, Option<OsString>, Option<OsString>),
    /// https://docs.phpdoc.org/guide/references/phpdoc/tags/param.html
    /// *  .0 type
    /// *  .1 Name Not actually optional, but declared as such to allow to parse badly declared params
    /// *  .2 Description  
    Param(UnionOfTypes, Option<OsString>, Option<OsString>),
    /// *  .0 type
    /// *  .2 Description (The first word of descripton might be misinterpreted as name)
    Return(UnionOfTypes, Option<OsString>),
    Description(OsString),
    General(OsString),
    GeneralWithParam(OsString, OsString),

    Anything(OsString),
    EmptyLine,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct PHPDocComment {
    pub raw: OsString,
    pub entries: Vec<PHPDocEntry>,
}

impl PHPDocComment {
    pub fn parse(input: &OsString, _range: &Range) -> Result<Self, OsString> {
        // -> IResult<&[u8], Vec<PHPDocEntry>> {
        let parse_result = parse_phpdoc(input.as_bytes())
            .map_err(|e| e.map_input(|i| OsStr::from_bytes(i)))
            .finish();
        match parse_result {
            Ok((remainder, entries)) => Ok(Self {
                raw: input.clone(),
                entries,
            }),
            Err(parse_err) => {
                todo!("ERR: {:?}", parse_err)
            }
        }
    }
}
