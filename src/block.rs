#![allow(dead_code)]
use std::fmt::Display;

use crate::expression::{AExp, BExp, Label, Variable};

/// represents a single statement in a program
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Block {
    Assignment(AssignmentBlock),
    Skip(SkipBlock),
    Test(TestBlock),
}
impl Block {
    pub fn get_label(&self) -> Label {
        match self {
            Self::Assignment(b) => b.label,
            Self::Skip(b) => b.label,
            Self::Test(b) => b.label,
        }
    }

    pub fn assignment(label: Label, var: Variable, expr: AExp) -> Self {
        Self::Assignment(AssignmentBlock { label, var, expr })
    }
    pub fn skip(label: Label) -> Self {
        Self::Skip(SkipBlock { label })
    }
    pub fn test(label: Label, expr: BExp) -> Self {
        Self::Test(TestBlock { label, expr })
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct AssignmentBlock {
    pub label: Label,
    pub var: Variable,
    pub expr: AExp,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct SkipBlock {
    pub label: Label,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct TestBlock {
    pub label: Label,
    pub expr: BExp,
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]^{}",
            match self {
                Block::Assignment(AssignmentBlock { var, expr, .. }) => {
                    [var, " := ", format!("{}", expr).as_str()].concat()
                }

                Block::Skip(_) => "skip".to_string(),

                Block::Test(TestBlock { expr, .. }) => format!("{}", expr),
            },
            self.get_label()
        )
    }
}
