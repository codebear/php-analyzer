use crate::{tests::get_inferred_return_value, value::PHPValue};

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
        assert!(false, "Didn't get proper value");
        return;
    };
    assert_eq!(&PHPValue::Int(3), &returned_value);
}
