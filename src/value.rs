use std::{
    cmp::Ordering,
    collections::HashSet,
    convert::TryInto,
    ffi::{OsStr, OsString},
    os::unix::prelude::OsStrExt,
};

use crate::{
    symbols::FullyQualifiedName,
    types::union::{DiscreteType, UnionType},
};

use std::hash::Hash;

#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct ObjectInstance {
    pub fq_name: FullyQualifiedName,
    pub constructor_args: Option<Vec<Option<PHPValue>>>,
    pub generic_concretes: Option<Vec<UnionType>>,
}

impl ObjectInstance {
    pub fn new(
        fq_name: FullyQualifiedName,
        constructor_args: Option<Vec<Option<PHPValue>>>,
    ) -> Self {
        Self {
            fq_name,
            constructor_args,
            generic_concretes: None,
        }
    }

    pub fn new_with_generics(
        fq_name: FullyQualifiedName,
        _generic_concretes: Vec<UnionType>,
        constructor_args: Option<Vec<Option<PHPValue>>>,
    ) -> Self {
        Self {
            fq_name,
            constructor_args,
            generic_concretes: None,
        }
    }

    // An object instance can't have unknown type, so it's not an option
    pub fn get_utype(&self) -> UnionType {
        self.fq_name.get_utype()
    }
}

/// We separate float into a separate type to handle eq and ord
/// more easily in a separate way
#[derive(Clone, Debug, PartialOrd)]
pub enum PHPFloat {
    Real(f64),
    NaN,
    Infinite,
}

impl PHPFloat {
    pub fn new(fval: f64) -> Self {
        if fval.is_infinite() {
            Self::Infinite
        } else if fval.is_nan() {
            return Self::NaN;
        } else {
            return Self::Real(fval);
        }
    }

    fn as_f64(&self) -> f64 {
        match self {
            PHPFloat::Real(f) => *f,
            PHPFloat::NaN => f64::NAN,
            PHPFloat::Infinite => f64::INFINITY,
        }
    }
}

impl Hash for PHPFloat {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl PartialEq for PHPFloat {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Real(l0), Self::Real(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

// FIXME verify that this holds
impl Eq for PHPFloat {}

impl Ord for PHPFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(ord) = self.partial_cmp(other) {
            ord
        } else {
            todo!();
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq, Hash)]
pub enum PHPArray {
    Empty,
    Vector(Vec<PHPValue>),
    HashMap(Vec<(PHPValue, PHPValue)>),
}

#[derive(Clone, Debug, Eq, Ord, PartialOrd, PartialEq, Hash)]
pub enum PHPValue {
    NULL,
    Boolean(bool),
    Int(i64),
    Float(PHPFloat),
    String(OsString),
    Array(PHPArray),
    // .0 = Fully qualified class name, .1 = Constructor arg-vector
    ObjectInstance(ObjectInstance),
}

impl PHPValue {
    pub fn get_utype(&self) -> Option<crate::types::union::UnionType> {
        Some(
            match self {
                PHPValue::NULL => DiscreteType::NULL,
                PHPValue::Boolean(_) => DiscreteType::Bool,
                PHPValue::Int(_) => DiscreteType::Int,
                PHPValue::Float(_) => DiscreteType::Float,
                PHPValue::String(_) => DiscreteType::String,
                PHPValue::Array(_) => DiscreteType::Array,
                PHPValue::ObjectInstance(i) => return Some(i.get_utype()),
            }
            .into(),
        )
    }

    pub fn as_php_string(&self) -> Option<PHPValue> {
        Some(
            match self {
                PHPValue::NULL => PHPValue::String(OsString::new()),
                PHPValue::Boolean(b) => PHPValue::String(if *b {
                    OsStr::from_bytes(b"1").to_os_string()
                } else {
                    OsString::new()
                }),
                PHPValue::Int(i) => {
                    let x = format!("{}", i);

                    PHPValue::String(OsStr::from_bytes(x.as_bytes()).to_os_string())
                }
                PHPValue::Float(_) => return crate::missing_none!("Cast float to string?"),
                PHPValue::String(s) => PHPValue::String(s.clone()),
                PHPValue::Array(_) => PHPValue::String(OsStr::from_bytes(b"Array").to_os_string()),
                PHPValue::ObjectInstance(_) => return crate::missing_none!("ObjectInstance?"),
            }
            .into(),
        )
    }

    pub fn as_os_string(&self) -> Option<OsString> {
        if let Some(Self::String(s)) = self.as_php_string() {
            Some(s)
        } else {
            None
        }
    }

