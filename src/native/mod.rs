use crate::analysis::state::AnalysisState;

pub mod php_8_1;

pub fn register(state: &mut AnalysisState) {
    // FIXME this could be a configuration-directive
    php_8_1::register(state);
}
