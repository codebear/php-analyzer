use std::{
    ffi::{OsStr, OsString},
    os::unix::prelude::OsStrExt,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_until},
    character::complete::{alpha1, alphanumeric0, space0, space1},
    combinator::opt,
    error::Error,
    multi::{many0, separated_list0},
    sequence::preceded,
    Err, IResult, Slice,
};
use tree_sitter::Range;

use crate::types::{parse_types::UnionOfTypes, parser::union_type};

use super::{position::PHPDocInput, types::PHPDocEntry};

fn our_tag<'a>(
    buf: &'a [u8],
) -> impl Fn(PHPDocInput<'a>) -> IResult<PHPDocInput<'a>, PHPDocInput<'a>> {
    tag(buf)
}

fn our_tag_no_case<'a>(
    buf: &'a [u8],
) -> impl Fn(PHPDocInput<'a>) -> IResult<PHPDocInput<'a>, PHPDocInput<'a>> {
    tag_no_case(buf)
}

pub fn parse_phpdoc(input: PHPDocInput) -> IResult<PHPDocInput, Vec<PHPDocEntry>> {
    let input_range = input.1.clone();

    let (input, _) = our_tag(b"/**")(input)?;
    let mut end = input.len();

    let last = input.slice(end - 2..end);

    // Verify string closes with
    our_tag(b"*/")(last)?;
    end -= 2;

    let subset = input.slice(0..end);

    let (_, entries) = separated_list0(our_tag(b"\n"), phpdoc_entry)(subset)?;
    let end_range = Range {
        start_byte: input_range.end_byte,
        end_byte: input_range.end_byte,
        start_point: input_range.end_point.clone(),
        end_point: input_range.end_point.clone(),
    };
    Ok((PHPDocInput(&[], end_range), entries))
}

fn phpdoc_entry(input: PHPDocInput) -> IResult<PHPDocInput, PHPDocEntry> {
    let (input, _) = space0(input)?;
    let (input, _) = many0(our_tag(b"*"))(input)?;
    let (input, _) = space0(input)?;
    phpdoc_entry_content(input)
}

fn phpdoc_entry_content(input: PHPDocInput) -> IResult<PHPDocInput, PHPDocEntry> {
    alt((var, desc, param, parse_return, general, anything))(input)
}

fn var(input: PHPDocInput) -> IResult<PHPDocInput, PHPDocEntry> {
    // https://docs.phpdoc.org/guide/references/phpdoc/tags/var.html
    let start_range = input.1;
    let (input, _) = our_tag_no_case(b"@var")(input)?;
    let (input, _) = space1(input)?;
    let (input, tdef) = our_union_type(input)?;
    let (input, name) = opt(preceded(space1, var_name))(input)?;
    let (input, desc) = opt(preceded(space1, description))(input)?;
    let range = from_until_ranges(start_range, input.1);
    let entry = PHPDocEntry::Var(range, tdef, name, desc);
    Ok((input, entry))
}

fn var_name(input: PHPDocInput) -> IResult<PHPDocInput, OsString> {
    // https://docs.phpdoc.org/guide/references/phpdoc/tags/var.html
    let (input, _) = our_tag(b"$")(input)?;
    let (input, name) = name(input)?;

    let mut var_name = OsString::from("$");
    var_name.push(name);

    Ok((input, var_name))
}

fn until_eol(input: PHPDocInput) -> IResult<PHPDocInput, OsString> {
    let (input, str) = take_until("\n")(input)?;
    let str: OsString = OsStr::from_bytes(str.0).into();
    Ok((input, str))
}

fn description(input: PHPDocInput) -> IResult<PHPDocInput, OsString> {
    let (input, desc) = match until_eol(input) {
        Ok(ok) => ok,
        Err(Err::Error(e)) => {
            let str: OsString = OsStr::from_bytes(e.input.0).into();
            (PHPDocInput(&[] as &[u8], e.input.1), str)
        }
        Err(e) => return Err(e),
    };
    // FIXME maybe this should exclude a potential \r in front of the \n
    if desc.len() > 0 {
        Ok((input, desc))
    } else {
        Err(Err::Error(Error {
            input,
            code: nom::error::ErrorKind::IsNot,
        }))
    }
}

