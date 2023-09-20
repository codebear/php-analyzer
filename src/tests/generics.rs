use crate::{
    symbols::FullyQualifiedName,
    tests::evaluate_php_buffers,
    types::union::{DiscreteType, UnionType},
};
use std::ffi::OsString;

#[test]
fn test_class_template() -> Result<(), &'static str> {
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
                    return $this->noe;
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
    let result = evaluate_php_buffers(Default::default(), buffers.to_vec(), false);
    // eprintln!("RESULT: {:?}", &result);
    let symbols = result.symbol_data.ok_or("Missing symbols")?;

    let func_data = symbols.functions.read().unwrap();

    let func_name: FullyQualifiedName = r"\test_return_class".into();

    let func = func_data
        .get(&func_name)
        .ok_or("data of function test_return_class not found")?;
    {
        let data = func.read().unwrap();
        let expected = UnionType::parse_simple("X<String>".into()).unwrap();
        assert_eq!(data.inferred_return_type, Some(expected));
    }

    let func_name: FullyQualifiedName = r"\test_return_method_string".into();
    let func = func_data
        .get(&func_name)
        .ok_or("data of function test_return_method_string not found")?;
    {
        let data = func.read().unwrap();
        assert_eq!(data.inferred_return_type, Some(DiscreteType::String.into()));
    }

    let func_name: FullyQualifiedName = r"\test_return_method_int".into();
    let func = func_data
        .get(&func_name)
        .ok_or("data of function test_return_method_int not found")?;
    {
        let data = func.read().unwrap();
        assert_eq!(data.inferred_return_type, Some(DiscreteType::Int.into()));
    }
    // assert_eq!(result.return_type, Some(DiscreteType::String.into()));
    assert_eq!(result.issues.len(), 0);
    Ok(())
}

#[test]
fn test_method_template() -> Result<(), &'static str> {
    let buffers: &[(OsString, OsString)] = &[(
        "foo/myinterface.php".into(),
        r#"<?php 

            class X {
        
                /**
                 * @template TF
                 * @param TF $balle
                 * @return TF
                 */
                function what($balle) {
                    return $balle;
                }
            }

            function bar() {
                $X = new X("foo");
                return $X->what(42);
            }
            "#
        .into(),
    )];
    let result = evaluate_php_buffers(Default::default(), buffers.to_vec(), false);
    // eprintln!("RESULT: {:?}", &result);
    let symbols = result.symbol_data.ok_or("Missing symbol data")?;

    let func_data = symbols
        .functions
        .read()
        .map_err(|_| "Missing function data")?;

    // Check bar
    let bar_data = func_data
        .get(&"bar".into())
        .ok_or("Missing bar function data")?;

    {
        let bar_unlocked_data = bar_data
            .read()
            .map_err(|_| "Unable to unlock function data")?;
        let inferred_return_type = bar_unlocked_data
            .inferred_return_type
            .clone()
            .ok_or("Should have a type")?;
        assert_eq!(inferred_return_type, DiscreteType::Int.into());
    }

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
    Ok(())
}

#[test]
fn test_gen_detect_duplicate_template() -> Result<(), &'static str> {
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
                    return $this->noe;
                }

                /**
                 * @template TF
                 * @param TF $balle
                 * @return TF
                 */
                function what($balle) {
                    return $balle;
                }
            }

            function foo() {
                $X = new X("foo");
                return $X->getNoe();
            }

            function bar() {
                $X = new X("foo");
                return $X->what(42);
            }
            "#
        .into(),
    )];
    let result = evaluate_php_buffers(Default::default(), buffers.to_vec(), false);
    // eprintln!("RESULT: {:?}", &result);
    let symbols = result.symbol_data.ok_or("Missing symbol data")?;

    let func_data = symbols
        .functions
        .read()
        .map_err(|_| "Missing function data")?;

    // Check foo

    let foo_data = func_data
        .get(&"foo".into())
        .ok_or("Missing foo function data")?;

    {
        let foo_unlocked_data = foo_data
            .read()
            .map_err(|_| "Unable to unlock function data")?;
        let inferred_return_type = foo_unlocked_data
            .inferred_return_type
            .clone()
            .ok_or("Should have a type")?;
        assert_eq!(inferred_return_type, DiscreteType::String.into());
    }

    // Check bar
    let bar_data = func_data
        .get(&"bar".into())
        .ok_or("Missing bar function data")?;

    {
        let bar_unlocked_data = bar_data
            .read()
            .map_err(|_| "Unable to unlock function data")?;
        let inferred_return_type = bar_unlocked_data
            .inferred_return_type
            .clone()
            .ok_or("Should have a type")?;
        assert_eq!(inferred_return_type, DiscreteType::Int.into());
    }

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
    Ok(())
}

#[test]
fn balle() {
    /*
         * uint8_t tab[0x1ff + 1];

    uint8_t f(int32_t x)
    {
        if (x < 0)
            return 0;
        int32_t i = x * 0x1ff / 0xffff;
        if (i >= 0 && i < sizeof(tab)) {
            printf("tab[%d] looks safe because %d is between [0;%d[\n", i, i, (int)sizeof(tab));
            return tab[i];
        }

        return 0;
    }

    int main(int ac, char **av)
    {
        return f(atoi(av[1]));
    }
         */
    use std::convert::TryInto;
    let tab: [u8; 0x1ff + 1] = [0; 0x1ff + 1];

    let x: i32 = str::parse("50000000").unwrap();
    {
        if x < 0 {
            return;
        }
        let i: usize = (x * 0x1ff / 0xffff).try_into().unwrap();
        if i < tab.len().try_into().unwrap() {
            println!(
                "tab[{i}] looks safe because {i} is between [0;{}]",
                tab.len()
            );
            let f: u8 = tab[i];
            println!("Fant {}", f);
        }
    }
}

fn hent_ut_noe_basert_paa_modifisert_str_index(
    in_string: &str,
) -> Result<Option<u8>, &'static str> {
    use std::convert::TryInto;
    let tab: [u8; 0x1ff + 1] = [0; 0x1ff + 1];

    let x: u32 = str::parse(in_string)
        .map_err(|_| "Kunne ikke tolke instringen som et saklig positivt tall")?;

    let i: usize = (x * 0x1ff / 0xffff)
        .try_into()
        .map_err(|_| "Kunne ikke beregne index-pos")?;
    Ok(tab.get(i).cloned())
}
