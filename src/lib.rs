pub mod parser;

pub mod config;
pub mod extra;
pub mod native;
pub mod phpdoc;
pub mod symbols;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

pub use tree_sitter::Point;
pub use tree_sitter::Range;
pub use tree_sitter::Tree;

pub mod analysis;
// mod nodes;
pub mod autonodes;
pub mod autotree;
pub mod description;
pub mod errornode;
pub mod issue;
pub mod nodeanalysis;
pub mod symboldata;
pub mod types;
pub mod value;

#[macro_use]
extern crate lazy_static;

extern crate nom;

static VERBOSE_MISSING: bool = false;

lazy_static! {
    static ref MISSING_FEATURES: Arc<RwLock<HashMap<String, usize>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

#[macro_export(local_inner_macros)]
macro_rules! missing {
    () => {
        {
            let s = std::format!("{}:{}: missing", std::file!(), std::line!());
            if crate::VERBOSE_MISSING {std::eprintln!("{}", &s);}
            *crate::MISSING_FEATURES.write().unwrap().entry(s).or_insert(0) += 1;
        }
    };
    ($($arg:tt)+) => {
        {
            let s = std::format!("{}:{}: missing: {}", std::file!(), std::line!(), std::format_args!($($arg)+));
            if crate::VERBOSE_MISSING { std::eprintln!("{}", &s);
        }
            *crate::MISSING_FEATURES.write().unwrap().entry(s).or_insert(0) += 1;
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! missing_none {
    () => {
        {
            let s = std::format!("{}:{}: missing", std::file!(), std::line!());
            if crate::VERBOSE_MISSING { std::eprintln!("{}", &s);
        }
            *crate::MISSING_FEATURES.write().unwrap().entry(s).or_insert(0) += 1;
            None
        }
    };
    ($($arg:tt)+) => {
        {
            let s = std::format!("{}:{}: missing: {}", std::file!(), std::line!(), std::format_args!($($arg)+));
            if crate::VERBOSE_MISSING { std::eprintln!("{}", &s); }
            *crate::MISSING_FEATURES.write().unwrap().entry(s).or_insert(0) += 1;
            None
        }
    };
}

pub fn dump_missing_stats() {
    eprintln!("...");
    let logged = MISSING_FEATURES.read().unwrap();
    let mut list: Vec<_> = logged.iter().collect();
    list.sort_by(|a, b| a.1.cmp(b.1));
    let len = list.len();
    if len == 0 {
        return;
    }
    for entry in list {
        eprintln!("{}: {}", entry.1, entry.0);
    }

    eprintln!(
        "Dumped {} missing implementations from {}:{}",
        len,
        file!(),
        line!()
    );
}
