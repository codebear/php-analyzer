use std::{
    ffi::{OsStr, OsString},
    os::unix::prelude::OsStrExt,
};



use super::{phpdoc::parse_phpdoc, position::{PHPDocInput, fake_range}};

#[test]
pub fn parse_test1() {
    let buf = b"/** @desc balle1 */";
    test_phpdoc(buf, 1);

    let buf = b"/** 
        * @desc balle2 
        */";
    test_phpdoc(buf, 3);

    let buf = b"/**
    * @var Y
    */";

    test_phpdoc(buf, 3);

    let buf = b"/** 
        * @desc balle 
        * @var int
        * @var int $balle
        * @var int Denne blir nice
        * @var int $klorin Balle
        * @param string Rock All
        * @param int
        * @param int|string $foobar
        * @return array<string> Why event bother
        */";
    test_phpdoc(buf, 11);
}

pub fn test_phpdoc(buf: &[u8], expect_entries: usize) {
    let buffer: OsString = OsStr::from_bytes(buf).into();
    let parse_input = PHPDocInput(buf, fake_range(&buffer));
    match parse_phpdoc(parse_input) {
        Ok((rest, phpdoc)) => {
            assert!(rest.len() == 0);
            assert_eq!(phpdoc.len(), expect_entries);
            eprintln!("BOLLOCKS: {:#?}", phpdoc);
        }
        Err(err) => {
            let err = err.map_input(|i| OsStr::from_bytes(i.0));
            eprintln!("Error parsing phpdoc: {:?}", err);
            assert!(false);
        }
    }
}
