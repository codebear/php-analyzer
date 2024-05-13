use std::ffi::OsString;

use tree_sitter::Point;

use crate::{
    parser::Range,
    phpdoc::{
        position::fake_range,
        types::{PHPDocComment, PHPDocEntry},
    },
    types::parser::union_type,
};

fn test_parse(buffer: OsString) -> Result<PHPDocComment, OsString> {
    let range = fake_range(&buffer);
    PHPDocComment::parse(&buffer, &range)
}

#[test]
pub fn test_var() {
    assert!(true);
    // void
    if let Ok(phpdoc) = test_parse("/** @var int */".into()) {
        let (_, reference_type) = union_type(false)(b"int").unwrap();
        // let range = fake_range(&OsString::new());
        assert_eq!(
            phpdoc.entries,
            vec![PHPDocEntry::Var(
                Range {
                    start_byte: 4,
                    start_point: Point { row: 0, column: 4 },
                    end_byte: 12,
                    end_point: Point { row: 0, column: 12 }
                },
                reference_type,
                None,
                None,
            )]
        );
    } else {
        unreachable!("Unable to parse doccomment");
    }
}

#[test]
pub fn test_var2() {
    // void
    if let Ok(phpdoc) = test_parse(
        "/** 
                * @var int
                */"
        .into(),
    ) {
        let (_, reference_type) = union_type(false)(b"int").unwrap();

        assert_eq!(
            phpdoc.entries,
            vec![
                PHPDocEntry::EmptyLine(Range {
                    start_byte: 4,
                    start_point: Point { row: 0, column: 4 },
                    end_byte: 4,
                    end_point: Point { row: 0, column: 4 }
                }),
                PHPDocEntry::Var(
                    Range {
                        start_byte: 23,
                        start_point: Point { row: 1, column: 19 },
                        end_byte: 31,
                        end_point: Point { row: 1, column: 27 }
                    },
                    reference_type,
                    None,
                    None
                ),
                PHPDocEntry::EmptyLine(Range {
                    start_byte: 48,
                    start_point: Point { row: 2, column: 17 },
                    end_byte: 48,
                    end_point: Point { row: 2, column: 17 }
                }),
            ]
        );
    } else {
        assert!(false, "Unable to parse doccomment");
    }
}

#[test]
pub fn test_var3() {
    assert!(true);
    // void
    if let Ok(phpdoc) = test_parse(
        "/** 
                * @var CantorPairing
                */"
        .into(),
    ) {
        let (_, reference_type) = union_type(false)(b"CantorPairing").unwrap();

        assert_eq!(
            phpdoc.entries,
            vec![
                PHPDocEntry::EmptyLine(Range {
                    start_byte: 4,
                    start_point: Point { row: 0, column: 4 },
                    end_byte: 4,
                    end_point: Point { row: 0, column: 4 }
                }),
                PHPDocEntry::Var(
                    Range {
                        start_byte: 23,
                        start_point: Point { row: 1, column: 19 },
                        end_byte: 41,
                        end_point: Point { row: 1, column: 37 }
                    },
                    reference_type,
                    None,
                    None,
                ),
                PHPDocEntry::EmptyLine(Range {
                    start_byte: 58,
                    start_point: Point { row: 2, column: 17 },
                    end_byte: 58,
                    end_point: Point { row: 2, column: 17 }
                }),
            ]
        );
    }
}
