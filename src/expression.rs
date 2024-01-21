use std::collections::HashSet;

use crate::{
    term::Term,
    types::{Label, Variable},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expression {
    pub label: usize,
    pub term: Term,
}

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

impl Expression {
    pub fn constraints(&self) -> HashSet<Constraint> {
        self.constr(&self)
    }

    fn subterms(&self) -> HashSet<Term> {
        let mut terms = HashSet::from([self.term.clone()]);

        match &self.term {
            Term::Closure(_, e0) | Term::RecursiveClosure(_, _, e0) => {
                terms.extend(e0.subterms());
            }

            Term::Application(e1, e2) | Term::Let(_, e1, e2) | Term::BinaryOp(e1, _, e2) => {
                terms.extend(e1.subterms());
                terms.extend(e2.subterms());
            }

            Term::IfThenElse(e0, e1, e2) => {
                terms.extend(e0.subterms());
                terms.extend(e1.subterms());
                terms.extend(e2.subterms());
            }

            _ => {}
        }

        terms
    }

    fn constr(&self, top_expr: &Expression) -> HashSet<Constraint> {
        let mut constraints: HashSet<Constraint> = HashSet::new();

        use ConSet::*;
        use Constraint::*;
        match &self.term {
            Term::Constant => {}

            Term::Variable(x) => {
                constraints.insert(Unconditional(Env(*x), Cache(self.label)));
            }

            Term::Closure(x, e0) => {
                constraints.insert(Unconditional(
                    SingleTerm(Term::Closure(*x, e0.clone())),
                    Cache(self.label),
                ));
                constraints.extend(e0.constr(top_expr));
            }

            Term::RecursiveClosure(x, f, e0) => {
                constraints.extend([
                    Unconditional(
                        SingleTerm(Term::RecursiveClosure(*x, *f, e0.clone())),
                        Cache(self.label),
                    ),
                    Unconditional(
                        SingleTerm(Term::RecursiveClosure(*x, *f, e0.clone())),
                        Env(*x),
                    ),
                ]);
                constraints.extend(e0.constr(top_expr));
            }

            Term::Application(e1, e2) => top_expr.subterms().iter().for_each(|t| {
                if let Term::Closure(x, e0) | Term::RecursiveClosure(x, _, e0) = t {
                    constraints.extend([
                        Conditional((t.clone(), Cache(e1.label)), Cache(e2.label), Env(*x)),
                        Conditional((t.clone(), Cache(e1.label)), Cache(e0.label), Env(*x)),
                    ]);
                }
            }),

            Term::IfThenElse(e0, e1, e2) => {
                constraints.extend(e0.constr(top_expr));
                constraints.extend(e1.constr(top_expr));
                constraints.extend(e2.constr(top_expr));
                constraints.extend([
                    Unconditional(Cache(e1.label), Cache(self.label)),
                    Unconditional(Cache(e2.label), Cache(self.label)),
                ]);
            }

            Term::Let(x, e1, e2) => {
                constraints.extend(e1.constr(top_expr));
                constraints.extend(e2.constr(top_expr));
                constraints.extend([
                    Unconditional(Cache(e1.label), Env(*x)),
                    Unconditional(Cache(e2.label), Cache(self.label)),
                ]);
            }

            Term::BinaryOp(e1, _, e2) => {
                constraints.extend(e1.constr(top_expr));
                constraints.extend(e2.constr(top_expr));
            }
        }

        constraints
    }
}
