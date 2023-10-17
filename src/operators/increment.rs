use crate::Range;

use super::operator::Operator;
#[derive(Clone, Debug)]

pub struct IncrementOperator(pub Range);

impl Operator for IncrementOperator {
    fn brief_desc(&self) -> String {
        "IncrementOperator".into()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "++"
    }
}
