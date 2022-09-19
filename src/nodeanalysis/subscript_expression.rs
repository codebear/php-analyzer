use crate::{
    analysis::state::AnalysisState,
    autonodes::{
        any::AnyNodeRef,
        subscript_expression::{SubscriptExpressionDereferenceable, SubscriptExpressionNode},
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
            .dereferenceable
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
        let mut ret_type = UnionType::new();
        for dtype in array_type.types {
            match dtype {
                DiscreteType::String => {
                    // String lookup. Index must be integer
                    if let Some(DiscreteType::Int) = index_type.single_type() {
                        ret_type.merge_into(UnionType::from(&[
                            DiscreteType::String,
                            DiscreteType::NULL,
                        ]
                            as &[DiscreteType]));
                    } else {
                        crate::missing!(
                            "subscript.get_utype(..) string indexing with none integer index-type: {:?}",
                            index_type,
                        );
                    }
                }
                DiscreteType::Named(_, _) => crate::missing!(
                    "subscript.get_utype(..) what get's when looking up in named type with {:?}",
                    index_type,
                ),
                DiscreteType::Generic(_, _) => crate::missing!(
                    "subscript.get_utype(..) what get's when looking up in generic type with {:?}",
                    index_type,
                ),
                DiscreteType::Int => {
                    // This should emit
                    crate::missing!("Emit something when attempting array lookup of Int");
                }
                _ => crate::missing!(
                    "subscript.get_utype(..) what get's when looking up in {:?} with a {:?}",
                    dtype,
                    index_type
                ),
            }
        }
        if ret_type.len() > 0 {
            Some(ret_type)
        } else {
            None
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        self.dereferenceable
            .as_ref()
            .map(|x| x.read_from(state, emitter));
        self.index.as_ref().map(|x| x.read_from(state, emitter));
    }

    pub fn get_key_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
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
            .dereferenceable
            .as_ref()
            .and_then(|x| x.get_php_value(state, emitter));

        let idx = self.get_key_value(state, emitter);
        match (val, idx) {
            (Some(PHPValue::Array(arr)), Some(val @ PHPValue::Int(_)))
            | (Some(PHPValue::Array(arr)), Some(val @ PHPValue::String(_)))
            | (Some(PHPValue::Array(arr)), Some(val @ PHPValue::NULL)) => arr.get_value_by_key(val),
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
        self.dereferenceable
            .as_ref()
            .map(|x| x.write_to(state, emitter, None, None));

        if let Some(_) = self.get_key_value(state, emitter) {
            crate::missing!("write_to subscript_expression_node with known index needs more logic");
        } else {
            crate::missing!(
                "write_to subscript_expression_node with unknown index needs more logic"
            );
        }
    }
}

impl SubscriptExpressionDereferenceable {
    pub fn write_to(
        &self,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn IssueEmitter,
        val_type: Option<UnionType>,
        value: Option<PHPValue>,
    ) {
        match self {
            SubscriptExpressionDereferenceable::ArrayCreationExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferenceable::CastExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferenceable::ClassConstantAccessExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferenceable::DynamicVariableName(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferenceable::EncapsedString(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferenceable::FunctionCallExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferenceable::Heredoc(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferenceable::MemberAccessExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferenceable::MemberCallExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferenceable::Name(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferenceable::NullsafeMemberAccessExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferenceable::NullsafeMemberCallExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferenceable::ParenthesizedExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferenceable::QualifiedName(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferenceable::ScopedCallExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferenceable::ScopedPropertyAccessExpression(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferenceable::String(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }
            SubscriptExpressionDereferenceable::SubscriptExpression(se) => {
                se.write_to(state, emitter, val_type, value)
            }
            SubscriptExpressionDereferenceable::VariableName(vn) => {
                vn.write_to(state, emitter, val_type, value)
            }
            SubscriptExpressionDereferenceable::Nowdoc(_) => {
                crate::missing!("{}.write_to()", self.kind())
            }

            SubscriptExpressionDereferenceable::Comment(_)
            | SubscriptExpressionDereferenceable::TextInterpolation(_)
            | SubscriptExpressionDereferenceable::Error(_) => (),
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
