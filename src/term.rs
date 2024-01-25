use std::fmt::Display;

use crate::{
    expression::Expression,
    types::{Constant, Operator, Variable},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Term {
    /// c
    Constant(Constant),
    /// `x`
    Variable(Variable),
    /// fn `x` -> `e0`
    Closure(Variable, Box<Expression>),
    /// fun `f` `x` -> `e0`
    RecursiveClosure(Variable, Variable, Box<Expression>),
    /// (`e1` `e2`)
    Application(Box<Expression>, Box<Expression>),
    /// if `e0` then `e1` else `e2`
    IfThenElse(Box<Expression>, Box<Expression>, Box<Expression>),
    /// let `x` = `e1` in `e2`
    Let(Variable, Box<Expression>, Box<Expression>),
    /// (`e1` `op` `e2`)
    BinaryOp(Box<Expression>, Operator, Box<Expression>),
}

impl Display for Term {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if formatter.alternate() {
            let level = formatter.width().unwrap_or(0);
            let sublevel = level + 4;
            let subsublevel = level + 8;
            match self {
                Self::Constant(c) => write!(formatter, "{c}"),
                Self::Variable(x) => write!(formatter, "{x}"),
                Self::Closure(x, e0) => write!(formatter, "fn {x} -> {e0:#sublevel$}"),
                Self::RecursiveClosure(f, x, e0) => write!(formatter, "fun {f} {x} -> {e0:#sublevel$}"),
                Self::Application(e1, e2) => write!(formatter, "{e1:#sublevel$} {e2:#sublevel$}"),
                Self::IfThenElse(e0, e1, e2) => write!(formatter, "if {e0:#sublevel$}\n\
                                                                                                                         {pad:sublevel$}then {e1:#subsublevel$}\n\
                                                                                                                         {pad:sublevel$}else {e2:#subsublevel$}", pad = ""),
                Self::Let(x, e1, e2) => write!(formatter, "let {x} = {e1:#sublevel$} \n\
                                                                                                      {pad:sublevel$}in {e2:#subsublevel$}", pad = ""),
                Self::BinaryOp(e1, op, e2) => write!(formatter, "{e1:#level$} {op}\n\
                                                                                                              {pad:level$}{e2:#level$}", pad = ""),
            }
        } else {
            match self {
                Self::Constant(c) => write!(formatter, "{c}"),
                Self::Variable(x) => write!(formatter, "{x}"),
                Self::Closure(x, e0) => write!(formatter, "fn {x} -> {e0}"),
                Self::RecursiveClosure(f, x, e0) => write!(formatter, "fun {f} {x} -> {e0}"),
                Self::Application(e1, e2) => write!(formatter, "{e1} {e2}"),
                Self::IfThenElse(e0, e1, e2) => write!(formatter, "if {e0} then {e1} else {e2}"),
                Self::Let(x, e1, e2) => write!(formatter, "let {x} = {e1} in {e2}"),
                Self::BinaryOp(e1, op, e2) => write!(formatter, "{e1} {op} {e2}"),
            }
        }
    }
}
