#![allow(dead_code)]

pub mod builder;

use std::fmt::Display;

use crate::{
    block::{Block, TestBlock},
    expression::Label,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Statement {
    // Assignment(Assignment),
    // Skip(Skip),
    // Test(Test),
    /// \[X := a\], \[skip\], \[b\]
    Atom(Block),

    /// S1; S2
    Sequence(Box<Statement>, Box<Statement>),

    /// if \[b\] then S1 else S2
    IfThenElse(TestBlock, Box<Statement>, Box<Statement>),

    /// while \[b\] do S
    While(TestBlock, Box<Statement>),

    // represents an empty program
    Empty,
}

impl Statement {
    pub fn get_label(&self) -> Label {
        match self {
            Self::Atom(block) => block.get_label(),

            Self::Sequence(stmt1, _) => stmt1.get_label(),

            Self::IfThenElse(test, _, _) => test.label,

            Self::While(test, _) => test.label,

            // TODO: this isn't great
            Self::Empty => panic!("An empty statement has no label"),
        }
    }

    pub fn append(self, next: Statement) -> Statement {
        match self {
            Statement::Empty => next,
            Statement::Sequence(stmt1, stmt2) => {
                Statement::Sequence(stmt1, Box::new(stmt2.append(next)))
            }
            other_first => Statement::Sequence(Box::new(other_first), Box::new(next)),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Atom(block) => block.to_string(),

                Self::Sequence(stmt1, stmt2) => {
                    format!("{}; {}", stmt1, stmt2)
                }

                Self::IfThenElse(test, stmt1, stmt2) => {
                    format!(
                        "(if {} then {} else {})",
                        Block::Test(test.clone()),
                        stmt1,
                        stmt2
                    )
                }

                Self::While(test, stmt1) => {
                    format!("while ({}) do ({})", Block::Test(test.clone()), stmt1,)
                }

                Self::Empty => "".to_string(),
            }
        )
    }
}

pub mod boxed {
    use super::Statement;
    use crate::{
        block::{Block, TestBlock},
        expression::{AExp, BExp, Label, Variable},
    };

    pub fn assignment(label: Label, var: Variable, expr: AExp) -> Box<Statement> {
        Box::new(Statement::Atom(Block::assignment(label, var, expr)))
    }
    pub fn skip(label: Label) -> Box<Statement> {
        Box::new(Statement::Atom(Block::skip(label)))
    }
    pub fn test(label: Label, expr: BExp) -> Box<Statement> {
        Box::new(Statement::Atom(Block::test(label, expr)))
    }

    pub fn sequence(stmt1: Box<Statement>, stmt2: Box<Statement>) -> Box<Statement> {
        Box::new(Statement::Sequence(stmt1, stmt2))
    }

    pub fn if_then_else(
        test: TestBlock,
        stmt1: Box<Statement>,
        stmt2: Box<Statement>,
    ) -> Box<Statement> {
        Box::new(Statement::IfThenElse(test, stmt1, stmt2))
    }

    pub fn while_(test: TestBlock, stmt1: Box<Statement>) -> Box<Statement> {
        Box::new(Statement::While(test, stmt1))
    }
}
