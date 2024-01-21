use std::collections::{HashMap, HashSet};

use crate::{
    expression::Expression,
    term::Term,
    types::{Label, Variable},
};

pub type AbstractCache = HashMap<Label, HashSet<Term>>;
pub type AbstractEnv = HashMap<Variable, HashSet<Term>>;

pub type Analysis = (AbstractCache, AbstractEnv);

pub fn algo(expr: &Expression) -> (AbstractCache, AbstractEnv) {
    let mut work_list: Vec<()> = Vec::new();

    todo!()
}
