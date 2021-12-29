#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);

    let mut parser = crate::parser::PHPParser::new();

    let source_code: &[u8] = b"<?php var_dump($foo);";
    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();
    assert_eq!(root_node.child_count(), 2);
}
