mod algorithm;
mod analysis;
mod block;
mod expression;
mod functions;
mod parser;
mod program;
mod statement;

use std::{env, io};

use crate::parser::parse;

// needed by the example program
// use crate::{
//     expression::{AExp::*, BExp::*},
//     program::Program,
//     statement::builder::StatementBuilder,
// };

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Enter statements here! An example program can be found at ./example_program");
    println!("To finish the program, press Ctrl+D");
    println!("To use an input file: Run {} < (input file name)", args[0]);

    let stdin = io::stdin();
    let mut input = String::new();
    let mut buf = String::new();
    while let Ok(count) = stdin.read_line(&mut buf) {
        if count == 0 {
            break;
        }

        input.push_str(&buf);
        buf.clear();
    }

    let program = parse(&input.trim());

    // uncomment this if for some reason the input doesn't work
    // let program = new(
    //     StatementBuilder::new(1)
    //         .assignment(0, Number(2))
    //         .assignment(1, Number(4))
    //         .assignment(0, Number(1))
    //         .begin_if(RelationalOp(Variable(1), ">".to_string(), Variable(0)))
    //         .assignment(2, Variable(1))
    //         .else_()
    //         .assignment(
    //             2,
    //             ArithmeticOp(
    //                 Box::new(Variable(1)),
    //                 "*".to_string(),
    //                 Box::new(Variable(1)),
    //             ),
    //         )
    //         .end_if()
    //         .assignment(0, Variable(2))
    //         .end(),
    // );

    println!("Program: {}", program);
    println!("Flow: {:?}", program.flow_r());
    println!();

    // let lva = algorithm::chaotic_iter::run(&program);
    let lva = algorithm::mfp::run(&program);

    for label in 1..=program.len {
        println!(
            "{label}: entry={:?}, exit={:?}",
            lva.entry[&label], lva.exit[&label],
        )
    }
}
