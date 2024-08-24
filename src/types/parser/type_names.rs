use nom::{branch::alt, bytes::complete::tag, multi::many1, sequence::preceded, IResult};

use crate::{symbols::FullyQualifiedName, types::parse_types::TypeName};

use super::{basetokens::simple_type_name, nomhelpers::separated_list2};

///
/// TypeName composing of a single simple type name
///
pub(super) fn only_simple_type_name(input: &[u8]) -> IResult<&[u8], TypeName> {
    let (input, name) = simple_type_name(input)?;
    Ok((input, TypeName::Name(name)))
}

///
/// TypeName composing of a fully qualified name
///
pub(super) fn qualified_name(input: &[u8]) -> IResult<&[u8], TypeName> {
    let (input, names) = many1(preceded(tag(b"\\"), simple_type_name))(input)?;

    Ok((input, TypeName::FQName(FullyQualifiedName::from(names))))
}

///
/// TypeName composing of a potential relative name (without the separating prefix)
///
pub(super) fn relative_name(input: &[u8]) -> IResult<&[u8], TypeName> {
    let (input, path) = separated_list2(tag(b"\\"), simple_type_name)(input)?;
    Ok((input, TypeName::RelativeName(path)))
}

///
/// Combining parser for these three TypeName types
///
pub(super) fn type_name(input: &[u8]) -> IResult<&[u8], TypeName> {
    // The order of these parsers is important
    alt((
        // relative name must be first
        relative_name,
        // then check for fully qualified name
        qualified_name,
        // simple name is last
        only_simple_type_name,
    ))(input)
}
