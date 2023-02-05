use std::ffi::OsString;

use crate::tests::evaluate_php_buffers;

#[test]
fn test_hardening_true() {
    let buffers: &[(OsString, OsString)] = &[(
        "foo/myinterface.php".into(),
        r#"<?php 

            class X {
                function foo() {}
            }
            function foo(X $x = null) {
                if ($x) {
                    $x->foo();
                }
            }
            "#
        .into(),
    )];
    let result = evaluate_php_buffers(Default::default(), buffers.to_vec(), false);
    eprintln!("RESULT: {:#?}", &result);
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
    //assert!(false)
}

#[test]
fn test_hardening_false() {
    let buffers: &[(OsString, OsString)] = &[(
        "foo/myinterface.php".into(),
        r#"<?php 

            class X {
                function foo() {}
            }   
            function foo(X $x = null) {
                if (!$x) {
                    echo "foo";
                } else {
                    $x->foo();
                }
            }
            "#
        .into(),
    )];
    let result = evaluate_php_buffers(Default::default(), buffers.to_vec(), false);
    eprintln!("RESULT: {:#?}", &result);
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
    //assert!(false)
}

#[test]
fn test_hardening_single_false_branch() {
    let buffers: &[(OsString, OsString)] = &[(
        "foo/myinterface.php".into(),
        r#"<?php 

            class X {
                function foo() {}
            }   
            function foo(X $x = null) {
                if (!$x) {
                    $x = new X();
                }
                $x->foo();
                
            }
            "#
        .into(),
    )];
    let result = evaluate_php_buffers(Default::default(), buffers.to_vec(), false);
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
    //assert!(false)
}

#[test]
fn test_hardening_instanceoof() {
    let buffers: &[(OsString, OsString)] = &[(
        "foo/myinterface.php".into(),
        r#"<?php 

            class X {
                function foo() {}
            }   
            function foo(X $x = null) {
                if ($x instanceof X) {
                    $x->foo();
                }
            }
            "#
        .into(),
    )];
    let result = evaluate_php_buffers(Default::default(), buffers.to_vec(), false);
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
    //assert!(false)
}

#[test]
fn test_hardening_assignment() {
    let buffers: &[(OsString, OsString)] = &[(
        "foo/myinterface.php".into(),
        r#"<?php 

            class X {
                function foo() {}
            }   
            function foo(X $x = null) {
                if ($y = $x) {
                    $y->foo();
                }
            }
            "#
        .into(),
    )];
    let result = evaluate_php_buffers(Default::default(), buffers.to_vec(), false);
    eprintln!("RESULT: {:#?}", &result);
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
    //assert!(false)
}

///
/// This one is/was PHAN strugling with...
#[test]
fn test_hardening_instanceoof_multipath_1() {
    let buffers: &[(OsString, OsString)] = &[(
        "foo/myinterface.php".into(),
        r#"<?php 

            class X {
                function foo() {}
            }   
            interface I {
                function bar();
            }
            function foo(X $x = null) {
                if ($x instanceof I) {
                    // Since X not implements I, we fallback to
                    // handling this branch as $x being of only type I
                    // (which is the only thing that makes sense)
                    // so it should emit foo() as unknown method
                    $x->foo();
                    $x->bar();
                }
            }
            "#
        .into(),
    )];
    let result = evaluate_php_buffers(Default::default(), buffers.to_vec(), false);
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
    //assert!(false)
}

///
/// This one is/was PHAN strugling with...
#[test]
fn test_hardening_instanceoof_multipath_2() {
    let buffers: &[(OsString, OsString)] = &[(
        "foo/myinterface.php".into(),
        r#"<?php 

            interface I {
                function bar();
            }
            class X {
                function foo() {}
            }   
            class Y extend X implement I {
                
            }
            function foo(X $x = null) {
                if ($x instanceof I) {
                    // Since X not implements I, we fallback to
                    // handling this branch as $x being of only type I
                    // (which is the only thing that makes sense)
                    // so it should emit foo() as unknown method
                    $x->foo();
                    $x->bar();
                }
            }
            "#
        .into(),
    )];
    let result = evaluate_php_buffers(Default::default(), buffers.to_vec(), false);
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
    //assert!(false)
}
