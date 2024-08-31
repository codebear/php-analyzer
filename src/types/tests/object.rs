use std::{ffi::OsStr, os::unix::ffi::OsStrExt};

use crate::{
    symbols::Name,
    types::{
        parse_types::{CompoundType, ParsedType, TypeName, TypeStruct},
        parser::compound_type,
    },
};

#[test]
fn test_object() {
    let input = b"this::T_SOMETYPE";

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

    let ParsedType::ClassType(TypeName::Name(class), typename) = ctype.ptype else {
        panic!("Did not find a Type");
    };

    assert_eq!(Name::from("T_SOMETYPE"), typename);
    assert_eq!(Name::from("this"), class);
}

#[test]
fn test_generics_and_object() {
    let input = b"class-string<this::T_SOMETYPE>";

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

    assert!(!ctype.nullable);

    let ParsedType::Type(noe) = ctype.ptype else {
        panic!("Did not find a Type");
    };

    let TypeStruct {
        type_name,
        generics,
    } = noe;

    assert_eq!(TypeName::Name(Name::from("class-string")), type_name);

    let Some(mut generics) = generics else {
        panic!("Missing generics");
    };

    assert!(generics.len() == 1);

    let CompoundType::Single(ctype) = generics.pop().expect("We checked len") else {
        panic!("Did not find a Single-type");
    };

    assert!(!ctype.nullable);

    let ParsedType::ClassType(TypeName::Name(class), typename) = ctype.ptype else {
        panic!("Did not find a Type");
    };

    assert_eq!(Name::from("T_SOMETYPE"), typename);
    assert_eq!(Name::from("this"), class);
}