fn name(input: PHPDocInput) -> IResult<PHPDocInput, OsString> {
    let (input, part1) = alpha1(input)?;
    let (input, part2) = alphanumeric0(input)?;
    let mut name = OsString::new();
    name.push(OsStr::from_bytes(part1.0));
    name.push(OsStr::from_bytes(part2.0));
    Ok((input, name))
}

fn name_or_var_name(input: PHPDocInput) -> IResult<PHPDocInput, OsString> {
    alt((var_name, name))(input)
}

fn desc(input: PHPDocInput) -> IResult<PHPDocInput, PHPDocEntry> {
    let start_range = input.1;
    let (input, _) = our_tag_no_case(b"@desc")(input)?;
    let (input, _) = space1(input)?;
    let (input, desc) = description(input)?;
    let range = from_until_ranges(start_range, input.1);
    Ok((input, PHPDocEntry::Description(range, desc)))
}

fn param(input: PHPDocInput) -> IResult<PHPDocInput, PHPDocEntry> {
    let start_range = input.1;
    let (input, _) = our_tag_no_case(b"@param")(input)?;
    let (input, _) = space1(input)?;
    let (input, utype) = our_union_type(input)?;
    let (input, name) = opt(preceded(space1, name_or_var_name))(input)?;
    let (input, desc) = opt(preceded(space1, description))(input)?;
    let range = from_until_ranges(start_range, input.1);
    Ok((input, PHPDocEntry::Param(range, utype, name, desc)))
}

pub fn our_union_type(input: PHPDocInput) -> IResult<PHPDocInput, UnionOfTypes> {
    let pre_length = input.0.len();
    let pre_range = input.1.clone();
    match union_type(false)(input.0) {
        Ok((remainder, utype)) => {
            let post_length = remainder.len();
            let consumed = pre_length - post_length;

            let mut range = input.1.clone();
            // We're using the union_type-parser in single-line-mode, therefore we shouldn't
            // have experienced any line changes. We're only realigning intraline
            range.start_byte += consumed;
            range.start_point.column += consumed;
            Ok((PHPDocInput(remainder, range), utype))
        }
        Err(e) => Err(e.map_input(|input| {
            assert!(pre_length == input.len());
            PHPDocInput(input, pre_range)
        })),
    }
}

fn parse_return(input: PHPDocInput) -> IResult<PHPDocInput, PHPDocEntry> {
    // https://docs.phpdoc.org/guide/references/phpdoc/tags/return.html
    // @return [type] <description>
    let start_range = input.1;
    let (input, _) = our_tag_no_case(b"@return")(input)?;
    let (input, _) = space1(input)?;
    let (input, tdef) = our_union_type(input)?;
    let (input, desc) = opt(preceded(space1, description))(input)?;
    let end_range = input.1;
    let range = from_until_ranges(start_range, end_range);

    let entry = PHPDocEntry::Return(range, tdef, desc);
    Ok((input, entry))
}

fn general(input: PHPDocInput) -> IResult<PHPDocInput, PHPDocEntry> {
    let start_range = input.1;
    let (input, _) = our_tag(b"@")(input)?;
    let (input, name) = name(input)?;
    let (input, param) = opt(preceded(space1, description))(input)?;
    let end_range = input.1;
    let range = from_until_ranges(start_range, end_range);
    let entry = if let Some(p) = param {
        PHPDocEntry::GeneralWithParam(range, name, p)
    } else {
        PHPDocEntry::General(range, name)
    };

    Ok((input, entry))
}

fn from_until_ranges(a: Range, b: Range) -> Range {
    assert!(a.start_byte <= b.start_byte);
    Range {
        start_byte: a.start_byte,
        start_point: a.start_point,
        end_byte: b.start_byte,
        end_point: b.start_point,
    }
}

fn anything(input: PHPDocInput) -> IResult<PHPDocInput, PHPDocEntry> {
    let start_range = input.1;
    let (input, _) = space0(input)?;
    let (input, rest_of_line) = opt(description)(input)?;
    let end_range = input.1;
    let range = from_until_ranges(start_range, end_range);
    Ok((
        input,
        if let Some(rest_of_line) = rest_of_line {
            if rest_of_line.len() == 0 {
                PHPDocEntry::EmptyLine(range)
            } else {
                PHPDocEntry::Anything(range, rest_of_line)
            }
        } else {
            PHPDocEntry::EmptyLine(range)
        },
    ))
}
