use std::sync::{Arc, RwLock};

use crate::{
    autonodes::{
        _expression::_ExpressionNode,
        _primary_expression::_PrimaryExpressionNode,
        assignment_expression::{AssignmentExpressionLeft, AssignmentExpressionNode},
        binary_expression::{
            BinaryExpressionNode, BinaryExpressionOperator, BinaryExpressionRight,
        },
        member_access_expression::MemberAccessExpressionNode,
        parenthesized_expression::ParenthesizedExpressionNode,
        unary_op_expression::{UnaryOpExpressionNode, UnaryOpExpressionOperator},
        variable_name::VariableNameNode,
    },
    issue::VoidEmitter,
    symboldata::class::ClassName,
    types::union::{DiscreteType, UnionType},
};

use super::{
    scope::{BranchSide, Scope},
    state::AnalysisState,
};

use crate::analysis::scope::BranchableScope;

pub trait BranchTypeHardening {
    fn branch_with_hardened_types_base_on_conditional_node(
        &self,
        scope: Arc<RwLock<Scope>>,
        branch_side: BranchSide,
        state: &mut AnalysisState,
    ) -> Arc<RwLock<Scope>>;
}

impl BranchTypeHardening for ParenthesizedExpressionNode {
    fn branch_with_hardened_types_base_on_conditional_node(
        &self,
        scope: Arc<RwLock<Scope>>,
        branch_side: BranchSide,
        state: &mut AnalysisState,
    ) -> Arc<RwLock<Scope>> {
        //        scope.branch()
        self.child
            .branch_with_hardened_types_base_on_conditional_node(scope, branch_side, state)
    }
}

impl BranchTypeHardening for _ExpressionNode {
    fn branch_with_hardened_types_base_on_conditional_node(
        &self,
        scope: Arc<RwLock<Scope>>,
        branch_side: BranchSide,
        state: &mut AnalysisState,
    ) -> Arc<RwLock<Scope>> {
        match self {
            _ExpressionNode::_PrimaryExpression(p) => {
                return p.branch_with_hardened_types_base_on_conditional_node(
                    scope,
                    branch_side,
                    state,
                );
            }
            _ExpressionNode::AssignmentExpression(a) => {
                return a.branch_with_hardened_types_base_on_conditional_node(
                    scope,
                    branch_side,
                    state,
                );
            }
            _ExpressionNode::AugmentedAssignmentExpression(_) => crate::missing!(),
            _ExpressionNode::BinaryExpression(b) => {
                return b.branch_with_hardened_types_base_on_conditional_node(
                    scope,
                    branch_side,
                    state,
                );
            }
            _ExpressionNode::CastExpression(_) => crate::missing!(),
            _ExpressionNode::CloneExpression(_) => crate::missing!(),
            _ExpressionNode::ConditionalExpression(_) => crate::missing!(),
            _ExpressionNode::ExponentiationExpression(_) => crate::missing!(),
            _ExpressionNode::IncludeExpression(_) => crate::missing!(),
            _ExpressionNode::IncludeOnceExpression(_) => crate::missing!(),
            _ExpressionNode::MatchExpression(_) => crate::missing!(),
            _ExpressionNode::ReferenceAssignmentExpression(_) => crate::missing!(),
            _ExpressionNode::RequireExpression(_) => crate::missing!(),
            _ExpressionNode::RequireOnceExpression(_) => crate::missing!(),
            _ExpressionNode::SilenceExpression(_) => crate::missing!(),
            _ExpressionNode::UnaryOpExpression(u) => {
                return u.branch_with_hardened_types_base_on_conditional_node(
                    scope,
                    branch_side,
                    state,
                );
            }
            _ExpressionNode::YieldExpression(_) => crate::missing!(),
            _ExpressionNode::Comment(_) => crate::missing!(),
            _ExpressionNode::TextInterpolation(_) => crate::missing!(),
            _ExpressionNode::Error(_) => crate::missing!(),
        }

        scope.branch()
    }
}

