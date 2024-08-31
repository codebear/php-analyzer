use std::{ffi::OsStr, os::unix::ffi::OsStrExt};

use crate::{
    symbols::Name,
    types::{
        parse_types::{CompoundType, ConcreteType, ParsedType, TypeName, TypeStruct},
        parser::compound_type,
    },
};

#[test]
pub fn test_complex0() {
    let input = b"Atype";

    let noe = match compound_type(false)(input) {
        Ok(x) => x,
        Err(x) => {
            match x {
                nom::Err::Incomplete(a) => eprintln!("Incomplete: {:?}", a),
                nom::Err::Error(b) => {
                    let remainder = OsStr::from_bytes(b.input);
                    eprintln!("Error: {:?}, remainder: {:?}", b, remainder);
                }
                nom::Err::Failure(c) => eprintln!("Failure: {:?}", c),
            }
            todo!();
        }
    };
    let (rest, what) = noe;
    eprintln!("Rest: {:?}", OsStr::from_bytes(rest));
    assert!(rest.is_empty());
    assert!(what.len() == 1);
    let ctype = what
        .first()
        .cloned()
        .expect("We checked with assertion that this is of len 1");

    let ParsedType::Type(noe) = ctype.ptype else {
        panic!("Did not find a Type");
    };

    let expected = TypeStruct {
        type_name: TypeName::Name(Name::from("Atype")),
        generics: None,
    };
    assert_eq!(expected, noe);
}

#[test]
pub fn test_complex01() {
    let input = b"(Atype)";

    let noe = match compound_type(false)(input) {
        Ok(x) => x,
        Err(x) => {
            match x {
                nom::Err::Incomplete(a) => eprintln!("Incomplete: {:?}", a),
                nom::Err::Error(b) => {
                    let remainder = OsStr::from_bytes(b.input);
                    eprintln!("Error: {:?}, remainder: {:?}", b, remainder);
                }
                nom::Err::Failure(c) => eprintln!("Failure: {:?}", c),
            }
            todo!();
        }
    };
    let (rest, what) = noe;
    eprintln!("Rest: {:?}", OsStr::from_bytes(rest));
    assert!(rest.is_empty());
    assert!(what.len() == 1);
    let ctype = what
        .first()
        .cloned()
        .expect("We checked with assertion that this is of len 1");

    let ParsedType::Parenthesized(noe) = ctype.ptype else {
        panic!("Did not find a Parenthesized-type");
    };
    let CompoundType::Single(ctype) = *noe else {
        panic!("Did not find a Single-type");
    };

    let expected = ConcreteType {
        nullable: false,
        ptype: ParsedType::Type(TypeStruct {
            type_name: TypeName::Name(Name::from("Atype")),
            generics: None,
        }),
    };
    assert_eq!(expected, ctype);
}

#[test]
pub fn test_complex02() {
    let input = b"?(Atype)";

    let noe = match compound_type(false)(input) {
        Ok(x) => x,
        Err(x) => {
            match x {
                nom::Err::Incomplete(a) => eprintln!("Incomplete: {:?}", a),
                nom::Err::Error(b) => {
                    let remainder = OsStr::from_bytes(b.input);
                    eprintln!("Error: {:?}, remainder: {:?}", b, remainder);
                }
                nom::Err::Failure(c) => eprintln!("Failure: {:?}", c),
            }
            panic!("Failed to parse as expected");
        }
    };
    let (rest, what) = noe;

    assert!(rest.is_empty());
    assert!(what.len() == 1);
    let ctype = what
        .first()
        .cloned()
        .expect("We checked with assertion that this is of len 1");

    assert!(ctype.nullable);

    let ParsedType::Parenthesized(noe) = ctype.ptype else {
        panic!("Did not find a Parenthesized-type");
    };
    let CompoundType::Single(ctype) = *noe else {
        panic!("Did not find a Single-type");
    };

    let expected = ConcreteType {
        nullable: false,
        ptype: ParsedType::Type(TypeStruct {
            type_name: TypeName::Name(Name::from("Atype")),
            generics: None,
        }),
    };
    assert_eq!(expected, ctype);
}

