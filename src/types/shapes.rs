use std::{collections::BTreeMap, fmt::Display};

use crate::{analysis::state::AnalysisState, issue::IssueEmitter, parser::Range};

use super::{
    parse_types::ShapeKey,
    union::{ShapeType, ShapeTypeKey, ShapeTypeValue},
};

impl ShapeType {
    pub fn new() -> Self {
        let map = BTreeMap::new();
        Self { map }
    }

    pub(crate) fn ensure_valid(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        range: &Range,
        allow_unforfilled_templates: bool,
    ) {
        for vtype in self.map.values() {
            vtype.ensure_valid(state, emitter, range, allow_unforfilled_templates);
        }
        crate::missing!("Determine if we need to validate shape keys in some way?");
    }
}

impl ShapeTypeValue {
    pub(crate) fn ensure_valid(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        range: &Range,
        allow_unforfilled_templates: bool,
    ) {
        self.utype
            .ensure_valid(state, emitter, range, allow_unforfilled_templates);
    }
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

impl Default for ShapeType {
    fn default() -> Self {
        Self::new()
    }
}
