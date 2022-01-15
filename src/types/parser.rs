use std::sync::atomic::{AtomicBool, Ordering};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, multispace0},
    combinator::opt,
    error::{Error, ErrorKind},
    multi::{many1, separated_list1},
    sequence::preceded,
    Err, IResult,
};

use crate::symbols::{FullyQualifiedName, Name};

use super::parse_types::{
    ArgumentVector, ConcreteType, ParsedType, ReturnType, ShapeEntry, ShapeKey, TypeName,
    TypeStruct, UnionOfTypes,
};

///
/// Make sure that all parse-function only accepts whitespace _before_ expected content. We don't want to
/// parse trailing whitespace, at it might have semantic meaning to others
///

fn nullable(input: &[u8]) -> IResult<&[u8], bool> {
    let (input, _) = tag(b"?")(input)?;
    Ok((input, true))
}

fn type_name(input: &[u8]) -> IResult<&[u8], TypeName> {
    alt((relative_name, qualified_name, only_simple_type_name))(input)
}

fn only_simple_type_name(input: &[u8]) -> IResult<&[u8], TypeName> {
    let (input, name) = simple_type_name(input)?;
    Ok((input, TypeName::Name(name)))
}

fn qualified_name(input: &[u8]) -> IResult<&[u8], TypeName> {
    let (input, names) = many1(preceded(tag(b"\\"), simple_type_name))(input)?;

    Ok((input, TypeName::FQName(FullyQualifiedName::from(names))))
}

fn relative_name(input: &[u8]) -> IResult<&[u8], TypeName> {
    let mut path = vec![];
    let (input, name) = simple_type_name(input)?;
    path.push(name);
    let (input, names) = many1(preceded(tag(b"\\"), simple_type_name))(input)?;
    path.extend(names);
    Ok((input, TypeName::RelativeName(path)))
}

fn simple_type_name(input: &[u8]) -> IResult<&[u8], Name> {
    let second = AtomicBool::new(false);
    let (input, result) = take_while1(move |x: u8| {
        let sec = second.load(Ordering::Relaxed);
        second.store(true, Ordering::Relaxed);
        x == b'_'
            || if sec {
                // We allow dash in type names, to allow for `class-string` and similar
                x.is_ascii_alphanumeric() || x == b'-'
            } else {
                x.is_ascii_alphabetic()
            }
    })(input)?;

    Ok((input, Name::from(result)))
}

pub fn union_type(input: &[u8]) -> IResult<&[u8], UnionOfTypes> {
    let (input, _) = multispace0(input)?;
    separated_list1(union_separator, concrete_type)(input)
}

fn union_separator(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(b"|")(input)?;
    Ok((input, ()))
}

fn normal_type(input: &[u8]) -> IResult<&[u8], ParsedType> {
    let (input, _) = multispace0(input)?;
    let (input, type_name) = type_name(input)?;

    let (input, generics) = opt(generic_args)(input)?;

    let type_struct = TypeStruct {
        type_name,
        generics,
    };

    Ok((input, ParsedType::Type(type_struct)))
}

fn shape_type(input: &[u8]) -> IResult<&[u8], ParsedType> {
    let (input, _) = tag(b"array")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(b"{")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, entries) = separated_list1(generic_separator, shape_entry)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(b"}")(input)?;
    Ok((input, ParsedType::Shape(entries)))
}

fn shape_key_num(input: &[u8]) -> IResult<&[u8], ShapeKey> {
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

fn shape_key_str(input: &[u8]) -> IResult<&[u8], ShapeKey> {
    let (input, str_key) = simple_type_name(input)?;

    Ok((input, ShapeKey::String(str_key)))
}

fn shape_key(input: &[u8]) -> IResult<&[u8], (ShapeKey, bool)> {
    let (input, _) = multispace0(input)?;

    let (input, key) = alt((shape_key_num, shape_key_str))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, optional_questionmark) = opt(tag("?"))(input)?;
    let optional = optional_questionmark.is_some();
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(b":")(input)?;
    Ok((input, (key, optional)))
}

fn shape_entry(input: &[u8]) -> IResult<&[u8], ShapeEntry> {
    let (input, _) = multispace0(input)?;
    let (input, key) = opt(shape_key)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, ctype) = union_type(input)?;
    Ok((input, ShapeEntry(key, ctype)))
}

fn concrete_type(input: &[u8]) -> IResult<&[u8], ConcreteType> {
    let (input, _) = multispace0(input)?;
    let (input, nullable) = opt(nullable)(input)?;
    let (input, mut parsed_type) = one_type(input)?;
    // Handle any `thing[]`-declarations
    let mut iter_input = input;
    loop {
        let (input, _) = multispace0(iter_input)?;
        let (input, post_decl_array) = opt(many1(tag(b"[]")))(input)?;
        if let Some(levels) = post_decl_array {
            for _ in levels {
                // Wrap parsed_type in an array, converting from `thing[]` to `array<thing>`
                parsed_type = ParsedType::Type(TypeStruct {
                    type_name: TypeName::Name(Name::from("array")),
                    generics: Some(vec![vec![ConcreteType {
                        nullable: false,
                        ptype: parsed_type,
                    }]]),
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

fn one_type(input: &[u8]) -> IResult<&[u8], ParsedType> {
    alt((shape_type, callable_type, normal_type))(input)
}

fn generic_separator(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(b",")(input)?;
    Ok((input, ()))
}

fn generic_args(input: &[u8]) -> IResult<&[u8], Vec<Vec<ConcreteType>>> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(b"<")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, types) = separated_list1(generic_separator, union_type)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(b">")(input)?;
    Ok((input, types))
}

fn callable_type(input: &[u8]) -> IResult<&[u8], ParsedType> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(b"callable")(input)?;
    let (input, details) = opt(callable_details)(input)?;
    let ptype = if let Some((types, return_type)) = details {
        ParsedType::Callable(types, return_type)
    } else {
        ParsedType::CallableUntyped
    };
    Ok((input, ptype))
}

fn callable_details(input: &[u8]) -> IResult<&[u8], (ArgumentVector, Option<ReturnType>)> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(b"(")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, types) = separated_list1(generic_separator, union_type)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(b")")(input)?;
    let (input, return_type) = opt(callable_return_type)(input)?;

    Ok((input, (types, return_type)))
}

fn callable_return_type(input: &[u8]) -> IResult<&[u8], ReturnType> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(b":")(input)?;
    let (input, _) = multispace0(input)?;
    union_type(input)
}