#[test]
pub fn test_complex03() {
    let input = b"Atype|Btype";

    let parse_result = match compound_type(false)(input) {
        Ok(x) => x,
        Err(x) => {
            match x {
                nom::Err::Incomplete(a) => eprintln!("Incomplete: {:?}", a),
                nom::Err::Error(b) => {
                    let remainder = OsStr::from_bytes(b.input);
                    eprintln!("Error: {:?}, remainder: {:?}", b, remainder);
                }
                nom::Err::Failure(c) => eprintln!("Failure: {:?}", c),
            }
            todo!();
        }
    };
    let (rest, parsed_type) = parse_result;

    assert!(rest.is_empty());

    eprintln!("Parsed: {:?}", parsed_type);

    assert_eq!(
        CompoundType::Union(vec![
            ConcreteType {
                nullable: false,
                ptype: ParsedType::Type(TypeStruct {
                    type_name: TypeName::Name(Name::from("Atype")),
                    generics: None,
                }),
            },
            ConcreteType {
                nullable: false,
                ptype: ParsedType::Type(TypeStruct {
                    type_name: TypeName::Name(Name::from("Btype")),
                    generics: None,
                }),
            },
        ]),
        parsed_type
    );
}

#[test]
pub fn test_complex1() {
    let input = b"Atype&Btype";

    let parse_result = match compound_type(false)(input) {
        Ok(x) => x,
        Err(x) => {
            match x {
                nom::Err::Incomplete(a) => eprintln!("Incomplete: {:?}", a),
                nom::Err::Error(b) => {
                    let remainder = OsStr::from_bytes(b.input);
                    eprintln!("Error: {:?}, remainder: {:?}", b, remainder);
                }
                nom::Err::Failure(c) => eprintln!("Failure: {:?}", c),
            }
            todo!();
        }
    };
    let (rest, parsed_type) = parse_result;

    assert!(rest.is_empty());

    assert!(parsed_type.len() == 2);
    let CompoundType::Intersection(mut intersection) = parsed_type else {
        panic!("Intersection not found");
    };

    assert!(intersection.len() == 2);

    let btype = intersection
        .pop()
        .expect("We checked that the length was 2");
    let atype = intersection
        .pop()
        .expect("We checked that the length was 2");

    assert!(intersection.is_empty());

    let expected_atype = ConcreteType {
        nullable: false,
        ptype: ParsedType::Type(TypeStruct {
            type_name: TypeName::Name(Name::from("Atype")),
            generics: None,
        }),
    };

    let expected_btype = ConcreteType {
        nullable: false,
        ptype: ParsedType::Type(TypeStruct {
            type_name: TypeName::Name(Name::from("Btype")),
            generics: None,
        }),
    };

    assert_eq!(expected_atype, atype);
    assert_eq!(expected_btype, btype);
}

#[test]
pub fn test_complex2() {
    let input = b"?(TypeA&TypeB)";

    let noe = match compound_type(false)(input) {
        Ok(x) => x,
        Err(x) => {
            match x {
                nom::Err::Incomplete(a) => eprintln!("Incomplete: {:?}", a),
                nom::Err::Error(b) => {
                    let remainder = OsStr::from_bytes(b.input);
                    eprintln!("Error: {:?}, remainder: {:?}", b, remainder);
                }
                nom::Err::Failure(c) => eprintln!("Failure: {:?}", c),
            }
            todo!();
        }
    };
    let (rest, what) = noe;

    assert!(rest.is_empty());
    assert!(what.len() == 1);

    let ctype = what
        .first()
        .cloned()
        .expect("We checked with assertion that this is of len 1");

    assert!(ctype.nullable);

    let ParsedType::Parenthesized(noe) = ctype.ptype else {
        panic!("Did not find a Parenthesized-type");
    };

    let CompoundType::Intersection(mut intersection) = *noe else {
        panic!("Intersection not found");
    };
    assert!(intersection.len() == 2);

    let btype = intersection
        .pop()
        .expect("We checked that the length was 2");
    let atype = intersection
        .pop()
        .expect("We checked that the length was 2");

    assert!(intersection.is_empty());

    let expected_atype = ConcreteType {
        nullable: false,
        ptype: ParsedType::Type(TypeStruct {
            type_name: TypeName::Name(Name::from("TypeA")),
            generics: None,
        }),
    };

    let expected_btype = ConcreteType {
        nullable: false,
        ptype: ParsedType::Type(TypeStruct {
            type_name: TypeName::Name(Name::from("TypeB")),
            generics: None,
        }),
    };

    assert_eq!(expected_atype, atype);
    assert_eq!(expected_btype, btype);
}
