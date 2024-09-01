use std::ffi::OsString;

use crate::tests::evaluate_php_buffers;

#[test]
fn test_traversable() {
    let buffers: &[(OsString, OsString)] = &[(
        "something.php".into(),
        r#"<?php 

            /**
             * @return Traversable<string>
             */
            function get_str_array() {
                return new ArrayIterator(["foo", "bar", "baz"]);
            }

            function get_something() {
                foreach(get_str_array() as $noe) {
                    return $noe;
                }
            }
          
            "#
        .into(),
    )];
    let result = evaluate_php_buffers(Default::default(), buffers.to_vec(), false);
    //eprintln!("FOO: {:?}", result);
    /*assert_eq!(
        result.return_type,
        Some(DiscreteType::Named("DOMElement".into(), "\\DOMElement".into()).into())
    );*/
    assert_eq!(result.issues.len(), 0);
    todo!(); // Missing real test case
}
