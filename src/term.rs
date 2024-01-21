use std::fmt::Display;

use crate::{
    expression::Expression,
    types::{Operator, Variable},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Term {
    Constant(i32),
    Variable(Variable),
    Closure(Variable, Box<Expression>),
    RecursiveClosure(Variable, Variable, Box<Expression>),
    Application(Box<Expression>, Box<Expression>),
    IfThenElse(Box<Expression>, Box<Expression>, Box<Expression>),
    Let(Variable, Box<Expression>, Box<Expression>),
    BinaryOp(Box<Expression>, Operator, Box<Expression>),
}

impl Display for Term {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Constant(c) => write!(formatter, "{c}"),
            Self::Variable(x) => write!(formatter, "{x}"),
            Self::Closure(x, e0) => write!(formatter, "fn {x} -> {e0}"),
            Self::RecursiveClosure(x, f, e0) => write!(formatter, "fun {f} {x} -> {e0}"),
            Self::Application(e1, e2) => write!(formatter, "{e1} {e2}"),
            Self::IfThenElse(e0, e1, e2) => write!(formatter, "if {e0} then {e1} else {e2}"),
            Self::Let(x, e1, e2) => write!(formatter, "let {x} = {e1} in {e2}"),
            Self::BinaryOp(e1, op, e2) => write!(formatter, "{e1} {op} {e2}"),
        }
    }
}
