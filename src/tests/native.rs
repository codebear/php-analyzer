use crate::{tests::evaluate_php_code_in_function, types::union::DiscreteType};

#[test]
fn test_dom_api1() {
    let result = evaluate_php_code_in_function(
        Default::default(),
        r#"
        $node = new DOMElement('div');
        $node->setAttribute('foo', 'bar');
        return $node;
        "#,
    );
    //eprintln!("FOO: {:?}", result);
    assert_eq!(
        result.return_type,
        Some(DiscreteType::Named("DOMElement".into(), "\\DOMElement".into()).into())
    );
    assert_eq!(result.issues.len(), 0);
}

#[test]
fn test_dom_api2() {
    let result = evaluate_php_code_in_function(
        Default::default(),
        r#"

        $doc = new DOMDocument();
        $node = $doc->createElement('div');
        $node->setAttribute('foo', 'bar');
        return $node;
        "#,
    );
    //eprintln!("FOO: {:?}", result);
    assert_eq!(
        result.return_type,
        Some(DiscreteType::Named("DOMElement".into(), "\\DOMElement".into()).into())
    );
    assert_eq!(result.issues.len(), 0);
}

#[test]
fn test_dom_api3() {
    let result = evaluate_php_code_in_function(
        Default::default(),
        r#"

        $doc = new DOMDocument();
        $node = $doc->createElement('div');
        $a = $doc->createElement('a');
        $node->appendChild($a);
        return $node;
        "#,
    );
    //eprintln!("FOO: {:?}", result);
    assert_eq!(
        result.return_type,
        Some(DiscreteType::Named("DOMElement".into(), "\\DOMElement".into()).into())
    );
    assert_eq!(result.issues.len(), 0);
}
