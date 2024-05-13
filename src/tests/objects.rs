use std::ffi::OsString;

use crate::{
    symbols::FullyQualifiedName,
    tests::{evaluate_php_buffers, get_inferred_return_type},
    types::union::DiscreteType,
    value::PHPValue,
};

use super::evaluate_php_code_in_function;

#[test]
fn object_type() {
    let ret_type = if let Some(_type) = get_inferred_return_type(
        "

        class Balle {
            /**
             * @var array<int>
             */
            public $foo = [];
        }

        $x = new Balle();
       
        return $x;
    ",
    ) {
        _type
    } else {
        assert!(false, "Didn't get proper type");
        return;
    };

    if let Some(DiscreteType::Named(_, fq)) = ret_type.single_type() {
        assert_eq!(FullyQualifiedName::from("\\Balle"), fq);
    } else {
        assert!(false);
    }
}

#[test]
fn object_method_call_1() {
    let result = evaluate_php_code_in_function(
        Default::default(),
        "

        class Balle {
            function ick() {
            }
        }

        $x = new Balle();
        $x->ick();
    ",
    );

    assert_eq!(0, result.issues.len());
}

#[test]
fn object_method_call_2() {
    let res = evaluate_php_code_in_function(
        Default::default(),
        "

        class Balle {
            function ick() {
                return 42;
            }
        }

        function xx(Balle $x) {
            return $x->ick();
        }

        $x = new Balle();
        return xx($x);
    ",
    );
    let ret_type = if let Some(_type) = res.return_type {
        _type
    } else {
        assert!(false, "Didn't get proper type");
        return;
    };
    assert_eq!(Some(DiscreteType::Int), ret_type.single_type());
}

#[test]
fn object_method_call_inherited() {
    let result = evaluate_php_code_in_function(
        Default::default(),
        "

        class A {
            function ick() {
                return 42;
            }
        }

        class B extends A {
            
        }

        $x = new B();
        return $x->ick();
    ",
    );
    // eprintln!("RES: {:?}", result);
    assert_eq!(0, result.issues.len(), "Expects no issues to be emitted");
    assert_eq!(
        Some(DiscreteType::Int),
        result.return_type.and_then(|x| x.single_type())
    );
    // assert_eq!(Some(PHPValue::Int(42)), result.return_value);
}

#[test]
fn test_static_member_variable() {
    let result = evaluate_php_code_in_function(
        Default::default(),
        r#"
        class X {
            static $foo = 42;
            static $bar = "foo";
            static $baz = 3.14;
        }
        $foo = 0;
        X::$foo++;
        echo X::$bar;
        return $foo;
    "#,
    );
    assert_eq!(result.return_value, Some(PHPValue::Int(0)));
    assert_eq!(result.issues.len(), 0);
}

#[test]
fn test_this_type() {
    let result = evaluate_php_code_in_function(
        Default::default(),
        r#"
        class X {
            function foo() {
                return $this;
            }
        }
        $X = new X();
        return $X->foo();
    "#,
    );
    assert_eq!(
        result.return_type,
        Some(DiscreteType::Named("X".into(), "\\X".into()).into())
    );
    assert_eq!(result.issues.len(), 0);
}

#[test]
fn test_property_native_type() {
    let result = evaluate_php_code_in_function(
        Default::default(),
        r#"
        class X {
            public int $foo = 0;
        }
        $X = new X();
        return $X->foo;
    "#,
    );
    //eprintln!("FOO: {:?}", result);
    assert_eq!(result.return_type, Some(DiscreteType::Int.into()));
    assert_eq!(result.issues.len(), 0);
}

#[test]
fn test_property_doccomment_native_type() {
    let result = evaluate_php_code_in_function(
        Default::default(),
        r#"
        class X {
            /**
             * @var string
             */
            public $bar = "foo";
        }
        $X = new X();
        return $X->bar;
    "#,
    );

    assert_eq!(result.return_type, Some(DiscreteType::String.into()));
    assert_eq!(result.issues.len(), 0);
}

#[test]
fn test_property_doccomment_class_type() {
    let result = evaluate_php_code_in_function(
        Default::default(),
        r#"
        class Y {
            function ick(): string {
                return "foo";
            }
        }
        class X {
            /**
             * @var Y
             */
            public $bar;
            function __construct() {
                $this->bar = new Y();
            }
        }
        $X = new X();
        return $X->bar->ick();
    "#,
    );
    //eprintln!("BAR: {:?}", result);

    assert_eq!(result.return_type, Some(DiscreteType::String.into()));
    assert_eq!(result.issues.len(), 0);
}

#[test]
fn test_property_doccomment_class_type_in_ns() {
    let buffers: &[(OsString, OsString)] = &[(
        r"".into(),
        r#"<?php
        namespace na\me\sp\ace;

        class Y {
            function ick(): string {
                return "foo";
            }
        }

        class X {
            /**
             * @var Y
             */
            public $bar;
            function __construct() {
                $this->bar = new Y();
            }

            function z() {
                return $this->bar->ick();
            }
        }

        function test_output() {
            $X = new X();
            return $X->z();
        }
    "#
        .into(),
    )];

    let result = evaluate_php_buffers(Default::default(), buffers.to_vec(), false);

    // eprintln!("BAR: {:#?}", result);
    assert_eq!(result.issues.len(), 0);

    if let Some(symbols) = result.symbol_data {
        let functions = symbols.functions.read().unwrap();
        let func_name = FullyQualifiedName::from(r"\na\me\sp\ace\test_output");
        let func = functions.get(&func_name);
        if let Some(func_data_handle) = func {
            let func_data = func_data_handle.read().unwrap();
            assert_eq!(
                func_data.inferred_return_type,
                Some(DiscreteType::String.into())
            );
        } else {
            unreachable!("mangler func_data");
        }
    } else {
        unreachable!("mangler symbol_data");
    }
}

#[test]
fn test_property_inherited() {
    let result = evaluate_php_code_in_function(
        Default::default(),
        r#"
        class A {
            public int $foo = 0;
        }
        class B extends A {}
        $B = new B();
        return $B->foo;
    "#,
    );
    //eprintln!("FOO: {:?}", result);
    assert_eq!(result.return_type, Some(DiscreteType::Int.into()));
    assert_eq!(result.issues.len(), 0);
}
