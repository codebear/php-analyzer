use tree_sitter::Point;

use crate::issue::IssuePosition;
use crate::symbols::FullyQualifiedName;
use crate::symbols::Name;
use crate::symbols::Symbol;
use crate::symbols::SymbolClass;
use crate::types::union::UnionType;
use crate::value::PHPValue;

use self::class::ClassName;
use self::class::ClassType;
use self::class::FunctionArgumentData;
use self::class::PropertyData;
use std::collections::HashMap;

use std::ffi::OsString;
use std::sync::Arc;
use std::sync::RwLock;

use self::class::MethodData;

pub mod class;

#[derive(Debug, Clone)]
pub struct FilePosition {
    pub byte: usize,
    pub line: usize,
    pub column: usize,
}

impl FilePosition {
    pub fn internal() -> Self {
        Self {
            byte: 0,
            line: 0,
            column: 0,
        }
    }
}

impl From<(usize, Point)> for FilePosition {
    fn from((byte, point): (usize, Point)) -> Self {
        Self {
            byte,
            line: point.row,
            column: point.column,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileLocation {
    pub uri: OsString,
    pub start: FilePosition,
    pub end: FilePosition,
}

impl FileLocation {
    pub(crate) fn new(pos: IssuePosition) -> Self {
        Self::from(pos)
    }
    pub fn internal() -> Self {
        Self {
            uri: "*internal*".into(),
            start: FilePosition::internal(),
            end: FilePosition::internal(),
        }
    }
}

impl From<IssuePosition> for FileLocation {
    fn from(pos: IssuePosition) -> Self {
        let uri = pos.uri;
        pos.range;
        let start: FilePosition = (pos.range.start_byte, pos.range.start_point).into();
        let end: FilePosition = (pos.range.end_byte, pos.range.end_point).into();
        Self { uri, start, end }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionData {
    pub name: FullyQualifiedName,
    pub position: FileLocation,
    pub php_return_type: Option<UnionType>,
    pub comment_return_type: Option<UnionType>,
    pub inferred_return_type: Option<UnionType>,
    pub arguments: Vec<FunctionArgumentData>,
    pub variadic: bool,
    pub deterministic: bool,
    pub pure: bool,
    pub return_value: Option<PHPValue>,
    pub overload_map: HashMap<Vec<PHPValue>, Option<UnionType>>,
}

#[derive(Debug)]
pub struct SymbolData {
    pub classes: Arc<RwLock<HashMap<FullyQualifiedName, Arc<RwLock<ClassType>>>>>,
    pub functions: Arc<RwLock<HashMap<FullyQualifiedName, Arc<RwLock<FunctionData>>>>>,
}

impl SymbolData {
    pub fn new() -> Self {
        SymbolData {
            classes: Arc::new(RwLock::new(HashMap::new())),
            // methods: Arc::new(RwLock::new(HashMap::new())),
            functions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_class(&self, name: &ClassName) -> Option<Arc<RwLock<ClassType>>> {
        self.classes
            .read()
            .unwrap()
            .get(&name.get_fq_name().to_ascii_lowercase())
            .cloned()
    }

    pub fn get_or_create_class(&self, name: &ClassName) -> Arc<RwLock<ClassType>> {
        {
            let reader = self.classes.read().unwrap();

            if let Some(x) = reader.get(&name.get_fq_name().to_ascii_lowercase()) {
                return x.clone();
            }
        }
        {
            let mut writer = self.classes.write().unwrap();
            // Noen kan han rukket å endre i mellomtiden
            if let Some(x) = writer.get(&name.get_fq_name().to_ascii_lowercase()) {
                return x.clone();
            }
            let cd = Arc::new(RwLock::new(ClassType::None));
            writer.insert(name.get_fq_name().to_ascii_lowercase().clone(), cd.clone());
            return cd;
        }
    }

    pub fn get_method(&self, class: &ClassName, method: &Name) -> Option<Arc<RwLock<MethodData>>> {
        let handle = self.classes.read().unwrap();
        let cdata = handle.get(&class.fq_name.to_ascii_lowercase())?.clone();

        let reader = cdata.read().unwrap();
        reader.get_own_method(method)
    }

    pub fn get_function(&self, fname: &FullyQualifiedName) -> Option<FunctionData> {
        let handle = self.functions.read().unwrap();
        let fdata_handle = handle.get(fname)?;
        let fdata = fdata_handle.read().unwrap();
        Some(fdata.clone())
    }

    pub fn get_or_create_method(
        &self,
        class: &ClassName,
        method: &Name,
        location: FileLocation,
    ) -> Arc<RwLock<MethodData>> {
        let handle = self.classes.read().unwrap();

        let cdata = handle
            .get(&class.get_fq_name().to_ascii_lowercase())
            .expect(&format!(
                "The class {:?} must exist when attempting to register method {:?}",
                class.fq_name.to_ascii_lowercase(),
                method
            ));

        let mut writable_cdata = cdata.write().unwrap();
        writable_cdata.get_or_create_method(method, location)
    }

    pub fn get_or_create_property(
        &self,
        class: &ClassName,
        property_name: &Name,
        location: FileLocation,
    ) -> Option<Arc<RwLock<PropertyData>>> {
        let handle = self.classes.read().unwrap();

        let cdata = handle
            .get(&class.get_fq_name().to_ascii_lowercase())
            .expect(&format!(
                "The class {:?} must exist when attempting to register method {:?}",
                class.fq_name.to_ascii_lowercase(),
                property_name
            ));

        let mut writable_cdata = cdata.write().unwrap();
        writable_cdata.get_or_create_property(property_name, location)
    }
}

pub trait ArcedSymbolAccess {
    fn get_class_for_symbol(&self, class: &SymbolClass) -> Option<Arc<RwLock<ClassType>>>;
    fn get_pos_for_symbol(&self, symbol: Symbol) -> Option<Vec<FileLocation>>;
}

impl ArcedSymbolAccess for Arc<SymbolData> {
    fn get_class_for_symbol(&self, class: &SymbolClass) -> Option<Arc<RwLock<ClassType>>> {
        let mut fq_name = class.ns.clone();
        fq_name.push(&class.name);

        let name = ClassName::new_with_names(class.name.clone(), fq_name);
        self.get_class(&name)
    }

    fn get_pos_for_symbol(&self, symbol: Symbol) -> Option<Vec<FileLocation>> {
        match symbol {
            Symbol::Class(class_info) => {
                let class_handle = self.get_class_for_symbol(&class_info)?;
                let class = class_handle.read().ok()?;
                match &*class {
                    ClassType::None => None,
                    ClassType::Class(c) => Some(vec![c.position.clone()]),
                    ClassType::Interface(i) => Some(vec![i.position.clone()]),
                    ClassType::Trait(t) => Some(vec![t.position.clone()]),
                }
            }
            Symbol::Method(m) => {
                let class_handle = self.get_class_for_symbol(&m.class)?;
                let class = class_handle.read().ok()?;
                let method_data = class.get_method(&m.name, self.clone())?;
                Some(vec![method_data.position.clone()])
            }
            Symbol::Function(_) => crate::missing_none!(),
            Symbol::Constant(_) => crate::missing_none!(),
            Symbol::ClassConstant(_) => crate::missing_none!(),
            Symbol::ClassProperty(_) => crate::missing_none!(),
            Symbol::None => crate::missing_none!(),
            Symbol::Native(_) => crate::missing_none!(),
        }
    }
}