impl BranchTypeHardening for _PrimaryExpressionNode {
    fn branch_with_hardened_types_base_on_conditional_node(
        &self,
        scope: Arc<RwLock<Scope>>,
        branch_side: BranchSide,
        state: &mut AnalysisState,
    ) -> Arc<RwLock<Scope>> {
        match self {
            _PrimaryExpressionNode::_Literal(_) => crate::missing!(),
            _PrimaryExpressionNode::AnonymousFunctionCreationExpression(_) => crate::missing!(),
            _PrimaryExpressionNode::ArrayCreationExpression(_) => crate::missing!(),
            _PrimaryExpressionNode::ArrowFunction(_) => crate::missing!(),
            _PrimaryExpressionNode::CastExpression(_) => crate::missing!(),
            _PrimaryExpressionNode::ClassConstantAccessExpression(_) => crate::missing!(),
            _PrimaryExpressionNode::DynamicVariableName(_) => crate::missing!(),
            _PrimaryExpressionNode::FunctionCallExpression(_) => return scope.branch(),
            _PrimaryExpressionNode::MemberAccessExpression(_) => {
                crate::missing!();
                /*                 return m.branch_with_hardened_types_base_on_conditional_node(
                    scope,
                    branch_side,
                    state,
                )*/
            }
            _PrimaryExpressionNode::MemberCallExpression(_) => crate::missing!(),
            _PrimaryExpressionNode::Name(_) => crate::missing!(),
            _PrimaryExpressionNode::NullsafeMemberAccessExpression(_) => crate::missing!(),
            _PrimaryExpressionNode::NullsafeMemberCallExpression(_) => crate::missing!(),
            _PrimaryExpressionNode::ObjectCreationExpression(_) => crate::missing!(),
            _PrimaryExpressionNode::ParenthesizedExpression(p) => {
                return p.branch_with_hardened_types_base_on_conditional_node(
                    scope,
                    branch_side,
                    state,
                )
            }
            _PrimaryExpressionNode::PrintIntrinsic(_) => return scope.branch(),
            _PrimaryExpressionNode::QualifiedName(_) => return scope.branch(),
            _PrimaryExpressionNode::ScopedCallExpression(_) => crate::missing!(),
            _PrimaryExpressionNode::ScopedPropertyAccessExpression(_) => crate::missing!(),
            _PrimaryExpressionNode::ShellCommandExpression(_) => return scope.branch(),
            _PrimaryExpressionNode::SubscriptExpression(_) => crate::missing!(),
            _PrimaryExpressionNode::ThrowExpression(_) => crate::missing!(),
            _PrimaryExpressionNode::UpdateExpression(_) => crate::missing!(),
            _PrimaryExpressionNode::VariableName(v) => {
                return v.branch_with_hardened_types_base_on_conditional_node(
                    scope,
                    branch_side,
                    state,
                );
            }

            _PrimaryExpressionNode::Comment(_) => crate::missing!(),
            _PrimaryExpressionNode::TextInterpolation(_) => crate::missing!(),
            _PrimaryExpressionNode::Error(_) => crate::missing!(),
        }
        return scope.branch();
    }
}

