use crate::Range;

use super::operator::Operator;
#[derive(Clone, Debug)]

pub struct DecrementOperator(pub Range);

impl Operator for DecrementOperator {
    fn brief_desc(&self) -> String {
        "DecrementOperator".into()
    }

    fn range(&self) -> Range {
        self.0
    }

    fn operator(&self) -> &'static str {
        "--"
    }
}
