use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::{OsStr, OsString},
    fmt::Display,
    iter::FromIterator,
    os::unix::prelude::OsStrExt,
};

use tree_sitter::Range;

use crate::{
    analysis::state::AnalysisState,
    issue::IssueEmitter,
    symboldata::class::ClassName,
    symbols::{FullyQualifiedName, Name},
};

use super::parse_types::{ConcreteType, ParsedType, ShapeKey, TypeName, TypeStruct};
use super::parser::union_type;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpecialType {
    Static,
    Self_,
}

impl Display for SpecialType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Static => write!(f, "static"),
            Self::Self_ => write!(f, "self"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ShapeTypeKey {
    String(Name),
    Int(i64),
}

impl Display for ShapeTypeKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShapeTypeKey::String(name) => write!(f, "{}", name),
            ShapeTypeKey::Int(int) => write!(f, "{}", int),
        }
    }
}

impl From<ShapeKey> for ShapeTypeKey {
    fn from(key: ShapeKey) -> Self {
        match key {
            ShapeKey::Num(n) => Self::Int(n),
            ShapeKey::String(s) => Self::String(s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ShapeTypeValue {
    optional: bool,
    utype: UnionType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ShapeType {
    pub map: BTreeMap<ShapeTypeKey, ShapeTypeValue>,
}

impl ShapeType {
    pub fn new() -> Self {
        let map = BTreeMap::new();
        Self { map }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DiscreteType {
    NULL,
    Void,
    Int,
    Float,
    Resource,
    String,
    Bool,

    False,
    /// General common array, of unknown content
    Array,
    Object,
    Callable,
    /// *  .0 = List of types for each argument to the callable
    /// *  .1 = Return type of the callable
    TypedCallable(Vec<UnionType>, UnionType),

    // Types with special (contextual) meaning, like static or self
    Special(SpecialType),

    Vector(UnionType),
    HashMap(UnionType, UnionType),
    Shape(ShapeType),
    Unknown,

    /// *  0 = Local name
    /// *  1 = FqName
    Named(Name, FullyQualifiedName),

    Generic(Box<DiscreteType>, Vec<UnionType>),
}
/*
impl Ord for DiscreteType {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}*/

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UnionType {
    pub types: BTreeSet<DiscreteType>,
}

impl Display for UnionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.types
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("|")
        )
    }
}
/*
impl Ord for UnionType {
    fn cmp(&self, other: &Self) -> Ordering {

        for (a, b) in self.types.iter().zip(other.types.iter()) {
            match a.cmp(b) {
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => (),
                Ordering::Greater => return Ordering::Greater,
            }
        }
        self.types().len().cmp(&other.types.len())
    }
}

impl PartialOrd for UnionType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
*/

impl UnionType {
    pub fn new() -> Self {
        UnionType {
            types: BTreeSet::new(),
        }
    }
    pub fn push(&mut self, t: DiscreteType) {
        self.types.insert(t);
    }

    pub fn reduce(list: Vec<Self>) -> Self {
        let mut utype = Self::new();
        for utype_list in list {
            for disc_type in utype_list.types {
                utype.push(disc_type)
            }
        }
        utype
    }

    pub fn merge_into(&mut self, other: UnionType) {
        for t in other.types {
            self.types.insert(t);
        }
    }

    pub(crate) fn is_callable(&self) -> bool {
        for t in &self.types {
            match t {
                DiscreteType::Callable => (),
                _ => return false,
            }
        }
        self.types.len() > 0
    }

    pub fn len(&self) -> usize {
        self.types.len()
    }

    ///
    /// Returns Some(<type>) if the union safely can coalesce into one single type
    /// if it is empty or has multiple types it will return None
    pub fn single_type(&self) -> Option<DiscreteType> {
        if self.types.len() == 1 {
            return self.types.iter().next().cloned();
        }
        None
    }

    pub fn parse_with_remainder(
        type_str: OsString,
        _range: Range,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> (Option<UnionType>, Option<OsString>) {
        let parse_result = union_type(type_str.as_bytes());

        let (rest, parsed_type) = if let Some((rest, parsed_type)) = parse_result.ok() {
            (rest, parsed_type)
        } else {
            return (None, Some(type_str.clone()));
        };

        let remainder = if rest.len() > 0 {
            let rest_str: OsString = OsStr::from_bytes(rest).into();
            Some(rest_str)
        } else {
            None
        };
        let found_types =
            if let Some(utype) = from_vec_parsed_type(parsed_type.clone(), state, Some(emitter)) {
                Some(utype)
            } else {
                eprintln!(
                    "Parsing of type: {:?} failed, parsed into: {:?}",
                    type_str, parsed_type
                );
                None
            };

        (found_types, remainder)
    }

    pub fn parse(
        type_str: OsString,
        range: Range,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        let (utype, remainder) =
            Self::parse_with_remainder(type_str.clone(), range, state, emitter);

        if let Some(rest) = remainder {
            if rest.len() > 0 {
                // Check if only whitespace
                eprintln!("Rest etter type-parsing av [{:?}]: {:?}", type_str, rest);
                return None;
            }
        }
        utype
    }

    pub fn to_markdown(&self) -> String {
        // void
        self.to_string()
    }

    pub(crate) fn single_type_excluding_null(&self) -> Option<DiscreteType> {
        let mut types = BTreeSet::new();
        for t in &self.types {
            match t {
                DiscreteType::NULL => (),
                _ => {
                    types.insert(t.clone());
                }
            }
        }
        if types.len() == 1 {
            types.iter().next().cloned()
        } else {
            None
        }
    }
}

impl DiscreteType {
    pub fn to_markdown(&self) -> String {
        self.to_string()
    }
}

fn from_vec_parsed_type(
    ptypes: Vec<ConcreteType>,
    state: &mut AnalysisState,
    maybe_emitter: Option<&dyn IssueEmitter>,
) -> Option<UnionType> {
    let mut utype = UnionType::new();
    for ptype in ptypes {
        utype.merge_into(from_parsed_type(ptype, state, maybe_emitter)?);
    }
    Some(utype)
}

fn from_parsed_type(
    ctype: ConcreteType,
    state: &mut AnalysisState,
    maybe_emitter: Option<&dyn IssueEmitter>,
) -> Option<UnionType> {
    let utype = match ctype.ptype {
        ParsedType::Type(type_struct) => from_type_struct(type_struct, state, maybe_emitter),
        ParsedType::Shape(entries) => {
            let mut shape = ShapeType::new();
            let mut key_idx = 0;
            for entry in entries {
                let (key, optional) = if let Some((k, optional)) = entry.0 {
                    if let ShapeKey::Num(idx) = k {
                        if idx >= key_idx {
                            key_idx = idx + 1;
                        }
                    }
                    (k, optional)
                } else {
                    let k = ShapeKey::Num(key_idx);
                    key_idx += 1;
                    (k, false)
                };
                let utype = from_vec_parsed_type(entry.1, state, maybe_emitter)?;
                shape
                    .map
                    .insert(key.into(), ShapeTypeValue { optional, utype });
            }
            Some(DiscreteType::Shape(shape).into())
        }
        ParsedType::Callable(args, cond_return) => {
            let return_type = match cond_return {
                Some(rt) if rt.len() > 0 => match from_vec_parsed_type(rt, state, maybe_emitter) {
                    Some(t) => t,
                    None => {
                        crate::missing!("Failed to parse return type correctly");
                        DiscreteType::Unknown.into()
                    }
                },
                _ => DiscreteType::Void.into(),
            };
            let arg_vector: Vec<UnionType> = args
                .iter()
                .map(
                    |x| match from_vec_parsed_type(x.clone(), state, maybe_emitter) {
                        Some(utype) => utype,
                        None => {
                            crate::missing!("Failed to parse argument type correctly");
                            DiscreteType::Unknown.into()
                        }
                    },
                )
                .collect();
            Some(DiscreteType::TypedCallable(arg_vector, return_type).into())
        }
        ParsedType::CallableUntyped => {
            Some(DiscreteType::Callable.into())
            //             crate::missing_none!("callable type with not details must be cast to UnionType")
        }
    };

    if let Some(mut utype) = utype {
        if ctype.nullable {
            utype.push(DiscreteType::NULL)
        }
        Some(utype)
    } else {
        None
    }
}

fn from_type_struct(
    type_struct: TypeStruct,
    state: &mut AnalysisState,
    maybe_emitter: Option<&dyn IssueEmitter>,
) -> Option<UnionType> {
    let dtype = if let TypeName::Name(tname) = &type_struct.type_name {
        let lc_type_str = tname.to_os_string().to_ascii_lowercase();
        // check for native types
        //    let type_str = lc_types.as_bytes();

        match lc_type_str.as_bytes() {
            b"string" => Some(DiscreteType::String),
            b"int" => Some(DiscreteType::Int),
            b"float" | b"double" => Some(DiscreteType::Float),
            b"boolean" | b"bool" => Some(DiscreteType::Bool),

            b"self" => Some(DiscreteType::Special(SpecialType::Self_)),
            b"static" => Some(DiscreteType::Special(SpecialType::Static)),
            _ => None,
        }
    } else {
        None
    };
    // ...

    let mut base_type = if let Some(DiscreteType::Special(SpecialType::Self_)) = dtype {
        match &state.in_class {
            Some(class_state) => {
                // void
                let dtype: DiscreteType = class_state.get_name().into();
                dtype
            }
            None => return crate::missing_none!("self outside of class"),
        }
    } else if let Some(dt) = dtype {
        dt
    } else {
        let cname = match &type_struct.type_name {
            TypeName::Name(name) => ClassName::new_with_analysis_state(name, state),
            TypeName::FQName(fq_name) => ClassName::new_with_fq_name(fq_name.clone()),
            TypeName::RelativeName(path) => {
                let mut fq_name = if let Some(ns) = &state.namespace {
                    ns.clone()
                } else {
                    FullyQualifiedName::new()
                };
                fq_name.append(path.clone());
                ClassName::new_with_fq_name(fq_name)
            }
        };
        cname.into()
    };

    if let Some(generic_args) = type_struct.generics {
        let mut generics: Vec<UnionType> = vec![];
        for gen_arg in generic_args {
            generics.push(from_vec_parsed_type(gen_arg, state, maybe_emitter)?);
        }

        base_type = DiscreteType::Generic(Box::new(base_type), generics);
    }

    let mut utype = UnionType::new();

    utype.push(base_type.into());
    Some(utype)
}

impl From<ClassName> for DiscreteType {
    fn from(cname: ClassName) -> Self {
        DiscreteType::Named(cname.get_name().clone(), cname.get_fq_name().clone())
    }
}

impl Display for DiscreteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DiscreteType::NULL => "*null*".into(),
                DiscreteType::Void => "*void*".into(),
                DiscreteType::Int => "int".to_string(),
                DiscreteType::Float => "double".to_string(),
                DiscreteType::Resource => "resource".to_string(),
                DiscreteType::String => "string".to_string(),
                DiscreteType::Bool => "boolean".to_string(),
                DiscreteType::Array => "array".to_string(),
                DiscreteType::Callable => "callable".to_string(),
                DiscreteType::TypedCallable(arg_types, return_type) => format!(
                    "callable({}):{}",
                    arg_types
                        .iter()
                        .map(|x| format!("{}", x))
                        .collect::<Vec<String>>()
                        .join(", "),
                    return_type
                ),
                DiscreteType::Special(s) => s.to_string(),
                DiscreteType::Vector(t) => format!("array<{}>", t.to_string()),
                DiscreteType::HashMap(k, v) =>
                    format!("array<{},{}>", k.to_string(), v.to_string()),
                DiscreteType::Unknown => "*unknown*".to_string(),
                DiscreteType::Named(_, t) => t.to_string(),
                DiscreteType::False => "false".to_string(),
                DiscreteType::Object => "object".to_string(),
                DiscreteType::Shape(shape) => {
                    let mut buf = String::new();
                    buf.push_str("array{");
                    let mut parts = vec![];
                    for (key, value) in &shape.map {
                        parts.push(format!(
                            "{}{}:{}",
                            key,
                            if value.optional { "?" } else { "" },
                            value.utype
                        ));
                    }
                    buf.push_str(&parts.join(","));
                    buf.push_str("}");
                    buf
                }
                DiscreteType::Generic(base_type, v) => {
                    let indre: Vec<_> = v.iter().map(|x| x.to_string()).collect();

                    format!("{}<{}>", base_type, indre.join(", "))
                }
            }
        )
    }
}

impl From<DiscreteType> for UnionType {
    fn from(discrete: DiscreteType) -> Self {
        let mut ut = UnionType::new();
        ut.push(discrete);
        ut
    }
}

impl From<&[&UnionType]> for UnionType {
    fn from(list: &[&UnionType]) -> Self {
        let mut utl = UnionType::new();
        for &ut in list {
            utl.merge_into(ut.clone());
        }
        utl
    }
}

impl From<&[DiscreteType]> for UnionType {
    fn from(list: &[DiscreteType]) -> Self {
        let mut ut = UnionType::new();
        for discrete in list {
            ut.push(discrete.clone());
        }
        ut
    }
}

impl From<Vec<DiscreteType>> for UnionType {
    fn from(list: Vec<DiscreteType>) -> Self {
        let mut ut = UnionType::new();
        for discrete in list {
            ut.push(discrete.clone());
        }
        ut
    }
}

impl From<Vec<UnionType>> for UnionType {
    fn from(list: Vec<UnionType>) -> Self {
        let mut ut = UnionType::new();
        for utype in list {
            ut.merge_into(utype.clone());
        }
        ut
    }
}

impl FromIterator<DiscreteType> for UnionType {
    fn from_iter<I: IntoIterator<Item = DiscreteType>>(list: I) -> Self {
        let mut ut = UnionType::new();
        for discrete in list {
            ut.push(discrete);
        }
        ut
    }
}

impl<'a> FromIterator<&'a DiscreteType> for UnionType {
    fn from_iter<I: IntoIterator<Item = &'a DiscreteType>>(list: I) -> Self {
        let mut ut = UnionType::new();
        for discrete in list {
            ut.push(discrete.clone());
        }
        ut
    }
}
