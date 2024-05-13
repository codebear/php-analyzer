use std::sync::{Arc, RwLock};

use crate::{
    autonodes::{
        _expression::_ExpressionNode,
        _primary_expression::_PrimaryExpressionNode,
        assignment_expression::{AssignmentExpressionLeft, AssignmentExpressionNode},
        binary_expression::{BinaryExpressionNode, BinaryExpressionOperator},
        member_access_expression::MemberAccessExpressionNode,
        parenthesized_expression::ParenthesizedExpressionNode,
        unary_op_expression::{UnaryOpExpressionNode, UnaryOpExpressionOperator},
        variable_name::VariableNameNode,
    },
    issue::VoidEmitter,
    operators::binary::BinaryOperatorBranchTypeHardening,
    types::union::{DiscreteType, UnionType},
};

use super::{
    scope::{BranchSide, Scope},
    state::AnalysisState,
};

use crate::analysis::scope::BranchableScope;

// Denne skal kanskje ikke vaere her eller noe. Finn ut.
pub fn new_scope_with_harden_variable_type_based_on_filter<P>(
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
            _ExpressionNode::IncludeExpression(_) => crate::missing!(),
            _ExpressionNode::IncludeOnceExpression(_) => crate::missing!(),
            _ExpressionNode::MatchExpression(_) => crate::missing!(),
            _ExpressionNode::ReferenceAssignmentExpression(_) => crate::missing!(),
            _ExpressionNode::RequireExpression(_) => crate::missing!(),
            _ExpressionNode::RequireOnceExpression(_) => crate::missing!(),
            _ExpressionNode::ErrorSuppressionExpression(_) => crate::missing!(),
            _ExpressionNode::UnaryOpExpression(u) => {
                return u.branch_with_hardened_types_base_on_conditional_node(
                    scope,
                    branch_side,
                    state,
                );
            }
            _ExpressionNode::YieldExpression(_) => crate::missing!(),
            _ExpressionNode::Extra(_) => crate::missing!(),
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

            _PrimaryExpressionNode::Extra(_) => crate::missing!(),
        }
        scope.branch()
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
            BinaryExpressionOperator::NotEqual(_) => {
                crate::missing!("BinaryExpressionOperator::NotEqual")
            }
            BinaryExpressionOperator::NotIdentical(_) => {
                crate::missing!("BinaryExpressionOperator::NotIdentical")
            }
            BinaryExpressionOperator::Mod(_) => crate::missing!("BinaryExpressionOperator::Mod"),
            BinaryExpressionOperator::BinaryAnd(_) => {
                crate::missing!("BinaryExpressionOperator::BinaryAnd")
            }
            BinaryExpressionOperator::BooleanAnd(_) => {
                crate::missing!("BinaryExpressionOperator::BooleanAnd")
            }
            BinaryExpressionOperator::Mult(_) => {
                crate::missing!("BinaryExpressionOperator::Mult")
            }
            BinaryExpressionOperator::Add(_) => crate::missing!("BinaryExpressionOperator::Add"),
            BinaryExpressionOperator::Sub(_) => crate::missing!("BinaryExpressionOperator::Sub"),
            BinaryExpressionOperator::Concat(_) => {
                crate::missing!("BinaryExpressionOperator::Concat")
            }
            BinaryExpressionOperator::Div(_) => crate::missing!("BinaryExpressionOperator::Div"),
            BinaryExpressionOperator::LessThan(_) => {
                crate::missing!("BinaryExpressionOperator::LessThan")
            }
            BinaryExpressionOperator::LeftShift(_) => {
                crate::missing!("BinaryExpressionOperator::LeftShift")
            }
            BinaryExpressionOperator::LessThanOrEqual(_) => {
                crate::missing!("BinaryExpressionOperator::LessThanOrEqual")
            }
            BinaryExpressionOperator::Spaceship(_) => {
                crate::missing!("BinaryExpressionOperator::Spaceship")
            }
            BinaryExpressionOperator::Equal(_) => {
                crate::missing!("BinaryExpressionOperator::Equal")
            }
            BinaryExpressionOperator::Identical(_) => {
                crate::missing!("BinaryExpressionOperator::Identical")
            }
            BinaryExpressionOperator::GreaterThan(_) => {
                crate::missing!("BinaryExpressionOperator::GreaterThan")
            }
            BinaryExpressionOperator::GreaterThanOrEqual(_) => {
                crate::missing!("BinaryExpressionOperator::GreaterThanOrEqual")
            }
            BinaryExpressionOperator::RightShift(_) => {
                crate::missing!("BinaryExpressionOperator::RightShift")
            }
            BinaryExpressionOperator::BinaryXor(_) => {
                crate::missing!("BinaryExpressionOperator::BinaryXor")
            }
            BinaryExpressionOperator::LogicalAnd(_) => {
                crate::missing!("BinaryExpressionOperator::LogicalAnd")
            }
            BinaryExpressionOperator::Instanceof(instanceof_operator) => {
                if let Some(x) = instanceof_operator
                    .branch_with_hardened_types_base_on_conditional_node(
                        &self.left,
                        &self.right,
                        scope.clone(),
                        branch_side,
                        state,
                    )
                {
                    return x;
                }
            }
            BinaryExpressionOperator::LogicalOr(_) => {
                crate::missing!("BinaryExpressionOperator::LogicalOr")
            }
            BinaryExpressionOperator::LogicalXor(_) => {
                crate::missing!("BinaryExpressionOperator::LogicalXor")
            }
            BinaryExpressionOperator::BinaryOr(_) => {
                crate::missing!("BinaryExpressionOperator::BinaryOr")
            }
            BinaryExpressionOperator::BooleanOr(_) => {
                crate::missing!("BinaryExpressionOperator::BooleanOr")
            }

            BinaryExpressionOperator::NullCoalesce(_) => {
                crate::missing!("BinaryExpressionOperator::NullCoalescing")
            }

            BinaryExpressionOperator::Exponential(_) => {
                crate::missing!("BinaryExpressionOperator::Exponential")
            }
            BinaryExpressionOperator::Extra(_) => (),
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
        match &*self.operator {
            UnaryOpExpressionOperator::Not(_) => {
                self.argument
                    .branch_with_hardened_types_base_on_conditional_node(
                        scope.clone(),
                        branch_side.inverse(),
                        state,
                    );
            }
            UnaryOpExpressionOperator::Add(_) => crate::missing!(),
            UnaryOpExpressionOperator::Sub(_) => crate::missing!(),
            UnaryOpExpressionOperator::BinaryNot(_) => crate::missing!(),

            UnaryOpExpressionOperator::Extra(_) => crate::missing!(),
        }

        scope.branch()
    }
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
            AssignmentExpressionLeft::Extra(_) => crate::missing!(),
        }
        scope.branch()
    }
}
