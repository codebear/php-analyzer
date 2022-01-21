use crate::{
    analysis::state::AnalysisState,
    autonodes::{
        any::AnyNodeRef,
        subscript_expression::{SubscriptExpressionDereferencable, SubscriptExpressionNode},
    },
    issue::{Issue, IssueEmitter},
    types::union::{DiscreteType, UnionType},
    value::PHPValue,
};

use crate::autotree::NodeAccess;

use super::analysis::ThirdPassAnalyzeableNode;

impl SubscriptExpressionNode {
    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        if let Some(val) = self.get_php_value(state, emitter) {
            return val.get_utype();
        }
        let array_type = self
            .dereferencable
            .as_ref()
            .map(|deref| deref.get_utype(state, emitter))??;

        let index = self.index.as_ref()?;
        let index_type = if let Some(itype) = index.get_utype(state, emitter) {
            itype
        } else {
            if state.pass == 3 {
                // Maybe it should be on pass 3

                // FIXME move this emitting to an analysis-method
                emitter.emit(Issue::UnknownIndexType(index.pos(state)));
            }
            return None;
        };
        // FIXME determine if a found index-type is usable, or emit

        if let Some(DiscreteType::Unknown) = array_type.single_type() {
            // If the array type is unknown, there is nothing more we can do...
            return None;
        }
        match array_type.single_type_excluding_null() {
            Some(DiscreteType::String) => {
                // String lookup. Index must be integer
                if let Some(DiscreteType::Int) = index_type.single_type() {
                    return Some(UnionType::from(&[DiscreteType::String, DiscreteType::NULL] as &[DiscreteType]));
                }
                crate::missing_none!(
                    "subscript.get_utype(..) string indexing with none integer index-type: {:?}",
                    index_type,
                )
            }
            Some(DiscreteType::Named(_,_)) => crate::missing_none!(
                "subscript.get_utype(..) what get's when looking up in named type with {:?}",
                index_type,
            ),
            Some(DiscreteType::Generic(_,_)) => crate::missing_none!(
                "subscript.get_utype(..) what get's when looking up in generic type with {:?}",
                index_type,
            ),
            _ => crate::missing_none!(
                "subscript.get_utype(..) what get's when looking up in {:?} with a {:?}",
                array_type,
                index_type
            ),
        }
        
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        self.dereferencable
            .as_ref()
            .map(|x| x.read_from(state, emitter));
        self.index.as_ref().map(|x| x.read_from(state, emitter));
    }


    pub fn get_key_value(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) -> Option<PHPValue> {
        self.index
            .as_ref()
            .and_then(|i| i.get_php_value(state, emitter))
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        let val = self
            .dereferencable
            .as_ref()
            .and_then(|x| x.get_php_value(state, emitter));

        let idx = self.get_key_value(state, emitter);
        match (val, idx) {
            (Some(PHPValue::Array(arr)), Some(val @ PHPValue::Int(_)))
            | (Some(PHPValue::Array(arr)), Some(val @ PHPValue::String(_)))
            | (Some(PHPValue::Array(arr)), Some(val @ PHPValue::NULL)) => {
                if let Some(array_key) = val.as_php_array_key() {
                    for (key, value) in &arr {
                        if key.equal_to(&array_key).unwrap_or(false) {
                            return Some(value.clone());
                        }
                    }
                }
                None
            }
            (Some(_), Some(_)) => crate::missing_none!(
                "{}.get_php_value(..) with known both index and dereferenceable",
                self.kind()
            ),

            (None, _) | (_, None) => None,
        }
    }

    pub fn write_to(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn IssueEmitter,
        _val_type: Option<UnionType>,
        _value: Option<PHPValue>,
    ) {
        // FIXME determine have this should be done...
        self.dereferencable
            .as_ref()
            .map(|x| x.write_to(state, emitter, None, None));
        
        if let Some(_) = self.get_key_value(state, emitter) {
            crate::missing!("write_to subscript_expression_node with known index needs more logic");
        } else {
            crate::missing!("write_to subscript_expression_node with unknown index needs more logic");
        }
    }
}

impl SubscriptExpressionDereferencable {
    pub fn write_to(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn IssueEmitter,
        val_type: Option<UnionType>,
        value: Option<PHPValue>,
    ) {
        match self {
            SubscriptExpressionDereferencable::ArrayCreationExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferencable::CastExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferencable::ClassConstantAccessExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferencable::DynamicVariableName(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferencable::EncapsedString(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferencable::FunctionCallExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferencable::Heredoc(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferencable::MemberAccessExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferencable::MemberCallExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferencable::Name(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferencable::NullsafeMemberAccessExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferencable::NullsafeMemberCallExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferencable::ParenthesizedExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferencable::QualifiedName(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferencable::ScopedCallExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferencable::ScopedPropertyAccessExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferencable::String(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferencable::SubscriptExpression(se) => {
                se.write_to(state, emitter, val_type, value)
            }
            SubscriptExpressionDereferencable::VariableName(vn) => {
                vn.write_to(state, emitter, val_type, value)
            }

            SubscriptExpressionDereferencable::Comment(_)
            | SubscriptExpressionDereferencable::TextInterpolation(_)
            | SubscriptExpressionDereferencable::Error(_) => (),
        }
    }
}

impl ThirdPassAnalyzeableNode for SubscriptExpressionNode {
    fn analyze_third_pass(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        path: &Vec<AnyNodeRef>,
    ) -> bool {
        self.index.as_ref().map(|i| i.read_from(state, emitter));
        self.analyze_third_pass_children(&self.as_any(), state, emitter, path)
    }
}
