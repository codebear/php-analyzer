use nom::{bytes::complete::tag, IResult};

use crate::types::parse_types::ParsedType;

use super::{basetokens::simple_type_name, nomhelpers::ourspace0, type_names::type_name};

pub(super) fn class_type(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], ParsedType> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        // FIXME type-name, tillater dash i "klasse"-navn, det er uheldig
        let (input, cname) = type_name(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b"::")(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, tname) = simple_type_name(input)?;
        Ok((input, ParsedType::ClassType(cname, tname)))
    }
}
