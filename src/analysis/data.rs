//use tree_sitter::Range;
use crate::parser::Range;

use crate::{symbols::Name, types::union::UnionType, value::PHPValue};

#[derive(Debug)]
pub struct VarData {
    pub name: Name,
    pub comment_declared_type: Option<UnionType>,
    pub php_declared_type: Option<UnionType>,
    pub default_value: Option<PHPValue>,
    pub all_written_data: Vec<(UnionType, Option<PHPValue>)>,
    pub last_written_data: Vec<(UnionType, Option<PHPValue>)>,
    pub written_to: usize,
    pub read_from: usize,
    pub referenced_ranges: Vec<Range>,
    pub is_argument: bool,
    // Some branches did not initialize this variable
    pub is_partial: bool,
}

impl VarData {
    pub fn new(name: Name) -> Self {
        Self {
            name,
            php_declared_type: None,
            comment_declared_type: None,
            default_value: None,
            all_written_data: vec![],
            last_written_data: vec![],
            written_to: 0,
            read_from: 0,
            referenced_ranges: vec![],
            is_argument: false,
            is_partial: false,
        }
    }

    ///
    /// Best guess on type from all three sources
    pub fn get_utype(&self) -> Option<UnionType> {
        None
    }

    pub fn get_declared_type(&self) -> Option<UnionType> {
        return self.php_declared_type.clone();
    }

    pub fn get_inferred_type(&self) -> Option<UnionType> {
        let types: Vec<_> = self.all_written_data.iter().map(|x| x.0.clone()).collect();
        if types.len() > 0 {
            Some(UnionType::reduce(types))
        } else {
            None
        }
    }

    pub fn single_write_to(&mut self, utype: UnionType, value: Option<PHPValue>) {
        let data = (utype, value);
        self.all_written_data.push(data.clone());
        self.last_written_data = vec![data];
        self.written_to += 1;
    }

    /// when multiple branches, which have all written to it is joined
    pub fn multi_write_to(
        &mut self,
        _last: Vec<(UnionType, Option<PHPValue>)>,
        _all_data: Vec<(UnionType, Option<PHPValue>)>,
    ) {
        todo!();
        /*         let data = (utype, value);
        self.all_written_data.push(data.clone());
        self.last_written_data = vec![data];
        self.written_to += 1; */
    }
}
