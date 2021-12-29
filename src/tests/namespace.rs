use std::ffi::OsString;

use crate::{
    symbols::{FullyQualifiedName, Name},
    tests::evaluate_php_buffers,
    types::union::DiscreteType,
};

#[test]
fn test_namespace_references() {
    let buffers: &[(OsString, OsString)] = &[
        (
            "foo/Y.php".into(),
            r#"<?php 

            namespace foo;
            class Y {
                function ick(): string {
                    return "foo";
                }
            }
            "#
            .into(),
        ),
        (
            "bar/X.php".into(),
            r#"<?php 

            namespace bar;

            use \foo\Y;

            class X {
                /**
                 * @var Y
                 */
                public $bar;
                /**
                 * @var Y
                 */
                private $baz;
                function __construct() {
                    $this->bar = new Y();
                }

                function z() {
                    $this->baz->ick();
                }
            }
            "#
            .into(),
        ),
        (
            "test.php".into(),
            r"<?php 
            use \bar\X;

            function test_return() {
                $X = new X();
                return $X->bar->ick();
            }
            
            "
            .into(),
        ),
    ];
    let result = evaluate_php_buffers(buffers.to_vec(), false);
    // eprintln!("RESULT: {:?}", &result);
    if let Some(symbols) = result.symbol_data {
        let func_data = symbols.functions.read().unwrap();
        let func_name = FullyQualifiedName::from(r"\test_return");
        if let Some(func) = func_data.get(&func_name) {
            let data = func.read().unwrap();
            assert_eq!(data.inferred_return_type, Some(DiscreteType::String.into()));
        } else {
            assert!(false, "data of function test_return not found");
        }
    }
    // assert_eq!(result.return_type, Some(DiscreteType::String.into()));
    assert_eq!(result.issues.len(), 0);
}

#[test]
fn test_namespace_interface_references() {
    let buffers: &[(OsString, OsString)] = &[
        (
            "foo/Y.php".into(),
            r#"<?php 

            namespace foo;
            interface Y {
                function ick(): string;
            }
            "#
            .into(),
        ),
        (
            "foo/YImpl.php".into(),
            r#"<?php

            namespace foo;
            class  YImpl implements Y {
                function ick(): string {
                    return "foo";
                }
            }
            "#
            .into(),
        ),
        (
            "bar/X.php".into(),
            r#"<?php 

            namespace bar;

            use \foo\Y;
            use \foo\YImpl;
        
            class X {
                /**
                 * @var Y
                 */
                public $bar;

                /**
                 * @var Y
                */
                private $baz;
                function __construct() {
                    $this->bar = new YImpl();
                    $this->baz = new YImpl();
                }

                function x() {
                    $this->bar->ick();
                }

                function z() {
                    $this->baz->ick();
                }
            }
            "#
            .into(),
        ),
        (
            "test.php".into(),
            r"<?php 
            use \bar\X;

            function test_return() {
                $X = new X();
                return $X->bar->ick();
            }
            
            "
            .into(),
        ),
    ];
    let result = evaluate_php_buffers(buffers.to_vec(), false);
    // eprintln!("RESULT: {:?}", &result);
    if let Some(symbols) = result.symbol_data {
        let func_data = symbols.functions.read().unwrap();
        let func_name = FullyQualifiedName::from(r"\test_return");
        if let Some(func) = func_data.get(&func_name) {
            let data = func.read().unwrap();
            assert_eq!(data.inferred_return_type, Some(DiscreteType::String.into()));
        } else {
            // eprintln!("RESULT: {:?}", result);
            assert!(false, "data of function test_return not found");
        }
    }
    // assert_eq!(result.return_type, Some(DiscreteType::String.into()));
    assert_eq!(result.issues.len(), 0);
}

#[test]
fn test_new_class_type_in_ns() {
    let buffers: &[(OsString, OsString)] = &[(
        r"foo.php".into(),
        r#"<?php
        namespace na\me\sp\ace;

        class X {

        }
        function test_output(){
            return new X();
        }

    "#
        .into(),
    )];

    let result = evaluate_php_buffers(buffers.to_vec(), false);
    assert_eq!(result.issues.len(), 0);
    let fname = FullyQualifiedName::from(r"\na\me\sp\ace\test_output");
    if let Some(func_name) = result.symbol_data.and_then(|x| x.get_function(&fname)) {
        assert_eq!(
            func_name.inferred_return_type,
            Some(
                DiscreteType::Named(
                    Name::from("X"),
                    FullyQualifiedName::from(r"\na\me\sp\ace\X")
                )
                .into()
            )
        );
    } else {
        assert!(false, "funksjonen mangler");
    }
}
