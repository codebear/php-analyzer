use std::{
    ffi::{OsStr, OsString},
    os::unix::prelude::OsStrExt,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_until},
    character::complete::{alpha1, alphanumeric0, line_ending, space0, space1},
    combinator::opt,
    error::Error,
    multi::{many0, separated_list0},
    sequence::preceded,
    Err, IResult,
};

use crate::types::parser::union_type;

use super::types::PHPDocEntry;

pub fn parse_phpdoc(input: &[u8]) -> IResult<&[u8], Vec<PHPDocEntry>> {
    let (input, _) = tag(b"/**")(input)?;
    let mut end = input.len();

    let last = &input[end - 2..end];

    // Verify string closes with
    tag(b"*/")(last)?;
    end -= 2;

    let subset = &input[0..end];

    let (_, entries) = separated_list0(line_ending, phpdoc_entry)(subset)?;
    Ok((&[], entries))
}

fn phpdoc_entry(input: &[u8]) -> IResult<&[u8], PHPDocEntry> {
    let (input, _) = space0(input)?;
    let (input, _) = many0(tag(b"*"))(input)?;
    let (input, _) = space0(input)?;
    phpdoc_entry_content(input)
}

fn phpdoc_entry_content(input: &[u8]) -> IResult<&[u8], PHPDocEntry> {
    alt((var, desc, param, parse_return, general, anything))(input)
}

fn var(input: &[u8]) -> IResult<&[u8], PHPDocEntry> {
    // https://docs.phpdoc.org/guide/references/phpdoc/tags/var.html
    let (input, _) = tag_no_case(b"@var")(input)?;
    let (input, _) = space1(input)?;
    let (input, tdef) = union_type(input)?;
    let (input, name) = opt(preceded(space1, var_name))(input)?;
    let (input, desc) = opt(preceded(space1, description))(input)?;

    let entry = PHPDocEntry::Var(tdef, name, desc);
    Ok((input, entry))
}

fn var_name(input: &[u8]) -> IResult<&[u8], OsString> {
    // https://docs.phpdoc.org/guide/references/phpdoc/tags/var.html
    let (input, _) = tag(b"$")(input)?;
    let (input, name) = name(input)?;

    let mut var_name = OsString::from("$");
    var_name.push(name);

    Ok((input, var_name))
}

fn until_eol(input: &[u8]) -> IResult<&[u8], OsString> {
    let (input, str) = take_until("\n")(input)?;
    let str: OsString = OsStr::from_bytes(str).into();
    Ok((input, str))
}

fn description(input: &[u8]) -> IResult<&[u8], OsString> {
    let (input, desc) = match until_eol(input) {
        Ok(ok) => ok,
        Err(Err::Error(e)) => {
            let str: OsString = OsStr::from_bytes(e.input).into();
            (&[] as &[u8], str)
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

fn name(input: &[u8]) -> IResult<&[u8], OsString> {
    let (input, part1) = alpha1(input)?;
    let (input, part2) = alphanumeric0(input)?;
    let mut name = OsString::new();
    name.push(OsStr::from_bytes(part1));
    name.push(OsStr::from_bytes(part2));
    Ok((input, name))
}

fn name_or_var_name(input: &[u8]) -> IResult<&[u8], OsString> {
    alt((var_name, name))(input)
}

fn desc(input: &[u8]) -> IResult<&[u8], PHPDocEntry> {
    let (input, _) = tag_no_case(b"@desc")(input)?;
    let (input, _) = space1(input)?;
    let (input, desc) = description(input)?;
    Ok((input, PHPDocEntry::Description(desc)))
}

fn param(input: &[u8]) -> IResult<&[u8], PHPDocEntry> {
    let (input, _) = tag_no_case(b"@param")(input)?;
    let (input, _) = space1(input)?;
    let (input, utype) = union_type(input)?;
    let (input, name) = opt(preceded(space1, name_or_var_name))(input)?;
    let (input, desc) = opt(preceded(space1, description))(input)?;
    Ok((input, PHPDocEntry::Param(utype, name, desc)))
}

fn parse_return(input: &[u8]) -> IResult<&[u8], PHPDocEntry> {
    // https://docs.phpdoc.org/guide/references/phpdoc/tags/return.html
    // @return [type] <description>
    let (input, _) = tag_no_case(b"@return")(input)?;
    let (input, _) = space1(input)?;
    let (input, tdef) = union_type(input)?;
    let (input, desc) = opt(preceded(space1, description))(input)?;

    let entry = PHPDocEntry::Return(tdef, desc);
    Ok((input, entry))
}

fn general(input: &[u8]) -> IResult<&[u8], PHPDocEntry> {
    let (input, _) = tag(b"@")(input)?;
    let (input, name) = name(input)?;
    let (input, param) = opt(preceded(space1, description))(input)?;

    let entry = if let Some(p) = param {
        PHPDocEntry::GeneralWithParam(name, p)
    } else {
        PHPDocEntry::General(name)
    };

    Ok((input, entry))
}

fn anything(input: &[u8]) -> IResult<&[u8], PHPDocEntry> {
    let (input, _) = space0(input)?;
    let (input, rest_of_line) = opt(description)(input)?;

    Ok((
        input,
        if let Some(rest_of_line) = rest_of_line {
            if rest_of_line.len() == 0 {
                PHPDocEntry::EmptyLine
            } else {
                PHPDocEntry::Anything(rest_of_line)
            }
        } else {
            PHPDocEntry::EmptyLine
        },
    ))
}
