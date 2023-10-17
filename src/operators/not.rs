use crate::Range;

use super::{operator::Operator, unary::UnaryOperator};

#[derive(Clone, Debug)]
pub struct NotOperator(pub Range);

impl Operator for NotOperator {
    fn brief_desc(&self) -> String {
        "NotOperator".into()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "!"
    }
}

impl UnaryOperator for NotOperator {}
