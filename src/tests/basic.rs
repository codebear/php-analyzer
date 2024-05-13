use std::ffi::OsString;

use crate::tests::get_inferred_return_type;

#[test]
pub fn test_int_return() {
    let return_type = if let Some(res_type) = get_inferred_return_type(OsString::from(
        "
        return 42;
    ",
    )) {
        res_type
    } else {
        unreachable!("Didn't get proper type");
    };

    assert_eq!("int", &return_type.to_string());
}

#[test]
pub fn test_string_return() {
    let return_type = if let Some(res_type) = get_inferred_return_type(OsString::from(
        "
        return 'balle';
    ",
    )) {
        res_type
    } else {
        unreachable!("Didn't get proper type");
    };

    assert_eq!("string", &return_type.to_string());
}

#[test]
pub fn test_conditional_array_1_return() {
    let return_type = if let Some(res_type) = get_inferred_return_type(OsString::from(
        "
        if (rand(0,1)) {
            $arr = 4;
        } else {
            $arr = 2;
        }
        return $arr;
        ",
    )) {
        res_type
    } else {
        unreachable!("Didn't get proper type");
    };

    assert_eq!("int", &return_type.to_string());
}

#[test]
pub fn test_conditional_array_2_return() {
    let return_type = if let Some(res_type) = get_inferred_return_type(OsString::from(
        "
        if (rand(0,1)) {
            $arr = [1,2,3];
        } else {
            $arr = [4,5,6];
        }
        return $arr;
        ",
    )) {
        res_type
    } else {
        unreachable!("Didn't get proper type");
    };

    assert_eq!("array<int>", &return_type.to_string());
}

#[test]
pub fn test_conditional_array_3_return() {
    let return_type = if let Some(res_type) = get_inferred_return_type(OsString::from(
        "
        if (rand(0,1)) {
            $arr = [1,2,3];
        } else {
            $arr = [4,5,6];
        }
        foreach($arr as $y) {
            return $y;
        }
        ",
    )) {
        res_type
    } else {
        unreachable!("Didn't get proper type");
    };

    assert_eq!("int", &return_type.to_string());
}

#[test]
pub fn test_conditional_array_4_return() {
    let return_type = if let Some(res_type) = get_inferred_return_type(OsString::from(
        "
        if (rand(0,1)) {
            $arr = [1,2,3];
        } else {
            $arr = [4,5,6];
        }
        foreach($arr as $k => $_) {
            return $k;
        }
        ",
    )) {
        res_type
    } else {
        unreachable!("Didn't get proper type");
    };

    assert_eq!("int", &return_type.to_string());
}

#[test]
pub fn test_var_usage_in_str() {
    let return_type = if let Some(res_type) = get_inferred_return_type(OsString::from(
        "
        $x = 42;
        return \"foo$x\";
        ",
    )) {
        res_type
    } else {
        unreachable!("Didn't get proper type");
    };

    assert_eq!("string", &return_type.to_string());
}

#[test]
pub fn test_noe1() {
    let return_type = if let Some(res_type) = get_inferred_return_type(OsString::from(
        r#"
        $x = 0;
        return $x ? "foo" : 3.14;
        "#,
    )) {
        res_type
    } else {
        unreachable!("Didn't get proper type");
    };

    assert_eq!("double", &return_type.to_string());
}

#[test]
pub fn test_noe2() {
    let return_type = if let Some(res_type) = get_inferred_return_type(OsString::from(
        r#"
        $x = 0;
        $x++;
        return $x ? "foo" : 3.14;
        "#,
    )) {
        res_type
    } else {
        unreachable!("Didn't get proper type");
    };

    assert_eq!("string", &return_type.to_string());
}
