use std::{
    ffi::{OsStr, OsString},
    os::unix::prelude::OsStrExt, fmt::Display,
};

use crate::{
    symboldata::class::ClassName,
    types::union::{DiscreteType, UnionType, SpecialType},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Name(OsString);

impl Name {
    pub fn new() -> Self {
        Self(OsString::new())
    }

    pub(crate) fn to_ascii_lowercase(&self) -> Self {
        Self(self.0.to_ascii_lowercase())
    }

    pub(crate) fn starts_with(&self, arg: u8) -> bool {
        let b = self.0.as_bytes();
        if b.len() < 1 {
            return false;
        }
        b[0] == arg
    }

    pub fn to_os_string(&self) -> OsString {
        self.0.clone()
    }

    pub(crate) fn eq_ignore_ascii_case<S>(&self, method_name: S) -> bool
    where
        S: AsRef<OsStr>,
    {
        self.0.eq_ignore_ascii_case(method_name)
    }
}

impl From<OsString> for Name {
    fn from(s: OsString) -> Self {
        Self(s)
    }
}
impl From<&OsString> for Name {
    fn from(str: &OsString) -> Self {
        Self(str.clone())
    }
}
impl From<&Name> for Name {
    fn from(n: &Name) -> Self {
        n.clone()
    }
}

impl From<String> for Name {
    fn from(s: String) -> Self {
        Self(OsString::from(s))
    }
}

impl From<&OsStr> for Name {
    fn from(str: &OsStr) -> Self {
        Self(OsString::from(str))
    }
}

impl From<&[u8]> for Name {
    fn from(byte_vec: &[u8]) -> Self {
        Self::from(OsStr::from_bytes(byte_vec))
    }
}

impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Self::from(String::from(s))
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string_lossy())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FullyQualifiedName {
    pub path: Vec<Name>,
}

impl PartialEq<&[u8]> for FullyQualifiedName {
    fn eq(&self, other: &&[u8]) -> bool {
        let other_fq_name = FullyQualifiedName::from(*other);
        *self == other_fq_name
    }
}

impl FullyQualifiedName {
    pub fn new() -> Self {
        Self { path: vec![] }
    }

    pub fn push<T>(&mut self, item: T)
    where
        T: Into<Name>,
    {
        self.path.push(item.into());
    }

    pub fn pop(&mut self) -> Option<Name> {
        self.path.pop()
    }

    // Return the last element of the full_qualified_path
    pub fn get_name(&self) -> Option<Name> {
        self.path.last().cloned()
    }

    pub fn append(&mut self, path: Vec<Name>) {
        let mut path = path;
        self.path.append(&mut path);
    }

    pub fn get_utype(&self) -> UnionType {
        let dtype: DiscreteType = ClassName::new_with_fq_name(self.clone()).into();
        dtype.into()
    }

    pub fn to_ascii_lowercase(&self) -> Self {
        Self {
            path: self.path.iter().map(|n| n.to_ascii_lowercase()).collect(),
        }
    }

    pub(crate) fn level(&self) -> usize {
        let len = self.path.len();
        if len > 0 {
            len - 1
        } else {
            0
        }
    }
}

impl Display for FullyQualifiedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for p in &self.path {
            write!(f, "\\{}", p)?;
        }
        Ok(())
    }
}

impl From<OsString> for FullyQualifiedName {
    fn from(fq_name: OsString) -> Self {
        let b_vec = fq_name.as_bytes();
        if b_vec.len() == 0 {
            return FullyQualifiedName::new();
        }
        let mut res_fq_name = FullyQualifiedName::new();
        let mut skip_first = b_vec[0] == b'\\';
        for part in b_vec.split(|x| *x == b'\\') {
            if skip_first {
                skip_first = false;
                continue;
            }
            res_fq_name.push(part);
        }

        res_fq_name
    }
}

impl From<&[u8]> for FullyQualifiedName {
    fn from(str_vec: &[u8]) -> Self {
        Self::from(OsString::from(OsStr::from_bytes(str_vec)))
    }
}

