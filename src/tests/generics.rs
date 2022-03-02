use crate::tests::evaluate_php_buffers;
use std::ffi::OsString;

#[test]
fn test_gen() {
    let buffers: &[(OsString, OsString)] = &[(
        "foo/myinterface.php".into(),
        r#"<?php 
            /**
             * @template T
             */
            class X {
                /**
                 * @var T 
                 */
                private $noe;
                /**
                 * @param T $noe
                 */
                function __construct($noe) {
                    $this->noe = $noe;
                }
                /**
                 * @return T
                 */
                function getNoe() {

                }
            }   
            function foo() {
                $X = new X("foo");
                return $X->getNoe();
            }
            "#
        .into(),
    )];
    let result = evaluate_php_buffers(buffers.to_vec(), false);
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
    //assert!(false)}
}

#[test]
fn test_gen_detect_duplicate_template() {
    let buffers: &[(OsString, OsString)] = &[(
        "foo/myinterface.php".into(),
        r#"<?php 
            /**
             * @template T
             */
            class X {
                /**
                 * @var T 
                 */
                private $noe;
                /**
                 * @param T $noe
                 */
                function __construct($noe) {
                    $this->noe = $noe;
                }
                /**
                 * @template T
                 * @param T $balle
                 */
                function what($balle) {

                }
            }   
            function foo() {
                $X = new X("foo");
                return $X->what(42);
            }
            "#
        .into(),
    )];
    let result = evaluate_php_buffers(buffers.to_vec(), false);
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
    //assert!(false)}
}
