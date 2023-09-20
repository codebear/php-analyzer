use std::collections::HashMap;

use crate::symbols::{FullyQualifiedName, Name};

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct TypeStruct {
    pub type_name: TypeName,
    pub generics: Option<Vec<Vec<ConcreteType>>>,
}

impl std::fmt::Display for TypeStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.type_name)?;
        if let Some(generics) = &self.generics {
            write!(f, "<")?;
            let mut i = 0;
            for gen_param in generics {
                let mut j = 0;
                if i > 0 {
                    write!(f, ", ")?;
                }
                i += 1;
                for utype_part in gen_param {
                    if j > 0 {
                        write!(f, " | ")?;
                    }
                    j += 1;
                    write!(f, "{}", utype_part)?;
                }
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ShapeKey {
    String(Name),
    Num(i64),
}

impl std::fmt::Display for ShapeKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShapeKey::String(s) => write!(f, "{}", s),
            ShapeKey::Num(n) => write!(f, "{}", n),
        }
    }
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

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct ShapeEntry(pub Option<(ShapeKey, bool)>, pub Vec<ConcreteType>);

impl std::fmt::Display for ShapeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((k, optional)) = &self.0 {
            write!(f, "{}", &k)?;
            if *optional {
                write!(f, "?")?;
            }
        }
        let x = 0;
        for v in &self.1 {
            if x > 0 {
                write!(f, " | ")?;
            }
            write!(f, "{}", &v)?
        }
        Ok(())
    }
}

pub type UnionOfTypes = Vec<ConcreteType>;

pub type ArgumentVector = Vec<UnionOfTypes>;

pub type ReturnType = UnionOfTypes;

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum ParsedType {
    Type(TypeStruct),
    Shape(Vec<ShapeEntry>),
    Callable(ArgumentVector, Option<ReturnType>),
    ClassType(TypeName, Name),
    CallableUntyped,
}

impl std::fmt::Display for ParsedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsedType::Type(t) => write!(f, "{}", t),
            ParsedType::Shape(s) => {
                write!(f, "array{{")?;
                let mut i = 0;
                for x in s {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    i += 1;
                    write!(f, "{}", x)?;
                }
                write!(f, "}}")
            }
            ParsedType::Callable(argv, rettype) => write!(f, "callable({:?}):{:?}", argv, rettype),
            ParsedType::ClassType(typename, name) => write!(f, "{}::{}", typename, name),
            ParsedType::CallableUntyped => write!(f, "callable"),
        }
    }
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum TypeName {
    Name(Name),
    FQName(FullyQualifiedName),
    RelativeName(Vec<Name>),
}

impl std::fmt::Display for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TypeName::Name(n) => write!(f, "{}", &n),
            TypeName::FQName(fq) => write!(f, "{}", &fq),
            TypeName::RelativeName(n) => {
                write!(f, "RELATIVENAME:")?;
                for x in n {
                    write!(f, "/{}", x)?;
                }
                Ok(())
            }
        }
    }
}
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct ConcreteType {
    pub nullable: bool,
    pub ptype: ParsedType,
}

impl std::fmt::Display for ConcreteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.nullable {
            write!(f, "?")?;
        }
        write!(f, "{}", &self.ptype)
    }
}
