use std::ffi::OsString;

use crate::{
    issue::Issue,
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
    let result = evaluate_php_buffers(Default::default(), buffers.to_vec(), false);
    // eprintln!("RESULT: {:?}", &result);
    if let Some(symbols) = result.symbol_data {
        let func_data = symbols.functions.read().unwrap();
        let func_name = FullyQualifiedName::from(r"\test_return");
        if let Some(func) = func_data.get(&func_name) {
            let data = func.read().unwrap();
            assert_eq!(data.inferred_return_type, Some(DiscreteType::String.into()));
        } else {
            unreachable!("data of function test_return not found");
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
    let result = evaluate_php_buffers(Default::default(), buffers.to_vec(), false);
    // eprintln!("RESULT: {:?}", &result);
    if let Some(symbols) = result.symbol_data {
        let func_data = symbols.functions.read().unwrap();
        let func_name = FullyQualifiedName::from(r"\test_return");
        if let Some(func) = func_data.get(&func_name) {
            let data = func.read().unwrap();
            assert_eq!(data.inferred_return_type, Some(DiscreteType::String.into()));
        } else {
            // eprintln!("RESULT: {:?}", result);
            unreachable!("data of function test_return not found");
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

    let result = evaluate_php_buffers(Default::default(), buffers.to_vec(), false);
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
        unreachable!("funksjonen mangler");
    }
}

#[test]
fn test_namespace_and_root_class_ref() -> Result<(), &'static str> {
    let buffers: &[(OsString, OsString)] = &[
        (
            r"RootClass.php".into(),
            r#"<?php

        class RootClass {

        }
       

    "#
            .into(),
        ),
        (
            r"bar/baz.php".into(),
            r#"<?php
        namespace bar;

        use RootClass;

        class X {
            /**
             * @desc Intentionally cased wrong
             * @var Rootclass
             */
            private $rootClass;
            function getRootSomething() {
                return $this->rootClass;
            }
        }
        function test_new_x(){
            return new X();
        }
        function test_new_root_class(){
            return new RootClass();
        }

        function test_return_root_class(RootClass $noe) {
            return $noe;
        }

        function get_root_class() {
            $X = new X();
            return $X->getRootSomething();
        }

    "#
            .into(),
        ),
    ];

    let result = evaluate_php_buffers(Default::default(), buffers.to_vec(), false);
    // should have one issue on bad casing
    assert_eq!(result.issues.len(), 1);
    for issue in result.issues {
        match issue {
            Issue::WrongClassNameCasing(_, _, _) => assert!(true),
            _ => unreachable!("Should only be a WrongClassNameCasing issue here"),
        }
    }
    let fname = FullyQualifiedName::from(r"\bar\test_new_x");

    let symbol_data = result.symbol_data.ok_or("Symbol data missing")?;
    let func_data = symbol_data
        .get_function(&fname)
        .ok_or("function \\bar\\test_new_x missing")?;

    assert_eq!(
        func_data.inferred_return_type,
        Some(DiscreteType::Named(Name::from("X"), FullyQualifiedName::from(r"\bar\X")).into())
    );

    let fname = FullyQualifiedName::from(r"\bar\test_return_root_class");
    let func_data = symbol_data
        .get_function(&fname)
        .ok_or("function \\bar\\test_return_root_class is missing")?;

    assert_eq!(
        func_data.inferred_return_type,
        Some(
            DiscreteType::Named(
                Name::from("RootClass"),
                FullyQualifiedName::from(r"\RootClass")
            )
            .into()
        )
    );

    let fname = FullyQualifiedName::from(r"\bar\test_new_root_class");
    let func_data = symbol_data
        .get_function(&fname)
        .ok_or("function \\bar\\test_new_root_class is missing")?;

    assert_eq!(
        func_data.inferred_return_type,
        Some(
            DiscreteType::Named(
                Name::from("RootClass"),
                FullyQualifiedName::from(r"\RootClass")
            )
            .into()
        )
    );

    let fname = FullyQualifiedName::from(r"\bar\get_root_class");
    let func_data = symbol_data
        .get_function(&fname)
        .ok_or("function \\bar\\get_root_class is missing")?;

    assert_eq!(
        func_data.inferred_return_type,
        Some(
            DiscreteType::Named(
                Name::from("RootClass"),
                FullyQualifiedName::from(r"\RootClass")
            )
            .into()
        )
    );

    Ok(())
}

#[test]
fn test_new_class_type_with_non_fq_name() {
    let buffers: &[(OsString, OsString)] = &[
        (
            r"na/me/sp/ace/X.php".into(),
            r#"<?php
        namespace na\me\sp\ace;

        class X {

        }

    "#
            .into(),
        ),
        (
            r"na/me/foo.php".into(),
            r#"<?php
        namespace na\me;

        function test_output(){
            return new sp\ace\X();
        }

        function test_output2(){
            return new \sp\ace\X();
        }

    "#
            .into(),
        ),
    ];

    let result = evaluate_php_buffers(Default::default(), buffers.to_vec(), false);
    assert_eq!(result.issues.len(), 0);
    let fname = FullyQualifiedName::from(r"\na\me\test_output");
    let symbol_data = result.symbol_data.as_ref().unwrap();
    if let Some(func_name) = symbol_data.get_function(&fname) {
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
        unreachable!("funksjonen mangler");
    }
    let fname = FullyQualifiedName::from(r"\na\me\test_output2");
    if let Some(func_name) = symbol_data.get_function(&fname) {
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
        unreachable!("funksjonen mangler");
    }
}
