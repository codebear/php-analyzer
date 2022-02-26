use std::ffi::OsString;

use crate::{symbols::FullyQualifiedName, tests::evaluate_php_buffers, types::union::DiscreteType};

#[test]
fn test_inline_doccomment() {
    let buffers: &[(OsString, OsString)] = &[(
        "test.php".into(),
        r#"<?php 

            function test_return(/** bool */ $bar) {
                return $bar;
            }
     
            "#
        .into(),
    )];
    let result = evaluate_php_buffers(buffers.to_vec(), false);
    // eprintln!("RESULT: {:?}", &result);
    if let Some(symbols) = result.symbol_data {
        let func_data = symbols.functions.read().unwrap();
        let func_name = FullyQualifiedName::from(r"\test_return");
        if let Some(func) = func_data.get(&func_name) {
            let data = func.read().unwrap();
            assert_eq!(data.inferred_return_type, Some(DiscreteType::Bool.into()));
        } else {
            assert!(false, "data of function test_return not found");
        }
    }
    // assert_eq!(result.return_type, Some(DiscreteType::String.into()));
    assert_eq!(result.issues.len(), 0);
}

#[test]
fn test_inline_returntype() {
    let buffers: &[(OsString, OsString)] = &[(
        "test.php".into(),
        r#"<?php 

            function test_return() /** : array<string> */ {
                return [];
            }
     
            "#
        .into(),
    )];
    let result = evaluate_php_buffers(buffers.to_vec(), false);
    // eprintln!("RESULT: {:?}", &result);
    if let Some(symbols) = result.symbol_data {
        let func_data = symbols.functions.read().unwrap();
        let func_name = FullyQualifiedName::from(r"\test_return");
        if let Some(func) = func_data.get(&func_name) {
            let data = func.read().unwrap();
            eprintln!("DATA: {:#?}", data);
            assert_eq!(data.inferred_return_type, Some(DiscreteType::Array.into()));
            let comment_return_type = data.comment_return_type.as_ref().map(|x| x.0.clone());
            assert_eq!(
                comment_return_type,
                Some(DiscreteType::Vector(DiscreteType::String.into()).into())
            );
        } else {
            assert!(false, "data of function test_return not found");
        }
    }
    // assert_eq!(result.return_type, Some(DiscreteType::String.into()));
    assert_eq!(result.issues.len(), 0);
}