impl BranchTypeHardening for BinaryExpressionNode {
    fn branch_with_hardened_types_base_on_conditional_node(
        &self,
        scope: Arc<RwLock<Scope>>,
        branch_side: BranchSide,
        state: &mut AnalysisState,
    ) -> Arc<RwLock<Scope>> {
        match &*self.operator {
            BinaryExpressionOperator::NotEqual(_, _) => {
                crate::missing!("BinaryExpressionOperator::NotEqual")
            }
            BinaryExpressionOperator::NotIdentical(_, _) => {
                crate::missing!("BinaryExpressionOperator::NotIdentical")
            }
            BinaryExpressionOperator::Mod(_, _) => crate::missing!("BinaryExpressionOperator::Mod"),
            BinaryExpressionOperator::BinaryAnd(_, _) => {
                crate::missing!("BinaryExpressionOperator::BinaryAnd")
            }
            BinaryExpressionOperator::BooleanAnd(_, _) => {
                crate::missing!("BinaryExpressionOperator::BooleanAnd")
            }
            BinaryExpressionOperator::Mult(_, _) => {
                crate::missing!("BinaryExpressionOperator::Mult")
            }
            BinaryExpressionOperator::Add(_, _) => crate::missing!("BinaryExpressionOperator::Add"),
            BinaryExpressionOperator::Sub(_, _) => crate::missing!("BinaryExpressionOperator::Sub"),
            BinaryExpressionOperator::Concat(_, _) => {
                crate::missing!("BinaryExpressionOperator::Concat")
            }
            BinaryExpressionOperator::Div(_, _) => crate::missing!("BinaryExpressionOperator::Div"),
            BinaryExpressionOperator::LessThan(_, _) => {
                crate::missing!("BinaryExpressionOperator::LessThan")
            }
            BinaryExpressionOperator::LeftShift(_, _) => {
                crate::missing!("BinaryExpressionOperator::LeftShift")
            }
            BinaryExpressionOperator::LessThanOrEqual(_, _) => {
                crate::missing!("BinaryExpressionOperator::LessThanOrEqual")
            }
            BinaryExpressionOperator::Spaceship(_, _) => {
                crate::missing!("BinaryExpressionOperator::Spaceship")
            }
            BinaryExpressionOperator::Equal(_, _) => {
                crate::missing!("BinaryExpressionOperator::Equal")
            }
            BinaryExpressionOperator::Identical(_, _) => {
                crate::missing!("BinaryExpressionOperator::Identical")
            }
            BinaryExpressionOperator::GreaterThan(_, _) => {
                crate::missing!("BinaryExpressionOperator::GreaterThan")
            }
            BinaryExpressionOperator::GreaterThanOrEqual(_, _) => {
                crate::missing!("BinaryExpressionOperator::GreaterThanOrEqual")
            }
            BinaryExpressionOperator::RightShift(_, _) => {
                crate::missing!("BinaryExpressionOperator::RightShift")
            }
            BinaryExpressionOperator::BinaryXor(_, _) => {
                crate::missing!("BinaryExpressionOperator::BinaryXor")
            }
            BinaryExpressionOperator::And(_, _) => crate::missing!("BinaryExpressionOperator::And"),
            BinaryExpressionOperator::Instanceof(_, _) => {
                // void
                match (&self.left, &self.right) {
                    (_ExpressionNode::_PrimaryExpression(left), right) => {
                        // Attempt to find a class-name to check against
                        let cname = match &**right {
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
                            BinaryExpressionRight::Comment(_) => crate::missing_none!(),
                            BinaryExpressionRight::TextInterpolation(_) => {
                                crate::missing_none!()
                            }
                            BinaryExpressionRight::Error(_) => crate::missing_none!(),
                        };
                        match (&**left, cname) {
                            (_PrimaryExpressionNode::VariableName(var_name), Some(cname)) => {
                                let symbol_data = state.symbol_data.clone();
                                return match branch_side {
                                    BranchSide::TrueBranch => {
                                        let class_name: ClassName = cname.clone().into();
                                        let _emitter = VoidEmitter::new();
                                        new_scope_with_harden_variable_type_based_on_filter(
                                            scope,
                                            &**var_name,
                                            state,
                                            move |dtype: &&DiscreteType| {
                                                let res = dtype.can_be_instance_of(
                                                    cname.clone(),
                                                    &symbol_data,
                                                );
                                                res
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
                                            &**var_name,
                                            state,
                                            move |dtype: &&DiscreteType| {
                                                !dtype
                                                    .can_be_instance_of(cname.clone(), &symbol_data)
                                            },
                                            None,
                                        )
                                    }
                                };
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
            BinaryExpressionOperator::Or(_, _) => crate::missing!("BinaryExpressionOperator::Or"),
            BinaryExpressionOperator::Xor(_, _) => crate::missing!("BinaryExpressionOperator::Xor"),
            BinaryExpressionOperator::BinaryOr(_, _) => {
                crate::missing!("BinaryExpressionOperator::BinaryOr")
            }
            BinaryExpressionOperator::BooleanOr(_, _) => {
                crate::missing!("BinaryExpressionOperator::BooleanOr")
            }

            BinaryExpressionOperator::NullCoalescing(_, _) => {
                crate::missing!("BinaryExpressionOperator::NullCoalescing")
            }

            BinaryExpressionOperator::Comment(_)
            | BinaryExpressionOperator::TextInterpolation(_)
            | BinaryExpressionOperator::Error(_) => (),
        }

        scope.branch()
    }
}

impl BranchTypeHardening for UnaryOpExpressionNode {
    fn branch_with_hardened_types_base_on_conditional_node(
        &self,
        scope: Arc<RwLock<Scope>>,
        branch_side: BranchSide,
        state: &mut AnalysisState,
    ) -> Arc<RwLock<Scope>> {
        if let Some(op) = &self.operator {
            match &**op {
                UnaryOpExpressionOperator::Not(_, _) => {
                    if let Some(e) = &self.expr {
                        return e.branch_with_hardened_types_base_on_conditional_node(
                            scope,
                            branch_side.inverse(),
                            state,
                        );
                    }
                }
                UnaryOpExpressionOperator::Add(_, _) => crate::missing!(),
                UnaryOpExpressionOperator::Sub(_, _) => crate::missing!(),
                UnaryOpExpressionOperator::BinaryNot(_, _) => crate::missing!(),

                UnaryOpExpressionOperator::Comment(_) => crate::missing!(),
                UnaryOpExpressionOperator::TextInterpolation(_) => crate::missing!(),
                UnaryOpExpressionOperator::Error(_) => crate::missing!(),
            }
        }
        scope.branch()
    }
}

fn new_scope_with_harden_variable_type_based_on_filter<P>(
    scope: Arc<RwLock<Scope>>,
    variable_node: &VariableNameNode,
    state: &mut AnalysisState,
    predicate: P,
    final_utype_wrapper: Option<Box<dyn FnOnce(UnionType) -> UnionType>>,
) -> Arc<RwLock<Scope>>
where
    P: Sized + FnMut(&&DiscreteType) -> bool,
{
    // FIXME
    // If this is a nullable type and we're evaluated as true, for other types than,
    // string, int, float and bool we can strip the nullable
    // and inversly, if we evaluate to false, then we're left only with null
    //
    let new_scope = scope.branch();
    let emitter = VoidEmitter::new();

    let utype = if let Some(utype) = variable_node.get_utype(state, &emitter) {
        utype
    } else {
        return new_scope;
    };

    let new_type = utype.filter_types(predicate);

    let new_type = if let Some(wrapper) = final_utype_wrapper {
        wrapper(new_type)
    } else {
        new_type
    };

    let var_name = variable_node.get_variable_name();

    let var_data = {
        let mut writeable_scope = new_scope.write().unwrap();
        writeable_scope.get_or_create_local_var(var_name)
    };

    {
        let mut data = var_data.write().unwrap();

        // FIXME for starters we do it this way
        data.all_written_data.push((new_type.clone(), None));
        data.last_written_data = vec![(new_type, None)];
    }

    new_scope
}

impl BranchTypeHardening for VariableNameNode {
    fn branch_with_hardened_types_base_on_conditional_node(
        &self,
        scope: Arc<RwLock<Scope>>,
        branch_side: BranchSide,
        state: &mut AnalysisState,
    ) -> Arc<RwLock<Scope>> {
        let predicate = match branch_side {
            BranchSide::TrueBranch => |dtype: &&DiscreteType| dtype.can_evaluate_to_true(),
            BranchSide::FalseBranch => |dtype: &&DiscreteType| dtype.can_evaluate_to_false(),
        };
        new_scope_with_harden_variable_type_based_on_filter(scope, self, state, predicate, None)
    }
}

impl BranchTypeHardening for MemberAccessExpressionNode {
    fn branch_with_hardened_types_base_on_conditional_node(
        &self,
        scope: Arc<RwLock<Scope>>,
        _branch_side: BranchSide,
        _state: &mut AnalysisState,
    ) -> Arc<RwLock<Scope>> {
        scope.branch()
        /*   let predicate = match branch_side {
            BranchSide::TrueBranch => |dtype: &&DiscreteType| dtype.can_evaluate_to_true(),
            BranchSide::FalseBranch => |dtype: &&DiscreteType| dtype.can_evaluate_to_false(),
        };
        new_scope_with_harden_variable_type_based_on_filter(scope, self, state, predicate, None)*/
    }
}

impl BranchTypeHardening for AssignmentExpressionNode {
    fn branch_with_hardened_types_base_on_conditional_node(
        &self,
        scope: Arc<RwLock<Scope>>,
        branch_side: BranchSide,
        state: &mut AnalysisState,
    ) -> Arc<RwLock<Scope>> {
        match &*self.left {
            AssignmentExpressionLeft::CastExpression(_) => crate::missing!(),
            AssignmentExpressionLeft::DynamicVariableName(_) => crate::missing!(),
            AssignmentExpressionLeft::FunctionCallExpression(_) => crate::missing!(),
            AssignmentExpressionLeft::ListLiteral(_) => crate::missing!(),
            AssignmentExpressionLeft::MemberAccessExpression(_) => crate::missing!(),
            AssignmentExpressionLeft::MemberCallExpression(_) => crate::missing!(),
            AssignmentExpressionLeft::NullsafeMemberAccessExpression(_) => crate::missing!(),
            AssignmentExpressionLeft::NullsafeMemberCallExpression(_) => crate::missing!(),
            AssignmentExpressionLeft::ScopedCallExpression(_) => crate::missing!(),
            AssignmentExpressionLeft::ScopedPropertyAccessExpression(_) => crate::missing!(),
            AssignmentExpressionLeft::SubscriptExpression(_) => crate::missing!(),
            AssignmentExpressionLeft::VariableName(v) => {
                return v.branch_with_hardened_types_base_on_conditional_node(
                    scope,
                    branch_side,
                    state,
                )
            }
            AssignmentExpressionLeft::Comment(_) => crate::missing!(),
            AssignmentExpressionLeft::TextInterpolation(_) => crate::missing!(),
            AssignmentExpressionLeft::Error(_) => crate::missing!(),
        }
        return scope.branch();
    }
}
