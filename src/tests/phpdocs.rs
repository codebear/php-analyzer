use std::ffi::OsString;

use tree_sitter::{Point, Range};

use crate::{
    phpdoc::types::{PHPDocComment, PHPDocEntry},
    types::{
        parser::union_type,
    },
};

fn fake_range(_buffer: &OsString) -> Range {
    let point = Point { row: 0, column: 0 };
    Range {
        start_byte: 0,
        end_byte: 1,
        start_point: point.clone(),
        end_point: point,
    }
}

fn test_parse(buffer: OsString) -> Result<PHPDocComment, OsString> {
    let range = fake_range(&buffer);
    PHPDocComment::parse(&buffer, &range)
}

#[test]
pub fn test_var() {
    assert!(true);
    // void
    if let Ok(phpdoc) = test_parse("/** @var int */".into()) {
        let (_, reference_type) = union_type(b"int").unwrap();

        assert_eq!(
            phpdoc.entries,
            vec![PHPDocEntry::Var(reference_type, None, None,)]
        );
    } else {
        assert!(false, "Unable to parse doccomment");
    }
}

#[test]
pub fn test_var2() {
    assert!(true);
    // void
    if let Ok(phpdoc) = test_parse(
        "/** 
                * @var int
                */"
        .into(),
    ) {
        let (_, reference_type) = union_type(b"int").unwrap();

        assert_eq!(
            phpdoc.entries,
            vec![
                PHPDocEntry::EmptyLine,
                PHPDocEntry::Var(reference_type, None, None),
                PHPDocEntry::EmptyLine,
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
        let (_, reference_type) = union_type(b"CantorPairing").unwrap();

        assert_eq!(
            phpdoc.entries,
            vec![
                PHPDocEntry::EmptyLine,
                PHPDocEntry::Var(reference_type, None, None,),
                PHPDocEntry::EmptyLine,
            ]
        );
    }
}
