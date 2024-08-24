use super::compound_type;
use super::nomhelpers::space0_and_separator;
use super::{basetokens::php_var_name, concrete_type, nomhelpers::ourspace0};
use nom::{
    bytes::complete::tag,
    combinator::opt,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated},
    IResult,
};

use super::super::parse_types::{ArgumentVector, CompoundType, ParsedType, ReturnType};

pub(super) fn callable_type(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], ParsedType> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b"callable")(input)?;
        let (input, details) = opt(callable_details(multiline))(input)?;
        let ptype = if let Some((types, return_type)) = details {
            ParsedType::Callable(types, return_type.map(Box::new))
        } else {
            ParsedType::CallableUntyped
        };
        Ok((input, ptype))
    }
}

pub(super) fn callable_details(
    multiline: bool,
) -> impl Fn(&[u8]) -> IResult<&[u8], (ArgumentVector, Option<ReturnType>)> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b"(")(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, types) = separated_list1(
            space0_and_separator(b",", multiline),
            terminated(
                compound_type(multiline),
                opt(preceded(ourspace0(multiline), php_var_name)),
            ),
        )(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b")")(input)?;
        let (input, return_type) = opt(callable_return_type(multiline))(input)?;

        Ok((input, (types, return_type)))
    }
}

pub(super) fn callable_return_type(
    multiline: bool,
) -> impl Fn(&[u8]) -> IResult<&[u8], ReturnType> {
    move |input: &[u8]| -> IResult<&[u8], ReturnType> {
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b":")(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        callable_return_type_details(multiline)(input)
    }
}

pub(super) fn callable_return_type_details(
    multiline: bool,
) -> impl Fn(&[u8]) -> IResult<&[u8], ReturnType> {
    move |input: &[u8]| -> IResult<&[u8], ReturnType> {
        if let (input, Some(return_type)) =
            opt(delimited(tag(b"("), compound_type(multiline), tag(b")")))(input)?
        {
            return Ok((input, return_type));
        };
        let (input, ctype) = concrete_type(multiline)(input)?;
        Ok((input, CompoundType::Single(ctype)))
    }
}
