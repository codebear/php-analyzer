use std::ffi::OsString;

use tree_sitter::Point;

use crate::{
    analysis::state::AnalysisState,
    issue::VoidEmitter,
    parser::Range,
    types::{
        type_parser::TypeParser,
        union::{DiscreteType, PHPType},
    },
};

#[test]
fn parse_array() {
    let mut state = AnalysisState::new();
    let range = Range {
        start_byte: 0,
        end_byte: 0,
        start_point: Point { row: 0, column: 0 },
        end_point: Point { row: 0, column: 0 },
    };
    let emitter = VoidEmitter::new();

    let res = TypeParser::parse(OsString::from("array<string>"), range, &mut state, &emitter);
    if let Some(utype) = res {
        let ref_utype = PHPType::from(DiscreteType::Vector(PHPType::from(DiscreteType::String)));
        assert_eq!(utype, ref_utype);
    } else {
        assert!(false);
    }
}
