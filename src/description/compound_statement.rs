use crate::autonodes::compound_statement::CompoundStatementNode;

use super::NodeDescription;

impl NodeDescription for CompoundStatementNode {
    fn intersect_up_traversal(&self) -> bool {
        true
    }
}
