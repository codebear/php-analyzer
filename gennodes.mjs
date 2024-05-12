import fs from 'fs';


const raw_json = fs.readFileSync("tree-sitter-php/php/src/node-types.json");

const additional_properties = {
    "class_declaration": "crate::nodeanalysis::class_declaration::ClassDeclarationState",
    "method_declaration": "crate::nodeanalysis::method_declaration::MethodDeclarationState",
};

const node_defs = JSON.parse(raw_json);
let operators = {
    // Assignments
    '%=': "ModAssign",
    '&=': "AndAssign",
    '**=': "PowAssign",
    '*=': "MultAssign",
    '+=': "AddAssign",
    '-=': "SubAssign",
    '.=': "ConcatAssign",
    '/=': "DivAssign",
    '<<=': "LeftShiftAssign",
    '>>=': "RightShiftAssign",
    '??=': "NullsafeAssign",
    '^=': "XorAssign",
    '|=': "OrAssign",

    // Binary operators
    "!=": "NotEqual",
    "!==": "NotIdentical",
    "%": "Mod",
    "&": "BinaryAnd",
    "&&": "BooleanAnd",
    "*": "Mult",
    "+": "Add",
    "-": "Sub",
    ".": "Concat",
    "/": "Div",
    "<": "LessThan",
    "<<": "LeftShift",
    "<=": "LessThanOrEqual",
    "<=>": "Spaceship",
    "<>": "NotEqual",
    "==": "Equal",
    "===": "Identical",
    ">": "GreaterThan",
    ">=": "GreaterThanOrEqual",
    ">>": "RightShift",
    "^": "BinaryXor",
    "and": "LogicalAnd",
    "instanceof": "Instanceof",
    "or": "LogicalOr",
    "xor": "LogicalXor",
    "|": "BinaryOr",
    "||": "BooleanOr",
    "**": "Exponential",

    "++": "Increment",
    "--": "Decrement",

    "??": "NullCoalesce",

    // Unary operators
    "!": "Not",
    "@": "Squelch",
    "~": "BinaryNot",

    // Noe
    "{": "OpenBrace",
    "}": "CloseBrace",
};
let type_name_map = {
    ...operators,

    // Div
    ",": "Comma",
    "\"": "DoubleQuote",
};



let type_map = {};

function to_camel_case(str) {
    let pre = str.match(/^_/) ? "_" : "";
    return pre + str.replace(/(_|^)(.)/g, function (_m0, _m1, m2) {
        return m2.toUpperCase();
    })
}

function get_rust_enum_name(type) {
    if (type in type_name_map) {
        return type_name_map[type];
    }
    return to_camel_case(type);
}

function get_rust_type_name(type, suffix = undefined) {
    if (type in type_map) {
        return type_map[type];
    }
    return to_camel_case(type) + (suffix !== undefined ? suffix : "Node");
}

