use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    issue::{Issue, IssueEmitter, IssuePosition},
    symbols::Name,
};

use super::{data::VarData, state::AnalysisState};

#[derive(Debug)]
pub struct Scope {
    pub vars: HashMap<Name, Arc<RwLock<VarData>>>,
    parent: Option<Arc<RwLock<Scope>>>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            vars: HashMap::new(),
            parent: None,
        }
    }

    pub fn has_var(&self, var_name: &Name) -> bool {
        match &self.parent {
            Some(p) => {
                let read = p.read().unwrap();
                if read.has_var(&var_name) {
                    return true;
                }
            }
            _ => (),
        }
        self.vars.get(var_name).is_some()
    }

    pub fn get_or_create_var(&mut self, var_name: Name) -> Arc<RwLock<VarData>> {
        if self.vars.contains_key(&var_name) {
            return self
                .vars
                .get(&var_name)
                .cloned()
                .expect("We just confirmed the entry is there");
        }
        match &self.parent {
            Some(p) => {
                let read = p.read().unwrap();
                if read.has_var(&var_name) {
                    return read.get_var(&var_name).expect("has_var returned true");
                }
            }
            _ => (),
        }

        self.get_or_create_local_var(var_name)
    }

    pub fn get_or_create_local_var(&mut self, var_name: Name) -> Arc<RwLock<VarData>> {
        if self.vars.get(&var_name).is_none() {
            self.vars.insert(
                var_name.clone(),
                Arc::new(RwLock::new(VarData::new(var_name.clone()))),
            );
        }

        self.vars
            .get(&var_name)
            .cloned()
            .expect("We just ensured this entry exists")
    }

    pub fn get_var(&self, var_name: &Name) -> Option<Arc<RwLock<VarData>>> {
        if self.vars.contains_key(&var_name) {
            return self.vars.get(&var_name).cloned();
        }
        match &self.parent {
            Some(p) => {
                let read = p.read().unwrap();
                if read.has_var(var_name) {
                    return read.get_var(var_name);
                }
            }
            _ => (),
        }

        self.vars.get(var_name).map(|x| x.clone())
    }
}
/*
impl std::fmt::Debug for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, data) in &self.vars {
            write!(f, "${}: ", name);
            let var_data = data.read().unwrap();
            write!(
                f,
                "comment_declared_type={:?},",
                var_data
                    .comment_declared_type
                    .as_ref()
                    .map(|x| x.to_string())
            );
            write!(
                f,
                "php_declared_type={:?},",
                var_data.php_declared_type.as_ref().map(|x| x.to_string())
            );
            let last_types: Vec<_> = var_data
                .last_written_data
                .iter()
                .cloned()
                .map(|x| x.0)
                .collect();
            write!(f, "last_types={:?}", last_types.iter().map(|x|x.to_string()).collect::<Vec<_>>());
            write!(f, "\n");
        }
        if let Some(p) = &self.parent {
            let read = p.read().unwrap();
            write!(f, "UP\n{:?}", *read);
        }
        Ok(())
        /*        f.debug_struct("Scope")
        .field("vars", &self.vars)
        .field("parent", &self.parent)
        .finish()*/
    }
}*/

pub trait BranchableScope {
    fn branch(&self) -> Arc<RwLock<Scope>>;

    fn join(&self, branches: Vec<Arc<RwLock<Scope>>>, emitter: &dyn IssueEmitter);

    fn analyze_for_unused_vars(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter);
}

#[derive(Clone, Copy, Debug)]
pub enum BranchSide {
    TrueBranch,
    FalseBranch,
}

impl BranchableScope for Arc<RwLock<Scope>> {
    fn branch(&self) -> Arc<RwLock<Scope>> {
        Arc::new(RwLock::new(Scope {
            vars: HashMap::new(),
            parent: Some(self.clone()),
        }))
    }

