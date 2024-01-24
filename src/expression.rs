use fmtastic::Superscript;
use std::{collections::HashSet, fmt::Display};

use crate::{
    constraint::{ConSet, Constraint},
    term::Term,
    types::{Label, Variable},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expression {
    pub label: usize,
    pub term: Term,
}

impl Expression {
    pub fn labels(&self) -> HashSet<Label> {
        self.subexprs().iter().map(|e| e.label).collect()
    }

    pub fn variables(&self) -> HashSet<Variable> {
        let mut variables = HashSet::new();

        match &self.term {
            Term::Constant(_) => {}

            Term::Variable(x) => {
                variables.insert(*x);
            }

            Term::Closure(x, e0) => {
                variables.insert(*x);
                variables.extend(e0.variables());
            }

            Term::RecursiveClosure(x, f, e0) => {
                variables.extend([x, f]);
                variables.extend(e0.variables());
            }

            Term::Application(e1, e2) => {
                variables.extend(e1.variables());
                variables.extend(e2.variables());
            }

            Term::IfThenElse(e0, e1, e2) => {
                variables.extend(e0.variables());
                variables.extend(e1.variables());
                variables.extend(e2.variables());
            }

            Term::Let(x, e1, e2) => {
                variables.insert(*x);
                variables.extend(e1.variables());
                variables.extend(e2.variables());
            }

            Term::BinaryOp(e1, _, e2) => {
                variables.extend(e1.variables());
                variables.extend(e2.variables());
            }
        }

        variables
    }

    pub fn constraints(&self) -> HashSet<Constraint> {
        self.constr(&self.subterms())
    }

    fn subexprs(&self) -> HashSet<&Expression> {
        let mut expressions = HashSet::from([self]);

        match &self.term {
            Term::Closure(_, e0) | Term::RecursiveClosure(_, _, e0) => {
                expressions.extend(e0.subexprs());
            }

            Term::Application(e1, e2) | Term::Let(_, e1, e2) | Term::BinaryOp(e1, _, e2) => {
                expressions.extend(e1.subexprs());
                expressions.extend(e2.subexprs());
            }

            Term::IfThenElse(e0, e1, e2) => {
                expressions.extend(e0.subexprs());
                expressions.extend(e1.subexprs());
                expressions.extend(e2.subexprs());
            }

            _ => {}
        }

        expressions
    }

    pub fn subterms(&self) -> HashSet<&Term> {
        self.subexprs().iter().map(|e| &e.term).collect()
    }

    fn constr(&self, subterms: &HashSet<&Term>) -> HashSet<Constraint> {
        let mut constraints: HashSet<Constraint> = HashSet::new();

        use ConSet::*;
        use Constraint::*;
        match &self.term {
            Term::Constant(_) => {}

            Term::Variable(x) => {
                constraints.insert(Unconditional(Env(*x), Cache(self.label)));
            }

            Term::Closure(_, e0) => {
                constraints.insert(Unconditional(
                    SingleTerm(self.term.clone()),
                    Cache(self.label),
                ));

                constraints.extend(e0.constr(subterms));
            }

            Term::RecursiveClosure(x, _, e0) => {
                constraints.extend([
                    Unconditional(SingleTerm(self.term.clone()), Cache(self.label)),
                    Unconditional(SingleTerm(self.term.clone()), Env(*x)),
                ]);

                constraints.extend(e0.constr(subterms));
            }

            Term::Application(e1, e2) => {
                constraints.extend(e1.constr(subterms));
                constraints.extend(e2.constr(subterms));

                subterms.iter().for_each(|&t| {
                    if let Term::Closure(x, e0) | Term::RecursiveClosure(x, _, e0) = t {
                        constraints.extend([
                            Conditional((t.clone(), Cache(e1.label)), Cache(e2.label), Env(*x)),
                            Conditional(
                                (t.clone(), Cache(e1.label)),
                                Cache(e0.label),
                                Cache(self.label),
                            ),
                        ]);
                    }
                });
            }

            Term::IfThenElse(e0, e1, e2) => {
                constraints.extend(e0.constr(subterms));
                constraints.extend(e1.constr(subterms));
                constraints.extend(e2.constr(subterms));

                constraints.extend([
                    Unconditional(Cache(e1.label), Cache(self.label)),
                    Unconditional(Cache(e2.label), Cache(self.label)),
                ]);
            }

            Term::Let(x, e1, e2) => {
                constraints.extend(e1.constr(subterms));
                constraints.extend(e2.constr(subterms));

                constraints.extend([
                    Unconditional(Cache(e1.label), Env(*x)),
                    Unconditional(Cache(e2.label), Cache(self.label)),
                ]);
            }

            Term::BinaryOp(e1, _, e2) => {
                constraints.extend(e1.constr(subterms));
                constraints.extend(e2.constr(subterms));
            }
        }

        constraints
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let use_parens = !matches!(self.term, Term::Constant(_) | Term::Variable(_));

        let inner = &self.term;
        let label = Superscript(self.label);

        if use_parens {
            let unpretty = format!("({inner}){label}");
            let level = f.width().unwrap_or(0);

            if f.alternate() && unpretty.len() + level >= 80 {
                if matches!(self.term, Term::IfThenElse(_, _, _) | Term::Let(_, _, _)) {
                    write!(
                        f,
                        "({inner:#level$}\n\
                    {pad:prev_level$}){label}",
                        pad = "",
                        prev_level = if level >= 4 { level - 4 } else { 0 }
                    )
                } else {
                    write!(f, "({inner:#level$}){label}",)
                }
            } else {
                write!(f, "{unpretty}")
            }
        } else {
            write!(f, "{inner}{label}")
        }
    }
}
