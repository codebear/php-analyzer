use crate::{tests::evaluate_php_code_in_function, types::union::DiscreteType};

#[test]
fn test_property_inherited() {
    let result = evaluate_php_code_in_function(
        Default::default(),
        r#"
        try {
            // void
        } catch(Exception $e) {
            return $e;
        }
        "#,
    );
    //eprintln!("FOO: {:?}", result);
    assert_eq!(
        result.return_type,
        Some(DiscreteType::Named("Exception".into(), "\\Exception".into()).into())
    );
    assert_eq!(result.issues.len(), 0);
}
