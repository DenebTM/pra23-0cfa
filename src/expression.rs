#![allow(dead_code)]
use std::{collections::HashSet, fmt::Display};

pub type Label = usize; // label index
pub type Variable = String; // variable index
pub type Value = i32; // an actual numeric value (only for displaying)

/// represents an arithmetic expression as it may appear in an assignment to a variable
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum AExp {
    // the index of a variable
    Variable(Variable),

    // the value of the number is irrelevant
    Number(Value),

    // + - * /; operator is irrelevant
    ArithmeticOp(Box<AExp>, String, Box<AExp>),
}
impl AExp {
    pub fn free_vars(&self) -> HashSet<Variable> {
        match self {
            AExp::Variable(var) => [var.clone()].into(),
            AExp::Number(_) => [].into(),
            AExp::ArithmeticOp(lhs, _, rhs) => [lhs.free_vars(), rhs.free_vars()]
                .iter()
                .flatten()
                .cloned()
                .collect(),
        }
    }
}

/// represents a boolean expression as it may appear (by itself) in a block
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum BExp {
    True,
    False,

    Not(Box<BExp>),

    // &&/||; operator is irrelevant
    BooleanOp(Box<BExp>, String, Box<BExp>),

    // > < ==; operator is irrelevant
    RelationalOp(AExp, String, AExp),
}
impl BExp {
    pub fn free_vars(&self) -> HashSet<Variable> {
        match self {
            BExp::True | BExp::False => [].into(),
            BExp::Not(inner) => inner.free_vars(),
            BExp::BooleanOp(lhs, _, rhs) => [lhs.free_vars(), rhs.free_vars()]
                .iter()
                .flatten()
                .cloned()
                .collect(),
            BExp::RelationalOp(lhs, _, rhs) => [lhs.free_vars(), rhs.free_vars()]
                .iter()
                .flatten()
                .cloned()
                .collect(),
        }
    }
}

impl Display for AExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AExp::Variable(var) => var.to_string(),

                AExp::Number(val) => val.to_string(),

                AExp::ArithmeticOp(lhs, op, rhs) =>
                    [lhs.to_string(), op.to_string(), rhs.to_string()].concat(),
            }
        )
    }
}

impl Display for BExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BExp::True => "true".to_string(),

                BExp::False => "false".to_string(),

                BExp::Not(val) => ["!(", &val.to_string(), ")"].concat(),

                BExp::BooleanOp(lhs, op, rhs) =>
                    [lhs.to_string(), op.to_string(), rhs.to_string()].concat(),

                BExp::RelationalOp(lhs, op, rhs) => [
                    lhs.to_string(),
                    " ".to_string(),
                    op.to_string(),
                    " ".to_string(),
                    rhs.to_string()
                ]
                .concat(),
            }
        )
    }
}
