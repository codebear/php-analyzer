use crate::{
    analysis::state::AnalysisState,
    symbols::{FullyQualifiedName, Name},
    types::union::UnionType,
    value::PHPValue,
};
use std::{
    collections::{BTreeSet, HashMap},
    sync::{Arc, RwLock},
};

use super::{FileLocation, SymbolData};

type MethodName = Name;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ClassName {
    pub name: Name,
    pub fq_name: FullyQualifiedName,
}

impl ClassName {
    pub fn new_with_names(name: Name, fq_name: FullyQualifiedName) -> Self {
        Self { name, fq_name }
    }

    pub fn new_with_fq_name(fq_name: FullyQualifiedName) -> Self {
        let name = fq_name.get_name().unwrap_or_else(|| Name::new());
        Self { name, fq_name }
    }

    pub fn new_with_analysis_state(name: &Name, state: &AnalysisState) -> Self {
        Self::new_with_names(name.clone(), state.get_fq_symbol_name_from_local_name(name))
    }

    /// For use on the declared name of a class, interface, trait
    pub fn new_with_analysis_state_without_aliasing(name: &Name, state: &AnalysisState) -> Self {
        Self {
            name: name.clone(),
            fq_name: state.get_fq_symbol_name_without_aliasing(name),
        }
    }

    pub fn get_fq_name(&self) -> &FullyQualifiedName {
        &self.fq_name
    }

    pub fn get_name(&self) -> &Name {
        &self.name
    }

    pub(crate) fn get_namespace(&self) -> FullyQualifiedName {
        let mut fq_name = self.get_fq_name().clone();
        fq_name.pop();
        fq_name
    }
}

impl From<FullyQualifiedName> for ClassName {
    fn from(fq_name: FullyQualifiedName) -> Self {
        ClassName::new_with_fq_name(fq_name)
    }
}

impl From<&FullyQualifiedName> for ClassName {
    fn from(fq_name: &FullyQualifiedName) -> Self {
        ClassName::new_with_fq_name(fq_name.clone())
    }
}


#[derive(Clone, Debug)]
pub enum ClassType {
    None,
    Class(ClassData),
    Interface(InterfaceData),
    Trait(TraitData),
}

impl ClassType {
    pub fn get_own_method(&self, method_name: &Name) -> Option<Arc<RwLock<MethodData>>> {
        match self {
            ClassType::None => None,
            ClassType::Class(c) => c.get_own_method(method_name),
            ClassType::Interface(i) => i.get_own_method(method_name),
            ClassType::Trait(t) => t.get_own_method(method_name),
        }
    }

    pub fn get_fq_name(&self) -> FullyQualifiedName {
        match self {
            ClassType::None => FullyQualifiedName::new(),
            ClassType::Class(c) => c.get_fq_name(),
            ClassType::Interface(i) => i.get_fq_name(),
            ClassType::Trait(t) => t.get_fq_name(),
        }
    }

    pub fn get_method(
        &self,
        method_name: &Name,
        symbol_data: Arc<SymbolData>,
    ) -> Option<MethodData> {
        match self {
            ClassType::None => None,
            ClassType::Class(c) => c.get_method(method_name, symbol_data),
            ClassType::Interface(i) => i.get_method(method_name, symbol_data),
            ClassType::Trait(t) => t.get_method(method_name, symbol_data),
        }
    }

    pub fn get_or_create_method(
        &mut self,
        method_name: &Name,
        position: FileLocation,
    ) -> Arc<RwLock<MethodData>> {
        match self {
            ClassType::None => panic!(),
            ClassType::Class(c) => c.get_or_create_method(method_name, position),
            ClassType::Interface(i) => i.get_or_create_method(method_name, position),
            ClassType::Trait(t) => t.get_or_create_method(method_name, position),
        }
    }

    pub fn get_or_create_property(
        &mut self,
        property_name: &Name,
        position: FileLocation,
    ) -> Option<Arc<RwLock<PropertyData>>> {
        match self {
            ClassType::None => panic!(),
            ClassType::Class(c) => c.get_or_create_property(property_name, position),
            ClassType::Interface(_) => None,
            ClassType::Trait(_) => None,
        }
    }