    /// Attempt to coaelesce the value into a PHP int.
    /// A None only indicates that we failed to safely guestimate a thrustworhty value
    pub fn as_php_int(&self) -> Option<PHPValue> {
        Some(match self {
            PHPValue::NULL => PHPValue::Int(0),
            PHPValue::Boolean(b) => PHPValue::Int(if *b { 1 } else { 0 }),
            PHPValue::Int(i) => PHPValue::Int(*i),
            PHPValue::Float(PHPFloat::Real(f)) => {
                const INTEGRAL_LIMIT: f64 = 9007199254740992.0;
                let f = if *f >= 0.0 { f.floor() } else { f.ceil() };
                // These out of bounds-cases must emit in round two
                let i_val = if f.is_nan() {
                    // Sigh
                    // PHP Safely cast this to 0 for some maniac reason
                    return None;
                } else if f < -INTEGRAL_LIMIT {
                    return None;
                } else if f > INTEGRAL_LIMIT {
                    return None;
                } else {
                    f as i64
                };

                PHPValue::Int(i_val)
            }
            PHPValue::Float(PHPFloat::NaN) => {
                return crate::missing_none!("Float(NaN).as_php_int()")
            }
            PHPValue::Float(PHPFloat::Infinite) => {
                return crate::missing_none!("Float(Infinite).as_php_int()")
            }
            PHPValue::String(_) => return crate::missing_none!("String.as_php_int()"),
            PHPValue::Array(_) => return self.as_php_bool().and_then(|x| x.as_php_int()),
            PHPValue::ObjectInstance(_) => {
                return crate::missing_none!("ObjectInstance.as_php_int()")
            }
        })
    }

    /// Attempt to coaelesce the value into a PHP boolean.
    /// A None only indicates that we failed to safely guestimate a thrustworhty value
    ///
    /// Done according to https://www.php.net/manual/en/language.types.boolean.php
    ///
    pub fn as_php_bool(&self) -> Option<PHPValue> {
        Some(match self {
            PHPValue::NULL => PHPValue::Boolean(false),
            PHPValue::Boolean(b) => PHPValue::Boolean(*b),
            PHPValue::Int(i) => PHPValue::Boolean(*i != 0),
            PHPValue::Float(PHPFloat::Real(f)) => PHPValue::Boolean(if *f == 0.0 {
                false
            } else { *f != -0.0 }),
            PHPValue::Float(_) => return crate::missing_none!("Non real float .as_php_bool()"),
            PHPValue::String(s) => {
                if s.len() == 0 {
                    PHPValue::Boolean(false)
                } else if s.eq("0") {
                    PHPValue::Boolean(false)
                } else {
                    PHPValue::Boolean(true)
                }
            }
            PHPValue::Array(v) => PHPValue::Boolean(v.len() > 0),
            PHPValue::ObjectInstance(_) => PHPValue::Boolean(true),
        })
    }

    // Returns a PHPValue, restricted to the enums Int and Float
    pub fn as_php_num(&self) -> Option<Self> {
        Some(match self {
            PHPValue::NULL => PHPValue::Int(0),
            PHPValue::Boolean(b) => PHPValue::Int(if *b { 1 } else { 0 }),
            PHPValue::Int(i) => PHPValue::Int(*i),
            PHPValue::Float(f) => PHPValue::Float(f.clone()),
            PHPValue::String(_) => return crate::missing_none!(),
            PHPValue::Array(_) => return crate::missing_none!(),
            PHPValue::ObjectInstance(_) => return crate::missing_none!(),
        })
    }

    pub fn as_php_float(&self) -> Option<Self> {
        Some(match self {
            PHPValue::NULL => PHPValue::Float(PHPFloat::new(0.0)),
            PHPValue::Boolean(b) => PHPValue::Float(PHPFloat::new(if *b { 1.0 } else { 0.0 })),
            PHPValue::Int(i) => {
                let int32: Result<i32, _> = (*i).try_into();
                let ival = match int32 {
                    Ok(i) => i,
                    Err(_) => return crate::missing_none!("i64 to f64 conversion is inadequate"),
                };
                let b_as_f: f64 = ival.into();
                PHPValue::Float(PHPFloat::new(b_as_f))
            }
            PHPValue::Float(f) => PHPValue::Float(f.clone()),
            PHPValue::String(_) => {
                return crate::missing_none!("string to f64 conversion is missing")
            }
            PHPValue::Array(_) => return self.as_php_bool().and_then(|x| x.as_php_float()),
            PHPValue::ObjectInstance(_) => {
                return crate::missing_none!("ObjectInstance to f64 conversion is inadequate")
            }
        })
    }

