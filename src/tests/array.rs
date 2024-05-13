use std::ffi::OsString;

use crate::{
    tests::{evaluate_php_buffers, get_inferred_return_value},
    value::PHPValue,
};

#[test]
fn return_array_sub() {
    let returned_value = if let Some(value) = get_inferred_return_value(
        "
        $x = [5,6,3,8];
        $i = 1;
        
        return $x[++$i];
    ",
    ) {
        value
    } else {
        unreachable!("Didn't get proper value");
    };
    assert_eq!(&PHPValue::Int(3), &returned_value);
}

#[test]
fn test_array() {
    let buffers: &[(OsString, OsString)] = &[(
        "something.php".into(),
        r#"<?php 

            /**
             * @return array<string>
             */
            function get_str_array() {
                return ["foo", "bar", "baz"];
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
}
