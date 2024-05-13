use std::collections::HashMap;

use crate::{
    analysis::state::AnalysisState,
    autonodes::{
        arguments::ArgumentsNode,
        object_creation_expression::{
            ObjectCreationExpressionChildren, ObjectCreationExpressionNode,
        },
    },
    issue::IssueEmitter,
    symboldata::class::ClassName,
    symbols::{FullyQualifiedName, Name},
    types::union::{DiscreteType, UnionType},
    value::{ObjectInstance, PHPValue},
};

#[derive(Debug, Clone)]
pub struct ObjectCreationData {
    name: Option<ObjectCreationExpressionChildren>,
    arguments: Option<Box<ArgumentsNode>>,
}

impl ObjectCreationExpressionNode {
    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        for child in &self.children {
            child.read_from(state, emitter)
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<crate::value::PHPValue> {
        let ctype = self.get_utype(state, emitter)?.single_type()?;
        let data = self.get_creation_data();

        let constructor_args = data
            .arguments
            .as_ref()
            .map(|args| args.get_argument_values(state, emitter));

        let (fq_name, generic_args) = match ctype {
            DiscreteType::Named(_n, fq) => (fq, None),
            DiscreteType::Generic(base_type, generic_template_concretes) => match &*base_type {
                DiscreteType::Named(_n, real_fq) => {
                    (real_fq.clone(), Some(generic_template_concretes))
                }
                _ => {
                    todo!(
                        "WHAT to do with generics: {} < {:?} >",
                        base_type,
                        generic_template_concretes
                    );
                }
            },
            _ => {
                eprintln!("WHASDF: {:#?}", ctype);
                return crate::missing_none!(
                    "{}.get_php_value(..) unknown type: {:?}",
                    self.kind(),
                    ctype
                );
            }
        };
        /*
                let class_data_handle = {
                    let cdata = state.symbol_data.classes.read().unwrap();

                    cdata.get(&fq_name)?.clone()
                };
                let _class_data = class_data_handle.read().unwrap();
        */

        // FIXME generics-analyse her eller i ObjectInstance
        //
        match generic_args {
            Some(generics) => {
                crate::missing!(
                    "Validate that generic args are within boundaries: {:?}",
                    generics
                );
                Some(PHPValue::ObjectInstance(ObjectInstance::new_with_generics(
                    fq_name.clone(),
                    generics,
                    constructor_args,
                )))
            }
            None => Some(PHPValue::ObjectInstance(ObjectInstance::new(
                fq_name.clone(),
                constructor_args,
            ))),
        }

        // crate::missing_none!("{}.get_php_value(..) fq: {:?}", self.kind(), &fq)
    }

    pub fn get_creation_data(&self) -> ObjectCreationData {
        let mut state = 0;
        let mut name: Option<ObjectCreationExpressionChildren> = None;
        let mut arguments: Option<Box<ArgumentsNode>> = None;
        for child in &self.children {
            match (state, &**child) {
                (0, ObjectCreationExpressionChildren::Name(n)) => {
                    state += 1;
                    name = Some(ObjectCreationExpressionChildren::Name(n.clone()));
                }
                (0, ObjectCreationExpressionChildren::QualifiedName(qn)) => {
                    state += 1;
                    name = Some(ObjectCreationExpressionChildren::QualifiedName(qn.clone()))
                }
                (0, ObjectCreationExpressionChildren::VariableName(v)) => {
                    state += 1;
                    name = Some(ObjectCreationExpressionChildren::VariableName(v.clone()))
                }
                (0, ObjectCreationExpressionChildren::SubscriptExpression(se)) => {
                    state += 1;
                    name = Some(ObjectCreationExpressionChildren::SubscriptExpression(
                        se.clone(),
                    ))
                }

                (1, ObjectCreationExpressionChildren::Arguments(a)) => {
                    state += 1;
                    arguments = Some(a.clone());
                }

                _ => crate::missing!(
                    "Unknown content {} in {} (state={})",
                    child.kind(),
                    self.kind(),
                    state
                ),
            }
        }
        ObjectCreationData { name, arguments }
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        let data = self.get_creation_data();
        let maybe_fq_name = match &data.name {
            Some(ObjectCreationExpressionChildren::Name(n)) => {
                let name = n.get_name();
                let fq_name = state.get_fq_symbol_name_from_local_name(&name);
                Some(fq_name)
            }
            Some(ObjectCreationExpressionChildren::QualifiedName(qn)) => {
                Some(qn.get_fq_name(state))
            }
            Some(ObjectCreationExpressionChildren::VariableName(vname)) => {
                let value = vname.get_php_value(state, emitter)?;
                match value {
                    PHPValue::String(vname_str) => Some(FullyQualifiedName::from(vname_str)),
                    _ => crate::missing_none!(
                        "get object-name from value of type: {:?}",
                        value.get_utype()
                    ),
                }
            }
            Some(noe) => {
                crate::missing_none!("get object-name from kind: {:?}", noe.kind())
            }
            None => {
                // FIXME should this emit somehow?
                return None;
            }
        };
        // println!("FQ_NAME: {:?}", maybe_fq_name);

        let fq_name = if let Some(fq_name) = maybe_fq_name {
            fq_name
        } else {
            return crate::missing_none!("{}.get_utype(..)", self.kind());
        };

        let name = fq_name
            .get_name()
            .unwrap_or_else(|| -> Name { Name::new() });

        if let Some(generic_type) = self
            .infer_generic_template_types_from_constructor(state, emitter, &name, &fq_name, &data)
        {
            return Some(generic_type);
        };

        let class_type = DiscreteType::Named(name, fq_name.clone());

        Some(class_type.into())
    }

    fn infer_generic_template_types_from_constructor(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        name: &Name,
        fq_name: &FullyQualifiedName,
        object_creation_data: &ObjectCreationData,
    ) -> Option<UnionType> {
        let fq_class_name: ClassName = fq_name.into();
        let shared_class_data = state.symbol_data.get_class(&fq_class_name)?;
        // FIXME improve structure here

        let ctype = shared_class_data.read().unwrap();
        let template_types = ctype.get_generic_templates()?;

        let mut inner_generic_map = HashMap::new();
        let constructor_data = ctype.get_constructor(state.symbol_data.clone())?;

        if let Some(args) = &object_creation_data.arguments {
            let in_arguments = args.get_argument_types(state, emitter);
            let mut in_argument_iter = in_arguments.iter();
            for func_arg in &constructor_data.arguments {
                let in_type = in_argument_iter.next().cloned();
                let arg_type = func_arg.get_type(state);
                match (in_type, arg_type) {
                    (Some(Some(in_type)), Some(arg_type)) => {
                        // void
                        if arg_type.contains_template() {
                            self.infer_generic_type_into_map(
                                &mut inner_generic_map,
                                arg_type,
                                in_type,
                            );
                        }
                    }
                    _ => (),
                }
                //eprintln!("ARG: {}: {:?}", func_arg.name, func_arg.get_type(state));
            }
        }

        let mut concrete_generic_args = vec![];
        for temp in template_types {
            if let Some(mapped_to) = inner_generic_map.get(&temp) {
                concrete_generic_args.push(mapped_to.clone());
            } else {
                concrete_generic_args.push(DiscreteType::Template(temp).into());
            }
        }
        let class_type = DiscreteType::Named(name.clone(), fq_name.clone());

        //eprintln!("{}<{:?}>", class_type, concrete_generic_args);

        Some(DiscreteType::Generic(Box::new(class_type), concrete_generic_args).into())
    }

    pub(crate) fn infer_generic_type_into_map(
        &self,
        generic_map: &mut HashMap<Name, UnionType>,
        templated_type: UnionType,
        in_type: UnionType,
    ) -> Option<()> {
        if templated_type.len() != 1 {
            crate::missing!("Handled of templated union types with multiple types");
            return None;
        }
        let template_type = templated_type.types.iter().next()?;

        match template_type {
            DiscreteType::Template(tname) => {
                generic_map.insert(tname.clone(), in_type);
            }
            DiscreteType::Generic(_base, _generics) => {
                crate::missing!("Handle templated generics");
            }
            _ => (),
        }
        None
    }
}
