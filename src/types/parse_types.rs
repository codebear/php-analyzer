use std::collections::HashMap;

use crate::symbols::{Name, FullyQualifiedName};




#[derive(Clone, Debug)]
pub struct TypeStruct {
    pub type_name: TypeName,
    pub generics: Option<Vec<Vec<ConcreteType>>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ShapeKey {
    String(Name),
    Num(i64),
}

#[derive(Clone, Debug)]
pub struct ShapeStruct {
    pub map: HashMap<ShapeKey, ConcreteType>,
}

impl ShapeStruct {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ShapeEntry(pub Option<(ShapeKey, bool)>, pub Vec<ConcreteType>);


pub type UnionOfTypes = Vec<ConcreteType>;

pub type ArgumentVector = Vec<UnionOfTypes>;

pub type ReturnType = UnionOfTypes;

#[derive(Clone, Debug)]
pub enum ParsedType {
    Type(TypeStruct),
    Shape(Vec<ShapeEntry>),
    Callable(ArgumentVector, Option<ReturnType>),
    CallableUntyped,
}

#[derive(Clone, Debug)]
pub enum TypeName {
    Name(Name),
    FQName(FullyQualifiedName),
    RelativeName(Vec<Name>),
}


#[derive(Clone, Debug)]
pub struct ConcreteType {
    pub nullable: bool,
    pub ptype: ParsedType,
}