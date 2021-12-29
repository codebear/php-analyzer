use crate::{types::union::DiscreteType, value::PHPValue};
use std::ffi::OsString;

use crate::tests::get_inferred_return_value;

use super::evaluate_php_code_in_function;

#[test]
fn return_int_42() {
    let returned_value = if let Some(value) = get_inferred_return_value(OsString::from(
        "
        return 42;
    ",
    )) {
        value
    } else {
        assert!(false, "Didn't get proper value");
        return;
    };

    assert_eq!(&PHPValue::Int(42), &returned_value);
}

#[test]
fn return_int_via_var() {
    let returned_value = if let Some(value) = get_inferred_return_value(OsString::from(
        "
        $x = 42;
        return $x;
    ",
    )) {
        value
    } else {
        assert!(false, "Didn't get proper value");
        return;
    };

    assert_eq!(&PHPValue::Int(42), &returned_value);
}

#[test]
fn return_var_incremented() {
    let returned_value = if let Some(value) = get_inferred_return_value(OsString::from(
        "
        $x = 42;
        $x++;
        return $x;
    ",
    )) {
        value
    } else {
        assert!(false, "Didn't get proper value");
        return;
    };

    assert_eq!(&PHPValue::Int(43), &returned_value);
}

#[test]
fn return_var_postfix_increment() {
    let returned_value = if let Some(value) = get_inferred_return_value(OsString::from(
        "
        $x = 42;
        return $x++;
    ",
    )) {
        value
    } else {
        assert!(false, "Didn't get proper value");
        return;
    };

    assert_eq!(&PHPValue::Int(42), &returned_value);
}

#[test]
fn balle() {
    let noe = evaluate_php_code_in_function(
        r#"
    switch(rand(0,10)) {
        case 42;
            $type = 'foo';
            break;
        default:
            $type = 'bar';
    }
    echo "noe $type greier";
    "#,
    );

    assert_eq!(0, noe.issues.len(), "expected to emit 0 issues");
}

#[test]
fn test_return_inline_string_variable() {
    let noe = evaluate_php_code_in_function(
        r#"
            $type = 'bar';
            return "noe $type greier";
        "#,
    );

    assert_eq!(0, noe.issues.len(), "expected to emit 0 issues");
    assert_eq!(Some(DiscreteType::String.into()), noe.return_type);
}

#[test]
fn test_echo_inline_string_variable() {
    let noe = evaluate_php_code_in_function(
        r#"
            $type = 'bar';
            echo "noe $type greier";
        "#,
    );

    assert_eq!(0, noe.issues.len(), "expected to emit 0 issues");
}
