use crate::{
    analysis::state::AnalysisState,
    issue::VoidEmitter,
    phpdoc::position::fake_range,
    symbols::{FullyQualifiedName, Name},
    types::{
        type_parser::TypeParser,
        union::{DiscreteType, PHPType},
    },
};

use std::ffi::OsString;

#[test]
fn test_parsing() {
    // void

    // disabled, as this is probably noe correct

    /*     let mut state = AnalysisState::new();
    let emitter = VoidEmitter::new();
    let buffer = OsString::from("class-string<this::T_FILE_RECORD> ");
    let (Some(ptype), Some(remainder)) =
        TypeParser::parse_with_remainder(buffer.clone(), fake_range(&buffer), &mut state, &emitter)
    else {
        panic!("Failed parsing");
    };
    assert_eq!(OsString::from(" "), remainder);
    eprintln!("Parsed: {}", ptype);
    panic!("WHAT {:?}", ptype);*/
}

#[test]
fn test_parsing2() {
    let mut state = AnalysisState::new();
    let emitter = VoidEmitter::new();
    let buffer = OsString::from("T_SOME_TYPE");
    let (Some(ptype), None) =
        TypeParser::parse_with_remainder(buffer.clone(), fake_range(&buffer), &mut state, &emitter)
    else {
        panic!("Failed parsing");
    };

    let PHPType::Discrete(discrete_type) = ptype else {
        panic!("Expected DiscreteType");
    };
    let expected = DiscreteType::Named(
        Name::from("T_SOME_TYPE"),
        FullyQualifiedName {
            path: vec![Name::from("T_SOME_TYPE")],
        },
    );
    assert_eq!(expected, *discrete_type);
}
