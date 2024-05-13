//use tree_sitter::Range;
use crate::parser::Range;
use crate::{
    autonodes::{
        any::AnyNodeRef,
        augmented_assignment_expression::AugmentedAssignmentExpressionOperator,
        binary_expression::BinaryExpressionOperator,
        unary_op_expression::UnaryOpExpressionOperator,
        update_expression::{UpdateExpressionPostfix, UpdateExpressionPrefix},
    },
    autotree::NodeAccess,
    types::union::UnionType,
    value::PHPValue,
};

pub trait Operator {
    fn range(&self) -> Range;

    fn operator(&self) -> &'static str;

    fn kind(&self) -> &'static str {
        self.operator()
    }

    fn brief_desc(&self) -> String;

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        vec![]
    }
}

pub trait OperatorOperandAccess {
    fn get_value(&self) -> Option<&PHPValue>;

    fn get_type(&self) -> Option<&UnionType>;
}

#[derive(Debug, Clone)]
pub enum Operators<'a> {
    Binary(&'a BinaryExpressionOperator),
    Unary(&'a UnaryOpExpressionOperator),
    AugmentedAssignment(&'a AugmentedAssignmentExpressionOperator),
    UpdateExpressionPrefix(&'a UpdateExpressionPrefix),
    UpdateExpressionPostfix(&'a UpdateExpressionPostfix),
}

impl<'a> From<&'a BinaryExpressionOperator> for Operators<'a> {
    fn from(value: &'a BinaryExpressionOperator) -> Self {
        Self::Binary(value)
    }
}

impl<'a> From<&'a UnaryOpExpressionOperator> for Operators<'a> {
    fn from(value: &'a UnaryOpExpressionOperator) -> Self {
        Self::Unary(value)
    }
}

impl<'a> From<&'a AugmentedAssignmentExpressionOperator> for Operators<'a> {
    fn from(value: &'a AugmentedAssignmentExpressionOperator) -> Self {
        Self::AugmentedAssignment(value)
    }
}

impl<'a> From<&'a UpdateExpressionPrefix> for Operators<'a> {
    fn from(value: &'a UpdateExpressionPrefix) -> Self {
        Self::UpdateExpressionPrefix(value)
    }
}

impl<'a> From<&'a UpdateExpressionPostfix> for Operators<'a> {
    fn from(value: &'a UpdateExpressionPostfix) -> Self {
        Self::UpdateExpressionPostfix(value)
    }
}

impl<'a> Operators<'a> {
    pub fn kind(&self) -> &'static str {
        match self {
            Operators::Binary(op) => op.kind(),
            Operators::Unary(op) => op.kind(),
            Operators::AugmentedAssignment(op) => op.kind(),
            Operators::UpdateExpressionPrefix(op) => op.kind(),
            Operators::UpdateExpressionPostfix(op) => op.kind(),
        }
    }
}

impl<'a> NodeAccess for Operators<'a> {
    fn brief_desc(&self) -> String {
        match self {
            Operators::Binary(op) => op.brief_desc(),
            Operators::Unary(op) => op.brief_desc(),
            Operators::AugmentedAssignment(op) => op.brief_desc(),
            Operators::UpdateExpressionPrefix(op) => op.brief_desc(),
            Operators::UpdateExpressionPostfix(op) => op.brief_desc(),
        }
    }

    fn range(&self) -> Range {
        match self {
            Operators::Binary(op) => op.range(),
            Operators::Unary(op) => op.range(),
            Operators::AugmentedAssignment(op) => op.range(),
            Operators::UpdateExpressionPrefix(op) => op.range(),
            Operators::UpdateExpressionPostfix(op) => op.range(),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            Operators::Binary(op) => op.as_any(),
            Operators::Unary(op) => op.as_any(),
            Operators::AugmentedAssignment(op) => op.as_any(),
            Operators::UpdateExpressionPrefix(op) => op.as_any(),
            Operators::UpdateExpressionPostfix(op) => op.as_any(),
        }
    }
}
