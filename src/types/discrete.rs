use std::{collections::BTreeMap, fmt::Display, sync::Arc};

use crate::{
    analysis::state::AnalysisState,
    issue::{Issue, IssueEmitter},
    missing,
    operators::binary::InstanceOfSymbol,
    parser::Range,
    symboldata::{class::ClassName, SymbolData},
    symbols::{FullyQualifiedName, Name},
};

use super::{
    phptype::TypeTraits,
    union::{Consequence, Consequences, DiscreteType, PHPType, SpecialType},
};

impl DiscreteType {
    pub fn to_markdown(&self) -> String {
        self.to_string()
    }

    pub(crate) fn ensure_valid(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        range: &Range,
        allow_unforfilled_templates: bool,
    ) {
        match self {
            DiscreteType::NULL => (),
            DiscreteType::Void => (),
            DiscreteType::Int => (),
            DiscreteType::Float => (),
            DiscreteType::Resource => (),
            DiscreteType::String => (),
            DiscreteType::Mixed => (),
            DiscreteType::Iterable => (),
            DiscreteType::Bool => (),
            DiscreteType::False => (),
            DiscreteType::True => (),
            DiscreteType::Array => (),
            DiscreteType::Object => (),
            DiscreteType::Callable => (),
            DiscreteType::TypedCallable(a, b) => {
                for u in a {
                    u.ensure_valid(state, emitter, range, allow_unforfilled_templates);
                }
                b.ensure_valid(state, emitter, range, allow_unforfilled_templates);
            }
            DiscreteType::Special(s) => {
                s.ensure_valid(state, emitter, range, allow_unforfilled_templates)
            }
            DiscreteType::Vector(v) => {
                v.ensure_valid(state, emitter, range, allow_unforfilled_templates)
            }
            DiscreteType::HashMap(k, v) => {
                // FIXME k needs to be constrained to string or int, but where is that validated?
                // Should we have a separate type/enum for hash-key?
                k.ensure_valid(state, emitter, range, allow_unforfilled_templates);
                v.ensure_valid(state, emitter, range, allow_unforfilled_templates);
            }
            DiscreteType::Shape(s) => {
                s.ensure_valid(state, emitter, range, allow_unforfilled_templates)
            }
            DiscreteType::Unknown => (),
            DiscreteType::Named(_, fqname) => {
                if let Some(_cdata_handle) = state.symbol_data.get_class(&fqname.into()) {
                    // alles ok?
                    crate::missing!("Validate that generic arguments are as expected");
                } else {
                    // let fqnames: String = format!("{}", fqname);

                    // eprintln!("BALLE3 Unknown class {}, {:?}", fqnames, fqname);
                    emitter.emit(Issue::UnknownClass(
                        state.pos_from_range(*range),
                        fqname.clone(),
                    ))
                }
            }
            DiscreteType::ClassType(_, _) => todo!(),
            DiscreteType::Template(t) => {
                // FIXME this should be done in a separate method
                // As calling this from two different points would result in all other bad types to be emitted twice
                if !allow_unforfilled_templates {
                    emitter.emit(Issue::EmptyTemplate(
                        state.pos_from_range(*range),
                        t.clone(),
                    ))
                }
            }
            DiscreteType::Generic(dtype, generic_arguments) => {
                dtype.ensure_valid(state, emitter, range, allow_unforfilled_templates);
                for generic_type in generic_arguments {
                    generic_type.ensure_valid(state, emitter, range, allow_unforfilled_templates);
                }
            }
        }
    }

    pub fn can_evaluate_to_true(&self) -> bool {
        match self {
            DiscreteType::NULL => false,
            DiscreteType::Void => false,
            DiscreteType::Int => true,
            DiscreteType::Float => true,
            DiscreteType::Resource => true,
            DiscreteType::String => true,
            DiscreteType::Bool => true,
            DiscreteType::Iterable => true,
            DiscreteType::Mixed => true,
            DiscreteType::False => false,
            DiscreteType::True => true,
            DiscreteType::Array => true,
            DiscreteType::Object => true,
            DiscreteType::Callable => true,
            DiscreteType::TypedCallable(_, _) => true,
            DiscreteType::Special(_) => true,
            DiscreteType::Vector(_) => true,
            DiscreteType::HashMap(_, _) => true,
            DiscreteType::Shape(_) => true,
            DiscreteType::Unknown => true,
            DiscreteType::Named(_, _) => true,
            DiscreteType::Generic(_, _) => true,
            DiscreteType::ClassType(_, _) => true,
            DiscreteType::Template(_) => true,
        }
    }

