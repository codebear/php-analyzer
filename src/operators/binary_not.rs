use crate::Range;

use super::{operator::Operator, unary::UnaryOperator};

#[derive(Clone, Debug)]
pub struct BinaryNotOperator(pub Range);

impl Operator for BinaryNotOperator {
    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "~"
    }

    fn brief_desc(&self) -> String {
        "BinaryNotOperator".into()
    }
}

impl UnaryOperator for BinaryNotOperator {}
