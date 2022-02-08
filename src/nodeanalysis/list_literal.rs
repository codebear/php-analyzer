use crate::{
    analysis::state::AnalysisState, autonodes::list_literal::{ListLiteralNode, ListLiteralChildren}, issue::IssueEmitter,
    types::union::UnionType, value::PHPValue,
};

impl ListLiteralNode {
    pub fn read_from(&self, _state: &mut AnalysisState, _emitter: &dyn IssueEmitter) {
        crate::missing!("{}.read_from(..)", self.kind());
    }

    pub fn get_php_value(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        crate::missing_none!("{}.get_php_value(..)", self.kind())
    }

    pub fn get_utype(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        crate::missing_none!("{}.get_utype(..)", self.kind())
    }

    pub fn write_to(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn IssueEmitter,
        val_type: Option<UnionType>,
        value: Option<PHPValue>,
    ) {
        if let Some(_) = val_type {
            crate::missing!("list(..) = .. har type, som blir ignorert");
        }
        if let Some(_) = value {
            crate::missing!("list(..) = .. har verdi, som blir ignorert");
        }
        for child in &self.children {
            match &**child {
                ListLiteralChildren::_Expression(_) => crate::missing!("list({:?}) write to", child.kind()),
                ListLiteralChildren::ByRef(_) => crate::missing!("list({:?}) write to", child.kind()),
                ListLiteralChildren::DynamicVariableName(_) => crate::missing!("list({:?}) write to", child.kind()),
                ListLiteralChildren::FunctionCallExpression(_) => crate::missing!("list({:?}) write to", child.kind()),
                ListLiteralChildren::ListLiteral(_) => crate::missing!("list({:?}) write to", child.kind()),
                ListLiteralChildren::MemberAccessExpression(_) => crate::missing!("list({:?}) write to", child.kind()),
                ListLiteralChildren::MemberCallExpression(_) => crate::missing!("list({:?}) write to", child.kind()),
                ListLiteralChildren::NullsafeMemberAccessExpression(_) => crate::missing!("list({:?}) write to", child.kind()),
                ListLiteralChildren::NullsafeMemberCallExpression(_) => crate::missing!("list({:?}) write to", child.kind()),
                ListLiteralChildren::ScopedCallExpression(_) => crate::missing!("list({:?}) write to", child.kind()),
                ListLiteralChildren::ScopedPropertyAccessExpression(_) => crate::missing!("list({:?}) write to", child.kind()),
                ListLiteralChildren::SubscriptExpression(_) => crate::missing!("list({:?}) write to", child.kind()),
                ListLiteralChildren::VariableName(vname) => {
                    vname.write_to(state, emitter, None, None);
                    crate::missing!("list({:?}) write to", child.kind())
                },

                ListLiteralChildren::Comment(_) |
                ListLiteralChildren::TextInterpolation(_) |
                ListLiteralChildren::Error(_) => ()
            }
        }
        crate::missing!("list literal write_to");
    }
}
