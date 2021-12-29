use crate::autonodes::formal_parameters::FormalParametersNode;

impl FormalParametersNode {}
/*
impl AnalyzeableRoundTwoNode for FormalParametersChildren {
    fn analyze_round_two(&self, state: &mut crate::analysis::AnalysisState, emitter: &dyn crate::issue::IssueEmitter) {
        /*match self {
            FormalParametersChildren::PropertyPromotionParameter(_) => todo!(),
            FormalParametersChildren::SimpleParameter() => todo!(),
            FormalParametersChildren::VariadicParameter(_) => todo!(),
            FormalParametersChildren::Comment(_) => todo!(),
            FormalParametersChildren::TextInterpolation(_) => todo!(),
            FormalParametersChildren::Error(_) => todo!(),
        }*/
        self.analyze_round_two_children(&self.as_any(), state, emitter)
    }
}*/
