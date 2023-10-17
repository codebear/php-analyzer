use crate::{
    analysis::state::AnalysisState, issue::IssueEmitter, types::union::UnionType, value::PHPValue,
};

pub trait UnaryOperator {
    fn get_operator_utype(
        &self,
        _arg_type: &UnionType,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        return None;
    }

    fn get_operator_php_value(
        &self,
        _arg_value: &Option<PHPValue>,
        _arg_type: &Option<UnionType>,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        None
    }
}
