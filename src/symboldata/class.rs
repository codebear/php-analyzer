use tree_sitter::Range;

use crate::{
    analysis::state::AnalysisState,
    phpdoc::types::{PHPDocComment, PHPDocEntry},
    symbols::{FullyQualifiedName, Name},
    types::union::{DiscreteType, UnionType},
    value::PHPValue,
};

use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    ffi::OsString,
    fmt::Display,
    sync::{Arc, RwLock},
};

use super::{FileLocation, SymbolData};
use crate::types::union::from_vec_parsed_type;

type MethodName = Name;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialOrd)]
pub struct ClassName {
    pub name: Name,
    pub fq_name: FullyQualifiedName,
}

impl PartialEq for ClassName {
    fn eq(&self, other: &Self) -> bool {
        self.fq_name == other.fq_name
    }
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

    pub fn to_os_string(&self) -> OsString {
        self.name.to_os_string()
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

impl Display for ClassName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fq_name)
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

    pub fn get_class_name(&self) -> ClassName {
        self.get_fq_name().into()
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

    pub fn get_own_methods(&self, symbol_data: Arc<SymbolData>) -> Vec<MethodData> {
        match self {
            ClassType::None => vec![],
            ClassType::Class(c) => c.get_own_methods(symbol_data),
            ClassType::Interface(i) => i.get_own_methods(symbol_data),
            ClassType::Trait(t) => t.get_own_methods(symbol_data),
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

    pub fn get_property(
        &self,
        property_name: &Name,
        state: &AnalysisState,
    ) -> Option<PropertyData> {
        match self {
            ClassType::None => panic!(),
            ClassType::Class(c) => c.get_property(property_name, state),
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

    /// check if the type implements a specific interface
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

    /// check if the type is an instancoef a specific class, interface or trait
    pub fn instanceof(&self, tname: &ClassName, symbol_data: Arc<SymbolData>) -> bool {
        match self {
            ClassType::None => false,
            ClassType::Class(c) => c.instanceof(tname, symbol_data),
            ClassType::Interface(i) => i.instanceof(tname, symbol_data),
            ClassType::Trait(_) => false,
        }
    }

    pub fn get_generic_templates(&self) -> Option<Vec<Name>> {
        match self {
            ClassType::None => None,
            ClassType::Class(cdata) => cdata.generic_templates.clone(),
            ClassType::Interface(idata) => idata.generic_templates.clone(),
            ClassType::Trait(tdata) => tdata.generic_templates.clone(),
        }
    }

    pub fn get_constructor(&self, symbol_data: Arc<SymbolData>) -> Option<MethodData> {
        let name: Name = "__construct".into();
        self.get_method(&name, symbol_data)
    }

    pub(crate) fn set_generic_concretes(&mut self, noe: BTreeMap<Name, UnionType>) {
        match self {
            ClassType::None => (),
            ClassType::Class(c) => c.generic_concretes = Some(noe),
            ClassType::Interface(_) => todo!(),
            ClassType::Trait(_) => todo!(),
        }
    }

    pub fn get_constant_value(
        &self,
        symbol_data: &Arc<SymbolData>,
        constant_name: &Name,
    ) -> Option<PHPValue> {
        match self {
            ClassType::None => None,
            ClassType::Class(cdata) => cdata.get_constant_value(symbol_data, constant_name),
            ClassType::Interface(idata) => idata.get_constant_value(symbol_data, constant_name),
            ClassType::Trait(_) => None,
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
pub struct TraitImport {
    pub trait_name: ClassName,
}

#[derive(Clone, Debug)]
pub struct ClassData {
    pub class_name: ClassName,
    pub position: FileLocation,
    pub base_class_name: Option<ClassName>,
    pub phpdoc_base_class_name: Option<DiscreteType>,
    pub interfaces: Vec<ClassName>,
    pub phpdoc_interfaces: Vec<DiscreteType>,
    pub modifier: ClassModifier,
    pub constants: HashMap<Name, Option<PHPValue>>,
    pub methods: HashMap<Name, Arc<RwLock<MethodData>>>,
    pub properties: HashMap<Name, Arc<RwLock<PropertyData>>>,
    pub is_native: bool,
    pub traits: Vec<TraitImport>,
    pub phpdoc: Option<PHPDocComment>,
    pub deprecated: Option<OsString>,
    pub generic_templates: Option<Vec<Name>>,
    pub generic_concretes: Option<BTreeMap<Name, UnionType>>,
}

impl ClassData {
    pub fn new(position: FileLocation, class_name: ClassName) -> Self {
        Self {
            class_name,
            position,
            base_class_name: None,
            phpdoc_base_class_name: None,
            interfaces: vec![],
            phpdoc_interfaces: vec![],
            modifier: ClassModifier::None,
            constants: HashMap::new(),
            methods: HashMap::new(),
            properties: HashMap::new(),
            traits: vec![],
            is_native: false,
            phpdoc: None,
            deprecated: None,
            generic_templates: None,
            generic_concretes: None,
        }
    }

    pub fn get_own_method(&self, method_name: &Name) -> Option<Arc<RwLock<MethodData>>> {
        self.methods.get(&method_name.to_ascii_lowercase()).cloned()
    }

    pub fn get_base_class_data(
        &self,
        symbol_data: &Arc<SymbolData>,
    ) -> Option<Arc<RwLock<ClassType>>> {
        let base = self.base_class_name.as_ref()?;
        symbol_data.get_class(base)
    }

    pub fn get_method(
        &self,
        method_name: &Name,
        symbol_data: Arc<SymbolData>,
    ) -> Option<MethodData> {
        if let Some(m) = self.methods.get(&method_name.to_ascii_lowercase()) {
            let mut mdata = m.read().unwrap().clone();
            mdata.generic_concretes = self.generic_concretes.clone();
            return Some(mdata);
        }

        if let Some(cdata_handle) = self.get_base_class_data(&symbol_data) {
            let cdata = cdata_handle.read().unwrap();
            if let Some(m) = (*cdata).get_method(method_name, symbol_data) {
                return Some(m);
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

    fn instanceof(&self, tname: &ClassName, symbol_data: Arc<SymbolData>) -> bool {
        if self.class_name == *tname {
            return true;
        }
        for iface in &self.interfaces {
            if let Some(iface_data) = &symbol_data.get_interface(iface) {
                if iface_data.instanceof(tname, symbol_data.clone()) {
                    return true;
                }
            }
        }
        if let Some(base) = &self.base_class_name {
            if let Some(cdata_handle) = symbol_data.get_class(base) {
                let cdata = cdata_handle.read().unwrap();
                return cdata.instanceof(tname, symbol_data);
            }
        }
        return false;
    }

    fn get_own_methods(&self, _symbol_data: Arc<SymbolData>) -> Vec<MethodData> {
        self.methods
            .iter()
            .map(|x| x.1.read().unwrap().clone())
            .collect()
    }

    fn get_constant_value(
        &self,
        symbol_data: &Arc<SymbolData>,
        constant_name: &Name,
    ) -> Option<PHPValue> {
        if let Some(Some(v)) = self.constants.get(constant_name) {
            return Some(v.clone());
        }

        if let Some(base) = self.get_base_class_data(symbol_data) {
            let unlocked = base.read().ok()?;
            if let Some(x) = unlocked.get_constant_value(symbol_data, constant_name) {
                return Some(x);
            }
        }

        for iface in &self.interfaces {
            if let Some(iface_data) = &symbol_data.get_interface(iface) {
                if let Some(const_val) = iface_data.get_constant_value(symbol_data, constant_name) {
                    return Some(const_val);
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct InterfaceData {
    pub interface_name: ClassName,
    pub position: FileLocation,
    pub base_interface_names: Option<Vec<ClassName>>,
    pub constants: HashMap<Name, Option<PHPValue>>,
    pub methods: HashMap<Name, Arc<RwLock<MethodData>>>,
    pub is_native: bool,
    pub phpdoc: Option<PHPDocComment>,
    pub generic_templates: Option<Vec<Name>>,
}

impl InterfaceData {
    pub fn new(position: FileLocation, interface_name: ClassName) -> Self {
        Self {
            interface_name,
            position,
            base_interface_names: None,
            constants: HashMap::new(),
            methods: HashMap::new(),
            is_native: false,
            phpdoc: None,
            generic_templates: None,
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

    fn get_own_methods(&self, _symbol_data: Arc<SymbolData>) -> Vec<MethodData> {
        self.methods
            .iter()
            .map(|x| x.1.read().unwrap().clone())
            .collect()
    }

    fn instanceof(&self, tname: &ClassName, symbol_data: Arc<SymbolData>) -> bool {
        if self.interface_name == *tname {
            return true;
        }
        let parent_inames = if let Some(i) = &self.base_interface_names {
            i
        } else {
            return false;
        };
        for parent_iname in parent_inames {
            if parent_iname == tname {
                return true;
            }
        }
        for parent_iname in parent_inames {
            if let Some(idata) = symbol_data.get_interface(&parent_iname) {
                if idata.instanceof(tname, symbol_data.clone()) {
                    return true;
                }
            }
        }
        false
    }

    fn get_constant_value(
        &self,
        symbol_data: &SymbolData,
        constant_name: &Name,
    ) -> Option<PHPValue> {
        if let Some(Some(v)) = self.constants.get(constant_name) {
            return Some(v.clone());
        }

        for iface in self.base_interface_names.as_ref()? {
            if let Some(iface_data) = &symbol_data.get_interface(iface) {
                if let Some(const_val) = iface_data.get_constant_value(symbol_data, constant_name) {
                    return Some(const_val);
                }
            }
        }
        None
    }
}

#[derive(Clone, Debug)]
pub struct TraitData {
    pub trait_name: ClassName,
    pub position: FileLocation,
    pub base_name: Option<ClassName>,
    pub methods: HashMap<Name, Arc<RwLock<MethodData>>>,
    pub is_native: bool,
    pub phpdoc: Option<PHPDocComment>,
    pub generic_templates: Option<Vec<Name>>,
}

impl TraitData {
    pub fn new(position: FileLocation, trait_name: ClassName) -> Self {
        Self {
            trait_name,
            position,
            base_name: None,
            methods: HashMap::new(),
            is_native: false,
            phpdoc: None,
            generic_templates: None,
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

    fn get_own_methods(&self, _symbol_data: Arc<SymbolData>) -> Vec<MethodData> {
        self.methods
            .iter()
            .map(|x| x.1.read().unwrap().clone())
            .collect()
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct FunctionArgumentData {
    pub name: Name,
    pub arg_type: Option<UnionType>,
    pub default_value: Option<PHPValue>,
    pub nullable: bool,
    pub optional: bool,
    pub inline_phpdoc_type: Option<(Range, UnionType)>,
    pub phpdoc_entry: Option<PHPDocEntry>,
    pub phpdoc_type: Option<UnionType>,
    pub variadic: bool,
}
impl FunctionArgumentData {
    pub fn get_type(&self, state: &mut AnalysisState) -> Option<UnionType> {
        // FIXME somewhere there needs to be emitted
        // an issue if the comment-type is incompatible with the native type
        if let Some(utype) = &self.phpdoc_type {
            return Some(utype.clone());
        }
        if let Some(PHPDocEntry::Param(_range, param, _name, _desc)) = &self.phpdoc_entry {
            // from_vec_parsed_type
            let utype = from_vec_parsed_type(param.clone(), state, None, None);
            return utype;
        }
        if let Some((_range, utype)) = &self.inline_phpdoc_type {
            return Some(utype.clone());
        }
        self.arg_type.clone()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MethodData {
    pub name: Name,
    pub description: String,
    pub declared_in: ClassName,
    pub position: FileLocation,
    pub return_count: usize,
    pub php_return_type: Option<UnionType>,
    pub comment_return_type: Option<(UnionType, Range)>,
    pub inferred_return_type: Option<UnionType>,
    pub arguments: Vec<FunctionArgumentData>,
    pub variadic: bool,
    pub modifier: ClassModifier,
    pub is_static: bool,
    pub visibility: ClassMemberVisibility,
    pub phpdoc: Option<PHPDocComment>,
    pub generic_templates: Option<Vec<Name>>,
    pub generic_concretes: Option<BTreeMap<Name, UnionType>>,
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
            return_count: 0,
            php_return_type: None,
            comment_return_type: None,
            inferred_return_type: None,
            arguments: vec![],
            variadic: false,
            is_static: false,
            modifier: ClassModifier::None,
            visibility: ClassMemberVisibility::Public,
            phpdoc: None,
            generic_templates: None,
            generic_concretes: None,
        }
    }

    pub(crate) fn get_return_type(&self) -> Option<UnionType> {
        let call_return_type = self
            .comment_return_type
            .as_ref()
            .map(|x| x.0.clone())
            .or(self.php_return_type.clone())
            .or(self.inferred_return_type.clone())?;

        if let Some(concrete) = &self.generic_concretes {
            let mut result = UnionType::new();
            for x in call_return_type.types {
                match x {
                    DiscreteType::Template(name) => {
                        let noe = concrete.get(&name);

                        if let Some(u) = noe {
                            result.merge_into(u.clone());
                        } else {
                            result.push(DiscreteType::Template(name));
                        }
                    }
                    t @ _ => result.push(t),
                }
            }

            Some(result)
        } else {
            Some(call_return_type)
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
    pub readonly: bool,
    pub default_value: Option<PHPValue>,
    pub declared_type: Option<UnionType>,
    pub comment_type: Option<(UnionType, Range)>,
    pub constructor_type: Option<UnionType>,
    pub constructor_value: Option<PHPValue>,
    pub read_from: usize,
    pub written_to: usize,
    pub written_data: Vec<(UnionType, Option<PHPValue>)>,
    pub phpdoc: Option<PHPDocComment>,
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
            readonly: false,
            default_value: None,
            declared_type: None,
            comment_type: None,
            constructor_type: None,
            constructor_value: None,
            read_from: 0,
            written_to: 0,
            written_data: vec![],
            phpdoc: None,
        }
    }

    // void
}
