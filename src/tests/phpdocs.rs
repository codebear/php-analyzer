use std::ffi::OsString;

use tree_sitter::{Point, Range};

use crate::phpdoccomment::PHPDocComment;

fn fake_range(_buffer: &OsString) -> Range {
    let point = Point { row: 0, column: 0 };
    Range {
        start_byte: 0,
        end_byte: 1,
        start_point: point.clone(),
        end_point: point,
    }
}

fn test_parse(buffer: OsString) -> Option<PHPDocComment> {
    let range = fake_range(&buffer);
    PHPDocComment::parse(&buffer, range)
}

#[test]
pub fn test_var() {
    assert!(true);
    // void
    if let Some(phpdoc) = test_parse("/** @var int */".into()) {
        if let Some((param, _)) = phpdoc.get_param("@var") {
            assert_eq!("int", param);
        } else {
            assert!(false, "Finner ikke @var");
        }
    } else {
        assert!(false, "Unable to parse doccomment");
    }
}

#[test]
pub fn test_var2() {
    assert!(true);
    // void
    if let Some(phpdoc) = test_parse(
        "/** 
                * @var int
                */"
        .into(),
    ) {
        if let Some((param, _range)) = phpdoc.get_param("@var") {
            assert_eq!("int", param);
        } else {
            assert!(false, "Finner ikke @var");
        }
    } else {
        assert!(false, "Unable to parse doccomment");
    }
}

#[test]
pub fn test_var3() {
    assert!(true);
    // void
    if let Some(phpdoc) = test_parse(
        "/** 
                * @var CantorPairing
                */"
        .into(),
    ) {
        if let Some((param, _range)) = phpdoc.get_param("@var") {
            assert_eq!("CantorPairing", param);
        } else {
            assert!(false, "Finner ikke @var");
        }
    }
}
