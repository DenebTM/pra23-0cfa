use std::{collections::HashSet, fmt::Display};

use crate::{
    block::{AssignmentBlock, Block, SkipBlock, TestBlock},
    expression::Label,
    functions,
    statement::Statement,
};

/// encapsulates a sequence of `Statement`s starting at `1`
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Program {
    pub contents: Statement,
    pub len: usize,
}

#[allow(dead_code)]
impl Program {
    /// creates a new program, labelling all its statements sequentially
    pub fn new(contents: Statement) -> Self {
        let (contents, next) = Program::relabel(contents, 1);
        Self {
            contents,
            len: next - 1,
        }
    }

    /// returns the block at a given label in the program
    pub fn at(&self, label: Label) -> Option<Block> {
        Program::stmt_at(&self.contents, label)
    }

    pub fn init_label(&self) -> Label {
        1
    }
    pub fn final_labels(&self) -> HashSet<Label> {
        functions::final_labels(&self.contents)
    }
    pub fn flow(&self) -> HashSet<(Label, Label)> {
        functions::flow(&self.contents)
    }
    pub fn flow_r(&self) -> HashSet<(Label, Label)> {
        functions::flow_r(&self.contents)
    }
    pub fn blocks(&self) -> HashSet<Block> {
        functions::blocks(&self.contents)
    }

    /// relabels a statement and returns it together with a following label (internal use)
    fn relabel(stmt: Statement, start: Label) -> (Statement, Label) {
        match stmt {
            Statement::Atom(block) => (
                Statement::Atom(match block {
                    Block::Assignment(AssignmentBlock { var, expr, .. }) => {
                        Block::assignment(start.clone(), var, expr.clone())
                    }
                    Block::Skip(SkipBlock { .. }) => Block::skip(start.clone()),
                    Block::Test(TestBlock { expr, .. }) => Block::test(start.clone(), expr.clone()),
                }),
                start + 1,
            ),

            Statement::Sequence(stmt1, stmt2) => {
                let (new_stmt1, stmt2_start) = Program::relabel(*stmt1, start);
                let (new_stmt2, next) = Program::relabel(*stmt2, stmt2_start);

                (
                    Statement::Sequence(Box::new(new_stmt1), Box::new(new_stmt2)),
                    next,
                )
            }

            Statement::IfThenElse(test, stmt1, stmt2) => {
                let (new_test, stmt1_start) = (
                    TestBlock {
                        label: start,
                        expr: test.expr.clone(),
                    },
                    start + 1,
                );
                let (new_stmt1, stmt2_start) = Program::relabel(*stmt1, stmt1_start);
                let (new_stmt2, next) = Program::relabel(*stmt2, stmt2_start);

                (
                    Statement::IfThenElse(new_test, Box::new(new_stmt1), Box::new(new_stmt2)),
                    next,
                )
            }

            Statement::While(test, stmt1) => {
                let (new_test, stmt1_start) = (
                    TestBlock {
                        label: start,
                        expr: test.expr.clone(),
                    },
                    start + 1,
                );
                let (new_stmt1, next) = Program::relabel(*stmt1, stmt1_start);

                (Statement::While(new_test, Box::new(new_stmt1)), next)
            }

            Statement::Empty => (Statement::Empty, start),
        }
    }

    /// returns the block at a given label in the program (internal use)
    fn stmt_at(stmt: &Statement, label: Label) -> Option<Block> {
        match stmt {
            Statement::Atom(block) => {
                if block.get_label() == label {
                    return Some(block.clone());
                }

                None
            }

            Statement::Sequence(stmt1, stmt2) => {
                if let Some(block) = Program::stmt_at(stmt1, label) {
                    return Some(block);
                }
                if let Some(block) = Program::stmt_at(stmt2, label) {
                    return Some(block);
                }

                None
            }

            Statement::IfThenElse(test, stmt1, stmt2) => {
                if test.label == label {
                    return Some(Block::Test(test.clone()));
                }

                if let Some(block) = Program::stmt_at(stmt1, label) {
                    return Some(block);
                }
                if let Some(block) = Program::stmt_at(stmt2, label) {
                    return Some(block);
                }

                None
            }

            Statement::While(test, stmt1) => {
                if test.label == label {
                    return Some(Block::Test(test.clone()));
                }

                if let Some(block) = Program::stmt_at(stmt1, label) {
                    return Some(block);
                }

                None
            }

            Statement::Empty => None,
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.contents.fmt(f)
    }
}