    pub fn as_i64(&self) -> Option<i64> {
        if let Some(PHPValue::Int(i)) = self.as_php_int() {
            Some(i)
        } else {
            None
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        if let Some(PHPValue::Float(PHPFloat::Real(f))) = self.as_php_float() {
            Some(f)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Some(PHPValue::Boolean(b)) = self.as_php_bool() {
            Some(b)
        } else {
            None
        }
    }

    pub fn identical_to(&self, rval: &PHPValue) -> Option<bool> {
        match (self, &rval) {
            (Self::NULL, Self::NULL) => Some(true),
            (Self::Int(a), Self::Int(b)) => Some(a == b),
            (Self::Float(a), Self::Float(b)) => Some(a == b),
            (Self::Boolean(a), Self::Boolean(b)) => Some(a == b),
            (Self::String(a), Self::String(b)) => Some(a == b),

            (PHPValue::Array(_), PHPValue::Array(_)) => crate::missing_none!("array === array"),

            (PHPValue::ObjectInstance(_), PHPValue::ObjectInstance(_)) => {
                crate::missing_none!("annet === annet")
            }

            _ => Some(false),
        }
    }

    pub fn equal_to(&self, rval: &PHPValue) -> Option<bool> {
        if let Some(eq) = self.identical_to(rval) {
            return Some(eq);
        }

        crate::missing_none!("[Two PHPValue of different kind].equal_to(..)")
    }

    pub fn common_value<'a, T>(values: T) -> Option<PHPValue>
    where
        T: IntoIterator<Item = &'a PHPValue>,
    {
        let mut first = None;
        for v in values {
            if first.is_none() {
                first = Some(v);
                continue;
            }

            if !first?.identical_to(v)? {
                return None;
            }
        }
        first.cloned()
    }

    pub(crate) fn as_php_array_key(&self) -> Option<PHPValue> {
        match self {
            PHPValue::NULL => Some(PHPValue::String("".into())),
            PHPValue::Boolean(_) => self.as_php_int(),
            PHPValue::Int(_) => Some(self.clone()),
            PHPValue::Float(_) => self.as_php_int(),
            PHPValue::String(_) => Some(self.clone()),
            PHPValue::Array(_) => None,
            PHPValue::ObjectInstance(_) => None,
        }
    }

    pub fn as_raw_php(&self) -> OsString {
        match self {
            PHPValue::NULL => OsStr::from_bytes(b"NULL").into(),
            PHPValue::Boolean(b) => OsStr::from_bytes(if *b { b"true" } else { b"false" }).into(),
            PHPValue::Int(i) => format!("{}", i).into(),
            PHPValue::Float(f) => format!("{}", (*f).as_f64()).into(),
            PHPValue::String(s) => format!(r#""{:?}""#, s).into(),
            PHPValue::Array(_) => todo!(),
            PHPValue::ObjectInstance(_) => todo!(),
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            PHPValue::NULL => true,
            _ => false,
        }
    }

    pub(crate) fn unique(values: &Vec<PHPValue>) -> Vec<PHPValue> {
        let map: HashSet<_> = values.iter().cloned().collect();
        map.iter().cloned().collect()
    }

    pub(crate) fn single_unique(values: &Vec<PHPValue>) -> Option<PHPValue> {
        let unique_values = Self::unique(values);
        if unique_values.len() != 1 {
            return None;
        }
        unique_values.first().cloned()
    }
}

impl PHPArray {
    pub fn len(&self) -> usize {
        match self {
            PHPArray::Empty => 0,
            PHPArray::Vector(v) => v.len(),
            PHPArray::HashMap(h) => h.len(),
        }
    }

    pub(crate) fn get_value_by_key(&self, key_val: PHPValue) -> Option<PHPValue> {
        let array_key = key_val.as_php_array_key()?;
        match self {
            PHPArray::Empty => None,
            PHPArray::Vector(v) => {
                let idx = array_key.as_i64()?;

                let uidx_res: Result<usize, _> = idx.try_into();
                if let Ok(idx) = uidx_res {
                    if idx > v.len() {
                        // FIXME: emit because we're out of range
                        None
                    } else {
                        v.get(idx).cloned()
                    }
                } else {
                    // FIXME: Emit error beacuse this will fail
                    None
                }
            }
            PHPArray::HashMap(arr) => {
                for (key, value) in arr {
                    if key.equal_to(&array_key).unwrap_or(false) {
                        return Some(value.clone());
                    }
                }
                None
            }
        }
    }

    pub(crate) fn get_type_by_key(&self, php_idx: PHPValue) -> Option<UnionType> {
        let v = self.get_value_by_key(php_idx)?;
        v.get_utype()
    }
}
