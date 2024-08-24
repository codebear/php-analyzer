use super::union::{from_vec_parsed_type, PHPType};
use std::{
    ffi::{OsStr, OsString},
    os::unix::prelude::OsStrExt,
};

use nom::error::Error;
//use tree_sitter::Range;

use crate::{
    analysis::state::AnalysisState,
    issue::{Issue, IssueEmitter, VoidEmitter},
    parser::Range,
    phpdoc::position::fake_range,
    symbols::Name,
};

use super::{
    parse_types::CompoundType,
    parser::{compound_type, compound_type_with_colon, only_generic_args},
};

pub struct TypeParser {}

impl TypeParser {
    pub fn parse_with_colon(
        type_str: OsString,
        range: Range,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPType> {
        let parse_result = compound_type_with_colon(true)(type_str.as_bytes());
        let (utype, remainder) =
            Self::handle_parse_result(type_str.clone(), parse_result, state, emitter);
        Self::handle_remainder(utype, remainder, state, emitter, range)
    }

    fn handle_parse_result(
        type_str: OsString,
        parse_result: Result<(&[u8], CompoundType), nom::Err<Error<&[u8]>>>,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> (Option<PHPType>, Option<OsString>) {
        let (rest, parsed_type) = if let Ok((rest, parsed_type)) = parse_result {
            (rest, parsed_type)
        } else {
            return (None, Some(type_str.clone()));
        };

        let remainder = if !rest.is_empty() {
            let rest_str: OsString = OsStr::from_bytes(rest).into();
            Some(rest_str)
        } else {
            None
        };
        let found_types = if let Some(utype) =
            from_vec_parsed_type(parsed_type.clone(), state, Some(emitter), None)
        {
            Some(utype)
        } else {
            eprintln!(
                "Parsing of type: {:?} failed, parsed into: {:?}",
                type_str, parsed_type
            );
            None
        };

        (found_types, remainder)
    }

    pub fn from_parsed_type(
        parsed_type: CompoundType,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        temp_generics: Option<&Vec<Name>>,
    ) -> Option<PHPType> {
        from_vec_parsed_type(parsed_type, state, Some(emitter), temp_generics)
    }

    pub fn parse_with_remainder(
        type_str: OsString,
        _range: Range,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> (Option<PHPType>, Option<OsString>) {
        let parse_result = compound_type(true)(type_str.as_bytes());

        Self::handle_parse_result(type_str.clone(), parse_result, state, emitter)
    }

    fn handle_parse_vec_result(
        type_str: OsString,
        parse_result: Result<(&[u8], Vec<CompoundType>), nom::Err<Error<&[u8]>>>,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        temp_generics: Option<&Vec<Name>>,
    ) -> (Option<Vec<Option<PHPType>>>, Option<OsString>) {
        let (rest, parsed_types) = if let Ok((rest, parsed_type)) = parse_result {
            (rest, parsed_type)
        } else {
            return (None, Some(type_str.clone()));
        };

        let remainder = if !rest.is_empty() {
            let rest_str: OsString = OsStr::from_bytes(rest).into();
            Some(rest_str)
        } else {
            None
        };

        let mut generics = vec![];

        for parsed_type in &parsed_types {
            let found_types = if let Some(utype) =
                from_vec_parsed_type(parsed_type.clone(), state, Some(emitter), temp_generics)
            {
                Some(utype)
            } else {
                eprintln!(
                    "Parsing of type: {:?} failed, parsed into: {:?}",
                    type_str, parsed_type
                );
                None
            };
            generics.push(found_types);
        }

        (Some(generics), remainder)
    }

    pub fn parse_generics(
        type_str: OsString,
        range: Range,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<Vec<Option<PHPType>>> {
        let parse_result = only_generic_args(true)(type_str.as_bytes());
        let (utype, remainder) =
            Self::handle_parse_vec_result(type_str.clone(), parse_result, state, emitter, None);
        Self::handle_remainder(utype, remainder, state, emitter, range)
    }

    pub fn parse_simple(type_str: OsString) -> Option<PHPType> {
        let range = fake_range(&type_str);
        let emitter = VoidEmitter::new();
        let mut state = AnalysisState::new();

        let (utype, remainder) =
            Self::parse_with_remainder(type_str.clone(), range, &mut state, &emitter);

        Self::handle_remainder(utype, remainder, &mut state, &emitter, range)
    }

    pub fn parse(
        type_str: OsString,
        range: Range,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPType> {
        let (utype, remainder) =
            Self::parse_with_remainder(type_str.clone(), range, state, emitter);

        Self::handle_remainder(utype, remainder, state, emitter, range)
    }

    fn handle_remainder<T>(
        utype: Option<T>,
        remainder: Option<OsString>,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        range: Range,
    ) -> Option<T> {
        let Some(rest) = remainder else {
            return utype;
        };
        if rest.is_empty() {
            return utype;
        }

        for ch in rest.as_bytes() {
            match ch {
                b' ' | b'\t' => (),
                _ => {
                    emitter.emit(Issue::PHPDocTypeError(
                        state.pos_from_range(range),
                        format!("Remainder from parsing: {:?}", rest),
                    ));
                    return None;
                }
            }
        }

        utype
    }
}
