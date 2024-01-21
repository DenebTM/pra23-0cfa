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
    let mut constraints: BTreeSet<Constraint> = BTreeSet::new();

    match &expr.term {
        Term::Constant => {}

        Term::Variable(x) => {
            constraints.insert((env[x].clone(), cache[&expr.label].clone()));
        }

        Term::Closure(x, e0) => {
            constraints.insert((
                BTreeSet::from([Term::Closure(*x, e0.clone())]),
                cache[&expr.label].clone(),
            ));

            constraints.append(&mut constr(e0, cache, env, top_expr));
        }

        Term::RecursiveClosure(x, f, e0) => {
            constraints.insert((
                BTreeSet::from([Term::RecursiveClosure(*x, *f, e0.clone())]),
                cache[&expr.label].clone(),
            ));
            constraints.insert((
                BTreeSet::from([Term::RecursiveClosure(*x, *f, e0.clone())]),
                env[x].clone(),
            ));
            constraints.append(&mut constr(e0, cache, env, top_expr));
        }

        Term::Application(e1, e2) => subterms(top_expr).iter().for_each(|t| {
            if let Term::Closure(x, e0) | Term::RecursiveClosure(x, _, e0) = t {
                if cache[&e1.label].contains(t) {
                    constraints.insert((cache[&e2.label].clone(), env[x].clone()));
                    constraints.insert((cache[&e0.label].clone(), cache[&expr.label].clone()));
                }
            }
        }),

        Term::IfThenElse(e0, e1, e2) => {
            constraints.append(&mut constr(e0, cache, env, top_expr));
            constraints.append(&mut constr(e1, cache, env, top_expr));
            constraints.append(&mut constr(e2, cache, env, top_expr));
            constraints.insert((cache[&e1.label].clone(), cache[&expr.label].clone()));
            constraints.insert((cache[&e2.label].clone(), cache[&expr.label].clone()));
        }

        Term::Let(x, e1, e2) => {
            constraints.append(&mut constr(e1, cache, env, top_expr));
            constraints.append(&mut constr(e2, cache, env, top_expr));
            constraints.insert((cache[&e1.label].clone(), env[x].clone()));
            constraints.insert((cache[&e2.label].clone(), cache[&expr.label].clone()));
        }

        Term::BinaryOp(e1, _, e2) => {
            constraints.append(&mut constr(e1, cache, env, top_expr));
            constraints.append(&mut constr(e2, cache, env, top_expr));
        }
    }

    constraints
}