impl From<&str> for FullyQualifiedName {
    fn from(s: &str) -> Self {
        Self::from(OsString::from(s))
    }
}

impl From<Name> for FullyQualifiedName {
    fn from(n: Name) -> Self {
        Self { path: vec![n] }
    }
}

impl From<&Name> for FullyQualifiedName {
    fn from(n: &Name) -> Self {
        Self {
            path: vec![n.clone()],
        }
    }
}

impl From<Vec<Name>> for FullyQualifiedName {
    fn from(path: Vec<Name>) -> Self {
        Self {
            path
        }
    }
}

#[derive(Clone, Debug)]
pub struct SymbolClass {
    pub name: Name,
    pub ns: FullyQualifiedName,
}

impl SymbolClass {
    pub fn new(name: Name, ns: FullyQualifiedName) -> Self {
        Self { name, ns }
    }

    pub fn new_from_cname(cname: ClassName) -> Self {
        let ns = cname.get_namespace();
        let name = cname.name;
        Self {
            name,
            ns
        }
    }
}

#[derive(Clone, Debug)]
pub struct SymbolMethod {
    pub name: Name,
    pub class: SymbolClass,
}

impl SymbolMethod {
    pub fn new(name: Name, class: SymbolClass) -> Self {
        SymbolMethod { name, class }
    }
}

#[derive(Clone, Debug)]
pub struct SymbolFunction {
    pub name: OsString,
    pub ns: OsString,
}

#[derive(Clone, Debug)]
pub struct SymbolConstant {
    pub name: OsString,
}
impl SymbolConstant {
    pub fn new(name: OsString) -> Self {
        SymbolConstant { name }
    }
}

#[derive(Clone, Debug)]
pub struct SymbolClassConstant {
    pub class: SymbolClass,
    pub constant: Name,
}

impl SymbolClassConstant {
    pub fn new(class: SymbolClass, constant: Name) -> Self {
        SymbolClassConstant { class, constant }
    }
}

#[derive(Clone, Debug)]
pub struct SymbolClassProperty {
    pub class: SymbolClass,
    pub property: Name,
}

impl SymbolClassProperty {
    pub fn new(class: SymbolClass, property: Name) -> Self {
        SymbolClassProperty { class, property }
    }
}

#[derive(Clone, Debug)]
pub enum Symbol {
    None,
    Native(&'static str),
    Class(SymbolClass),
    Method(SymbolMethod),
    Function(SymbolFunction),
    Constant(SymbolConstant),
    ClassConstant(SymbolClassConstant),
    ClassProperty(SymbolClassProperty),
}

impl From<DiscreteType> for Symbol {
    fn from(dtype: DiscreteType) -> Self {
        match dtype {
            DiscreteType::NULL => Symbol::None,
            DiscreteType::Void => Symbol::None,
            DiscreteType::Int => Symbol::Native("int"),
            DiscreteType::Float => Symbol::Native("float"),
            DiscreteType::Resource => Symbol::Native("resource"),
            DiscreteType::String => Symbol::Native("string"),
            DiscreteType::Bool => Symbol::Native("bool"),
            DiscreteType::False => Symbol::Native("bool"),
            DiscreteType::Array => Symbol::Native("array"),
            DiscreteType::Object => Symbol::Native("object"),
            DiscreteType::Callable => Symbol::Native("callable"),
            // FIXME, maybe to a better one here
            DiscreteType::TypedCallable(_,_) => Symbol::Native("callable"),
            DiscreteType::Vector(_) => Symbol::Native("array"),
            DiscreteType::HashMap(_, _) => Symbol::Native("array"),
            DiscreteType::Special(SpecialType::Static) => Symbol::Native("static"),
            DiscreteType::Special(SpecialType::Self_) => Symbol::Native("self"),
            DiscreteType::Unknown => Symbol::None,
            DiscreteType::Named(name, fqname) => {
                let cname = ClassName::new_with_names(name, fqname);
                Symbol::Class(SymbolClass::new(cname.name.clone(), cname.get_namespace()))
            }
            DiscreteType::Generic(_, _) => todo!(),
            DiscreteType::Shape(_) => todo!(),
        }
    }
}
