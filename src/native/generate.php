<?php
$OVERRIDE = [];
require_once(__DIR__ . "/override.php");

function putfile(string $path, string $buffer)
{
    $dir = dirname($path);
    if (!is_dir($dir)) {
        mkdir($dir, 0755, true);
    }
    file_put_contents($path, $buffer);
}

function rust_value($val)
{
    if ($val === NULL) {
        return "PHPValue::NULL";
    }
    if (is_int($val)) {
        return "PHPValue::Int($val)";
    }
    if (is_bool($val)) {
        return "PHPValue::Boolean(" . ($val ? "true" : "false") . ")";
    }
    if (is_string($val)) {
        $strval = 'b"' .  addcslashes($val, "\r\n\t\"\\") . '"';
        return "PHPValue::String(OsStr::from_bytes($strval).to_os_string())";
    }
    if (is_float($val)) {
        return sprintf("PHPValue::Float(PHPFloat::Real(%.f))", $val);
    }
    if (is_array($val)) {
        $arr = [];
        foreach ($val as $a) {
            $arr[] = rust_value($a);
        }
        if (count($arr)) {
            return "PHPValue::Array(PHPArray(Vector(vec![" . implode(",", $arr) . "]))";

        }
        return "PHPValue::Array(PHPArray::Empty)";
    }
    var_dump(['m' => __METHOD__, 'l' => __LINE__, 'v' => $val]);
    die;
}

function rust_type(ReflectionType $type)
{
    if ($type instanceof ReflectionNamedType || $type instanceof FakeNamedType) {
        if ($type->isBuiltin()) {
            switch ($type->getName()) {
                case 'string':
                    return "DiscreteType::String.into()";
                case 'int':
                    return "DiscreteType::Int.into()";
                case 'float':
                    return "DiscreteType::Float.into()";
                case 'void':
                    return "DiscreteType::Void.into()";
                case 'bool':
                    return "DiscreteType::Bool.into()";
                case 'false':
                    return "DiscreteType::False.into()";
                case 'mixed':
                    return NULL;
                case 'array':
                    return "DiscreteType::Array.into()";
                case 'null':
                    return "DiscreteType::NULL.into()";
                case 'callable':
                    return "DiscreteType::Callable.into()";
                case 'object':
                    return "DiscreteType::Object.into()";
            }
            var_dump(__METHOD__, __LINE__, $type->getName());
            die;
        } else {
            $cname = $type->getName();
            if ($cname[0] == '\\') {
                $fq_cname = $cname;
                $parts = explode("\\", $cname);
                $cname = array_pop($parts);
            } else {
                $fq_cname = "\\" . $type->getName();
            }
            return sprintf(
                'DiscreteType::Named(r"%s".into(), r"%s".into()).into()',
                $cname,
                $fq_cname
            );
        }
    }
    if ($type instanceof ReflectionUnionType || $type instanceof ArrayOfTypes) {
        $types = [];
        foreach ($type->getTypes() as $utype) {
            $rust_type = rust_type($utype);

            $types[] = $rust_type;
        }
        return "UnionType::from(&[" . implode(",", $types) . "] as &[DiscreteType])";
    }

    var_dump("UNKNOWN TYPE");
    var_dump($type);
    die;
}

class ArrayOfTypes extends ReflectionType
{
    private $_types;
    function __construct($arr)
    {
        $types = [];
        foreach ($arr as $t) {
            $types[] = new FakeNamedType($t);
        }
        $this->_types = $types;
    }

    function getTypes()
    {
        return $this->_types;
    }
}

class FakeNamedType extends ReflectionType
{
    private $_type;
    function __construct($type)
    {
        $this->_type = $type;
    }

    function getName()
    {
        return $this->_type;
    }

    function isBuiltin()
    {
        return in_array($this->getName(), [
            'string',
            'int',
            'float',
            'void',
            'bool',
            'false',
            'mixed',
            'array',
            'null',
            'callable',
            'object',
        ]);
    }
}

function rust_type_opt(ReflectionType $type)
{
    if ($type instanceof ReflectionNamedType) {
        $x = rust_type($type);
        if ($x === NULL) {
            return "None";
        }
        return "Some($x)";
    }

    if ($type instanceof ReflectionUnionType) {
        foreach ($type->getTypes() as $type) {
            $x = rust_type($type);
            if ($x === NULL) {
                error_log(__METHOD__ . ":" . __LINE__ . ": Unable to process union type");
                return "None";
            }
            $types[] = $x;
        }
        return "Some(UnionType::from(&[" . implode(",", $types) . "] as &[DiscreteType]))";
    }
    var_dump(__METHOD__, __LINE__, $type);
    die;
}

