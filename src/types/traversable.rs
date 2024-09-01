use std::sync::Arc;

use crate::{symboldata::SymbolData, symbols::FullyQualifiedName, types::union::DiscreteType};

use super::union::{DiscretlyAccessedType, PHPType, UnionType};

pub fn get_key_type(traversable_type: &PHPType, symbol_data: Arc<SymbolData>) -> Option<PHPType> {
    let mut types = UnionType::new();

    for datype in traversable_type.as_discrete_variants() {
        match datype {
            DiscretlyAccessedType::Discrete(ttype) => {
                match get_key_type_from_discrete_type(&ttype, symbol_data.clone()) {
                    Some(t) => types.append(t),
                    None => {
                        return crate::missing_none!(
                            "Failed to extract key-type from traversable-type 1"
                        )
                    }
                };
            }
            DiscretlyAccessedType::Intersection(_) => {
                return crate::missing_none!("Failed to extract key-type from traversable-type 2")
            }
        }
    }

    if !types.is_empty() {
        Some(types.into())
    } else {
        None
    }
}

fn get_key_type_from_discrete_type(
    ttype: &DiscreteType,
    symbol_data: Arc<SymbolData>,
) -> Option<PHPType> {
    // todo!("Get key from {:?}", utype);

    match ttype {
        // Vectors are indiced by int
        DiscreteType::Vector(_) => Some(DiscreteType::Int.into()),
        DiscreteType::HashMap(k, _) => Some(k.clone()),
        DiscreteType::Generic(gen, args) => match &**gen {
            DiscreteType::Array if args.len() == 1 => Some(DiscreteType::Int.into()),
            DiscreteType::Array if args.len() == 2 => Some(args[0].clone()),
            DiscreteType::Named(_, fqname) => {
                get_key_type_from_class_type(fqname, args, symbol_data)
            }

            _ => crate::missing_none!("Extracting key-type from traversable-type gave generic"),
        },
        d => crate::missing_none!("Extracting key_type from traversable-type gave {:?}", d),
    }
}

pub fn get_value_type(traversable_type: &PHPType, symbol_data: Arc<SymbolData>) -> Option<PHPType> {
    let mut types = UnionType::new();
    for datype in traversable_type.as_discrete_variants() {
        match datype {
            DiscretlyAccessedType::Discrete(ttype) => {
                match get_value_type_from_discrete_type(&ttype, symbol_data.clone()) {
                    Some(t) => types.append(t),
                    None => {
                        return crate::missing_none!(
                            "Failed to extract value-type from traversable-type 1"
                        )
                    }
                };
            }
            DiscretlyAccessedType::Intersection(_) => {
                return crate::missing_none!("Failed to extract value-type from traversable-type 2")
            }
        };
    }

    if !types.is_empty() {
        Some(types.into())
    } else {
        None
    }
}

fn get_value_type_from_discrete_type(
    traversable_type: &DiscreteType,
    symbol_data: Arc<SymbolData>,
) -> Option<PHPType> {
    match traversable_type {
        // Vectors are indiced by int
        DiscreteType::Vector(v) => Some(v.clone()),
        DiscreteType::HashMap(_, v) => Some(v.clone()),
        DiscreteType::Named(_, _) => {
            crate::missing_none!("Need to extract a value_type from a class-type , Perhaps it's traversable or similar")
        }
        DiscreteType::Generic(gen, args) => match &**gen {
            DiscreteType::Array if args.len() == 1 => Some(args[0].clone()),
            DiscreteType::Array if args.len() == 2 => Some(args[1].clone()),
            DiscreteType::Named(_, fqname) => {
                get_value_type_from_class_type(fqname, args, symbol_data)
            }
            _ => {
                crate::missing_none!("Need to extract a value_type from a generic-type , Perhaps it's traversable or similar")
            }
        },
        DiscreteType::Int | DiscreteType::Float | DiscreteType::Bool => None,

        // FIXME determine if we are certain that this is correct for String
        DiscreteType::String => None,

        DiscreteType::Array => {
            // crate::missing_none!("Extracting value_type from traversable-type of {:?}", arr)
            // We have an array of unknown type
            None
        }

        d => {
            crate::missing_none!("Extracting value_type from traversable-type of {:?}", d)
        }
    }
}

fn get_value_type_from_class_type(
    fq_name: &FullyQualifiedName,
    generic_args: &Vec<PHPType>,
    symbol_data: Arc<SymbolData>,
) -> Option<PHPType> {
    let locked_cdata = symbol_data.get_class(&fq_name.into())?;
    let cdata = locked_cdata.read().ok()?;
    let cdata = cdata.with_generic_args(generic_args);
    if cdata.implements(
        &FullyQualifiedName::from("\\Iterator").into(),
        symbol_data.clone(),
    ) {
        crate::missing_none!("Fant Iterator interface, slå opp return-typen fra ->next()-metoden")
    } else if cdata.implements(
        &FullyQualifiedName::from("\\IteratorAggregate").into(),
        symbol_data.clone(),
    ) {
        crate::missing_none!(
            "Fant IteratorAggregate, Hent ut iterator, og slå opp return-type fra ->next()-metoden"
        )
    } else if cdata.implements(
        &FullyQualifiedName::from("\\Traversable").into(),
        symbol_data.clone(),
    ) {
        crate::missing_none!("Fant traversable, Håndter traversable")
    } else if cdata.implements(&FullyQualifiedName::from("\\Iterable").into(), symbol_data) {
        crate::missing_none!("Fant Iterable, Håndter Iterable")
    } else {
        crate::missing_none!(
        "Need to extract a value_type from a generic-type , Perhaps it's traversable or similar"
    )
    }
}

fn get_key_type_from_class_type(
    _fq_name: &FullyQualifiedName,
    _generic_args: &[PHPType],
    _symbol_data: Arc<SymbolData>,
) -> Option<PHPType> {
    // todo!("BALLE2: {:?}", fq_name);
    crate::missing_none!(
        "Need to extract a value_type from a generic-type , Perhaps it's traversable or similar"
    )
}
