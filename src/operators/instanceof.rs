use crate::{
    analysis::{hardening::new_scope_with_harden_variable_type_based_on_filter, scope::BranchSide},
    autonodes::{
        _expression::_ExpressionNode, _primary_expression::_PrimaryExpressionNode,
        binary_expression::BinaryExpressionRight,
    },
    issue::VoidEmitter,
    symboldata::class::ClassName,
    types::union::{DiscreteType, UnionType},
    value::PHPValue,
    Range,
};

use super::{
    binary::{BinaryOperator, BinaryOperatorBranchTypeHardening, BinaryOperatorOperandAccess},
    operator::Operator,
};
#[derive(Clone, Debug)]

pub struct InstanceofOperator(pub Range);

impl Operator for InstanceofOperator {
    fn brief_desc(&self) -> String {
        "InstanceofOperator".into()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "instanceof"
    }
}

impl BinaryOperator for InstanceofOperator {
    fn get_operator_utype(
        &self,
        _operands: &impl BinaryOperatorOperandAccess,
        _state: &mut crate::analysis::state::AnalysisState,
        _emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<UnionType> {
        Some(DiscreteType::Bool.into())
    }

    fn get_operator_php_value(
        &self,
        operands: &impl BinaryOperatorOperandAccess,
        state: &mut crate::analysis::state::AnalysisState,
        emitter: &dyn crate::issue::IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        let Some(expected_fq_name) = operands.get_right_symbol(state) else {
            return crate::missing_none!(
                "{}[{}].get_operator_php_value(..)",
                self.brief_desc(),
                self.operator()
            );
        };

        let subject_type = operands.get_left_type(state)?;

        let is_instanceof = subject_type.is_instanceof(expected_fq_name)?;

        Some(PHPValue::Boolean(is_instanceof))
        /*
        let subject = operands.get_left_value(state);
        let expected_class = operands.get_right_value(state);
        eprintln!("[{subject:?}] instanceof [{expected_class:?}]");

        let expected_class_type = operands.get_right_type(state);
        eprintln!("[{subject_type:?}] instanceof [{expected_class_type:?}]");

        eprintln!("Right symbol: {:?}", operands.get_right_symbol(state));

        crate::missing_none!(
            "{}[{}].get_operator_php_value(..)",
            self.brief_desc(),
            self.operator()
        )*/
    }
}

impl BinaryOperatorBranchTypeHardening for InstanceofOperator {
    fn branch_with_hardened_types_base_on_conditional_node(
        &self,
        left: &_ExpressionNode,
        right: &BinaryExpressionRight,
        scope: std::sync::Arc<std::sync::RwLock<crate::analysis::scope::Scope>>,
        branch_side: crate::analysis::scope::BranchSide,
        state: &mut crate::analysis::state::AnalysisState,
    ) -> Option<std::sync::Arc<std::sync::RwLock<crate::analysis::scope::Scope>>> {
        match (left, right) {
            (_ExpressionNode::_PrimaryExpression(left), right) => {
                // Attempt to find a class-name to check against
                let cname = match right {
                    BinaryExpressionRight::_Expression(_) => crate::missing_none!(),
                    BinaryExpressionRight::DynamicVariableName(_) => {
                        crate::missing_none!()
                    }
                    BinaryExpressionRight::MemberAccessExpression(_) => {
                        crate::missing_none!()
                    }
                    BinaryExpressionRight::Name(n) => {
                        Some(state.get_fq_symbol_name_from_local_name(&n.get_name()))
                    }
                    BinaryExpressionRight::NullsafeMemberAccessExpression(_) => {
                        crate::missing_none!()
                    }
                    BinaryExpressionRight::QualifiedName(q) => Some(q.get_fq_name(state)),
                    BinaryExpressionRight::ScopedPropertyAccessExpression(_) => {
                        crate::missing_none!()
                    }
                    BinaryExpressionRight::SubscriptExpression(_) => {
                        crate::missing_none!()
                    }
                    BinaryExpressionRight::VariableName(_) => crate::missing_none!(),
                    BinaryExpressionRight::Extra(_) => crate::missing_none!(),
                };
                match (&**left, cname) {
                    (_PrimaryExpressionNode::VariableName(var_name), Some(cname)) => {
                        let symbol_data = state.symbol_data.clone();
                        return Some(match branch_side {
                            BranchSide::TrueBranch => {
                                let class_name: ClassName = cname.clone().into();
                                let _emitter = VoidEmitter::new();
                                new_scope_with_harden_variable_type_based_on_filter(
                                    scope,
                                    var_name,
                                    state,
                                    move |dtype: &&DiscreteType| {
                                        dtype.can_be_instance_of(cname.clone(), &symbol_data)
                                    },
                                    Some(Box::new(move |mut utype: UnionType| {
                                        if utype.len() == 0 {
                                            // In the case of no valid types left,
                                            // for best DX we inject the type we checked against, because it
                                            // is the only thing that will make sense
                                            // inside the branch, however,
                                            // the conditional should have detected this as an always-false
                                            // statement, and emitted accordingly
                                            utype.push(class_name.into());
                                        }
                                        utype
                                    })),
                                )
                            }
                            BranchSide::FalseBranch => {
                                new_scope_with_harden_variable_type_based_on_filter(
                                    scope,
                                    var_name,
                                    state,
                                    move |dtype: &&DiscreteType| {
                                        !dtype.can_be_instance_of(cname.clone(), &symbol_data)
                                    },
                                    None,
                                )
                            }
                        });
                    }
                    _ => (),
                }
            }
            _ => (),
        }
        None
    }
}
