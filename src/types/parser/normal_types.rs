use super::compound_type;
use super::nomhelpers::{ourspace0, space0_and_separator};
use super::type_names::type_name;
use nom::{branch::alt, bytes::complete::tag, combinator::opt, multi::separated_list1, IResult};

use crate::symbols::Name;

use super::super::parse_types::{CompoundType, ConcreteType, ParsedType, TypeName, TypeStruct};

pub(super) fn normal_type_with_optional_generics(
    multiline: bool,
) -> impl Fn(&[u8]) -> IResult<&[u8], ParsedType> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, type_name) = type_name(input)?;

        let (input, generics) = opt(generic_args(multiline))(input)?;

        let type_struct = TypeStruct {
            type_name,
            generics,
        };

        Ok((input, ParsedType::Type(type_struct)))
    }
}

pub(super) fn generic_args(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], Vec<CompoundType>> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b"<")(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, types) = alt((
            // what
            single_generic_mixed,
            // haa
            generic_args_list(multiline),
        ))(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b">")(input)?;
        Ok((input, types))
    }
}

fn single_generic_mixed(input: &[u8]) -> IResult<&[u8], Vec<CompoundType>> {
    let (input, _) = tag(b"*")(input)?;

    let ptype = ParsedType::Type(TypeStruct {
        type_name: TypeName::Name(Name::from("mixed")),
        generics: None,
    });
    let concrete_type = ConcreteType {
        nullable: false,
        ptype,
    };
    let compound_type = CompoundType::Single(concrete_type);
    Ok((input, vec![compound_type]))
}

fn generic_args_list<'a>(
    multiline: bool,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Vec<CompoundType>> {
    // -> impl Fn(&[u8]) -> IResult<&[u8], Vec<CompoundType>> {
    separated_list1(
        space0_and_separator(b",", multiline),
        compound_type(multiline),
    )
}
