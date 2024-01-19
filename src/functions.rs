#![allow(dead_code)]

use std::collections::HashSet;

use crate::{block::Block, expression::Label, statement::Statement};

pub fn init_label(stmt: &Statement) -> Label {
    use crate::statement::Statement::*;
    match stmt {
        Atom(block) => block.get_label(),

        Sequence(stmt1, _) => init_label(stmt1),

        IfThenElse(test, _, _) => test.label,

        While(test, _) => test.label,

        Empty => panic!("An empty statement has no initial label"),
    }
}

pub fn final_labels(stmt: &Statement) -> HashSet<Label> {
    use crate::statement::Statement::*;
    match stmt {
        Atom(block) => [block.get_label()].into(),

        Sequence(_, stmt2) => final_labels(stmt2),

        IfThenElse(_, stmt1, stmt2) => final_labels(stmt1)
            .union(&final_labels(stmt2))
            .cloned()
            .collect(),

        While(test, _) => [test.label].into(),

        Empty => [].into(),
    }
}

pub fn blocks(stmt: &Statement) -> HashSet<Block> {
    use crate::statement::Statement::*;
    match stmt {
        // pad with empty sets so that all match arms have the return type [HashSet<(Label, Label)>; 3]
        Atom(block) => [[block.clone()].into(), HashSet::new(), HashSet::new()],

        Sequence(stmt1, stmt2) => [blocks(stmt1), blocks(stmt2), HashSet::new()],

        IfThenElse(test, stmt1, stmt2) => [
            [Block::Test(test.clone())].into(),
            blocks(stmt1),
            blocks(stmt2),
        ],

        While(test, stmt1) => [
            [Block::Test(test.clone())].into(),
            blocks(stmt1),
            HashSet::new(),
        ],

        Empty => [HashSet::new(), HashSet::new(), HashSet::new()],
    }
    .iter()
    .flatten()
    .cloned()
    .collect()
}

pub fn flow(stmt: &Statement) -> HashSet<(Label, Label)> {
    use crate::statement::Statement::*;
    match stmt {
        // pad with empty sets so that all match arms have the return type [HashSet<(Label, Label)>; 3]
        Atom(_) | Empty => [HashSet::new(), HashSet::new(), HashSet::new()],

        Sequence(stmt1, stmt2) => [
            // flow(S1) U flow(S2) ...
            flow(stmt1),
            flow(stmt2),
            // ... U {(l, init(S2)) | l in final(S1)}
            final_labels(stmt1)
                .iter()
                .map(|stmt1_final| (stmt1_final.clone(), init_label(stmt2)))
                .collect(),
        ],

        IfThenElse(test, stmt1, stmt2) => [
            // flow(S1) U flow(S2) ...
            flow(&stmt1),
            flow(&stmt2),
            // ... U {(l, init(S1)), (l, init(S2))}
            HashSet::from([
                (test.label, init_label(stmt1)),
                (test.label, init_label(stmt2)),
            ]),
        ],

        While(test, stmt1) => [
            // flow(S1) ...
            flow(stmt1),
            // ... U {(l, init(S))} ...
            [(test.label, init_label(stmt1))].into(),
            // ... U {(l, init(S2)) | l in final(S1)}
            final_labels(stmt1)
                .iter()
                .map(|stmt1_final| (stmt1_final.clone(), test.label))
                .collect(),
        ],
    }
    .iter()
    .flatten()
    .cloned()
    .collect()
}

pub fn flow_r(stmt: &Statement) -> HashSet<(Label, Label)> {
    flow(stmt).iter().map(|x| (x.1, x.0)).collect()
}
