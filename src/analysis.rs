use std::collections::{HashMap, HashSet};

use crate::{
    constraint::{ConSet, Constraint},
    expression::Expression,
    term::Term,
    types::{Label, Variable},
};

pub type AbstractCache = HashMap<Label, HashSet<Term>>;
pub type AbstractEnv = HashMap<Variable, HashSet<Term>>;

fn add(
    index: &ConSet,
    terms: HashSet<Term>,
    node_data: &mut HashMap<ConSet, HashSet<Term>>,
    work_list: &mut Vec<ConSet>,
) {
    if !terms.is_subset(&node_data[index]) {
        node_data.get_mut(index).unwrap().extend(terms);
        work_list.insert(0, index.clone());
    }
}

/**
 * expected to be called with `expr` and `expr.constraints()`
 *
 * this is so that the constraints can be obtained and used beforehand, e.g. for printing
 */
pub fn analyse(
    expr: &Expression,
    constraints: &HashSet<Constraint>,
) -> (AbstractCache, AbstractEnv) {
    let nodes: HashSet<ConSet> = expr
        .labels()
        .iter()
        .map(|l| ConSet::Cache(*l))
        .chain(expr.variables().iter().map(|x| ConSet::Env(*x)))
        .collect();

    // Step 1: Initialization
    let mut work_list: Vec<ConSet> = Vec::new();

    let (mut node_data, mut edges): (
        HashMap<ConSet, HashSet<Term>>,
        HashMap<ConSet, HashSet<&Constraint>>,
    ) = nodes
        .iter()
        .map(|q| ((q.clone(), HashSet::new()), (q.clone(), HashSet::new())))
        .unzip();

    // Step 2: Building the graph
    for constraint in constraints {
        use ConSet::*;
        use Constraint::*;
        match &constraint {
            Unconditional(p1, p2) => match p1 {
                SingleTerm(t) => add(
                    p2,
                    HashSet::from([t.clone()]),
                    &mut node_data,
                    &mut work_list,
                ),
                _ => {
                    edges.get_mut(p1).unwrap().insert(constraint);
                }
            },

            Conditional((_t, p), p1, _p2) => {
                edges.get_mut(p1).unwrap().insert(constraint);
                edges.get_mut(p).unwrap().insert(constraint);
            }
        }
    }

    // Step 3: Iteration
    while work_list.len() > 0 {
        let q = work_list.remove(0);

        for constraint in &edges[&q] {
            use Constraint::*;
            match &constraint {
                Unconditional(p1, p2) => {
                    add(
                        p2,
                        node_data.get(p1).unwrap().clone(),
                        &mut node_data,
                        &mut work_list,
                    );
                }

                Conditional((t, p), p1, p2) => {
                    if node_data[p].contains(t) {
                        add(p2, node_data[p1].clone(), &mut node_data, &mut work_list)
                    }
                }
            }
        }
    }

    // Step 4: Recording the solution
    let mut cache: AbstractCache = AbstractCache::new();
    let mut env: AbstractEnv = AbstractEnv::new();
    for (key, value) in node_data {
        use ConSet::*;
        match key {
            Cache(l) => cache.insert(l, value),
            Env(x) => env.insert(x, value),

            _ => panic!("Non-label/variable key in node_data: {:?}", key),
        };
    }

    (cache, env)
}
