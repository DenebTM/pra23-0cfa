use std::{env, io};

use expression::Expression;
use term::Term;

mod analysis;
mod constraint;
mod expression;
mod term;
mod types;

fn main() {
    // let args: Vec<String> = env::args().collect();

    // println!("Enter statements here! An example program can be found at ./example_program");
    // println!("To finish the program, press Ctrl+D");
    // println!("To use an input file: Run {} < (input file name)", args[0]);

    // let stdin = io::stdin();
    // let mut input = String::new();
    // let mut buf = String::new();
    // while let Ok(count) = stdin.read_line(&mut buf) {
    //     if count == 0 {
    //         break;
    //     }

    //     input.push_str(&buf);
    //     buf.clear();
    // }

    let program = Expression {
        label: 5,
        term: Term::Application(
            Box::new(Expression {
                label: 2,
                term: Term::Closure(
                    'x',
                    Box::new(Expression {
                        label: 1,
                        term: Term::Variable('x'),
                    }),
                ),
            }),
            Box::new(Expression {
                label: 4,
                term: Term::Closure(
                    'y',
                    Box::new(Expression {
                        label: 3,
                        term: Term::Variable('y'),
                    }),
                ),
            }),
        ),
    };
    println!("Program: {program}");

    println!("Constraints:");
    for constraint in program.constraints() {
        println!("  {constraint}");
    }
}
