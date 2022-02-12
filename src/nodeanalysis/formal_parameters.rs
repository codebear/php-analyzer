use std::sync::{Arc, RwLock};

use crate::{
    analysis::state::AnalysisState,
    autonodes::formal_parameters::{FormalParametersChildren, FormalParametersNode},
    issue::IssueEmitter,
    symboldata::class::{FunctionArgumentData, MethodData},
};

use super::analysis::FirstPassAnalyzeableNode;
use crate::autotree::NodeAccess;

impl FormalParametersNode {
    pub(crate) fn analyze_first_pass_parameters(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        _method_data: Arc<RwLock<MethodData>>,
    ) -> Vec<FunctionArgumentData> {
        let mut params = vec![];
        for child in &self.children {
            match &**child {
                FormalParametersChildren::PropertyPromotionParameter(_) => {
                    crate::missing!("PropertyPromotionParameter")
                }
                FormalParametersChildren::SimpleParameter(s) => {
                    let name = s.get_variable_name();
                    let arg_type = s.get_utype(state, emitter);
                    let default_value = s.get_default_value(state, emitter);
                    crate::missing!("Determine optional and nullable and phpdoc-properties");
                    let optional = false;
                    let nullable = false;

                    let data = FunctionArgumentData {
                        name,
                        arg_type,
                        default_value,
                        nullable,
                        optional,
                        own_phpdoc: None,
                        phpdoc_entry: None,
                    };

                    params.push(data);
                }
                FormalParametersChildren::VariadicParameter(_) => {
                    crate::missing!("Variadic parameter")
                }

                FormalParametersChildren::Comment(_)
                | FormalParametersChildren::TextInterpolation(_)
                | FormalParametersChildren::Error(_) => (),
            }
        }

        params
    }
}