    fn join(&self, branches: Vec<Arc<RwLock<Scope>>>, _emitter: &dyn IssueEmitter) {
        let branch_count = branches.len();

        let mut vars = HashMap::new();
        // Gather all variables from all branches
        for b in branches {
            let scope = b.read().unwrap();
            for (var, data) in &scope.vars {
                let key = var.clone();
                let entry = vars.entry(key).or_insert_with(|| vec![]);
                entry.push(data.clone());
            }
        }
        {
            let mut write = self.write().unwrap();
            for (key, data) in &vars {
                let wr_ref = write.get_or_create_var(key.clone());
                let mut write_var = wr_ref.write().unwrap();
                let mut written_data: Vec<_> = vec![];
                let mut last_data: Vec<Vec<_>> = vec![];
                for e in data {
                    let reader = e.read().unwrap();
                    write_var.read_from += reader.read_from;
                    write_var.written_to += reader.written_to;
                    write_var.is_partial |= reader.is_partial;

                    written_data.extend(reader.all_written_data.iter().cloned());
                    last_data.push(reader.last_written_data.iter().cloned().collect());
                    //   eprintln!("Har data for var {:?}: {:?}", key, *reader);
                    // FIXME written_data
                }
                write_var.all_written_data.extend(written_data);
                if data.len() < branch_count {
                    write_var.is_partial = true;
                    write_var.last_written_data = vec![];
                } else {
                    let mut last_flattened = vec![];
                    for last in last_data {
                        last_flattened.extend(last);
                    }
                    write_var.last_written_data = last_flattened;
                    crate::missing!("Check this joining");
                    /*
                    let mut written_type = UnionType::new();
                    let mut has_missing_type = false;
                    let mut has_missing_data = false;
                    let mut written_values = vec![];
                    for written in last_data {
                        if let Some((dtype, data)) = written {
                            written_type.merge_into(dtype);
                            if let Some(dt) = data {
                                written_values.push(dt);
                            } else {
                                has_missing_data = true;
                            }
                        } else {
                            has_missing_type = true;
                            has_missing_data = true;
                        }
                    }

                    let written_value = if has_missing_data {
                        None
                    } else if written_values.len() > 1 {
                        crate::missing_none!("Found multiple values while joining scopes, check if they all are the same value")
                    } else if written_values.len() == 1 {
                        written_values.pop()
                    } else {
                        None
                    };

                    if has_missing_type {
                        // Stuff something in there
                        write_var
                            .all_written_data
                            .push((DiscreteType::Unknown.into(), None));
                    } else {
                        write_var.all_written_data.push((written_type, written_value));
                    }*/
                }
            }
        }
    }

    fn analyze_for_unused_vars(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        {
            let scope = self.read().unwrap();
            for (var_name, data_handle) in &scope.vars {
                let data = data_handle.read().unwrap();
                //  eprintln!("Checking var {:?}: {:?}", &var_name, &data);
                if data.read_from != 0 {
                    continue;
                }
                if !var_name.starts_with(b'_') {
                    for range in &data.referenced_ranges {
                        if data.is_argument {
                            emitter.emit(Issue::UnusedArgument(
                                IssuePosition::new(&state.filename, range.clone()),
                                var_name.clone(),
                            ));
                        } else {
                            emitter.emit(Issue::UnusedVariable(
                                IssuePosition::new(&state.filename, range.clone()),
                                var_name.clone(),
                            ));
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ScopeStack {
    pub stack: Vec<Arc<RwLock<Scope>>>,
}

impl ScopeStack {
    pub fn new() -> Self {
        Self {
            stack: vec![Arc::new(RwLock::new(Scope::new()))],
        }
    }

    pub fn top(&self) -> Arc<RwLock<Scope>> {
        self.stack
            .last()
            .cloned()
            .expect("There should always be a scope")
    }

    pub fn push(&mut self, scope: Arc<RwLock<Scope>>) {
        self.stack.push(scope);
    }

    pub fn pop(&mut self) -> Arc<RwLock<Scope>> {
        self.stack.pop().expect("Should always be a scope to pop")
    }
}

impl BranchSide {
    pub(crate) fn inverse(&self) -> BranchSide {
        match self {
            BranchSide::TrueBranch => BranchSide::FalseBranch,
            BranchSide::FalseBranch => BranchSide::TrueBranch,
        }
    }
}
