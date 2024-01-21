use std::fmt::Display;

use crate::{
    term::Term,
    types::{Label, Variable},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConSet {
    Cache(Label),
    Env(Variable),
    SingleTerm(Term),
}

impl Display for ConSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cache(l) => write!(f, "C({l})"),
            Self::Env(x) => write!(f, "r({x})"),
            Self::SingleTerm(t) => write!(f, "{{{t}}}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Constraint {
    Unconditional(ConSet, ConSet),
    Conditional((Term, ConSet), ConSet, ConSet),
}

impl Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unconditional(lhs, rhs) => write!(f, "{lhs} ⊆ {rhs}"),
            Self::Conditional((t, rhs_), lhs, rhs) => {
                write!(f, "{{{t}}} ⊆ {rhs_} => {lhs} ⊆ {rhs}")
            }
        }
    }
}
