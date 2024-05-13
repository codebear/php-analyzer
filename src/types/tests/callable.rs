use crate::types::parser::union_type;

#[test]
pub fn test_callable1() {
    let input = b"callable(PersonContext):T[]";

    let (rest, what) = union_type(false)(input).unwrap();

    assert!(rest.is_empty());
    assert!(what.len() == 1);
    let ctype = &what[0];
    let _ptype = &ctype.ptype;
}

#[test]
pub fn test_callable2() {
    let input = b"callable(PersonContext $ctx):T[]";

    let (rest, what) = union_type(false)(input).unwrap();

    assert!(rest.is_empty(), "Remainder from parsing is not empty");
    assert!(what.len() == 1);
    let ctype = &what[0];
    let _ptype = &ctype.ptype;
}
