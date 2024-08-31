use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
    iter::FromIterator,
    os::unix::prelude::OsStrExt,
};

//use tree_sitter::Range;

use crate::{
    analysis::state::AnalysisState,
    issue::IssueEmitter,
    operators::binary::InstanceOfSymbol,
    parser::Range,
    symboldata::class::ClassName,
    symbols::{FullyQualifiedName, Name},
};

use super::parse_types::CompoundType;
use super::{
    parse_types::{ConcreteType, ParsedType, ShapeKey, TypeName, TypeStruct},
    phptype::TypeTraits,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpecialType {
    Static,
    Self_,
    ClassString(Option<FullyQualifiedName>),
}

impl Display for SpecialType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Static => write!(f, "static"),
            Self::Self_ => write!(f, "self"),
            Self::ClassString(class) => {
                if let Some(c) = class {
                    write!(f, "class-string<{}>", c)
                } else {
                    write!(f, "class-string")
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ShapeTypeKey {
    String(Name),
    Int(i64),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ShapeTypeValue {
    pub(in crate::types) optional: bool,
    pub(in crate::types) utype: PHPType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ShapeType {
    pub map: BTreeMap<ShapeTypeKey, ShapeTypeValue>,
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
    Mixed,
    /// Requires PHP 7.1
    Iterable,

    False,
    True,
    /// General common array, of unknown content
    Array,
    Object,
    Callable,
    /// *  .0 = List of types for each argument to the callable
    /// *  .1 = Return type of the callable
    TypedCallable(Vec<PHPType>, PHPType),

    // Types with special (contextual) meaning, like static or self
    Special(SpecialType),

    Vector(PHPType),
    HashMap(PHPType, PHPType),
    Shape(ShapeType),
    Unknown,

    /// *  0 = Local name
    /// *  1 = FqName
    Named(Name, FullyQualifiedName),

    Generic(Box<DiscreteType>, Vec<PHPType>),

    ClassType(FullyQualifiedName, Name),
    Template(Name),
}

/*
impl Ord for DiscreteType {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}*/

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PHPType {
    Union(UnionType),
    Intersection(IntersectionType),
    Discrete(Box<DiscreteType>),
}

impl Display for PHPType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PHPType::Union(u) => u.fmt(f),
            PHPType::Intersection(i) => i.fmt(f),
            PHPType::Discrete(d) => d.fmt(f),
        }
    }
}

impl PHPType {
    /// Mulig den her bør va en anna plass
    pub(crate) fn to_markdown(&self) -> String {
        match self {
            PHPType::Union(u) => u.to_markdown(),
            PHPType::Intersection(_) => todo!(),
            PHPType::Discrete(d) => d.to_markdown(),
        }
    }

    pub(crate) fn insert_nullable(&mut self) {
        let other_type = match self {
            PHPType::Union(u) => {
                u.append(DiscreteType::NULL);
                return;
            }
            PHPType::Intersection(_) | PHPType::Discrete(_) => self.clone(),
        };

        *self = UnionType::from_pair(DiscreteType::NULL, other_type).into();
    }

    pub(crate) fn simplify(&self) -> PHPType {
        match self {
            PHPType::Union(u) => u.simplify(),
            PHPType::Intersection(i) => i.simplify(),
            v @ PHPType::Discrete(_) => v.clone(),
        }
    }

    pub(crate) fn concretize_templates(&self, concrete: &BTreeMap<Name, PHPType>) -> PHPType {
        match self {
            PHPType::Union(u) => u.concretize_templates(concrete),
            PHPType::Intersection(i) => i.concretize_templates(concrete),
            PHPType::Discrete(d) => (**d).concretize_templates(concrete),
        }
    }

    /// See [DiscretlyAccessedType] for more details
    pub fn as_discrete_variants(&self) -> Vec<DiscretlyAccessedType> {
        let simple = self.simplify();
        match simple {
            PHPType::Union(u) => u.as_discrete_variants(),
            PHPType::Intersection(i) => i.as_discrete_variants(),
            PHPType::Discrete(d) => vec![DiscretlyAccessedType::Discrete(*d.clone())],
        }
    }
}

///
/// When analyzing a type, which in it's broader sense is a union of possible types,
/// we can flatten it to some extent, and find concrete discrete types, but we can
/// also end up with non-resolvable intersecting types, and we better expose these
/// directly for concrete analysis where applicable.
///
#[derive(Clone)]
pub enum DiscretlyAccessedType {
    Discrete(DiscreteType),
    Intersection(IntersectionType),
}

impl From<DiscretlyAccessedType> for PHPType {
    fn from(value: DiscretlyAccessedType) -> Self {
        match value {
            DiscretlyAccessedType::Discrete(d) => d.into(),
            DiscretlyAccessedType::Intersection(i) => i.into(),
        }
    }
}

impl From<&DiscretlyAccessedType> for PHPType {
    fn from(value: &DiscretlyAccessedType) -> Self {
        match value {
            DiscretlyAccessedType::Discrete(d) => d.into(),
            DiscretlyAccessedType::Intersection(i) => i.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IntersectionType {
    pub types: BTreeSet<PHPType>,
}
impl IntersectionType {
    fn concretize_templates(&self, concrete: &BTreeMap<Name, PHPType>) -> PHPType {
        let mut types = BTreeSet::new();
        for t in &self.types {
            match t.concretize_templates(concrete) {
                u @ PHPType::Union(_) => {
                    types.insert(u);
                }
                PHPType::Intersection(i) => {
                    for x in i.types {
                        types.insert(x);
                    }
                }
                d @ PHPType::Discrete(_) => {
                    types.insert(d);
                }
            }
        }
        Self { types }.into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UnionType {
    pub types: BTreeSet<PHPType>,
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

impl Default for UnionType {
    fn default() -> Self {
        Self::new()
    }
}

impl UnionType {
    pub fn new() -> Self {
        UnionType {
            types: BTreeSet::new(),
        }
    }

    pub fn from_pair<TA, TB>(a: TA, b: TB) -> Self
    where
        TA: Into<PHPType>,
        TB: Into<PHPType>,
    {
        let pair: Vec<PHPType> = vec![a.into(), b.into()];
        UnionType::from(pair)
    }

    pub fn push<T>(&mut self, t: T)
    where
        T: Into<PHPType>,
    {
        self.types.insert(t.into());
    }

    pub fn flatten(list: Vec<PHPType>) -> Self {
        let mut utype = Self::new();
        for entry in list {
            utype.append(entry);
        }
        utype
    }

    pub fn append<T>(&mut self, other: T)
    where
        T: Into<PHPType>,
    {
        match other.into() {
            PHPType::Union(u) => {
                for t in u.types {
                    self.types.insert(t);
                }
            }
            d @ PHPType::Discrete(_) => {
                self.types.insert(d);
            }
            i @ PHPType::Intersection(_) => {
                self.types.insert(i);
            }
        }
    }

    #[deprecated(note = "Use append instead")]
    pub fn merge_into<T>(&mut self, _other: T)
    where
        T: Into<PHPType>,
    {
        let ptype: PHPType = _other.into();
        for t in ptype.as_discrete_variants() {
            let pt: PHPType = t.into();
            self.types.insert(pt);
        }
    }

    pub(crate) fn is_instanceof(
        &self,
        fqname: &InstanceOfSymbol,
        state: &AnalysisState,
    ) -> Option<bool> {
        let mut is_true = false;
        let mut is_false = false;

        for t in &self.types {
            if let Some(x) = t.is_instanceof(fqname, state) {
                if x {
                    is_true = true;
                } else {
                    is_false = true;
                }
            } else {
                return None;
            }
        }

        match (is_true, is_false) {
            (true, false) => Some(true),
            (false, true) => Some(false),
            _ => None,
        }
    }

    pub fn len(&self) -> usize {
        self.types.len()
    }

    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }

    ///
    /// Returns Some(<type>) if the union safely can coalesce into one single type
    /// if it is empty or has multiple types it will return None
    pub fn single_type(&self) -> Option<DiscreteType> {
        let dist_types = self.as_discrete_variants();
        if dist_types.len() == 1 {
            match dist_types.iter().next().cloned() {
                Some(DiscretlyAccessedType::Discrete(d)) => Some(d),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn to_markdown(&self) -> String {
        let str_types: Vec<_> = self.types.iter().map(|x| x.to_markdown()).collect();
        let buffer = str_types.join("|");

        // let buffer = str::replace(&buffer, "\\", "\\\\");
        let buffer = str::replace(&buffer, "|", "\\|");

        let x = format!("`{}`", buffer);
        eprintln!("DEBUG: markdown generated: {}", x);
        x
    }

    /*  pub(crate) fn single_type_excluding_null(&self) -> Option<DiscreteType> {
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
    } */

    pub(crate) fn ensure_valid(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        range: &Range,
        allow_unforfilled_templates: bool,
    ) {
        for dtype in &self.types {
            dtype.ensure_valid(state, emitter, range, allow_unforfilled_templates);
        }
    }

    pub fn filter_types<P>(&self, predicate: P) -> UnionType
    where
        P: Sized + FnMut(&&DiscretlyAccessedType) -> bool,
    {
        let types: Vec<PHPType> = self
            .as_discrete_variants()
            .iter()
            .filter(predicate)
            .map(|x| x.into())
            .collect();
        UnionType::from(types)
    }

    pub(crate) fn contains_template(&self) -> bool {
        for t in &self.types {
            if t.contains_template() {
                return true;
            }
        }
        false
    }

    pub(crate) fn check_type_casing(
        &self,
        range: Range,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) {
        for t in &self.types {
            t.check_type_casing(range, state, emitter)
        }
    }

    pub(crate) fn is_same_type(&self, other: &UnionType) -> bool {
        if self.types.len() != other.types.len() {
            return false;
        }
        for (a, b) in self.types.iter().zip(other.types.iter()) {
            if !a.is_same_type(b) {
                return false;
            }
        }
        true
    }

    pub(crate) fn map(&self, discrete: &impl Fn(DiscreteType) -> DiscreteType) -> Self {
        let mut utype = UnionType::new();
        for t in &self.types {
            utype.push(t.map(discrete));
        }
        utype
    }

    fn simplify(&self) -> PHPType {
        let x = Self::flatten(self.types.iter().cloned().collect());
        if x.types.len() == 1 {
            x.types.first().expect("We checked that len is 1").clone()
        } else {
            PHPType::Union(x)
        }
    }

    fn concretize_templates(&self, concrete: &BTreeMap<Name, PHPType>) -> PHPType {
        UnionType::from(
            self.types
                .iter()
                .map(|x| x.concretize_templates(concrete))
                .collect::<Vec<_>>(),
        )
        .into()
    }

    fn as_discrete_variants(&self) -> Vec<DiscretlyAccessedType> {
        self.types
            .iter()
            .flat_map(|t| match t {
                PHPType::Union(u) => u.as_discrete_variants(),
                PHPType::Intersection(i) => i.as_discrete_variants(),
                PHPType::Discrete(d) => vec![DiscretlyAccessedType::Discrete(*d.clone())],
            })
            .collect()
    }
}

impl TypeTraits for UnionType {
    fn is_callable(&self) -> bool {
        for t in &self.types {
            if !t.is_callable() {
                return false;
            }
        }
        !self.types.is_empty()
    }

    fn is_float(&self) -> bool {
        for t in &self.types {
            if !t.is_float() {
                return false;
            }
        }
        !self.types.is_empty()
    }

    fn is_int(&self) -> bool {
        for t in &self.types {
            if !t.is_int() {
                return false;
            }
        }
        !self.types.is_empty()
    }

    fn is_nullable(&self) -> bool {
        for t in &self.types {
            if !t.is_nullable() {
                return false;
            }
        }
        !self.types.is_empty()
    }

    fn is_string(&self) -> bool {
        for t in &self.types {
            if !t.is_string() {
                return false;
            }
        }
        !self.types.is_empty()
    }

    fn is_bool(&self) -> bool {
        for t in &self.types {
            if !t.is_bool() {
                return false;
            }
        }
        !self.types.is_empty()
    }

    fn is_same_type(&self, _other: &Self) -> bool {
        todo!()
    }

    fn can_be_cast_to_string(&self) -> Option<Consequences> {
        let mut consequences: Consequences = Default::default();
        for tp in &self.types {
            let Some(child_consequences) = tp.can_be_cast_to_string() else {
                continue;
            };
            consequences.append(child_consequences);
        }
        if consequences.is_empty() {
            None
        } else {
            Some(consequences)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Consequence {
    /// Ok
    Ok,
    /// Will run OK, but not idiomatic
    Nonidiomatic(&'static str),
    /// May emit notice in some cases
    Notice(&'static str),
    /// May emit warning in some cases
    Warning(&'static str),
    /// May fail with error in some cases
    Error(&'static str),
}

#[derive(Default)]
pub struct Consequences(Vec<Consequence>);

impl Consequences {
    pub fn most_severe(&self) -> Option<Consequence> {
        todo!()
    }

    pub fn push(&mut self, x: Consequence) {
        self.0.push(x);
    }

    pub fn append<T>(&mut self, other: T)
    where
        T: Into<Vec<Consequence>>,
    {
        let mut z = other.into();
        self.0.append(&mut z);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<Consequences> for Vec<Consequence> {
    fn from(c: Consequences) -> Self {
        c.0
    }
}

impl From<Consequence> for Consequences {
    fn from(c: Consequence) -> Self {
        Self(vec![c])
    }
}

impl Consequence {
    #[deprecated(note = "Use Consequences-type instead")]
    fn most_severe(consequences: Vec<Consequence>) -> Option<Consequence> {
        Consequences(consequences).most_severe()
    }
}

impl Eq for Consequence {
    // void
}

pub(crate) fn from_vec_parsed_type(
    compound_type: CompoundType,
    state: &mut AnalysisState,
    maybe_emitter: Option<&dyn IssueEmitter>,
    temp_generics: Option<&Vec<Name>>,
) -> Option<PHPType> {
    match compound_type {
        CompoundType::Union(union) => {
            let mut utype = UnionType::new();
            for t in union {
                utype.append(from_parsed_type(t, state, maybe_emitter, temp_generics)?);
            }
            if utype.is_empty() {
                None
            } else {
                Some(utype.into())
            }
        }
        CompoundType::Intersection(parsed_intersection) => {
            // void
            let mut types = vec![];
            for t in parsed_intersection {
                let Some(xx) = from_parsed_type(t, state, maybe_emitter, temp_generics) else {
                    return crate::missing_none!("Failed to parse intersection type");
                };
                types.push(xx);
            }

            Some(IntersectionType::from(types).into())
        }
        CompoundType::Single(single) => {
            from_parsed_type(single, state, maybe_emitter, temp_generics)
        }
    }
}

fn from_parsed_type(
    ctype: ConcreteType,
    state: &mut AnalysisState,
    maybe_emitter: Option<&dyn IssueEmitter>,
    temp_generics: Option<&Vec<Name>>,
) -> Option<PHPType> {
    let _generic_templates = state.get_generic_templates(temp_generics);
    /*   eprintln!(
        "CTYPE: {}, AVAILABLE GENERICS: {:?}, TEMP_GENERICS: {:?}",
        ctype, generic_templates, temp_generics
    );*/
    let utype = match ctype.ptype {
        ParsedType::Type(type_struct) => {
            from_type_struct(type_struct, state, maybe_emitter, temp_generics)
        }
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
                let utype = from_vec_parsed_type(entry.1, state, maybe_emitter, temp_generics)?;
                shape
                    .map
                    .insert(key.into(), ShapeTypeValue { optional, utype });
            }
            Some(DiscreteType::Shape(shape).into())
        }
        ParsedType::Callable(args, cond_return) => {
            let return_type = match cond_return {
                Some(rt) if !rt.is_empty() => {
                    match from_vec_parsed_type(*rt, state, maybe_emitter, temp_generics) {
                        Some(t) => t,
                        None => {
                            crate::missing!("Failed to parse return type correctly");
                            DiscreteType::Unknown.into()
                        }
                    }
                }
                _ => DiscreteType::Void.into(),
            };
            let arg_vector: Vec<PHPType> = args
                .iter()
                .map(|x| {
                    match from_vec_parsed_type(x.clone(), state, maybe_emitter, temp_generics) {
                        Some(utype) => utype,
                        None => {
                            crate::missing!("Failed to parse argument type correctly");
                            DiscreteType::Unknown.into()
                        }
                    }
                })
                .collect();
            Some(DiscreteType::TypedCallable(arg_vector, return_type).into())
        }
        ParsedType::CallableUntyped => {
            Some(DiscreteType::Callable.into())
            //             crate::missing_none!("callable type with not details must be cast to UnionType")
        }
        ParsedType::ClassType(cname, tname) => {
            let fq_name = match cname {
                TypeName::Name(symbol_name) => {
                    state.get_fq_symbol_name_from_local_name(&symbol_name)
                }
                TypeName::FQName(fq) => fq,
                TypeName::RelativeName(_r) => {
                    return crate::missing_none!(
                        "Missing support for types in relative class-names"
                    );
                }
            };
            Some(DiscreteType::ClassType(fq_name, tname).into())
        }
        ParsedType::Parenthesized(ptype) => {
            from_vec_parsed_type(*ptype, state, maybe_emitter, temp_generics)
        }
    };

    if let Some(utype) = utype {
        if ctype.nullable {
            Some(UnionType::from_pair(utype, DiscreteType::NULL).into())
        } else {
            Some(utype)
        }
    } else {
        None
    }
}

fn from_type_struct(
    type_struct: TypeStruct,
    state: &mut AnalysisState,
    maybe_emitter: Option<&dyn IssueEmitter>,
    temp_generics: Option<&Vec<Name>>,
) -> Option<PHPType> {
    let dtype = if let TypeName::Name(tname) = &type_struct.type_name {
        let lc_type_str = tname.to_os_string().to_ascii_lowercase();
        // check for native types
        //    let type_str = lc_types.as_bytes();
        // FIXME ensure that any non-used generics are being reported
        match lc_type_str.as_bytes() {
            b"string" => Some(DiscreteType::String),
            b"int" => Some(DiscreteType::Int),
            b"integer" => Some(DiscreteType::Int),
            b"float" | b"double" => Some(DiscreteType::Float),
            b"boolean" | b"bool" => Some(DiscreteType::Bool),
            b"false" => Some(DiscreteType::False),
            b"true" => Some(DiscreteType::True),
            b"resource" => Some(DiscreteType::Resource),
            b"self" => Some(DiscreteType::Special(SpecialType::Self_)),
            b"static" => Some(DiscreteType::Special(SpecialType::Static)),
            b"mixed" => Some(DiscreteType::Mixed),
            b"void" => Some(DiscreteType::Void),
            b"iterable" => Some(DiscreteType::Iterable),
            b"null" => Some(DiscreteType::NULL),
            b"class-string" => {
                let result = if let Some(gen) = &type_struct.generics {
                    // FIXME emit or othervise make sure that any problems here aren't overlooked

                    if gen.len() != 1 {
                        return None;
                    }

                    let x = &gen[0];

                    let noe = if let Some(y) = x.if_single_type() {
                        if y.nullable {
                            return None;
                        }
                        match &y.ptype {
                            ParsedType::Type(z) if z.generics.is_none() => z.type_name.clone(),
                            // ParsedType::ClassType(fqname, constant_name) => fqname.clone(),
                            unknown_something => {
                                return crate::missing_none!("This could probably be improved to handle more complex types: {:?}", unknown_something);
                            }
                        }
                    } else {
                        crate::missing!(
                            "This could probably be improved to handle more complex types"
                        );
                        return None;
                    };

                    let fqname = match noe {
                        TypeName::Name(name) => state.get_fq_symbol_name_from_local_name(&name),
                        TypeName::FQName(fq) => fq,
                        TypeName::RelativeName(_) => todo!(),
                    };
                    Some(DiscreteType::Special(SpecialType::ClassString(Some(
                        fqname,
                    ))))
                } else {
                    Some(DiscreteType::Special(SpecialType::ClassString(None)))
                };

                if let Some(noe) = result {
                    return Some(noe.into());
                }
                None
            }
            b"object" => Some(DiscreteType::Object),
            b"array" => {
                if let Some(gen) = &type_struct.generics {
                    if gen.len() == 2 {
                        let key = from_vec_parsed_type(
                            gen[0].clone(),
                            state,
                            maybe_emitter,
                            temp_generics,
                        )?;
                        let value = from_vec_parsed_type(
                            gen[1].clone(),
                            state,
                            maybe_emitter,
                            temp_generics,
                        )?;

                        Some(DiscreteType::HashMap(key, value))
                    } else if gen.len() == 1 {
                        let value = from_vec_parsed_type(
                            gen[0].clone(),
                            state,
                            maybe_emitter,
                            temp_generics,
                        )?;
                        Some(DiscreteType::Vector(value))
                    } else {
                        // void
                        None
                    }
                } else {
                    Some(DiscreteType::Array)
                }
            }
            _ => None,
        }
    } else {
        None
    };

    let generic_templates = state.get_generic_templates(temp_generics);

    if type_struct.generics.is_none() {
        if let TypeName::Name(x) = &type_struct.type_name {
            if let Some(data) = &generic_templates {
                if data.contains(x) {
                    return Some(DiscreteType::Template(x.clone()).into());
                }
            }
        }
    }
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
            TypeName::Name(name) => {
                match (&type_struct.generics, &generic_templates) {
                    (None, Some(templates)) if templates.contains(name) => {
                        // The type-name is in the list of available templates
                        eprintln!(
                            "Trying to create a union-type from a type-struct: {:?}",
                            &type_struct
                        );
                        eprintln!("generic templates: {:?}", generic_templates);
                        todo!();
                    }
                    (Some(_), Some(templates)) if templates.contains(name) => {
                        // The type-name is in the list of available templates
                        // but it also has generic arguments itself?
                        // this is probably wrong in some sense
                        panic!();
                    }
                    (None, Some(_templates)) => {
                        ClassName::new_with_analysis_state(name, state)
                        /*                         eprintln!("TEMPLATES: {:?}", templates);
                        eprintln!("name: {:?}", name);
                        eprintln!("contains {:?}", templates.contains(name));
                        eprintln!(
                            "BALLE: {:?}",
                            ClassName::new_with_analysis_state(name, state)
                        );
                        eprintln!(
                            "Trying to create a union-type from a type-struct: {:?}",
                            &type_struct
                        );
                        eprintln!("generic templates: {:?}", state.get_generic_templates(None));
                        panic!();
                        todo!();*/
                    }
                    (None, None) => ClassName::new_with_analysis_state(name, state),
                    (Some(_), None) => ClassName::new_with_analysis_state(name, state),
                    (Some(_a), Some(_b)) => {
                        ClassName::new_with_analysis_state(name, state)
                        /*                         eprintln!(
                            "\nTrying to create a union-type from a type-struct: {}",
                            &type_struct
                        );
                        //                         eprintln!("Type: {}", &type_struct);
                        eprintln!(
                            "\ngeneric templates: {:?}",
                            state.get_generic_templates(None)
                        );

                        eprintln!("\na={:?}, b={:?}\n", &a, &b);
                        panic!();*/
                        //    ClassName::new_with_analysis_state(name, state),
                    }
                }
            }
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
    match &base_type {
        DiscreteType::Vector(_) | DiscreteType::HashMap(_, _) => return Some(base_type.into()),
        _ => (),
    }

    if let Some(generic_args) = type_struct.generics {
        let mut generics: Vec<_> = vec![];
        for gen_arg in generic_args {
            generics.push(from_vec_parsed_type(
                gen_arg,
                state,
                maybe_emitter,
                temp_generics,
            )?);
        }

        base_type = DiscreteType::Generic(Box::new(base_type), generics);
    }

    let mut utype = UnionType::new();

    utype.push(base_type);
    Some(utype.into())
}

impl From<ClassName> for DiscreteType {
    fn from(cname: ClassName) -> Self {
        DiscreteType::Named(cname.get_name().clone(), cname.get_fq_name().clone())
    }
}

impl From<UnionType> for PHPType {
    fn from(mut utype: UnionType) -> Self {
        if utype.len() == 1 {
            utype.types.pop_first().expect("We checked that len is 1")
        } else {
            PHPType::Union(utype)
        }
    }
}
impl From<&UnionType> for PHPType {
    fn from(utype: &UnionType) -> Self {
        utype.into()
    }
}

impl From<IntersectionType> for PHPType {
    fn from(itype: IntersectionType) -> Self {
        PHPType::Intersection(itype)
    }
}
impl From<&IntersectionType> for PHPType {
    fn from(itype: &IntersectionType) -> Self {
        PHPType::Intersection(itype.clone())
    }
}

impl From<DiscreteType> for PHPType {
    fn from(discrete: DiscreteType) -> Self {
        PHPType::Discrete(Box::new(discrete))
    }
}

impl From<&DiscreteType> for PHPType {
    fn from(discrete: &DiscreteType) -> Self {
        PHPType::Discrete(Box::new(discrete.clone()))
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
            ut.append(utype.clone());
        }
        ut
    }
}

impl From<Vec<PHPType>> for UnionType {
    fn from(list: Vec<PHPType>) -> Self {
        let mut ut = UnionType::new();
        for utype in list {
            ut.append(utype.clone());
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
