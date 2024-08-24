mod basetokens;
mod callable;
mod class_type;
mod nomhelpers;
mod normal_types;
mod shape;
mod type_names;

use callable::callable_type;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::delimited,
    IResult,
};
use nomhelpers::{ourspace0, separated_list2, space0_and_separator};
use normal_types::{generic_args, normal_type_with_optional_generics};
use shape::shape_type;

use crate::symbols::Name;

use super::parse_types::{
    CompoundType, ConcreteType, IntersectionOfTypes, ParsedType, ShapeEntry, TypeName, TypeStruct,
    UnionOfTypes,
};

///
/// COMPOUNDED TYPE PARTS
///

///
/// Make sure that all parse-function only accepts whitespace _before_ expected content. We don't want to
/// parse trailing whitespace, at it might have semantic meaning to others
///
///

fn nullable(input: &[u8]) -> IResult<&[u8], bool> {
    let (input, _) = tag(b"?")(input)?;
    Ok((input, true))
}

pub fn compound_type(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], CompoundType> {
    move |input| {
        alt((
            // intersection type
            map(intersection_type(multiline), CompoundType::Intersection),
            // union type
            map(union_type(multiline), CompoundType::Union),
            // parenthesises
            parenthesized_type(multiline),
            // single type
            map(concrete_type(multiline), CompoundType::Single),
        ))(input)
    }
}

pub fn compound_type_with_colon(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], CompoundType> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        compound_type(multiline)(input)
        //      union_type(multiline)
    }
}

pub fn intersection_type(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], IntersectionOfTypes> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        separated_list2(
            space0_and_separator(b"&", multiline),
            concrete_type(multiline),
        )(input)
    }
}

pub fn union_type(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], UnionOfTypes> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        separated_list2(
            space0_and_separator(b"|", multiline),
            concrete_type(multiline),
        )(input)
    }
}

pub fn only_generic_args(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], Vec<CompoundType>> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        generic_args(multiline)(input)
    }
}

fn concrete_type(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], ConcreteType> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, nullable) = opt(nullable)(input)?;
        let (input, mut parsed_type) = one_type(multiline)(input)?;
        // Handle any `thing[]`-declarations
        let mut iter_input = input;
        loop {
            let (input, _) = ourspace0(multiline)(iter_input)?;

            let (input, post_decl_array) = opt(many1(tag(b"[]")))(input)?;
            if let Some(levels) = post_decl_array {
                for _ in levels {
                    // Wrap parsed_type in an array, converting from `thing[]` to `array<thing>`
                    let compound_type = CompoundType::Single(ConcreteType {
                        nullable: false,
                        ptype: parsed_type,
                    });
                    parsed_type = ParsedType::Type(TypeStruct {
                        type_name: TypeName::Name(Name::from("array")),
                        generics: Some(vec![compound_type]),
                    })
                }
                // prepare for next iteration
                iter_input = input;
            } else {
                break;
            }
        }
        let input = iter_input;
        let nullable = nullable.unwrap_or(false);

        let concrete_type = ConcreteType {
            nullable,
            ptype: parsed_type,
        };
        Ok((input, concrete_type))
    }
}

fn one_type(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], ParsedType> {
    move |input| {
        alt((
            class_type::class_type(multiline),
            shape_type(multiline),
            callable_type(multiline),
            tuple_type(multiline),
            normal_type_with_optional_generics(multiline),
        ))(input)
    }
}

fn parenthesized_type(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], CompoundType> {
    move |input| {
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b"(")(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, ptype) = compound_type(multiline)(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b")")(input)?;
        Ok((input, ptype))
    }
}

fn tuple_type(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], ParsedType> {
    move |input| {
        let (input, mut type_entries) =
            delimited(tag(b"("), inner_tuple_list(multiline), tag(b")"))(input)?;

        let shape_entries: Vec<ShapeEntry> = type_entries
            .drain(..)
            .map(|vtypes| ShapeEntry(None, vtypes))
            .collect();

        Ok((input, ParsedType::Shape(shape_entries)))
    }
}

fn inner_tuple_list(multiline: bool) -> impl Fn(&[u8]) -> IResult<&[u8], Vec<CompoundType>> {
    move |input| {
        // foobar
        let (input, first) = compound_type(multiline)(input)?;
        let (input, _) = ourspace0(multiline)(input)?;
        let (input, _) = tag(b",")(input)?;
        let (input, mut remaining) = separated_list1(tag(b","), compound_type(multiline))(input)?;

        let mut all = vec![first];
        all.append(&mut remaining);

        Ok((input, all))
    }
}