    pub fn can_evaluate_to_false(&self) -> bool {
        match self {
            DiscreteType::NULL => true,
            DiscreteType::Void => true,
            DiscreteType::Int => true,
            DiscreteType::Float => true,
            DiscreteType::Resource => false,
            DiscreteType::String => true,
            DiscreteType::Bool => true,
            // An iterable might be an array, and arrays
            // can evaluate as false
            DiscreteType::Iterable => true,
            DiscreteType::Mixed => true,
            DiscreteType::False => true,
            DiscreteType::True => false,
            DiscreteType::Array => true,
            DiscreteType::Object => false,
            DiscreteType::Callable => false,
            DiscreteType::TypedCallable(_, _) => false,
            DiscreteType::Special(_) => false,
            DiscreteType::Vector(_) => true,
            DiscreteType::HashMap(_, _) => true,
            DiscreteType::Shape(_) => true,
            DiscreteType::Unknown => true,
            DiscreteType::Named(_, _) => false,
            DiscreteType::Generic(_, _) => {
                crate::missing!("ensure that a generic type never can evaluate to boolean false");
                false
            }
            DiscreteType::ClassType(_, _) => true,
            DiscreteType::Template(_) => true,
        }
    }

    ///
    /// When someting with this type is an argument to `<some> instanceof SomeThing`
    /// could this type evaluate to true?
    pub fn can_be_instance_of(
        &self,
        check_cname: FullyQualifiedName,
        symbol_data: &Arc<SymbolData>,
    ) -> bool {
        match self {
            DiscreteType::NULL => false,
            DiscreteType::Void => false,
            DiscreteType::Int => false,
            DiscreteType::Float => false,
            DiscreteType::Resource => false,
            DiscreteType::String => false,
            DiscreteType::Bool => false,
            DiscreteType::Mixed => true,
            DiscreteType::Iterable => true,
            DiscreteType::False => false,
            DiscreteType::True => false,
            DiscreteType::Array => false,
            DiscreteType::Object => true,
            DiscreteType::Callable => true,
            DiscreteType::TypedCallable(_, _) => true,
            DiscreteType::Special(_) => {
                // Needs more thight hardening
                crate::missing!();
                true
            }
            DiscreteType::Vector(_) => false,
            DiscreteType::HashMap(_, _) => false,
            DiscreteType::Shape(_) => false,
            DiscreteType::Unknown => true,
            DiscreteType::Named(_, fq_named) => {
                let cname: ClassName = fq_named.into();
                let check_cname: ClassName = check_cname.into();
                if let Some(class_data) = symbol_data.get_class(&cname) {
                    return class_data
                        .read()
                        .unwrap()
                        .instanceof(&check_cname, symbol_data.clone());
                }
                false
            }
            DiscreteType::Generic(_, _) => {
                crate::missing!();
                true
            }
            DiscreteType::ClassType(_, _) => {
                crate::missing!();
                true
            }
            DiscreteType::Template(_) => {
                crate::missing!();
                true
            }
        }
    }

    pub(crate) fn is_instanceof(
        &self,
        potential_parent_type: &InstanceOfSymbol,
        state: &AnalysisState,
    ) -> Option<bool> {
        match self {
            DiscreteType::Named(_, potential_child_type) => {
                let potential_child_name: ClassName = potential_child_type.into();
                let potential_parent_type_name: ClassName =
                    potential_parent_type.try_into().ok()?;
                let locked_child_type_data = state.symbol_data.get_class(&potential_child_name)?;
                let child_type_data = locked_child_type_data.read().ok()?;
                let is_instance_of = child_type_data
                    .instanceof(&potential_parent_type_name, state.symbol_data.clone());
                Some(is_instance_of)
                //return false;
            }
            DiscreteType::Generic(_a, _b) => {
                crate::missing_none!("Check generic against InstanceOfSymbol")
                //return false;
            }
            DiscreteType::ClassType(_a, _b) => {
                crate::missing_none!("Check ClassType against InstanceOfSymbol")
                //return false;
            }
            DiscreteType::Unknown => None,
            DiscreteType::NULL => Some(false),
            _ => {
                crate::missing_none!("Check ClassType against {:?}", self)
            }
        }
    }

