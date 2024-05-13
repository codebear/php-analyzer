use std::{
    ffi::OsString,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};

//use tree_sitter::Range;
use crate::parser::Range;

use crate::{
    symbols::{FullyQualifiedName, Name},
    types::union::UnionType,
};

#[derive(Clone, Debug)]

pub enum Severity {
    Hint,
    Error,
    Warning,
    Information,
}

#[derive(Clone, Debug)]

pub struct IssuePosition {
    pub uri: OsString,
    pub range: Range,
}

impl IssuePosition {
    pub fn new(fname: &Option<PathBuf>, range: Range) -> Self {
        Self {
            range,
            uri: fname
                .as_ref()
                .map(|x| x.as_os_str().to_os_string())
                .unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Issue {
    UnusedVariable(IssuePosition, Name),
    UnusedArgument(IssuePosition, Name),
    UnknownVariable(IssuePosition, Name),
    UnknownFunction(IssuePosition, FullyQualifiedName),
    UnknownClass(IssuePosition, FullyQualifiedName),
    UnknownType(IssuePosition, OsString),

    /// .0 position
    /// .1 fq_class_name
    /// .2 property name
    UnknownProperty(IssuePosition, FullyQualifiedName, Name),

    DuplicateClass(IssuePosition, FullyQualifiedName),
    DuplicateSymbol(IssuePosition, Name),
    // We're not able to verify that variable is callable
    NotAVerifiedCallableVariable(IssuePosition, Name),
    // The variable is not callable
    NotACallableVariable(IssuePosition, Name),
    DecrementIsIllegalOnType(IssuePosition, UnionType),
    IncrementIsIllegalOnType(IssuePosition, UnionType),

    UnknownConstant(IssuePosition, FullyQualifiedName),
    UnreachableCode(IssuePosition),

    EmptyTemplate(IssuePosition, Name),

    /// We're unable to determine the type of the target of the method call
    /// *  .0 position
    /// *  .1 type if available
    /// *  .2 method_name if avaiable
    MethodCallOnUnknownType(IssuePosition, Option<FullyQualifiedName>, Option<Name>),

    /// The target of the method call is nullable
    /// *  .0 position
    /// *  .1 method_name if avaiable
    MethodCallOnNullableType(IssuePosition, Option<Name>),

    /// We're unable to determine the type of the target of the property access
    /// *  .0 position
    /// *  .1 property_name if avaiable
    PropertyAccessOnUnknownType(IssuePosition, Option<Name>),

    /// You can't access properties on something only known as an interface
    /// *  .0 position
    /// *  .1 interface_name
    /// *  .2 property_name if avaiable
    PropertyAccessOnInterfaceType(IssuePosition, FullyQualifiedName, Option<Name>),

    /// We're unable to determine the name of the property of the property access
    /// *  .0 position
    /// *  .1 fq_class_name if avaiable
    IndeterminablePropertyName(IssuePosition, Option<FullyQualifiedName>),
    /// *  .0 position
    /// *  .1 fq_class_name
    /// *  .2 method_name
    UnknownMethod(IssuePosition, FullyQualifiedName, Name),
    TraversalOfUnknownType(IssuePosition),

    // Constant declaration in a conditional branch
    ConditionalConstantDeclaration(IssuePosition),

    /// *  .0 position
    /// *  .1 function or method-name
    /// *  .2 expected argcount
    /// *  .3 provided argcount
    WrongNumberOfArguments(IssuePosition, Name, usize, usize),

    /// The function name is cased differently than the declaration
    /// *  .0 position
    /// *  .1 expected fq_name
    /// *  .2 provided fq_name
    WrongFunctionNameCasing(IssuePosition, FullyQualifiedName, FullyQualifiedName),

    DuplicateConstant(IssuePosition, FullyQualifiedName),
    DuplicateFunction(IssuePosition, FullyQualifiedName),

    /// *  .0 position
    /// *  .1 expected class
    /// *  .2 provided const name
    UnknownClassConstant(IssuePosition, FullyQualifiedName, Name),

    /// *  .0 position
    /// *  .1 expected class
    /// *  .2 provided const name
    DuplicateClassConstant(IssuePosition, FullyQualifiedName, Name),

    /// Duplicates of other declarations, used i.e. on PHPDoc-entries
    DuplicateDeclaration(IssuePosition, OsString),

    DuplicateTemplate(IssuePosition, Name),

    UnknownIndexType(IssuePosition),

    WrongClassNameCasing(IssuePosition, Name, FullyQualifiedName),

    /// The analyzer arrived at a parse-state it considers impossible
    ParseAnomaly(IssuePosition, OsString),
    VariableNotInitializedInAllBranhces(IssuePosition, Name),

    PHPDocParseError(IssuePosition),

    /// Parse error while parsing a type in a phpdoc-comment
    PHPDocTypeError(IssuePosition, String),

    MisplacedPHPDocEntry(IssuePosition, OsString),
    InvalidPHPDocEntry(IssuePosition, OsString),
    // PHPDocEntry which is not needed
    RedundantPHPDocEntry(IssuePosition, OsString),

    UnknownPHPDocEntry(IssuePosition, OsString),
}

impl Issue {
    pub fn severity(&self) -> Severity {
        match self {
            // Warnings
            Self::UnusedVariable(_, _) => Severity::Warning,
            Self::UnusedArgument(_, _) => Severity::Hint,
            Self::VariableNotInitializedInAllBranhces(_, _) => Severity::Warning,
            Self::NotAVerifiedCallableVariable(_, _) => Severity::Warning,

            // Hints
            Self::ConditionalConstantDeclaration(_) => Severity::Hint,

            // Remaining we classify as errors
            _ => Severity::Error,
        }
    }

    pub fn issue_file(&self) -> OsString {
        let pos = self.issue_pos();
        pos.uri
    }

    pub fn issue_pos(&self) -> IssuePosition {
        match self {
            Self::UnusedVariable(pos, _)
            | Self::UnusedArgument(pos, _)
            | Self::UnknownVariable(pos, _)
            | Self::UnknownFunction(pos, _)
            | Self::UnknownClass(pos, _)
            | Self::UnknownType(pos, _)
            | Self::UnknownProperty(pos, _, _)
            | Self::DuplicateClass(pos, _)
            | Self::DuplicateSymbol(pos, _)
            | Self::NotAVerifiedCallableVariable(pos, _)
            | Self::NotACallableVariable(pos, _)
            | Self::DecrementIsIllegalOnType(pos, _)
            | Self::IncrementIsIllegalOnType(pos, _)
            | Self::UnknownConstant(pos, _)
            | Self::UnreachableCode(pos)
            | Self::UnknownMethod(pos, _, _)
            | Self::MethodCallOnUnknownType(pos, _, _)
            | Self::MethodCallOnNullableType(pos, _)
            | Self::TraversalOfUnknownType(pos)
            | Self::ConditionalConstantDeclaration(pos)
            | Self::WrongNumberOfArguments(pos, _, _, _)
            | Self::DuplicateConstant(pos, _)
            | Self::DuplicateFunction(pos, _)
            | Self::UnknownClassConstant(pos, _, _)
            | Self::DuplicateClassConstant(pos, _, _)
            | Self::DuplicateDeclaration(pos, _)
            | Self::DuplicateTemplate(pos, _)
            | Self::UnknownIndexType(pos)
            | Self::ParseAnomaly(pos, _)
            | Self::WrongFunctionNameCasing(pos, _, _)
            | Self::PropertyAccessOnUnknownType(pos, _)
            | Self::PropertyAccessOnInterfaceType(pos, _, _)
            | Self::IndeterminablePropertyName(pos, _)
            | Self::VariableNotInitializedInAllBranhces(pos, _)
            | Self::WrongClassNameCasing(pos, _, _)
            | Self::PHPDocParseError(pos)
            | Self::PHPDocTypeError(pos, _)
            | Self::MisplacedPHPDocEntry(pos, _)
            | Self::InvalidPHPDocEntry(pos, _)
            | Self::RedundantPHPDocEntry(pos, _)
            | Self::UnknownPHPDocEntry(pos, _)
            | Self::EmptyTemplate(pos, _) => pos.clone(),
        }
    }

    pub fn range(&self) -> Range {
        self.issue_pos().range
    }

    pub fn filename(&self) -> Option<OsString> {
        Some(self.issue_pos().uri)
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Self::UnusedVariable(_, _) => "UnusedVariable",
            Self::UnusedArgument(_, _) => "UnusedArgument",
            Self::UnknownVariable(_, _) => "UnknownVariable",
            Self::UnknownFunction(_, _) => "UnknownFunction",
            Self::UnknownClass(_, _) => "UnknownClass",
            Self::UnknownType(_, _) => "UnknownType",
            Self::UnknownProperty(_, _, _) => "UnknownProperty",
            Self::DuplicateClass(_, _) => "DuplicateClass",
            Self::DuplicateSymbol(_, _) => "DuplicateSymbol",
            Self::DuplicateTemplate(_, _) => "DuplicateTemplate",
            Self::NotAVerifiedCallableVariable(_, _) => "NotAVerifiedCallableVariable",
            Self::NotACallableVariable(_, _) => "NotACallableVariable",
            Self::DecrementIsIllegalOnType(_, _) => "DecrementIsIllegalOnType",
            Self::IncrementIsIllegalOnType(_, _) => "IncrementIsIllegalOnType",
            Self::UnknownConstant(_, _) => "UnknownConstant",
            Self::UnreachableCode(_) => "UnreachableCode",
            Self::MethodCallOnUnknownType(_, _, _) => "MethodCallOnUnknownType",
            Self::MethodCallOnNullableType(_, _) => "MethodCallOnNullableType",
            Self::PropertyAccessOnUnknownType(_, _) => "PropertyAccessOnUnknownType",
            Self::PropertyAccessOnInterfaceType(_, _, _) => "PropertyAccessOnInterfaceType",
            Self::IndeterminablePropertyName(_, _) => "IndeterminablePropertyName",
            Self::UnknownMethod(_, _, _) => "UnknownMethod",
            Self::TraversalOfUnknownType(_) => "TraversalOfUnknownType",
            Self::ConditionalConstantDeclaration(_) => "ConditionalConstantDeclaration",
            Self::WrongNumberOfArguments(_, _, _, _) => "WrongNumberOfArguments",
            Self::WrongFunctionNameCasing(_, _, _) => "WrongFunctionNameCasing",
            Self::DuplicateConstant(_, _) => "DuplicateConstant",
            Self::DuplicateFunction(_, _) => "DuplicateFunction",
            Self::UnknownClassConstant(_, _, _) => "UnknownClassConstant",
            Self::DuplicateClassConstant(_, _, _) => "DuplicateClassConstant",
            Self::DuplicateDeclaration(_, _) => "DuplicateDeclaration",
            Self::UnknownIndexType(_) => "UnknownIndexType",
            Self::ParseAnomaly(_, _) => "ParseAnomaly",
            Self::VariableNotInitializedInAllBranhces(_, _) => {
                "VariableNotInitializedInAllBranhces"
            }
            Self::WrongClassNameCasing(_, _, _) => "WrongCasingOfSymbolName",
            Self::PHPDocParseError(_) => "PHPDocParseError",
            Self::PHPDocTypeError(_, _) => "PHPDocTypeError",
            Self::MisplacedPHPDocEntry(_, _) => "MisplacedPHPDocEntry",
            Self::InvalidPHPDocEntry(_, _) => "InvalidPHPDocEntry",
            Self::RedundantPHPDocEntry(_, _) => "RedundantPHPDocEntry",
            Self::UnknownPHPDocEntry(_, _) => "UnknownPHPDocEntry",
            Self::EmptyTemplate(_, _) => "EmptyTemplate",
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Self::UnusedVariable(_, vn) => format!("Unused variable ${}", vn),
            Self::UnusedArgument(_, vn) => format!("Unused argument ${}", vn),
            Self::UnknownVariable(_, vn) => format!("Unknown variable ${}", vn),
            Self::UnknownFunction(_, fun) => format!("Unknown function {}", fun),
            Self::UnknownClass(_, c) => format!("Unknown class {}", c),
            Self::UnknownType(_, c) => format!("Unknown type {:?}", c),
            Self::UnknownProperty(_, c, p) => format!("Unknown property {} in {}", p, c),
            Self::DuplicateClass(_, c) => format!("Duplicate class {}", c),
            Self::DuplicateSymbol(_, s) => format!("Duplicate symbol {}", s),
            Self::DuplicateTemplate(_pos, t) => format!("Duplicate template {}", t),

            Self::NotAVerifiedCallableVariable(_, vn) => {
                format!("Could not verify that variable ${} is callable", vn)
            }
            Self::NotACallableVariable(_, vn) => format!("Variable ${} is not callable", vn),
            Self::DecrementIsIllegalOnType(_, n) => format!("<expr>-- is illegal on {}", n),
            Self::IncrementIsIllegalOnType(_, n) => format!("<expr>++ is illegal on {}", n),
            Self::UnknownConstant(_, c) => format!("Unknown constant {}", c),
            Self::UnreachableCode(_) => "Unreachable code".to_string(),
            Self::MethodCallOnUnknownType(_, cname, mname) => {
                let mname = mname.clone().unwrap_or_else(Name::new);
                let cname = cname.clone().unwrap_or_else(FullyQualifiedName::new);
                format!(
                    "Method call {} on a target with unidentifiyable type {}",
                    mname, cname
                )
            }
            Self::MethodCallOnNullableType(_, mname) => {
                format!("Method call {:?} on a target which can be null", mname)
            }
            Self::UnknownMethod(_, c, m) => format!("Unknown method {} on {}", m, c),
            Self::TraversalOfUnknownType(_) => "Traversal of unknown type".to_string(),
            Self::ConditionalConstantDeclaration(_) => {
                "Conditional declaration of constant is not recommended".to_string()
            }
            Self::WrongNumberOfArguments(_, fname, expected_argcount, got_argcount) => format!(
                "Wrong number of arguments to {}, got {}, expected {}",
                fname, expected_argcount, got_argcount
            ),
            Self::DuplicateConstant(_, c) => format!("Duplicate constant {}", c),
            Self::DuplicateFunction(_, f) => format!("Duplicate function {}", f),
            Self::UnknownClassConstant(_, class, cons) => {
                format!("Unknown class constant {}::{}", class, cons)
            }

            Self::DuplicateClassConstant(_, class, cons) => {
                format!("Duplicate class constant {}::{}", class, cons)
            }
            Self::DuplicateDeclaration(_, desc) => {
                format!("Duplicate declaration: {}", desc.to_string_lossy())
            }
            Self::UnknownIndexType(_) => "Unknown index type".to_string(),
            Self::ParseAnomaly(_, pa) => format!("Arrived at an unexpected parse state: {:?}", pa),
            Self::VariableNotInitializedInAllBranhces(_, vname) => {
                format!("Variable ${} is not initialized in all branches", vname)
            }

            Self::WrongFunctionNameCasing(_, expected, provided) => format!(
                "Function name is cased differently [{}] than in the declaration [{}]",
                provided, expected
            ),
            Self::WrongClassNameCasing(_, provided, expected) => format!(
                "Class name is cased differently [{}] than in the declaration [{}]",
                provided, expected
            ),
            Self::PropertyAccessOnUnknownType(_, property_name) => {
                format!("Property {:?} accessed on unknown type", property_name)
            }
            Self::PropertyAccessOnInterfaceType(_, interface_name, property_name) => {
                format!(
                    "Not possible to access property {:?} on interface {}.",
                    property_name, interface_name
                )
            }
            Self::IndeterminablePropertyName(_, cname) => format!(
                "Unable to determine the name of the property, accessed on {:?}",
                cname
            ),
            Self::PHPDocParseError(_) => "Unable to parse PHP Doc-comment".to_string(),
            Self::PHPDocTypeError(_, err) => {
                format!("Parse error while parsing type in phpdoc-comment: {}", err)
            }
            Self::MisplacedPHPDocEntry(_, reason) => format!(
                "PHPDoc-entry used in the wrong context: {}",
                reason.to_string_lossy()
            ),
            Self::InvalidPHPDocEntry(_, reason) => {
                format!("Invalid PHPDoc-entry: {}", reason.to_string_lossy())
            }
            Self::RedundantPHPDocEntry(_, reason) => {
                format!("Redundant PHPDoc-entry: {}", reason.to_string_lossy())
            }
            Self::UnknownPHPDocEntry(_, reason) => {
                format!("Unknown PHPDoc-entry: {}", reason.to_string_lossy())
            }
            Self::EmptyTemplate(_, name) => format!("Generic template {} is unforfilled", name),
        }
    }

    pub fn as_string_with_pos(&self) -> String {
        let mut desc = self.as_string();
        let fname: String = self
            .filename()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        desc.push_str(&format!(
            " on {}:{}",
            fname,
            self.range().start_point.row + 1
        ));
        desc
    }
}

pub trait IssueEmitter {
    fn emit(&self, issue: Issue);

    fn get_status(&self) -> Option<String> {
        None
    }
}

pub struct VoidEmitter {
    pub count: AtomicUsize,
}

impl Default for VoidEmitter {
    fn default() -> Self {
        Self::new()
    }
}

impl VoidEmitter {
    pub fn new() -> Self {
        VoidEmitter {
            count: AtomicUsize::new(0),
        }
    }
}

impl IssueEmitter for VoidEmitter {
    fn emit(&self, _issue: Issue) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    fn get_status(&self) -> Option<String> {
        let cnt = self.count.load(Ordering::Relaxed);
        Some(format!("Found {} issues", cnt))
    }
}
