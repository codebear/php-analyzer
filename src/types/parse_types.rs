use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use crate::symbols::{FullyQualifiedName, Name};

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct TypeStruct {
    pub type_name: TypeName,
    pub generics: Option<Vec<CompoundType>>,
}

impl std::fmt::Display for TypeStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.type_name)?;
        if let Some(generics) = &self.generics {
            write!(f, "<")?;
            for (i, gen_param) in generics.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", gen_param)?;
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

impl Default for ShapeStruct {
    fn default() -> Self {
        Self::new()
    }
}

impl ShapeStruct {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct ShapeEntry(pub Option<(ShapeKey, bool)>, pub CompoundType);

impl Display for ShapeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((k, optional)) = &self.0 {
            write!(f, "{}", &k)?;
            if *optional {
                write!(f, "?")?;
            }
        }
        Display::fmt(&self.1, f)
    }
}

pub type UnionOfTypes = Vec<ConcreteType>;

pub type IntersectionOfTypes = Vec<ConcreteType>;

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum CompoundType {
    Union(UnionOfTypes),
    Intersection(IntersectionOfTypes),
    Single(ConcreteType),
}

impl CompoundType {
    pub fn len(&self) -> usize {
        match self {
            CompoundType::Union(u) => u.len(),
            CompoundType::Intersection(i) => i.len(),
            CompoundType::Single(_) => 1,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn first(&self) -> Option<&ConcreteType> {
        match self {
            CompoundType::Union(u) => u.first(),
            CompoundType::Intersection(i) => i.first(),
            CompoundType::Single(s) => Some(s),
        }
    }

    pub(crate) fn if_single_type(&self) -> Option<&ConcreteType> {
        match self {
            CompoundType::Union(u) if u.len() == 1 => Some(&u[0]),
            CompoundType::Intersection(i) if i.len() == 1 => {
                crate::missing!("Probably wrong as an intersection could evaluate to single type?");
                Some(&i[0])
            }
            CompoundType::Single(x) => Some(x),
            _ => None,
        }
    }
}

impl Display for CompoundType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompoundType::Union(u) => u.fmt(f),
            CompoundType::Intersection(i) => i.fmt(f),
            CompoundType::Single(s) => write!(f, "{}", s),
        }
    }
}

impl IntoIterator for CompoundType {
    type Item = ConcreteType;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            CompoundType::Union(u) => u.into_iter(),
            CompoundType::Intersection(i) => i.into_iter(),
            CompoundType::Single(s) => vec![s].into_iter(),
        }
    }
}

pub type ArgumentVector = Vec<CompoundType>;

pub type ReturnType = CompoundType;

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum ParsedType {
    Type(TypeStruct),
    Shape(Vec<ShapeEntry>),
    Callable(ArgumentVector, Option<Box<ReturnType>>),
    ClassType(TypeName, Name),
    CallableUntyped,
    Parenthesized(Box<CompoundType>),
}

impl std::fmt::Display for ParsedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsedType::Type(t) => write!(f, "{}", t),
            ParsedType::Shape(s) => {
                write!(f, "array{{")?;
                for (i, x) in s.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", x)?;
                }
                write!(f, "}}")
            }
            ParsedType::Callable(argv, rettype) => write!(f, "callable({:?}):{:?}", argv, rettype),
            ParsedType::ClassType(typename, name) => write!(f, "{}::{}", typename, name),
            ParsedType::CallableUntyped => write!(f, "callable"),
            ParsedType::Parenthesized(pt) => write!(f, "({})", pt),
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
