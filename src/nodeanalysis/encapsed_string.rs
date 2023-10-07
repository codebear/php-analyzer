use crate::{
    analysis::state::AnalysisState,
    autonodes::encapsed_string::{EncapsedStringChildren, EncapsedStringNode},
    extra::ExtraChild,
    issue::IssueEmitter,
    types::union::{DiscreteType, UnionType},
    value::PHPValue,
};
use std::{ffi::OsStr, os::unix::ffi::OsStrExt};

impl EncapsedStringNode {
    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        for child in &self.children {
            child.read_from(state, emitter);
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        let mut parts: Vec<_> = vec![];

        for child in &self.children {
            match &**child {
                EncapsedStringChildren::Extra(ExtraChild::Comment(_))
                | EncapsedStringChildren::Extra(ExtraChild::TextInterpolation(_)) => continue,

                _ => parts.push(child.get_php_value(state, emitter)?.as_php_string()?),
            }

            // If any of the children has a unknown value, this whole string is undeterminable
        }

        let vec_set: Vec<Vec<u8>> = parts
            .iter()
            .filter_map(|x| {
                if let PHPValue::String(z) = x {
                    Some(z.as_bytes().into())
                } else {
                    None
                }
            })
            .collect();
        let joined_str: Vec<u8> = vec_set.into_iter().flatten().collect();
        let noe = OsStr::from_bytes(&joined_str).to_os_string();
        Some(PHPValue::String(noe))
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        Some(DiscreteType::String.into())
    }
}