$ver = "php_" . PHP_MAJOR_VERSION . "_" . PHP_MINOR_VERSION;

$root_mods = [];
$func_mods = [];

$mod_register = [];

$root_mods_func = [];

foreach (get_defined_functions()['internal'] as $function) {
    $ref = new ReflectionFunction($function);
    $mod = "std";
    $ext = $ref->getExtension();
    if ($ext) {
        $mod = strtolower(preg_replace("/\s/", "_", $ext->name));
    }

    if (!array_key_exists($mod, $func_mods)) {
        $func_mods[$mod] = "";
    }
    if (!array_key_exists($mod, $root_mods)) {
        $root_mods[$mod] = [];
    }
    $root_mods[$mod]['functions'] = "pub mod functions;";

    $function_name = preg_replace('/\\\\/', '_', $function);
    if ($function_name ==  '_') {
        $function_name = '__';
    }

    $ret_type = "None";

    $rtype = $ref->getReturnType();
    if ($rtype) {
        $ret_type = rust_type_opt($rtype);
    }
    $args = array();
    foreach ($ref->getParameters() as $param) {
        $arg = sprintf(
            "FunctionArgumentData {
            name: \"%s\".into(),
            
            // pub arg_type: Option<UnionType>,
            arg_type: %s,
            // pub default_value: Option<PHPValue>,
            default_value: %s,
            // pub nullable: bool,
            nullable: %s,
            // pub optional: bool,
            optional: %s,

            inline_phpdoc_type: None,
            phpdoc_entry: None,
            phpdoc_type: None,
        }",
            $param->name,
            "None",
            $param->isDefaultValueAvailable() ? "Some(" . rust_value($param->getDefaultValue()) . ")" : "None",
            $param->allowsNull() ? "true" : "false",
            $param->isOptional() ? "true" : "false"
        );
        $args[] = $arg;
    }
    $arguments = "vec!(" . implode(",", $args) . ")";

    // $fname = preg_replace('/\\\\/', '\\', $function);
    $function = "\\$function";

    $buffer = "
    use crate::analysis::state::AnalysisState;
    use crate::symboldata::FileLocation;
    use crate::symboldata::FunctionData;
    use crate::symboldata::class::FunctionArgumentData;
    use crate::symbols::FullyQualifiedName;
    use crate::symbols::Name;
    use crate::types::union::DiscreteType;
    use std::sync::Arc;
    use std::sync::RwLock;
    use std::collections::HashMap;

    use std::os::unix::prelude::OsStrExt;
    use std::ffi::OsStr;
    use std::ffi::OsString;

    use crate::value::PHPValue;
    use crate::value::PHPFloat;
    use crate::value::PHPArray;
    use crate::types::union::UnionType;


    pub fn register_$function_name(state: &mut AnalysisState) {
        let func_name = FullyQualifiedName::from(r\"$function\");
        let func_data = FunctionData {
            name: func_name.clone(),
            position: FileLocation::internal(),
            php_return_type: $ret_type,
            comment_return_type: None,
            inferred_return_type: None,
            arguments: $arguments,
            variadic: false,
            deterministic: false,
            pure: false,
            return_value: None,
            overload_map: HashMap::new(),
            generic_templates: None,
        };
        {
            let mut functions = state.symbol_data.functions.write().unwrap();

            functions.insert(func_name, Arc::new(RwLock::new(func_data)));
        }
    }
    ";
    putfile("$ver/$mod/functions/$function_name.rs", $buffer);
    $func_mods[$mod] .= "pub mod $function_name;\n";
    if (!array_key_exists($mod, $mod_register)) {
        $mod_register[$mod] = "";
    }
    $mod_register[$mod] .= "$function_name::register_$function_name(state);\n";
}

foreach ($func_mods as $mod => $func_mod) {
    $func_mod .= "\n\nuse crate::analysis::state::AnalysisState;\n";
    $func_mod .= "\n\npub fn register(state: &mut AnalysisState) {";
    $func_mod .= $mod_register[$mod];
    $func_mod .= "}";
    if (!array_key_exists($mod, $root_mods_func)) {
        $root_mods_func[$mod] = "";
    }
    $root_mods_func[$mod] .= "functions::register(state);\n";
    putfile("$ver/$mod/functions/mod.rs", $func_mod);
}

