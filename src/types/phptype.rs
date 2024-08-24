use crate::{
    analysis::state::AnalysisState, issue::IssueEmitter, missing,
    operators::binary::InstanceOfSymbol, parser::Range,
};

use super::union::{Consequences, DiscreteType, PHPType};

impl PHPType {
    pub(crate) fn is_same_type(&self, other: &PHPType) -> bool {
        match (self, other) {
            (PHPType::Union(a), PHPType::Union(b)) => a.is_same_type(b),
            (PHPType::Intersection(a), PHPType::Intersection(b)) => todo!("a.is_same_type(b)"),
            (PHPType::Discrete(a), PHPType::Discrete(b)) => a.is_same_type(b),
            _ => {
                missing!("Check if types are the same");
                false
            }
        }
    }

    pub(crate) fn is_callable(&self) -> bool {
        match self {
            PHPType::Union(u) => u.is_callable(),
            PHPType::Intersection(i) => i.is_callable(),
            PHPType::Discrete(d) => d.is_callable(),
        }
    }

    ///
    /// Returns Some(<type>) if the compound safely can coalesce into one single type
    /// if it is empty or has multiple types it will return None
    pub fn single_type(&self) -> Option<DiscreteType> {
        match self {
            PHPType::Union(u) => u.single_type(),
            PHPType::Intersection(i) => i.single_type(),
            PHPType::Discrete(d) => Some((**d).clone()),
        }
    }

    pub(crate) fn ensure_valid(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        range: &Range,
        allow_unforfilled_templates: bool,
    ) {
        match self {
            PHPType::Union(u) => u.ensure_valid(state, emitter, range, allow_unforfilled_templates),
            PHPType::Intersection(i) => {
                i.ensure_valid(state, emitter, range, allow_unforfilled_templates)
            }
            PHPType::Discrete(d) => {
                d.ensure_valid(state, emitter, range, allow_unforfilled_templates)
            }
        }
    }

    pub(crate) fn contains_template(&self) -> bool {
        match self {
            PHPType::Union(u) => u.contains_template(),
            PHPType::Intersection(i) => i.contains_template(),
            PHPType::Discrete(d) => d.contains_template(),
        }
    }

    pub(crate) fn check_type_casing(
        &self,
        range: Range,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) {
        match self {
            PHPType::Union(u) => u.check_type_casing(range, state, emitter),
            PHPType::Intersection(i) => i.check_type_casing(range, state, emitter),
            PHPType::Discrete(d) => d.check_type_casing(range, state, emitter),
        }
    }

    pub(crate) fn is_instanceof(&self, other: &InstanceOfSymbol) -> Option<bool> {
        todo!();
    }

    pub(crate) fn map(&self, discrete: &impl Fn(DiscreteType) -> DiscreteType) -> Self {
        match self {
            PHPType::Union(u) => PHPType::Union(u.map(discrete)),
            PHPType::Intersection(i) => PHPType::Intersection(i.map(discrete)),
            PHPType::Discrete(d) => PHPType::Discrete(Box::new(discrete(*d.clone()))),
        }
    }
}

pub trait TypeTraits {
    ///
    /// Returns true if the type is always callable
    ///
    fn is_callable(&self) -> bool;

    ///
    /// Returns true if the type is always an integer
    ///
    fn is_int(&self) -> bool;

    ///
    /// Returns true if the type is always a string
    ///
    fn is_string(&self) -> bool;

    ///
    /// Returns true if the type is always a float
    ///
    fn is_float(&self) -> bool;

    ///
    /// Returns true if the type is always nullable
    ///
    fn is_nullable(&self) -> bool;

    ///
    /// Returns true if the type is always a boolean
    ///
    fn is_bool(&self) -> bool;

    ///
    /// Returns true if the type is always the same type
    ///
    fn is_same_type(&self, other: &Self) -> bool;

    fn can_be_cast_to_string(&self) -> Option<Consequences>;
}

impl TypeTraits for PHPType {
    fn is_int(&self) -> bool {
        match self {
            PHPType::Union(u) => u.is_int(),
            PHPType::Intersection(i) => i.is_int(),
            PHPType::Discrete(d) => (**d).is_int(),
        }
    }

    fn is_string(&self) -> bool {
        match self {
            PHPType::Union(u) => u.is_string(),
            PHPType::Intersection(i) => i.is_string(),
            PHPType::Discrete(d) => d.is_string(),
        }
    }

    fn is_float(&self) -> bool {
        match self {
            PHPType::Union(u) => u.is_float(),
            PHPType::Intersection(i) => i.is_float(),
            PHPType::Discrete(d) => d.is_float(),
        }
    }

    fn is_nullable(&self) -> bool {
        match self {
            PHPType::Union(u) => u.is_nullable(),
            PHPType::Intersection(_) => todo!(),
            PHPType::Discrete(d) => d.is_nullable(),
        }
    }

    fn is_callable(&self) -> bool {
        match self {
            PHPType::Union(u) => u.is_callable(),
            PHPType::Intersection(i) => i.is_callable(),
            PHPType::Discrete(d) => d.is_callable(),
        }
    }

    fn is_bool(&self) -> bool {
        match self {
            PHPType::Union(u) => u.is_bool(),
            PHPType::Intersection(i) => i.is_bool(),
            PHPType::Discrete(d) => d.is_bool(),
        }
    }

    fn is_same_type(&self, other: &Self) -> bool {
        todo!("Check if types are the same");
    }

    fn can_be_cast_to_string(&self) -> Option<Consequences> {
        match self {
            PHPType::Union(u) => u.can_be_cast_to_string(),
            PHPType::Intersection(i) => i.can_be_cast_to_string(),
            PHPType::Discrete(d) => d.can_be_cast_to_string(),
        }
    }
}

impl From<Vec<DiscreteType>> for PHPType {
    fn from(types: Vec<DiscreteType>) -> Self {
        PHPType::Union(types.into())
    }
}
