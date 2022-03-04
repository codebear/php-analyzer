use crate::{
    analysis::state::AnalysisState,
    autonodes::{object_creation_expression::{
        ObjectCreationExpressionChildren, ObjectCreationExpressionNode,
    }, arguments::ArgumentsNode},
    issue::IssueEmitter,
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
        if let DiscreteType::Named(_n, fq) = ctype {
            let class_data_handle = {
                let cdata = state.symbol_data.classes.read().unwrap();

                cdata.get(&fq)?.clone()
            };
            let _class_data = class_data_handle.read().unwrap();

            let arguments =
                if let Some(args) = &data.arguments {
                    Some(args.get_argument_values(state, emitter))
                } else {
                    None
                };
            // FIXME generics-analyse her eller i ObjectInstance
            //
            Some(PHPValue::ObjectInstance(ObjectInstance::new(
                fq.clone(),
                arguments,
            )))

            // crate::missing_none!("{}.get_php_value(..) fq: {:?}", self.kind(), &fq)
        } else {
            crate::missing_none!("{}.get_php_value(..) unknown type", self.kind())
        }
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
            Some(ObjectCreationExpressionChildren::QualifiedName(qn)) => Some(qn.get_fq_name()),
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
        if let Some(args) = data.arguments {
            let noe = args.get_argument_types(state, emitter);
        }
        
        let noe = state.symbol_data.get_class(&fq_name.clone().into());
        if let Some(x) = noe {
            let ctype = x.read().unwrap();
            if let Some(_x) = ctype.get_generic_templates() {
                if let Some(c) = ctype.get_constructor(state.symbol_data.clone()) {
                    for func_arg in &c.arguments {
                        eprintln!("ARG: {}: {:?}", func_arg.name, func_arg.get_type(state));
                    }
                    self.children;
                    todo!("Discover generic properties");
                }
            }
        }

        return Some(
            DiscreteType::Named(
                fq_name
                    .get_name()
                    .unwrap_or_else(|| -> Name { Name::new() }),
                fq_name,
            )
            .into(),
        );
    }
}