$class_mods = [];
foreach (get_declared_classes() as $class) {
    $ref = new ReflectionClass($class);
    if ($ref->isUserDefined()) {
        continue;
    }
    if (array_key_exists($class, $OVERRIDE['classes'])) {
        $override = $OVERRIDE['classes'][$class];
    } else {
        $override = [
            'methods' => []
        ];
    }
    $ext = $ref->getExtension();
    if ($ext) {
        $mod = strtolower(preg_replace("/\s/", "_", $ext->name));
    } else {
        $mod = 'std';
    }

    if (!array_key_exists($mod, $root_mods)) {
        $root_mods[$mod] = [];
    }
    $root_mods[$mod]['classes'] = "pub mod classes;";

    $lccname = strtolower($class);
    $nlccname = strtolower(preg_replace("/\\\\/", "_", $lccname));

    $class_rs = "";
    $class_rs .= "\n\n";
    $class_rs .= "use crate::analysis::state::AnalysisState;\n";
    $class_rs .= "use crate::symboldata::FileLocation;\n";
    $class_rs .= "use crate::symbols::FullyQualifiedName;\n";
    $class_rs .= "use crate::symbols::Name;\n";
    $class_rs .= "use std::ffi::OsString;\n";
    $class_rs .= "use crate::symboldata::class::ClassData;\n";
    $class_rs .= "use crate::symboldata::class::ClassName;\n";
    $class_rs .= "use crate::symboldata::class::ClassModifier;\n";
    $class_rs .= "use crate::symboldata::class::ClassType;\n";
    $class_rs .= "use crate::symboldata::class::MethodData;\n";
    $class_rs .= "use crate::symboldata::class::ClassMemberVisibility;\n";
    $class_rs .= "use std::sync::Arc;\n";
    $class_rs .= "use std::sync::RwLock;\n";
    $class_rs .= "use crate::types::union::DiscreteType;\n";
    $class_rs .= "use crate::types::union::UnionType;\n";
    $class_rs .= "use crate::value::PHPFloat;\n";
    $class_rs .= "use crate::value::PHPArray;\n";


    $modifier = "None";
    if ($ref->isFinal()) {
        $modifier = "Final";
    } else if ($ref->isAbstract()) {
        $modifier = "Abstract";
    }
    $interfaces = [];
    foreach ($ref->getInterfaceNames() as $name) {
        $rust_name = sprintf('FullyQualifiedName::from(r#"\\%s"#)', $name);
        $interfaces[] = "ClassName::new_with_names($rust_name.get_name().unwrap_or_else(|| Name::new()), $rust_name)";
    }
    $traits = [];
    foreach ($ref->getTraitNames() as $name) {
        $rust_name = sprintf('FullyQualifiedName::from(r#"\\%s"#)', $name);
        $traits[] = "ClassName::new_with_names($rust_name.get_name().unwrap_or_else(|| Name::new()), $rust_name)";
    }

    $class_rs .= "\n\npub fn register_$nlccname(state: &mut AnalysisState) {\n
        let fq_cname = FullyQualifiedName::from(r#\"\\$class\"#);
        let class_name = ClassName::new_with_names(fq_cname.get_name().unwrap_or_else(|| Name::new()), fq_cname.clone());
        let cdata_handle = state.symbol_data.get_or_create_class(&class_name);
        let mut class_data = ClassData::new(FileLocation::internal(), class_name.clone());
        class_data.is_native = true;
//         class_data.class_name = class_name.clone();
        class_data.modifier = ClassModifier::$modifier;
        class_data.interfaces = vec![" . implode(",\n", $interfaces) . "];
        class_data.traits = vec![" . implode(",\n", $traits) . "];
    ";
    if ($parent = $ref->getParentClass()) {
        $class_rs .= sprintf("let parent_class_name = FullyQualifiedName::from(r#\"%s\\%s\"#);\n", $parent->getNamespaceName(), $parent->getName());
        $class_rs .= "        class_data.base_class_name  = Some(ClassName::new_with_names(parent_class_name.get_name().unwrap_or_else(|| Name::new()), parent_class_name.clone()));\n";
    }
    foreach ($ref->getConstants() as $const_name => $const_val) {
        $class_rs .= "
        class_data.constants.insert(r#\"$const_name\"#.into(), " . rust_value($const_val) . ");
        ";
    }

    foreach ($ref->getMethods() as $method) {
        if ($method->getDeclaringClass() != $ref) {
            // Only register methods declared in this class, and not any traits or parent classes
            continue;
        }
        if (array_key_exists($method->getName(), $override['methods'])) {
            $method_override = $override['methods'][$method->getName()];
        } else {
            $method_override = [];
        }
        $class_rs .= "{\n";
        $class_rs .= "  let mname = Name::from(r#\"" . $method->getName() . "\"#);\n";
        $class_rs .= "  let mut mdata = MethodData::new_with_name(FileLocation::internal(), class_name.clone(), mname.clone());\n";
        if ($method->isAbstract()) {
            $class_rs .= "  mdata.modifier = ClassModifier::Abstract;\n";
        } else if ($method->isFinal()) {
            $class_rs .= "  mdata.modifier = ClassModifier::Final;\n";
        }

        $class_rs .= "  mdata.visibility = ";
        if ($method->isPrivate()) {
            $class_rs .= "ClassMemberVisibility::Private;\n";
        } else if ($method->isProtected()) {
            $class_rs .= "ClassMemberVisibility::Protected;\n";
        } else {
            $class_rs .= "ClassMemberVisibility::Public;\n";
        }
        $ret_type = $method->getReturnType();
        if (!$ret_type) {
            if (array_key_exists('return', $method_override)) {
                $ret_type = new ArrayOfTypes($method_override['return']);
            }
        }
        if ($ret_type) {
            $rust_type = rust_type($ret_type);
            if ($rust_type) {
                $class_rs .= "mdata.php_return_type = Some($rust_type);\n";
            }
        }
        $class_rs .= "  mdata.is_static = " . ($method->isStatic() ? "true" : "false") . ";\n";
        $class_rs .= "  class_data.methods.insert(mname.to_ascii_lowercase(), Arc::new(RwLock::new(mdata)));\n";
        $class_rs .= "}\n";
    }
    $class_rs .= "
        // class_data.constants = HashMap::new();
        // class_data.methods = HashMap::new();
        ";

    $class_rs .= "        
        {
            let mut writeable = cdata_handle.write().unwrap();
            *writeable = ClassType::Class(class_data);
        }
        
    }";

    if (preg_match("/PHPValue/", $class_rs)) {
        $class_rs = "use crate::value::PHPValue;\n" . $class_rs;
    }
    if (preg_match("/\bOsStr\b/", $class_rs)) {
        $class_rs = "use std::ffi::OsStr;\n" . $class_rs;
    }
    if (preg_match("/\bOsStr::from_bytes\b/", $class_rs)) {
        $class_rs = "use std::os::unix::prelude::OsStrExt;\n" . $class_rs;
    }

    if (!array_key_exists($mod, $root_mods_func)) {
        $root_mods_func[$mod] = "";
    }
    if (!array_key_exists($mod, $class_mods)) {
        $class_mods[$mod] = "";
    }
    $root_mods_func[$mod] .= " classes::$nlccname::register_$nlccname(state);\n";
    $class_mods[$mod] .= "pub mod $nlccname;\n";
    $fname = "$ver/$mod/classes/$nlccname.rs";
    putfile($fname, $class_rs);
}

foreach ($class_mods as $mod => $mod_rs) {
    putfile("$ver/$mod/classes/mod.rs", $mod_rs);
}

foreach (get_declared_interfaces() as $interfaces) {
}

foreach (get_declared_traits() as $traits) {
}


$ver_mod = "";
$ver_mod_func = "";
foreach ($root_mods as $mod => $root_mod) {
    $root_mod = implode("", $root_mod);
    $root_mod .= "\nuse crate::analysis::state::AnalysisState;
    \n\npub fn register(state: &mut AnalysisState) {\n";
    if (array_key_exists($mod,  $root_mods_func)) {
        $root_mod .= " " . $root_mods_func[$mod];
    }
    $root_mod .= "}\n";
    putfile("$ver/$mod/mod.rs", $root_mod);
    $ver_mod .= "pub mod $mod;\n";
    $ver_mod_func .= " $mod::register(state);\n";
}
$ver_mod .= "\nuse crate::analysis::state::AnalysisState;
\n\npub fn register(state: &mut AnalysisState) {\n";
$ver_mod .= $ver_mod_func;
$ver_mod .= "}\n";

putfile("$ver/mod.rs", $ver_mod);
