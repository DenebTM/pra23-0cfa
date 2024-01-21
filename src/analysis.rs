use std::collections::{BTreeSet, HashMap};

use crate::{
    expression::Expression,
    term::Term,
    types::{Label, Variable},
};

type AbstractCache = HashMap<Label, BTreeSet<Term>>;
type AbstractEnv = HashMap<Variable, BTreeSet<Term>>;

type Analysis = (AbstractCache, AbstractEnv);

type ConstraintLhs = BTreeSet<Term>;
type ConstraintRhs = BTreeSet<Term>;
type Constraint = (ConstraintLhs, ConstraintRhs);

fn subexprs() -> BTreeSet<Term> {
    todo!()
}

pub fn constr(expr: &Expression, cache: &AbstractCache, env: &AbstractEnv) -> BTreeSet<Constraint> {
    match &expr.term {
        Term::Constant => [].into(),

        Term::Variable(x) => [(env[x].clone(), cache[&expr.label].clone())].into(),

        Term::Closure(x, e0) => BTreeSet::from([(
            BTreeSet::from([Term::Closure(*x, e0.clone())]),
            cache[&expr.label].clone(),
        )])
        .union(&constr(e0, cache, env))
        .cloned()
        .collect(),

        Term::RecursiveClosure(x, f, e0) => BTreeSet::from([
            (
                BTreeSet::from([Term::RecursiveClosure(*x, *f, e0.clone())]),
                cache[&expr.label].clone(),
            ),
            (
                BTreeSet::from([Term::RecursiveClosure(*x, *f, e0.clone())]),
                env[x].clone(),
            ),
        ])
        .union(&constr(e0, cache, env))
        .cloned()
        .collect(),

        Term::Application(e1, e2) => subexprs()
            .iter()
            .map(|t| match t {
                Term::Closure(x, subexpr) | Term::RecursiveClosure(x, _, subexpr) => {
                    if cache[&e1.label].contains(t) {
                        Some(BTreeSet::from([
                            (cache[&e2.label].clone(), env[x].clone()),
                            (cache[&subexpr.label].clone(), cache[&expr.label].clone()),
                        ]))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .flatten()
            .flatten()
            .collect(),

        Term::IfThenElse(e0, e1, e2) => [
            constr(e0, cache, env),
            constr(e1, cache, env),
            constr(e2, cache, env),
            BTreeSet::from([
                (cache[&e1.label].clone(), cache[&expr.label].clone()),
                (cache[&e2.label].clone(), cache[&expr.label].clone()),
            ]),
        ]
        .iter()
        .flatten()
        .cloned()
        .collect(),

        Term::Let(x, e1, e2) => [
            constr(e1, cache, env),
            constr(e2, cache, env),
            BTreeSet::from([
                (cache[&e1.label].clone(), env[x].clone()),
                (cache[&e2.label].clone(), cache[&expr.label].clone()),
            ]),
        ]
        .iter()
        .flatten()
        .cloned()
        .collect(),

        Term::BinaryOp(e1, _, e2) => constr(e1, cache, env)
            .union(&constr(e2, cache, env))
            .cloned()
            .collect(),
    }
}
