use std::{
    collections::{HashMap, HashSet},
    ops::Sub,
};

use crate::{
    block::{AssignmentBlock, Block, TestBlock},
    expression::{Label, Variable},
    program::Program,
};

pub fn gen_lv(block: Block) -> HashSet<Variable> {
    match block {
        Block::Assignment(AssignmentBlock { expr, .. }) => expr.free_vars(),
        Block::Test(TestBlock { expr, .. }) => expr.free_vars(),
        Block::Skip(_) => [].into(),
    }
}

pub fn kill_lv(block: Block) -> HashSet<Variable> {
    match block {
        Block::Assignment(AssignmentBlock { var, .. }) => [var].into(),
        Block::Test(TestBlock { .. }) => [].into(),
        Block::Skip(_) => [].into(),
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LVAnalysis {
    pub exit: LVExit,
    pub entry: LVEntry,
}
impl LVAnalysis {
    pub fn new(label_count: usize) -> Self {
        Self {
            exit: (1..=label_count)
                .map(|label| (label, HashSet::new()))
                .collect(),
            entry: (1..=label_count)
                .map(|label| (label, HashSet::new()))
                .collect(),
        }
    }
}

pub type LVExitAtLabel = HashSet<Variable>;
pub type LVExit = HashMap<Label, HashSet<Variable>>;
pub type LVEntryAtLabel = HashSet<Variable>;
pub type LVEntry = HashMap<Label, HashSet<Variable>>;

/// return the LVExit' mapping based on LVEntry
pub fn lv_exit(program: &Program, lv_entry: &LVEntry) -> LVExit {
    (1..=program.len)
        .map(|label| (label, lv_exit_at(program, lv_entry, label)))
        .collect()
}

/// return LVExit'(l) based on LVEntry
pub fn lv_exit_at(program: &Program, lv_entry: &LVEntry, label: Label) -> LVExitAtLabel {
    assert!(
        program.at(label) != None,
        "Label '{}' does not exist in program",
        label
    );

    if program.final_labels().contains(&label) {
        HashSet::new()
    } else {
        program
            .flow_r()
            .iter()
            .filter(|(_, l)| l == &label)
            .map(|(l_prime, _)| lv_entry[l_prime].clone())
            .flatten()
            .collect()
    }
}

/// return the LVEntry' mapping based on LVExit
pub fn lv_entry(program: &Program, lv_exit: &LVExit) -> LVExit {
    (1..=program.len)
        .map(|label| (label, lv_entry_at(program, lv_exit, label)))
        .collect()
}

/// return LVEntry'(l) based on LVExit
pub fn lv_entry_at(program: &Program, lv_exit: &LVExit, label: Label) -> LVEntryAtLabel {
    let block = program.at(label).unwrap();

    lv_exit[&label]
        .sub(&kill_lv(block.clone()))
        .union(&gen_lv(block))
        .cloned()
        .collect()
}
