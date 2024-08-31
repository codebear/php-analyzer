use std::fmt::Display;

use crate::{
    analysis::state::AnalysisState, issue::IssueEmitter, missing,
    operators::binary::InstanceOfSymbol, parser::Range, types::union::DiscretlyAccessedType,
};

use super::{
    phptype::TypeTraits,
    union::{Consequences, DiscreteType, IntersectionType, PHPType},
};

impl IntersectionType {
    pub fn new() -> Self {
        Self {
            types: Default::default(),
        }
    }
    ///
    /// Returns Some(<type>) if the union safely can coalesce into one single type
    /// if it is empty or has multiple types it will return None
    pub fn single_type(&self) -> Option<DiscreteType> {
        todo!("HELP AGAIN");
        //if self.types.len() == 1 {
        //    return self.types.iter().next().cloned();
        //}
        // None
    }

    pub(crate) fn ensure_valid(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        range: &Range,
        allow_unforfilled_templates: bool,
    ) {
        missing!("Validate that the intersecting types actually can intersect");
        for t in &self.types {
            t.ensure_valid(state, emitter, range, allow_unforfilled_templates);
        }
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
            t.check_type_casing(range, state, emitter);
        }
    }

    pub(crate) fn is_instanceof(
        &self,
        _other: &InstanceOfSymbol,
        _state: &AnalysisState,
    ) -> Option<bool> {
        crate::missing_none!("Check instanceof for an intersection-type")
    }

    pub(crate) fn map(&self, discrete: &impl Fn(DiscreteType) -> DiscreteType) -> Self {
        let mut itypes = vec![];
        for t in &self.types {
            itypes.push(t.map(discrete));
        }
        IntersectionType::from(itypes)
    }

    pub(crate) fn simplify(&self) -> PHPType {
        if self.types.len() == 1 {
            self.types
                .first()
                .expect("We checked that len is 1")
                .clone()
        } else {
            self.into()
        }
    }

    /// See [DiscretlyAccessedType] for more details
    pub(crate) fn as_discrete_variants(&self) -> Vec<DiscretlyAccessedType> {
        if self.types.len() == 1 {
            self.types
                .first()
                .expect("We checked that len is 1")
                .as_discrete_variants()
        } else {
            crate::missing!("We maybe can do better here");

            vec![DiscretlyAccessedType::Intersection(self.clone())]
        }
    }
}

impl Default for IntersectionType {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for IntersectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.types
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("&")
        )
    }
}

impl TypeTraits for IntersectionType {
    fn is_callable(&self) -> bool {
        for i in &self.types {
            if !i.is_callable() {
                return false;
            }
        }
        !self.types.is_empty()
    }

    fn is_int(&self) -> bool {
        for i in &self.types {
            if !i.is_int() {
                return false;
            }
        }
        !self.types.is_empty()
    }

    fn is_string(&self) -> bool {
        for i in &self.types {
            if !i.is_string() {
                return false;
            }
        }
        !self.types.is_empty()
    }

    fn is_float(&self) -> bool {
        for i in &self.types {
            if !i.is_float() {
                return false;
            }
        }
        !self.types.is_empty()
    }

    fn is_nullable(&self) -> bool {
        for i in &self.types {
            if !i.is_nullable() {
                return false;
            }
        }
        !self.types.is_empty()
    }

    fn is_bool(&self) -> bool {
        for i in &self.types {
            if !i.is_bool() {
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

impl From<Vec<PHPType>> for IntersectionType {
    fn from(types: Vec<PHPType>) -> Self {
        let mut types = types;
        let types = types.drain(..).collect();
        Self { types }
    }
}
