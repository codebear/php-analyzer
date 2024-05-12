use crate::{
    autonodes::class_constant_access_identifier::{
        ClassConstantAccessIdentifierChildren, ClassConstantAccessIdentifierNode,
    },
    symbols::Name,
};

impl ClassConstantAccessIdentifierNode {
    pub fn get_name(&self) -> Name {
        let Some(children) = &self.child else {
            panic!("Expected Name");
        };
        match &**children {
            ClassConstantAccessIdentifierChildren::_Expression(_) => todo!(),
            ClassConstantAccessIdentifierChildren::Name(n) => n.get_name(),
            ClassConstantAccessIdentifierChildren::Extra(_) => todo!(),
        }
    }
}
