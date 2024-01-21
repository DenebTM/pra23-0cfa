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

fn subterms(expr: &Expression) -> BTreeSet<Term> {
    let mut terms = BTreeSet::from([expr.term.clone()]);

    match &expr.term {
        Term::Closure(_, e0) | Term::RecursiveClosure(_, _, e0) => {
            terms.append(&mut subterms(&e0));
        }

        Term::Application(e1, e2) | Term::Let(_, e1, e2) | Term::BinaryOp(e1, _, e2) => {
            terms.append(&mut subterms(&e1));
            terms.append(&mut subterms(&e2));
        }

        Term::IfThenElse(e0, e1, e2) => {
            terms.append(&mut subterms(&e0));
            terms.append(&mut subterms(&e1));
            terms.append(&mut subterms(&e2));
        }

        _ => {}
    }

    terms
}

pub fn constr(
    expr: &Expression,
    cache: &AbstractCache,
    env: &AbstractEnv,
    top_expr: &Expression,
) -> BTreeSet<Constraint> {
    match &expr.term {
        Term::Constant => [].into(),

        Term::Variable(x) => [(env[x].clone(), cache[&expr.label].clone())].into(),

        Term::Closure(x, e0) => BTreeSet::from([(
            BTreeSet::from([Term::Closure(*x, e0.clone())]),
            cache[&expr.label].clone(),
        )])
        .union(&constr(e0, cache, env, top_expr))
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
        .union(&constr(e0, cache, env, top_expr))
        .cloned()
        .collect(),

        Term::Application(e1, e2) => subterms(top_expr)
            .iter()
            .map(|t| match t {
                Term::Closure(x, e0) | Term::RecursiveClosure(x, _, e0) => {
                    if cache[&e1.label].contains(t) {
                        Some(BTreeSet::from([
                            (cache[&e2.label].clone(), env[x].clone()),
                            (cache[&e0.label].clone(), cache[&expr.label].clone()),
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
            constr(e0, cache, env, top_expr),
            constr(e1, cache, env, top_expr),
            constr(e2, cache, env, top_expr),
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
            constr(e1, cache, env, top_expr),
            constr(e2, cache, env, top_expr),
            BTreeSet::from([
                (cache[&e1.label].clone(), env[x].clone()),
                (cache[&e2.label].clone(), cache[&expr.label].clone()),
            ]),
        ]
        .iter()
        .flatten()
        .cloned()
        .collect(),

        Term::BinaryOp(e1, _, e2) => constr(e1, cache, env, top_expr)
            .union(&constr(e2, cache, env, top_expr))
            .cloned()
            .collect(),
    }
}
