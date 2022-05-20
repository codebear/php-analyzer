use crate::{
    symbols::FullyQualifiedName,
    tests::evaluate_php_buffers,
    types::union::{DiscreteType, UnionType},
};
use std::ffi::OsString;

#[test]
fn test_gen() -> Result<(), &'static str> {
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
            function test_return_class() {
                $X = new X("foo");
                return $X;
            }  
            function test_return_method_string() {
                $X = new X("foo");
                return $X->getNoe();
            }

             function test_return_method_int() {
                $X = new X(42);
                return $X->getNoe();
            }
            "#
        .into(),
    )];
    let result = evaluate_php_buffers(buffers.to_vec(), false);
    // eprintln!("RESULT: {:?}", &result);
    let symbols = result.symbol_data.ok_or("Missing symbols")?;
    
    let func_data = symbols.functions.read().unwrap();

    let func_name: FullyQualifiedName = r"\test_return_class".into();

    let func = func_data.get(&func_name).ok_or("data of function test_return_class not found")?;
    {
        let data = func.read().unwrap();
        let expected = UnionType::parse_simple("X<String>".into()).unwrap();
        assert_eq!(data.inferred_return_type, Some(expected));
    }

    let func_name: FullyQualifiedName = r"\test_return_method_string".into();
    let func = func_data.get(&func_name).ok_or("data of function test_return_method_string not found")?;
    {
        let data = func.read().unwrap();
        assert_eq!(data.inferred_return_type, Some(DiscreteType::String.into()));
    }

    let func_name: FullyQualifiedName = r"\test_return_method_int".into();
    let func = func_data.get(&func_name).ok_or("data of function test_return_method_int not found")?;
    {
        let data = func.read().unwrap();
        assert_eq!(data.inferred_return_type, Some(DiscreteType::Int.into()));
    }
    // assert_eq!(result.return_type, Some(DiscreteType::String.into()));
    assert_eq!(result.issues.len(), 0);
    //assert!(false)}
    Ok(())
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