    pub(crate) fn check_type_casing(
        &self,
        range: Range,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) {
        match self {
            DiscreteType::NULL => (),
            DiscreteType::Void => (),
            DiscreteType::Int => (),
            DiscreteType::Float => (),
            DiscreteType::Resource => (),
            DiscreteType::String => (),
            DiscreteType::Bool => (),
            DiscreteType::Mixed => (),
            DiscreteType::Iterable => (),
            DiscreteType::False => (),
            DiscreteType::True => (),
            DiscreteType::Array => (),
            DiscreteType::Object => (),
            DiscreteType::Callable => (),
            DiscreteType::TypedCallable(_, _) => (),
            DiscreteType::Special(_) => (),
            DiscreteType::Vector(_) => (),
            DiscreteType::HashMap(_, _) => (),
            DiscreteType::Shape(_) => (),
            DiscreteType::Unknown => (),
            DiscreteType::Named(name, fqname) => {
                if let Some(fq_last_name) = fqname.get_name() {
                    if fq_last_name.eq_ignore_ascii_case(name.to_os_string())
                        && *name != fq_last_name
                    {
                        emitter.emit(Issue::WrongClassNameCasing(
                            state.pos_from_range(range),
                            name.clone(),
                            fqname.clone(),
                        ));
                    }
                }
            }
            DiscreteType::Generic(base, generic_args) => {
                base.check_type_casing(range, state, emitter);
                for t in generic_args {
                    t.check_type_casing(range, state, emitter);
                }
            }
            DiscreteType::ClassType(_, _) => (),

            DiscreteType::Template(_) => (),
        }
    }

    pub(crate) fn is_same_type(&self, other: &DiscreteType) -> bool {
        match (self, other) {
            (DiscreteType::NULL, DiscreteType::NULL) => true,
            (DiscreteType::Void, DiscreteType::Void) => true,
            (DiscreteType::Int, DiscreteType::Int) => true,
            (DiscreteType::Float, DiscreteType::Float) => true,
            (DiscreteType::Resource, DiscreteType::Resource) => true,
            (DiscreteType::String, DiscreteType::String) => true,
            (DiscreteType::Bool, DiscreteType::Bool) => true,
            (DiscreteType::Mixed, DiscreteType::Mixed) => true,
            (DiscreteType::Iterable, DiscreteType::Iterable) => true,
            (DiscreteType::False, DiscreteType::False) => true,
            (DiscreteType::True, DiscreteType::True) => true,
            (DiscreteType::Array, DiscreteType::Array) => true,
            (DiscreteType::Object, DiscreteType::Object) => true,
            (DiscreteType::Callable, DiscreteType::Callable) => true,
            (DiscreteType::Special(a), DiscreteType::Special(b)) => a == b,
            (DiscreteType::Vector(a), DiscreteType::Vector(b)) => a == b,
            (DiscreteType::HashMap(a, b), DiscreteType::HashMap(c, d)) => a == c && b == d,
            (DiscreteType::Shape(a), DiscreteType::Shape(b)) => a == b,
            (DiscreteType::Unknown, DiscreteType::Unknown) => true,
            (DiscreteType::Named(a, b), DiscreteType::Named(c, d)) => {
                if a == c && b == d {
                    return true;
                }

                if !a.to_ascii_lowercase().eq(&c.to_ascii_lowercase()) {
                    return false;
                }

                if !b.to_ascii_lowercase().eq(&d.to_ascii_lowercase()) {
                    return false;
                }

                true
            }
            (DiscreteType::Generic(a, b), DiscreteType::Generic(c, d)) => a == c && b == d,
            (DiscreteType::ClassType(a, b), DiscreteType::ClassType(c, d)) => a == c && b == d,
            (DiscreteType::Template(a), DiscreteType::Template(b)) => a == b,
            _ => false,
        }
    }

    pub(crate) fn contains_template(&self) -> bool {
        match self {
            DiscreteType::Template(_) => return true,
            DiscreteType::Generic(_gtype, utypes) => {
                // gtype is not allowed to be generic
                for u in utypes {
                    if u.contains_template() {
                        return true;
                    }
                }
            }
            _ => (),
        }
        false
    }

