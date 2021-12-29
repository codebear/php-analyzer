use std::{collections::BTreeSet, ffi::OsString, iter::FromIterator, os::unix::prelude::OsStrExt};

use tree_sitter::Range;

use crate::{
    analysis::state::AnalysisState,
    issue::{Issue, IssueEmitter, IssuePosition},
    symboldata::class::ClassName,
    symbols::{FullyQualifiedName, Name},
};

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

    Vector(UnionType),
    HashMap(UnionType, UnionType),
    Unknown,

    /// *  0 = Local name
    /// *  1 = FqName
    Named(Name, FullyQualifiedName),
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

impl ToString for UnionType {
    fn to_string(&self) -> String {
        self.types
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("|")
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

    pub fn parse(
        type_str: OsString,
        range: Range,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        let lc_type = type_str.to_ascii_lowercase();
        let type_str_bytes = lc_type.as_bytes();
        match type_str_bytes {
            b"string" => return Some(DiscreteType::String.into()),
            b"int" => return Some(DiscreteType::Int.into()),
            b"float" | b"double" => return Some(DiscreteType::Float.into()),
            b"boolean" | b"bool" => return Some(DiscreteType::Bool.into()),

            b"self" => match &state.in_class {
                Some(class_state) => {
                    // void
                    let cname = class_state.get_name();
                    return Some(
                        DiscreteType::Named(cname.get_name().clone(), cname.get_fq_name().clone())
                            .into(),
                    );
                }
                None => return crate::missing_none!("self outside of class"),
            },

            _ => (),
        }
        if type_str_bytes.contains(&b'<') {
            return crate::missing_none!("Type string med generics: {:?}", type_str);
            // } else {
            // crate::missing!("Ukjent type-string: {:?} pretends it's a class", type_str);
        }

        //let noe = state.get_fq_name(type_str);
        let cname = ClassName::new_with_analysis_state(&Name::from(&type_str), state);
        if state.symbol_data.get_class(&cname).is_none() {
            // type is unknown
            // FIXME determine if this detection of missing types should be somewhere else?
            if state.pass == 2 {
                let pos = IssuePosition::new(&state.filename, range);
                emitter.emit(Issue::UnknownType(pos, type_str));
            }
        }
        return Some(DiscreteType::Named(cname.name, cname.fq_name).into());
    }

    pub fn to_markdown(&self) -> String {
        // void
        String::from("MISSING")
    }
}

impl DiscreteType {
    pub fn to_markdown(&self) -> String {
        self.to_string()
    }
}

impl ToString for DiscreteType {
    fn to_string(&self) -> String {
        match self {
            DiscreteType::NULL => todo!(),
            DiscreteType::Void => todo!(),
            DiscreteType::Int => "int".to_string(),
            DiscreteType::Float => "double".to_string(),
            DiscreteType::Resource => "resource".to_string(),
            DiscreteType::String => "string".to_string(),
            DiscreteType::Bool => "boolean".to_string(),
            DiscreteType::Array => "array".to_string(),
            DiscreteType::Callable => "callable".to_string(),
            DiscreteType::Vector(t) => format!("array<{}>", t.to_string()),
            DiscreteType::HashMap(k, v) => format!("array<{},{}>", k.to_string(), v.to_string()),
            DiscreteType::Unknown => todo!(),
            DiscreteType::Named(_, t) => t.to_string(),
            DiscreteType::False => "false".to_string(),
            DiscreteType::Object => "object".to_string(),
        }
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
