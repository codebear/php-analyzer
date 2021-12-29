use std::ffi::OsString;

use crate::tests::evaluate_php_buffers;

#[test]
fn test_interface_references() {
    let buffers: &[(OsString, OsString)] = &[
        (
            "foo/myinterface.php".into(),
            r#"<?php 

            namespace foo;
            interface myInterface {
                function getString(): string;
            }
            "#
            .into(),
        ),
        (
            "bar/X.php".into(),
            r#"<?php 

            namespace bar;

            use \foo\myInterface;

            class X {
               
                function foo(myInterface $ting) {
                    return $ting->getString();
                }
            }
            "#
            .into(),
        ),
    ];
    let result = evaluate_php_buffers(buffers.to_vec(), false);
    // eprintln!("RESULT: {:?}", &result);
    /* if let Some(symbols) = result.symbol_data {
        let func_data = symbols.functions.read().unwrap();
        let func_name: OsString = r"\test_return".into();
        if let Some(func) = func_data.get(&func_name) {
            let data = func.read().unwrap();
            assert_eq!(data.inferred_return_type, Some(DiscreteType::String.into()));
        } else {
            assert!(false, "data of function test_return not found");
        }
    }*/
    // assert_eq!(result.return_type, Some(DiscreteType::String.into()));
    assert_eq!(result.issues.len(), 0);
}