    pub(crate) fn concretize_templates(&self, concrete: &BTreeMap<Name, PHPType>) -> PHPType {
        match self {
            dtype @ DiscreteType::Template(name) => {
                if let Some(concrete_type) = concrete.get(name) {
                    concrete_type.clone()
                } else {
                    dtype.into()
                }
            }
            t => t.into(),
        }
    }
}

impl TypeTraits for DiscreteType {
    fn is_int(&self) -> bool {
        matches!(self, DiscreteType::Int)
    }

    fn is_float(&self) -> bool {
        matches!(self, DiscreteType::Float)
    }

    fn is_string(&self) -> bool {
        matches!(self, DiscreteType::String)
    }

    fn is_bool(&self) -> bool {
        matches!(self, DiscreteType::Bool)
    }

    fn is_callable(&self) -> bool {
        match self {
            DiscreteType::Array => missing!(),
            DiscreteType::Object => missing!(),
            DiscreteType::Callable => return true,
            DiscreteType::TypedCallable(_, _) => return true,
            DiscreteType::Special(_) => missing!(),
            DiscreteType::Vector(_) => missing!(),
            DiscreteType::Shape(_) => missing!(),
            _ => return false,
        }
        false
    }

    fn is_nullable(&self) -> bool {
        match self {
            DiscreteType::False => false,
            DiscreteType::True => false,
            DiscreteType::Bool => false,
            DiscreteType::String => false,
            DiscreteType::Float => false,
            DiscreteType::NULL => true,
            DiscreteType::Callable => false,
            DiscreteType::Object => false,
            DiscreteType::Named(_, _) => false,
            DiscreteType::Unknown => false,
            DiscreteType::Generic(base_type, _generic_type_vector) => base_type.is_nullable(),
            _ => {
                crate::missing!(
                    "Is [{:?}] nullable, as return false? THIS IS WRONG! I ASSUME! ",
                    self
                );
                true
            }
        }
    }

    fn is_same_type(&self, _other: &Self) -> bool {
        todo!()
    }

    fn can_be_cast_to_string(&self) -> Option<Consequences> {
        match self {
            DiscreteType::String | DiscreteType::Int => Some(Consequence::Ok.into()),
            DiscreteType::Float => Some(
                Consequence::Nonidiomatic("Floats may be indeterministic in string-representation")
                    .into(),
            ),
            DiscreteType::Array => {
                Some(Consequence::Warning("Cast of array to string will trigger warning").into())
            }
            DiscreteType::Named(_, _) => {
                // Check if object implements __toString
                crate::missing_none!("Check if type {} can be cast to string", self)
            }
            DiscreteType::Special(SpecialType::ClassString(_)) => Some(Consequence::Ok.into()),
            _ => {
                crate::missing_none!("Check if type {} can be cast to string", self)
            }
        }
    }
}

impl Display for DiscreteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DiscreteType::NULL => "null".into(),
                DiscreteType::Void => "void".into(),
                DiscreteType::Int => "int".to_string(),
                DiscreteType::Float => "double".to_string(),
                DiscreteType::Resource => "resource".to_string(),
                DiscreteType::String => "string".to_string(),
                DiscreteType::Bool => "bool".to_string(),
                DiscreteType::False => "false".to_string(),
                DiscreteType::True => "true".to_string(),
                DiscreteType::Array => "array".to_string(),
                DiscreteType::Callable => "callable".to_string(),
                DiscreteType::Mixed => "mixed".to_string(),
                DiscreteType::Iterable => "iterable".to_string(),
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
                DiscreteType::Vector(t) => format!("array<{}>", t),
                DiscreteType::HashMap(k, v) => format!("array<{},{}>", k, v),
                DiscreteType::Unknown => "*unknown*".to_string(),
                DiscreteType::Named(_, t) => t.to_string(),
                DiscreteType::Object => "object".to_string(),

                DiscreteType::Template(t) => t.to_string(),

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
                    buf.push('}');
                    buf
                }
                DiscreteType::Generic(base_type, v) => {
                    let indre: Vec<_> = v.iter().map(|x| x.to_string()).collect();

                    format!("{}<{}>", base_type, indre.join(", "))
                }
                DiscreteType::ClassType(fq_cname, tname) => {
                    format!("{}::{}", fq_cname, tname)
                }
            }
        )
    }
}
