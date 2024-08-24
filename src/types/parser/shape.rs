use super::compound_type;
use super::nomhelpers::ourspace0;
use super::{basetokens::simple_type_name, nomhelpers::space0_and_separator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::opt,
    error::{Error, ErrorKind},
    multi::separated_list1,
    Err, IResult,
};

use super::super::parse_types::{ParsedType, ShapeEntry, ShapeKey};

pub(super) fn shape_type(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], ParsedType> {
    move |input| {
        let (input, _) = tag(b"array")(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b"{")(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, entries) = separated_list1(
            space0_and_separator(b",", multiline),
            shape_entry(multiline, false),
        )(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b"}")(input)?;
        Ok((input, ParsedType::Shape(entries)))
    }
}

pub(super) fn shape_key_num(input: &[u8]) -> IResult<&[u8], ShapeKey> {
    let (input, digits) = digit1(input)?;

    // digits should only contain ... digits, so this should be perfectly safe
    let str: String = unsafe { String::from_utf8_unchecked(digits.to_vec()) };
    let ival: i64 = if let Ok(i) = str.parse::<i64>() {
        i
    } else {
        return Err(Err::Error(Error {
            input,
            code: ErrorKind::Digit,
        }));
    };

    Ok((input, ShapeKey::Num(ival)))
}

pub(super) fn shape_key_str(input: &[u8]) -> IResult<&[u8], ShapeKey> {
    let (input, str_key) = simple_type_name(input)?;

    Ok((input, ShapeKey::String(str_key)))
}

pub(super) fn shape_key(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], (ShapeKey, bool)> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;

        let (input, key) = alt((shape_key_num, shape_key_str))(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, optional_questionmark) = opt(tag("?"))(input)?;
        let optional = optional_questionmark.is_some();
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b":")(input)?;
        Ok((input, (key, optional)))
    }
}

pub(super) fn shape_entry(
    multiline: bool,
    require_key: bool,
) -> impl Fn(&[u8]) -> IResult<&[u8], ShapeEntry> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, key) = if require_key {
            let (input, key) = shape_key(multiline)(input)?;
            (input, Some(key))
        } else {
            opt(shape_key(multiline))(input)?
        };
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, ctype) = compound_type(multiline)(input)?;
        Ok((input, ShapeEntry(key, ctype)))
    }
}