function rust_str(value) {
    if (value.match(/"|\\/)) {
        return 'r#"' + value + '"#';
    }
    return '"' + value + '"';
}

function rustify_name(name) {
    if (name == 'type') {
        return 'type_';
    }
    return name;
}

for (const node_def of node_defs) {
    const nostrings = [
        'null',
        'string',
        'float'
    ];
    if (!node_def.named && nostrings.indexOf(node_def.type) == -1) {
        if (node_def.type in operators) {
            type_map[node_def.type] = operators[node_def.type] + "Operator"; // "&'static str";
        } else {
            type_map[node_def.type] = "&'static str";
        }
    }
}

function get_enum_matches(enum_name, types, callback) {
    let buf = "";
    let [capture, closure] = callback("extra", get_rust_type_name("extra"));
    buf += `${enum_name}::Extra(${capture}) => ${closure},` + "\n";
    // [capture, closure] = callback("text_interpolation", get_rust_type_name("text_interpolation"));
    // buf += `${enum_name}::TextInterpolation(${capture}) => ${closure},` + "\n";
    // [capture, closure] = callback("ERROR", get_rust_type_name("ERROR"));
    // buf += `${enum_name}::Error(${capture}) => ${closure},` + "\n";

    let map = {};
    for (let enum_type of types) {
        let tname = get_rust_enum_name(enum_type.type);
        if (tname in map) {
            continue;
        }
        let [capture, closure] = callback(enum_type.type, get_rust_type_name(enum_type.type));
        buf += `    ${enum_name}::${tname}(${capture}) => ${closure},` + "\n";
        map[tname] = 1;
    }
    return buf;
}

function create_enum_for_types(name, types) {
    let enum_uses = [
        //"tree_sitter::Range",
        "crate::parser::Range",
        "crate::autotree::NodeAccess",
        "crate::autotree::ParseError",
        "crate::autonodes::any::AnyNodeRef",
        "crate::extra::ExtraChild",
    ];
    let enum_entries = new Set();
    let has_node_access = true;
    let has_utype_and_value_access = true;
    for (const enum_type of types) {
        let tname = get_rust_enum_name(enum_type.type);
        let ttype = get_rust_type_name(enum_type.type);
        enum_uses.push("crate::autonodes::" + enum_type.type + "::" + ttype);
        let entry = tname + "(";
        let type_match;
        if (ttype.match(/'static/)) {
            entry += ttype + ", Range";
        } else if (type_match = ttype.match(/^(.*)Operator$/)) {
            let parts = type_match[1].replace(/([A-Z])/g, "_$1").toLowerCase().split("_");
            parts.shift();
            let operator_module = parts.join("_");
            if (operator_module == "mod") {
                operator_module = "modulus";
            }
            enum_uses.push("crate::operators::operator::Operator");
            enum_uses.push("crate::operators::" + operator_module + "::" + ttype);
            entry += ttype;
            has_node_access = false;
            has_utype_and_value_access = false;
        } else {
            entry += "Box<" + ttype + ">";
        }
        entry += ")";
        enum_entries.add(entry);
    }
    let child_enum_buffer = "\n#[derive(Debug, Clone)]\n";
    child_enum_buffer += "pub enum " + name + " {\n";
    for (const entry of enum_entries) {
        child_enum_buffer += "  " + entry + ",\n";
    }

    // uses.push("crate::autonodes::any::AnyNode");
    enum_uses.push("crate::autonodes::comment::CommentNode");
    enum_uses.push("crate::errornode::ErrorNode");
    // enum_uses.push("crate::autonodes::text_interpolation::TextInterpolationNode");
    // child_enum_buffer += "   Unknown(Box<AnyNode>),\n";
    child_enum_buffer += "  Extra(ExtraChild),\n";

    // child_enum_buffer += "   Comment(Box<CommentNode>),\n";
    // child_enum_buffer += "   TextInterpolation(Box<TextInterpolationNode>),\n";
    // child_enum_buffer += "   Error(Box<ErrorNode>),\n";
    child_enum_buffer += "}\n\n";


    let match_enum_buffer = "";
    match_enum_buffer += `      "comment" => ${name}::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(node, source)?))),` + "\n";
    // match_enum_buffer += `      "text_interpolation" => ${name}::Extra(ExtraChild::TextInterpolation(Box::new(TextInterpolationNode::parse(node, source)?))),` + "\n";
    match_enum_buffer += `      "ERROR" => ${name}::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?))),` + "\n";
    let opt_wildcard = "";
    let new_wildcard = "";
    for (let type of types) {
        let child_type = get_rust_type_name(type.type);
        let child_enum_variant = get_rust_enum_name(type.type);
        let opt_parsing = child_type + "::parse_opt(node, source)?";
        let parsing = `${child_type}::parse(node, source)?`;
        if (child_type.match(/'static/)) {
            parsing = rust_str(type.type);
            parsing += ", node.range().into()";
            opt_parsing = 'Some(todo!("Det lyt fiksas"))';
        } else if (child_type.match(/Operator$/)) {
            parsing = child_type + "(node.range().into())";
            opt_parsing = 'Some(todo!("Det lyt fiksas"))';
        } else {
            parsing = "Box::new(" + parsing + ")";
            opt_parsing += ".map(|x| Box::new(x))";
        }
        let match = rust_str(type.type);
        if (type.type.match(/^_/)) {
            match = "_";
        }
        let entry = `                    ${match} => ${name}::${child_enum_variant}(${parsing}),` + "\n";
        opt_parsing += `.map(|y| ${name}::${child_enum_variant}(y))`;
        if (match == '_') {
            new_wildcard += `if let Some(x) = ${opt_parsing} { x } else `;
            opt_wildcard += `if let Some(x) = ${opt_parsing} { Some(x) } else `;
        } else {
            match_enum_buffer += entry;
        }
    }

    let parse_wildcard = "";
    let parse_opt_wildcard = "";
    if (new_wildcard) {
        parse_wildcard += `_ => ${new_wildcard} { return Err(ParseError::new(node.range(), format!("Parse error, unexpected node-type: {}", node.kind()))); },`;
        parse_opt_wildcard += `_ => return Ok(${opt_wildcard} { None }),`;

    } else {
        parse_wildcard += `        _ => return Err(ParseError::new(node.range(), format!("Parse error, unexpected node-type: {}", node.kind()))),`;
        parse_opt_wildcard = `_ => return Ok(None),`;
    }
    enum_uses.push("crate::analysis::state::AnalysisState");
    enum_uses.push("crate::autotree::NodeParser");
    enum_uses.push("crate::issue::IssueEmitter");
    enum_uses.push("crate::value::PHPValue");
    enum_uses.push("crate::types::union::UnionType");
    child_enum_buffer += `\nimpl NodeParser for ${name} {
        fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
            Ok(match node.kind() {
                ${match_enum_buffer}
                ${parse_wildcard}
            })
        }
    }
        `;
    child_enum_buffer += `\nimpl ${name} {

        pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
            Ok(Some(match node.kind() {
                ${match_enum_buffer}
                ${parse_opt_wildcard}
            }))
        }

        pub fn kind(&self) -> &'static str {
            match self {
                ${get_enum_matches(name, types, function (child_type, rust_type) {
        if (rust_type.match(/'static/)) {
            enum_uses.push("crate::types::union::DiscreteType");

            return ["y, _", 'y'];
        }
        return ['y', 'y.kind()'];
    })}
            }
        }

        pub fn parse_vec<'a, I>(children: I, source: &Vec<u8>) -> Result<Vec<Box<Self>>,ParseError>
        where I: Iterator<Item=Node<'a>> {
            let mut res: Vec<Box<Self>> = vec!();
            for child in children {
                res.push(Box::new(Self::parse(child, source)?));
            }
            Ok(res)
        }
`;
    if (has_utype_and_value_access) {

        child_enum_buffer += `
        pub fn get_utype(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) -> Option<UnionType> {
            match self {
    ${get_enum_matches(name, types, function (child_type, rust_type) {
            if (rust_type.match(/'static/)) {
                enum_uses.push("crate::types::union::DiscreteType");

                return ["_,_", 'Some(DiscreteType::String.into())'];
            }
            return ['x', 'x.get_utype(state, emitter)'];
        })}
            }
        }

        pub fn get_php_value(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) -> Option<PHPValue> {
            match self {
    ${get_enum_matches(name, types, function (child_type, rust_type) {
            if (rust_type.match(/'static/)) {
                enum_uses.push("std::ffi::OsStr");
                return ["a,_", 'Some(PHPValue::String(OsStr::new(a).to_os_string()))'];
            }
            return ['x', 'x.get_php_value(state, emitter)'];
        })}
            }
        }

        pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
            match self {
    ${get_enum_matches(name, types, function (child_type, rust_type) {
            if (rust_type.match(/'static/)) {
                return ["_,_", '()'];
            }
            return ['x', 'x.read_from(state, emitter)'];
        })}
            }
        }`;
    }
    child_enum_buffer += `
    }
    `;
    if (has_node_access) {
        child_enum_buffer += `

    impl NodeAccess for ${name} {

        fn brief_desc(&self) -> String {
            match self {
    ${get_enum_matches(name, types, function (child_type, rust_type) {
            if (rust_type.match(/'static/)) {
                return ["a,_", `a.to_string()`];
            }
            return ["x", `format!("${name}::${child_type}({})", x.brief_desc())`];
        })}
            }
        }

        fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
            match self {
    ${get_enum_matches(name, types, function (child_type, rust_type) {
            if (rust_type.match(/'static/)) {
                return ["a,b", `AnyNodeRef::StaticExpr(a, *b)`];
            }
            if (rust_type.match(/Operator/)) {
                return ["op", `AnyNodeRef::Operator(op)`];
            }
            return ["x", 'x.as_any()'];
        })}
            }
        }

        fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
            match self {
    ${get_enum_matches(name, types, function (child_type, rust_type) {
            if (rust_type.match(/'static/)) {
                return ["_,_", 'todo!("Crap")'];
            }
            return ["x", "x.children_any()"];
        })}
            }       
        }

        fn range(&self) -> Range {
            match self {
    ${get_enum_matches(name, types, function (child_type, rust_type) {
            if (rust_type.match(/'static/)) {
                return ["_,r", "*r"];
            }
            return ["x", "x.range()"];
        })}
            }
        }

    }
    `;

    }
    return [child_enum_buffer, enum_uses];
}

let any_uses = {
    "crate::errornode::ErrorNode": "",
    "crate::autotree::NodeParser": "",
};

let any_names = [];

let any_buffer = "\n\n";
any_buffer += "\n#[derive(Debug, Clone)]\n";
any_buffer += "pub enum AnyNode {\n";

let any_ref_buffer = "\n\n";
any_ref_buffer += "\nuse crate::operators::operator::Operators;\n\n";

any_ref_buffer += "\n#[derive(Debug, Clone)]\n";
any_ref_buffer += `pub enum AnyNodeRef<'a> {
    StaticExpr(&'static str, Range),
    Error(&'a ErrorNode),
    Operator(Operators<'a>),
`;

let mod_buffer = "";
mod_buffer += "pub mod any;\n";
for (const node_def of node_defs) {
    if (!node_def.named) {
        continue;
    }
    console.log("## " + node_def.type + " ##");

    const node_name = get_rust_type_name(node_def.type);
    let uses = {};
    let node_buffer = "";
    let child_enum_buffer = "";
    let impl_buffer = "";
    let declares = [];
    let any_name = get_rust_type_name(node_def.type, "");
    let child_cast = "";
    if ("subtypes" in node_def) {
        declares.push(node_name);
        let [sub_node_buffer, usings] = create_enum_for_types(node_name, node_def.subtypes);
        usings.map(function (u) {
            uses[u] = "";
        });
        child_enum_buffer += sub_node_buffer;

        uses["tree_sitter::Node"] = "";

    } else {
        //uses["tree_sitter::Range"] = "";
        uses["crate::parser::Range"] = "";
        uses["crate::autotree::NodeAccess"] = "";
        uses["crate::autotree::NodeParser"] = "";
        uses["crate::autotree::ChildNodeParser"] = "";
        uses["crate::autotree::ParseError"] = "";
        declares.push(node_name);
        node_buffer += "\n#[derive(Debug, Clone)]\n";
        node_buffer += `pub struct ${node_name} {\n`;
        node_buffer += "    pub range: Range,\n";
        let child_count = 0;
        let impl_prequel = "";

        let impl_members = "";
        if ("fields" in node_def && Object.values(node_def.fields).length) {
            if ("children" in node_def && Object.values(node_def.children).length) {
                impl_prequel += "let mut skip_nodes: Vec<usize> = vec!();\n";
            }
            for (const field_name in node_def.fields) {
                child_count++;
                const field_def = node_def.fields[field_name];
                let field_type;
                // console.log(field_def);

                let impl_prequel_entry = "";
                let impl_prequel_entry2 = "";

                let rust_name = rustify_name(field_name);
                //impl_prequel_entry += 'node.children_by_field_name("' + field_name + '", &mut node.walk())';
                impl_prequel_entry2 += "node.parse_child(\"" + field_name + "\"";
                impl_prequel_entry += "Result::from(node.parse_child(\"" + field_name + "\", source)";

                let boxed = false;
                if (field_def.types.length == 1) {
                    let raw_type = field_def.types[0].type;
                    field_type = get_rust_type_name(raw_type);
                    uses["crate::autonodes::" + raw_type + "::" + field_type] = "";
                    //uses["crate::autotree::NodeParserHelper"] = "";
                    /*if (field_type.match(/'static/)) {
                        impl_prequel_entry += "/ * hva * /";
                    } else {
                    impl_prequel_entry += "\n\t\t\t\t.map(|chnode1| " + field_type + "::parse(chnode1, source))";
                    }*/
                    impl_prequel_entry2 += ", |chnode1| " + field_type + "::parse(chnode1, source)";

                    //                    impl_prequel_entry += ".collect::<Result<Vec<_>, ParseError>>()?.drain(..)";
                } else if (field_def.types.length > 1) {
                    // uses["crate::autotree::NodeParserHelper"] = "";

                    // impl_prequel_enrty = "";

                    let field_enum_name = get_rust_type_name(node_def.type, to_camel_case(field_name));
                    // impl_prequel_entry += "\n\t\t\t\t.map(|chnode2| " + field_enum_name + "::parse(chnode2, source))";
                    // impl_prequel_entry += "            .collect::<Result<Vec<_>, ParseError>>()?.drain(..)";

                    impl_prequel_entry2 += ", |chnode2| " + field_enum_name + "::parse(chnode2, source)";

                    let [part_child_enum_buffer, usings] = create_enum_for_types(field_enum_name, field_def.types);
                    child_enum_buffer += part_child_enum_buffer;
                    usings.map(function (u) {
                        uses[u] = "";
                    });
                    // impl_prequel_entry += ".boxed()";
                    //                     impl_prequel_entry += "\n\t\t\t\t.map(|z| Box::new(z))";
                    field_type = "Box<" + field_enum_name + ">";
                    boxed = true;
                    // console.log(field_def);
                }
                impl_prequel_entry2 += ")";
                if ("children" in node_def && Object.values(node_def.children).length) {
                    //impl_prequel_entry += "\n\t\t\t\t.map(|chnode| { skip_nodes.push(chnode.id()); chnode })\n";
                    impl_prequel_entry += ".mark_skipped_node(&mut skip_nodes)";
                    //                    impl_prequel_entry += ", Some(&mut skip_nodes)";
                    //                } else {
                    //                    impl_prequel_entry += ", None";
                    //impl_prequel_entry += ", Some(&mut skip_nodes)).into())?";
                } else {
                    //impl_prequel_entry += ", None).into())?";

                }
                let child_cast_entry = "";
                if (boxed) {
                    // impl_prequel_entry += "\n\t\t\t\t.into()";
                    impl_prequel_entry2 += ".boxed()";
                } else {
                    // impl_prequel_entry += ".to_owned()"
                }

                if (field_def.multiple) {
                    field_type = "Vec<" + field_type + ">";
                    // impl_prequel_entry += "\n\t\t\t\t.collect::<" + field_type + ">()";
                    impl_prequel_entry2 += "\n\t\t\t\t.many()";
                    if (field_def.required) {
                        child_cast_entry = `child_vec.extend(self.${rust_name}.iter().map(|v| v.as_any()));` + "\n";
                    } else {
                        child_cast_entry = `if let Some(x) = &self.${rust_name} { child_vec.extend(x.iter().map(|z| z.as_any()));}` + "\n";
                    }
                } else if (field_def.required) {
                    // impl_prequel_entry += "\n\t\t\t\t.next()";
                    impl_prequel_entry2 += "\n\t\t\t\t.one()";
                } else {
                    impl_prequel_entry2 += "\n\t\t\t\t.maybe_one()";
                }

                //                 if let Some(x) = self.child { &[x.as_any()] } else { &[] },\n
                if (!field_def.required) {
                    field_type = "Option<" + field_type + ">";
                    if (!child_cast_entry) child_cast_entry += `if let Some(x) = &self.${rust_name} { child_vec.push(x.as_any()); }` + "\n";
                } else {
                    if (!child_cast_entry) child_cast_entry += `child_vec.push(self.${rust_name}.as_any());` + "\n";
                    //if (!field_def.multiple) {
                    //    impl_prequel_entry += '\n\t\t\t\t.expect("Field ' + field_name + ' should exist")';
                    //}
                }
                child_cast += child_cast_entry;




                impl_members += rust_name + ",\n";

                impl_prequel += "       let " + rust_name + ": " + field_type + " = " + impl_prequel_entry + ".into())?;";


                // console.log(field_def);
                node_buffer += `    pub ${rustify_name(field_name)}: ${field_type},\n`;
            }
        }
        if ("children" in node_def && Object.values(node_def.children).length) {
            child_count++;
            let children_def = node_def.children;
            // console.log(node_def.children);
            let children_type;
            let base_children_type;
            if (children_def.types.length == 1) {
                let raw_type = children_def.types[0].type;
                base_children_type = children_type = get_rust_type_name(raw_type);
                let using = "crate::autonodes::" + raw_type + "::" + children_type;
                uses[using] = "";
                children_type = "Box<" + children_type + ">";
            } else if (children_def.types.length > 1) {
                base_children_type = children_type = get_rust_type_name(node_def.type, "Children");
                declares.push(children_type);
                let [rs_enum, usings] = create_enum_for_types(children_type, children_def.types);
                usings.map(function (u) {
                    uses[u] = "";
                });

                children_type = "Box<" + children_type + ">";
                child_enum_buffer += rs_enum;
            } else {
                children_type = "BROKEN";
            }
            let extra = `extras: ExtraChild::parse_vec(
        node.named_children(&mut node.walk())
            .filter(|node| node.kind() == "comment")`;
            if (children_def.multiple) {
                node_buffer += "    pub children: Vec<" + children_type + ">,\n";


                let children_filter = "";

                if ("fields" in node_def && Object.values(node_def.fields).length) {
                    children_filter += ".filter(|node| !skip_nodes.contains(&node.id()))";
                    extra += ".filter(|node| !skip_nodes.contains(&node.id()))";
                }
                children_filter += '.filter(|node| node.kind() != "comment")';
                impl_members += "    children: " + base_children_type + "::parse_vec(node.named_children(&mut node.walk())" + children_filter + ", source)?,\n";
                extra += ",\nsource)?,";
                impl_members += extra;
                child_cast += "child_vec.extend(self.children.iter().map(|n| n.as_any()));\n"
            } else {
                let impl_decl = "node.named_children(&mut node.walk())";

                if ("fields" in node_def && Object.values(node_def.fields).length) {
                    impl_decl += ".filter(|node| !skip_nodes.contains(&node.id()))";
                    extra += ".filter(|node| !skip_nodes.contains(&node.id()))";
                }
                impl_decl += '.filter(|node| node.kind() != "comment")';
                impl_decl += ".map(|k| " + base_children_type + "::parse(k, source))";
                impl_decl += ".collect::<Result<Vec<" + base_children_type + ">,ParseError>>()?";
                impl_decl += ".drain(..)";
                if (children_type.match(/^Box</)) {
                    impl_decl += ".map(|j| Box::new(j))";
                }
                impl_decl += ".next()";
                if (children_def.required) {
                    impl_decl += ".expect(\"Should be a child\")";
                    child_cast += "child_vec.push(self.child.as_any()); \n";
                } else {
                    children_type = "Option<" + children_type + ">";
                    child_cast += "if let Some(x) = &self.child { child_vec.push(x.as_any()); }\n";
                }



                node_buffer += "    pub child: " + children_type + ",\n";

                impl_members += "    child: " + impl_decl + ",\n";

                extra += ",\n source)?,";
                impl_members += extra;
            }
            child_cast += "     child_vec.extend(self.extras.iter().map(|n| n.as_any()));\n";

        } else if (child_count) {
            // impl_members += "extras: vec![], // todo lookup unused nodes\n";
            impl_members += `extras: ExtraChild::parse_vec(
        node.named_children(&mut node.walk())
            .filter(|node| node.kind() == "comment"), source).unwrap(),`;
        }
        let raw_getter = "";
        if (!child_count) {
            node_buffer += "    pub raw: Vec<u8>,\n";
            impl_members += "   raw: source[range.start_byte..range.end_byte].to_vec(),\n";
            raw_getter = `
    pub fn get_raw(&self) -> OsString {
        OsStr::from_bytes(&self.raw).to_os_string()
    }
`;
            uses['std::ffi::OsStr'] = '';
            uses['std::ffi::OsString'] = '';
            uses['std::os::unix::ffi::OsStrExt'] = '';

        } else {
            uses["crate::extra::ExtraChild"] = "";
            node_buffer += "    pub extras: Vec<Box<ExtraChild>>,\n";
        }

        if (node_def.type in additional_properties) {
            uses["std::sync::OnceLock"] = "";
            let prop_name = additional_properties[node_def.type];
            node_buffer += "    pub state: OnceLock<" + prop_name + ">,\n";
            impl_members += "   state: OnceLock::new(),\n";
        }

        node_buffer += "}\n";

        // Implement parsing
        uses["tree_sitter::Node"] = "";
        impl_buffer += "\n";
        impl_buffer += `

impl NodeParser for ${node_name} {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "${node_def.type}" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [${node_def.type}] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        ${impl_prequel}
        Ok(Self {
            range,
            ${impl_members}
        })
    }
}

impl ${node_name} {


    pub fn kind(&self) -> &'static str {
        "${node_def.type}"
    }
    ${raw_getter}
}

impl NodeAccess for ${node_name} {

        fn brief_desc(&self) -> String {
            "${node_name}".into()
        }

        fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
            AnyNodeRef::${any_name}(self)
        }

        fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
            `;
        if (child_cast) {
            impl_buffer += `
            let mut child_vec: Vec<AnyNodeRef<'a>> = vec!();

            // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
            ${child_cast}
            child_vec.sort_by(|a, b|a.range().start_byte.cmp(&b.range().start_byte));
            child_vec
            `;
        } else {
            impl_buffer += "vec!()"
        }
        impl_buffer += `
        }
    
        fn range(&self) -> Range {
            self.range
        }

   
        }
        `;
        uses["crate::autonodes::any::AnyNodeRef"] = "";
    }

    any_names.push(any_name);
    any_buffer += `     ${any_name}(Box<${node_name}>),` + "\n";
    any_ref_buffer += `     ${any_name}(&'a ${node_name}),` + "\n";
    let any_use = "crate::autonodes::" + node_def.type + "::" + node_name;
    any_uses[any_use] = "";

    let use_buffer = "";
    usages: for (const usage in uses) {
        if (usage.match(/'static/)) {
            continue;
        }
        if (usage.match(/Operator$/) && !usage.match(/::operators::/)) {
            continue;
        }
        for (let decl of declares) {
            // Skip imports of structs declared in this file
            let rx = new RegExp("::" + decl + "\\b");
            if (usage.match(rx)) {
                continue usages;
            }
        }
        use_buffer += "use " + usage + ";\n";
    }
    use_buffer += "\n";
    node_buffer += "\n";

    fs.writeFileSync(`src/autonodes/${node_def.type}.rs`, use_buffer + child_enum_buffer + node_buffer + impl_buffer);
    mod_buffer += `pub mod ${node_def.type};\n`;
}
any_buffer += "}\n";

any_uses["crate::autotree::NodeAccess"] = "";
any_uses["crate::autotree::ParseError"] = "";
//any_uses["tree_sitter::Range"] = "";
any_uses["crate::parser::Range"] = "";
any_uses["tree_sitter::Node"] = "";

any_buffer += `

impl AnyNode {
    pub fn kind(&self) -> &'static str {
        self.as_any().kind()
    }

    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            // "comment" => 
            // "text_interpolation" => 
${Object.values(node_defs).map(node_def => {
    const name = get_rust_type_name(node_def.type);
    if (name.match(/'static/)) {
        return "";
    }
    if (!node_def.named) {
        return "";
    }
    let tname = get_rust_enum_name(node_def.type);
    return `"${node_def.type}" => AnyNode::${tname}(Box::new(${name}::parse(node, source)?)),`
}).join("\n")}
            _ => return Err(ParseError::new(node.range(), format!("Unknown node kind {}", node.kind()))),
        })
    }

    pub fn parse_vec<'a, I>(children: I, source: &Vec<u8>) -> Result<Vec<Self>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
    {
        let mut res: Vec<Self> = vec![];
        for child in children {
            res.push(Self::parse(child, source)?);
        }
        Ok(res)
    }
}

impl NodeAccess for AnyNode {

    fn brief_desc(&self) -> String {
        match self {
        ${Object.values(node_defs).map(node_def => {
    const name = get_rust_type_name(node_def.type);
    if (name.match(/'static/)) {
        return "";
    }
    if (!node_def.named) {
        return "";
    }
    let tname = get_rust_enum_name(node_def.type);
    return `AnyNode::${tname}(x) => x.brief_desc(),`
}).join("\n")}
        }
    }

    fn range(&self) -> Range {
        match self {
${Object.values(node_defs).map(node_def => {
    const name = get_rust_type_name(node_def.type);
    if (name.match(/'static/)) {
        return "";
    }
    if (!node_def.named) {
        return "";
    }
    let tname = get_rust_enum_name(node_def.type);
    return `AnyNode::${tname}(x) => x.range(),`
}).join("\n")}
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
${Object.values(node_defs).map(node_def => {
    const name = get_rust_type_name(node_def.type);
    if (name.match(/'static/)) {
        return "";
    }
    if (!node_def.named) {
        return "";
    }
    let tname = get_rust_enum_name(node_def.type);
    return `AnyNode::${tname}(x) => x.as_any(),`
}).join("\n")}
        }    
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        todo!("NEKJ");
    }

}

`;

any_ref_buffer += `
}

impl <'a>  AnyNodeRef<'a> {
    pub fn kind(&self) -> &'static str {
        match self {
            AnyNodeRef::StaticExpr(x, _) => x,
            AnyNodeRef::Error(e) => e.kind(),
            AnyNodeRef::Operator(op) => op.kind(),
            ${any_names.map(any_name => `AnyNodeRef::${any_name}(n) => n.kind(),`).join("\n")}
        }
    }
}

impl <'a> NodeAccess for AnyNodeRef<'a> {

    fn brief_desc(&self) -> String {
        match self {
            AnyNodeRef::StaticExpr(x, _) => x.to_string(),
            AnyNodeRef::Error(e) => e.brief_desc(),
            AnyNodeRef::Operator(op) => op.brief_desc(),
            ${any_names.map(any_name => `AnyNodeRef::${any_name}(n) => n.brief_desc(),`).join("\n")}
        }
    }

    fn range(&self) -> Range {
        match self {
            AnyNodeRef::StaticExpr(_, r) => *r,
            AnyNodeRef::Error(e) => e.range(),
            AnyNodeRef::Operator(op) => op.range(),
            ${any_names.map(any_name => `AnyNodeRef::${any_name}(n) => n.range(),`).join("\n")}
        }
    }

    fn as_any<'b>(&'b self) -> AnyNodeRef<'b> {
        self.clone()
    }

    fn children_any<'b>(&'b self) -> Vec<AnyNodeRef<'b>> {
        match self {
            AnyNodeRef::StaticExpr(_,_) => vec!(),
            AnyNodeRef::Error(e) => e.children_any(),
            AnyNodeRef::Operator(op) => op.children_any(),
            ${any_names.map(any_name => `AnyNodeRef::${any_name}(n) => n.children_any(),`).join("\n")}
        }
    }

}

`;
for (let usage in any_uses) {
    any_ref_buffer = "use " + usage + ";\n" + any_ref_buffer;
}

fs.writeFileSync('src/autonodes/any.rs', any_ref_buffer + any_buffer);
fs.writeFileSync(`src/autonodes/mod.rs`, mod_buffer);
