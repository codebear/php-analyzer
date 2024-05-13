pub mod class_declaration;
pub mod compound_statement;
pub mod function_definition;
pub mod member_access_expression;
pub mod member_call_expression;
pub mod method_declaration;
pub mod object_creation_expression;
pub mod scoped_call_expression;
pub mod variable_name;

use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;

use crate::autotree::NodeAccess;

pub trait NodeDescription {
    ///
    /// Find the first relevant node, and describe it

    fn description(
        &self,
        path: Option<&[AnyNodeRef]>,
        state: &mut AnalysisState,
    ) -> Option<String> {
        if let Some(path) = path {
            let mut last = path.len();
            for ancestor in path.iter().rev() {
                eprintln!("Klatrer: {}", ancestor.brief_desc());
                last -= 1;

                let path_to_him = &path[..last];
                if let Some(mut desc) = ancestor.describe_node(Some(path_to_him), state) {

                    desc.push_str(&format!("\n### Additional info:\nAST-Path: {}\n\n", path_vec_to_string(path_to_him)));
                    return Some(desc);
                }
                if ancestor.intersect_up_traversal() {
                    eprintln!("Avbryter desc-leting med en {}", ancestor.brief_desc());
                    return None;
                }
            }
            eprintln!("Søkte gjennom hele pathen og fant ingenting å describe");
            return None;
        }
        eprintln!("Hadde ikke noe path vector inn");
        self.describe_node(None, state)
    }
    ///
    /// Describe the current node
    fn describe_node(
        &self,
        _path: Option<&[AnyNodeRef]>,
        _state: &mut AnalysisState,
    ) -> Option<String> {
        None
    }

    fn intersect_up_traversal(&self) -> bool {
        false
    }
}

trait IntoNodeDescribable {
    fn with_describeable<T, CB>(&self, cb: Box<CB>) -> Option<T>
    where
        CB: FnMut(&dyn NodeDescription) -> T;
}

impl<'a> IntoNodeDescribable for AnyNodeRef<'a> {
    fn with_describeable<T, CB>(&self, mut cb: Box<CB>) -> Option<T>
    where
        CB: FnMut(&dyn NodeDescription) -> T,
    {
        match self {
            Self::VariableName(n) => Some(cb(*n)),
            Self::FunctionDefinition(f) => Some(cb(*f)),
            Self::MethodDeclaration(m) => Some(cb(*m)),
            Self::ClassDeclaration(c) => Some(cb(*c)),
            Self::ScopedCallExpression(n) => Some(cb(*n)),
            Self::CompoundStatement(n) => Some(cb(*n)),
            Self::MemberCallExpression(n) => Some(cb(*n)),
            Self::MemberAccessExpression(n) => Some(cb(*n)),
            Self::ObjectCreationExpression(o) => Some(cb(*o)),
            _ => None,
        }
    }
}

fn path_vec_to_string(path: &[AnyNodeRef]) -> String {
    let mut parts = vec![];
    for node in path {
        parts.push(node.kind());
    }
    parts.join(" -> ").to_string()
}

impl<'a> NodeDescription for AnyNodeRef<'a> {
    fn describe_node(
        &self,
        path: Option<&[AnyNodeRef]>,
        state: &mut AnalysisState,
    ) -> Option<String> {
        self.with_describeable(Box::new(|x: &dyn NodeDescription| {
            x.describe_node(path, state)
        }))?
    }

    fn intersect_up_traversal(&self) -> bool {
        if let Some(x) = self.with_describeable(Box::new(|x: &dyn NodeDescription| {
            x.intersect_up_traversal()
        })) {
            x
        } else {
            false
        }
    }
}