    pub fn with_generic_args(&self, generic_args: &Vec<UnionType>) -> Self {
        if generic_args.len() > 0 {
            crate::missing!("Gi ut en type som er typesatt med generiske argumenter");
        }
        self.clone()
    }

    pub fn implements(&self, iname: &ClassName, symbol_data: Arc<SymbolData>) -> bool {
        match self {
            ClassType::None => false,
            ClassType::Class(c) => c.implements(iname, symbol_data),
            ClassType::Interface(i) => i.implements(iname, symbol_data),
            ClassType::Trait(_) => {
                // traits don't have interface-support, yet 
                // https://wiki.php.net/rfc/traits-with-interfaces
                false
            }
        }  
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ClassModifier {
    Abstract,
    Final,
    None,
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum ClassMemberVisibility {
    Public,
    Private,
    Protected,
}

#[derive(Clone, Debug)]
pub struct ClassData {
    pub class_name: ClassName,
    pub position: FileLocation,
    pub base_class_name: Option<ClassName>,
    pub interfaces: Vec<ClassName>,
    pub modifier: ClassModifier,
    pub constants: HashMap<Name, PHPValue>,
    pub methods: HashMap<Name, Arc<RwLock<MethodData>>>,
    pub properties: HashMap<Name, Arc<RwLock<PropertyData>>>,
    pub is_native: bool,
    // FIXME, trait-imports are much more complex
    pub traits: Vec<ClassName>,
}

impl ClassData {
    pub fn new(position: FileLocation, class_name: ClassName) -> Self {
        Self {
            class_name,
            position,
            base_class_name: None,
            interfaces: vec![],
            modifier: ClassModifier::None,
            constants: HashMap::new(),
            methods: HashMap::new(),
            properties: HashMap::new(),
            traits: vec![],
            is_native: false,
        }
    }

    pub fn get_own_method(&self, method_name: &Name) -> Option<Arc<RwLock<MethodData>>> {
        self.methods.get(&method_name.to_ascii_lowercase()).cloned()
    }

    pub fn get_method(
        &self,
        method_name: &Name,
        symbol_data: Arc<SymbolData>,
    ) -> Option<MethodData> {
        if let Some(m) = self.methods.get(&method_name.to_ascii_lowercase()) {
            return Some(m.read().unwrap().clone());
        }

        if let Some(base) = &self.base_class_name {
            if let Some(cdata_handle) = symbol_data.get_class(base) {
                let cdata = cdata_handle.read().unwrap();
                if let Some(m) = (*cdata).get_method(method_name, symbol_data) {
                    return Some(m);
                }
            }
        }
        for _ptrait in &self.traits {
            // FIXME trait-imports are more complex than this, we're just skipping it for now
            crate::missing!("look for unknown method in imported trait");
        }

        None
    }

    fn get_methods_from_interfaces(
        &self,
        method_name: &Name,
        symbol_data: Arc<SymbolData>,
    ) -> Option<Vec<MethodData>> {
        let mut set: BTreeSet<MethodData> = BTreeSet::new();

        for iface in &self.interfaces {
            if let Some(iface_data) = &symbol_data.get_interface(iface) {
                if let Some(mdata) = &iface_data.get_method(&method_name, symbol_data.clone()) {
                    set.insert(mdata.clone());
                }
            }
        }
        if set.len() > 0 {
            Some(set.iter().cloned().collect())
        } else {
            None
        }
    }

    pub fn get_or_create_method(
        &mut self,
        method_name: &Name,
        position: FileLocation,
    ) -> Arc<RwLock<MethodData>> {
        let class_name = self.class_name.clone();
        let entry = self
            .methods
            .entry(method_name.to_ascii_lowercase())
            .or_insert_with(|| Arc::new(RwLock::new(MethodData::new(position, class_name))));

        entry.clone()
    }

    fn get_fq_name(&self) -> FullyQualifiedName {
        self.class_name.fq_name.clone()
    }

    pub(crate) fn get_or_create_property(
        &mut self,
        property_name: &Name,
        position: FileLocation,
    ) -> Option<Arc<RwLock<PropertyData>>> {
        let entry = self
            .properties
            .entry(property_name.clone())
            .or_insert_with(|| {
                Arc::new(RwLock::new(PropertyData::new(
                    position,
                    property_name.clone(),
                )))
            });

        Some(entry.clone())
    }

    pub(crate) fn get_mut_property(
        &self,
        property_name: &Name,
        _state: &mut AnalysisState,
    ) -> Option<Arc<RwLock<PropertyData>>> {
        self.properties.get(property_name).cloned()
    }

    pub fn get_property(
        &self,
        property_name: &Name,
        state: &AnalysisState,
    ) -> Option<PropertyData> {
        if let Some(m) = self.properties.get(property_name) {
            return Some(m.read().unwrap().clone());
        }

        if let Some(base) = &self.base_class_name {
            if let Some(cdata_handle) = state.symbol_data.get_class(base) {
                let cdata = cdata_handle.read().unwrap();
                match &*cdata {
                    ClassType::Class(c) => {
                        return c.get_property(&property_name, state);
                    }
                    _ => (),
                }
            }
        }
        /* Traits don't have properties?
        for _ptrait in &self.traits {
            // FIXME trait-imports are more complex than this, we're just skipping it for now
            crate::missing!("look for unknown method in imported trait");
        }*/

        None
    }

    pub fn implements(&self, iname: &ClassName, symbol_data: Arc<SymbolData>) -> bool {
        for iface in &self.interfaces {
            if let Some(iface_data) = &symbol_data.get_interface(iface) {
                if iface_data.implements(iname, symbol_data.clone()) {
                    return true;
                }
            }
        }
        if let Some(base) = &self.base_class_name {
            if let Some(cdata_handle) = symbol_data.get_class(base) {
                let cdata = cdata_handle.read().unwrap();
                return cdata.implements(iname, symbol_data);
            }
        }
        return false;
    }
}

#[derive(Debug, Clone)]
pub struct InterfaceData {
    pub interface_name: ClassName,
    pub position: FileLocation,
    pub base_interface_names: Option<Vec<ClassName>>,
    pub constants: HashMap<Name, PHPValue>,
    pub methods: HashMap<Name, Arc<RwLock<MethodData>>>,
}

impl InterfaceData {
    pub fn new(position: FileLocation, interface_name: ClassName) -> Self {
        Self {
            interface_name,
            position,
            base_interface_names: None,
            constants: HashMap::new(),
            methods: HashMap::new(),
        }
    }

    pub fn get_own_method(&self, method_name: &Name) -> Option<Arc<RwLock<MethodData>>> {
        self.methods.get(method_name).cloned()
    }

    pub fn get_method(
        &self,
        method_name: &Name,
        symbol_data: Arc<SymbolData>,
    ) -> Option<MethodData> {
        let lc_mname = method_name.to_ascii_lowercase();
        if let Some(mdata) = self.methods.get(&lc_mname) {
            Some(mdata.read().unwrap().clone())
        } else if let Some(bases) = &self.base_interface_names {
            for base in bases {
                if let Some(locked_idata) = symbol_data.get_class(base) {
                    let unlocked = locked_idata.read().unwrap();
                    match &*unlocked {
                        ClassType::Interface(idata) => {
                            if let Some(mdata) = idata.get_method(&lc_mname, symbol_data.clone()) {
                                return Some(mdata);
                            }
                        }
                        _ => crate::missing!(
                            "Found non-interface as interface-parent, should emit something?"
                        ),
                    }
                }
            }

            crate::missing_none!("Interface.get_method(..) look in parent interface(s)?")
        } else {
            None
        }
    }

    pub fn get_or_create_method(
        &mut self,
        method_name: &Name,
        position: FileLocation,
    ) -> Arc<RwLock<MethodData>> {
        let interface_name = self.interface_name.clone();
        let entry = self
            .methods
            .entry(method_name.to_ascii_lowercase())
            .or_insert_with(|| Arc::new(RwLock::new(MethodData::new(position, interface_name))));

        entry.clone()
    }

    pub fn get_fq_name(&self) -> FullyQualifiedName {
        self.interface_name.fq_name.clone()
    }

    pub fn implements(&self, iname: &ClassName, symbol_data: Arc<SymbolData>) -> bool {
        let fq_iname = iname.get_fq_name();
        if self.get_fq_name().eq(iname.get_fq_name()) {
            return true;
        }
        let parent_inames = if let Some(i) = &self.base_interface_names {
            i
        } else {
            return false;
        };

        for parent_iname in parent_inames {
            if parent_iname.get_fq_name().eq(fq_iname) {
                return true;
            }
        }
        for parent_iname in parent_inames {
            if let Some(idata) = symbol_data.get_interface(&parent_iname) {
                if idata.implements(iname, symbol_data.clone()) {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Clone, Debug)]
pub struct TraitData {
    pub trait_name: ClassName,
    pub position: FileLocation,
    pub base_name: Option<ClassName>,
    pub methods: HashMap<Name, Arc<RwLock<MethodData>>>,
    pub is_native: bool,
}

impl TraitData {
    pub fn new(position: FileLocation, trait_name: ClassName) -> Self {
        Self {
            trait_name,
            position,
            base_name: None,
            methods: HashMap::new(),
            is_native: false,
        }
    }
    pub fn get_own_method(&self, method_name: &Name) -> Option<Arc<RwLock<MethodData>>> {
        self.methods.get(&method_name.to_ascii_lowercase()).cloned()
    }
    pub fn get_method(
        &self,
        _method_name: &Name,
        _symbol_data: Arc<SymbolData>,
    ) -> Option<MethodData> {
        crate::missing_none!("Trait.get_method(..)")
    }

    pub fn get_or_create_method(
        &mut self,
        method_name: &Name,
        position: FileLocation,
    ) -> Arc<RwLock<MethodData>> {
        let trait_name = self.trait_name.clone();
        let entry = self
            .methods
            .entry(method_name.to_ascii_lowercase())
            .or_insert_with(|| Arc::new(RwLock::new(MethodData::new(position, trait_name))));

        entry.clone()
    }

    fn get_fq_name(&self) -> FullyQualifiedName {
        self.trait_name.fq_name.clone()
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct FunctionArgumentData {
    pub name: Name,
    pub arg_type: Option<UnionType>,
    pub default_value: Option<PHPValue>,
    pub nullable: bool,
    pub optional: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MethodData {
    pub name: Name,
    pub description: String,
    pub declared_in: ClassName,
    pub position: FileLocation,
    pub php_return_type: Option<UnionType>,
    pub comment_return_type: Option<UnionType>,
    pub inferred_return_type: Option<UnionType>,
    pub arguments: Vec<FunctionArgumentData>,
    pub variadic: bool,
    pub modifier: ClassModifier,
    pub is_static: bool,
    pub visibility: ClassMemberVisibility,
}

impl MethodData {
    pub fn new(position: FileLocation, class_name: ClassName) -> Self {
        Self::new_with_name(position, class_name, Name::new())
    }

    pub fn new_with_name(position: FileLocation, class_name: ClassName, name: Name) -> MethodData {
        Self {
            name,
            description: "".into(),
            declared_in: class_name,
            position,
            php_return_type: None,
            comment_return_type: None,
            inferred_return_type: None,
            arguments: vec![],
            variadic: false,
            is_static: false,
            modifier: ClassModifier::None,
            visibility: ClassMemberVisibility::Public,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PropertyData {
    pub name: Name,
    pub position: FileLocation,
    pub modifier: ClassModifier,
    pub visibility: ClassMemberVisibility,
    pub is_static: bool,
    pub default_value: Option<PHPValue>,
    pub declared_type: Option<UnionType>,
    pub comment_type: Option<UnionType>,
    pub constructor_type: Option<UnionType>,
    pub constructor_value: Option<PHPValue>,
    pub read_from: usize,
    pub written_to: usize,
    pub written_data: Vec<(UnionType, Option<PHPValue>)>,
    // void
}

impl PropertyData {
    pub fn new(position: FileLocation, name: Name) -> Self {
        Self {
            name,
            position,
            modifier: ClassModifier::None,
            visibility: ClassMemberVisibility::Public,
            is_static: false,
            default_value: None,
            declared_type: None,
            comment_type: None,
            constructor_type: None,
            constructor_value: None,
            read_from: 0,
            written_to: 0,
            written_data: vec![],
        }
    }

    // void
}
