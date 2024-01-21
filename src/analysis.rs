use std::collections::{HashMap, HashSet};

use crate::{
    expression::Expression,
    term::Term,
    types::{Label, Variable},
};

type AbstractCache = HashMap<Label, HashSet<Term>>;
type AbstractEnv = HashMap<Variable, HashSet<Term>>;

type Analysis = (AbstractCache, AbstractEnv);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConSet {
    Cache(Label),
    Env(Variable),
    SingleTerm(Term),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Constraint {
    Unconditional(ConSet, ConSet),
    Conditional((Term, ConSet), ConSet, ConSet),
}

fn subterms(expr: &Expression) -> HashSet<Term> {
    let mut terms = HashSet::from([expr.term.clone()]);

    match &expr.term {
        Term::Closure(_, e0) | Term::RecursiveClosure(_, _, e0) => {
            terms.extend(subterms(&e0));
        }

        Term::Application(e1, e2) | Term::Let(_, e1, e2) | Term::BinaryOp(e1, _, e2) => {
            terms.extend(subterms(&e1));
            terms.extend(subterms(&e2));
        }

        Term::IfThenElse(e0, e1, e2) => {
            terms.extend(subterms(&e0));
            terms.extend(subterms(&e1));
            terms.extend(subterms(&e2));
        }

        _ => {}
    }

    terms
}

pub fn constr(expr: &Expression, top_expr: &Expression) -> HashSet<Constraint> {
    let mut constraints: HashSet<Constraint> = HashSet::new();

    use ConSet::*;
    use Constraint::*;
    match &expr.term {
        Term::Constant => {}

        Term::Variable(x) => {
            constraints.insert(Unconditional(Env(*x), Cache(expr.label)));
        }

        Term::Closure(x, e0) => {
            constraints.insert(Unconditional(
                SingleTerm(Term::Closure(*x, e0.clone())),
                Cache(expr.label),
            ));
            constraints.extend(constr(e0, top_expr));
        }

        Term::RecursiveClosure(x, f, e0) => {
            constraints.extend([
                Unconditional(
                    SingleTerm(Term::RecursiveClosure(*x, *f, e0.clone())),
                    Cache(expr.label),
                ),
                Unconditional(
                    SingleTerm(Term::RecursiveClosure(*x, *f, e0.clone())),
                    Env(*x),
                ),
            ]);
            constraints.extend(constr(e0, top_expr));
        }

        Term::Application(e1, e2) => subterms(top_expr).iter().for_each(|t| {
            if let Term::Closure(x, e0) | Term::RecursiveClosure(x, _, e0) = t {
                constraints.extend([
                    Conditional((t.clone(), Cache(e1.label)), Cache(e2.label), Env(*x)),
                    Conditional((t.clone(), Cache(e1.label)), Cache(e0.label), Env(*x)),
                ]);
            }
        }),

        Term::IfThenElse(e0, e1, e2) => {
            constraints.extend(constr(e0, top_expr));
            constraints.extend(constr(e1, top_expr));
            constraints.extend(constr(e2, top_expr));
            constraints.extend([
                Unconditional(Cache(e1.label), Cache(expr.label)),
                Unconditional(Cache(e2.label), Cache(expr.label)),
            ]);
        }

        Term::Let(x, e1, e2) => {
            constraints.extend(constr(e1, top_expr));
            constraints.extend(constr(e2, top_expr));
            constraints.extend([
                Unconditional(Cache(e1.label), Env(*x)),
                Unconditional(Cache(e2.label), Cache(expr.label)),
            ]);
        }

        Term::BinaryOp(e1, _, e2) => {
            constraints.extend(constr(e1, top_expr));
            constraints.extend(constr(e2, top_expr));
        }
    }

    constraints
}
