use tree_sitter::Range;

use crate::{types::union::UnionType, value::PHPValue};

pub trait Operator {
    fn range(&self) -> Range;

    fn operator(&self) -> &'static str;

    fn kind(&self) -> &'static str {
        self.operator()
    }

    fn brief_desc(&self) -> String;
}

pub trait OperatorOperandAccess {
    fn get_value(&self) -> Option<&PHPValue>;

    fn get_type(&self) -> Option<&UnionType>;
}
