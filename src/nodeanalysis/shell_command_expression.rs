use crate::{
    analysis::state::AnalysisState,
    autonodes::shell_command_expression::ShellCommandExpressionNode,
    issue::IssueEmitter,
    types::union::{DiscreteType, UnionType},
};

impl ShellCommandExpressionNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        // FIXME, maybe tree-sitter-parsing of this is icomplete? It should have children as there might be inlined variables?
        
        //        crate::missing!("{}.read_from(..)", self.kind());
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        None
        //        crate::missing_none!("{}.get_php_value(..)", self.kind())
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        Some(UnionType::from(
            &[DiscreteType::String, DiscreteType::NULL] as &[DiscreteType]
        ))
    }
}
