use std::{
    env,
    io::{self, IsTerminal},
};

use expression::Expression;
use rustyline::DefaultEditor;
use term::Term;

use crate::analysis::analyse;

mod analysis;
mod constraint;
mod expression;
mod parser;
mod term;
mod types;

fn main() {
    let args: Vec<String> = env::args().collect();

    let is_terminal = io::stdin().is_terminal();
    let mut rl = DefaultEditor::new().unwrap();
    let mut rl_prompt = ">>> ";

    if is_terminal {
        println!(
            "Enter statements here! Examples can be found in ./example1, ./example2 and ./example3"
        );
        println!("To finish the program, press Ctrl+D or enter a blank line.");
        println!("To use an input file, run: {} < (path/to/file)", args[0]);
    }

    let mut input = String::new();
    while let Ok(line) = rl.readline(rl_prompt) {
        if line.len() == 0 && is_terminal {
            break;
        }

        input.push_str(&line);
        input.push('\n');
        rl_prompt = "... ";
    }

    input = input.trim_end().to_string();
    if input.len() == 0 {
        return;
    }
    let program = parser::parse(&input);

    // uncomment this example program if the parser fails for whatever reason
    // let program = Expression {
    //     label: 5,
    //     term: Term::Application(
    //         Box::new(Expression {
    //             label: 2,
    //             term: Term::Closure(
    //                 'x',
    //                 Box::new(Expression {
    //                     label: 1,
    //                     term: Term::Variable('x'),
    //                 }),
    //             ),
    //         }),
    //         Box::new(Expression {
    //             label: 4,
    //             term: Term::Closure(
    //                 'y',
    //                 Box::new(Expression {
    //                     label: 3,
    //                     term: Term::Variable('y'),
    //                 }),
    //             ),
    //         }),
    //     ),
    // };

    // parse error -> print location of the error
    if let Err(err) = program {
        println!(
            "\nError parsing program at line {}, column {}:",
            err.location.line, err.location.column
        );

        let line = input.split('\n').take(err.location.line).last().unwrap();
        println!("{line}");
        println!("{:>col$}", "^", col = err.location.column);

        println!("Expected {}", err.expected);

        return;
    }

    // parsed successfully -> proceed with analysis
    let program = program.unwrap();
    println!("\nProgram:\n{program:#}");

    let labels = {
        let labels_unsorted = program.labels();
        let mut vec = Vec::from_iter(labels_unsorted);
        vec.sort();
        vec
    };
    let variables = {
        let variables_unsorted = program.variables();
        let mut vec = Vec::from_iter(variables_unsorted);
        vec.sort();
        vec
    };

    let constraints = program.constraints();
    println!("\nConstraints:");
    for constraint in &constraints {
        println!("  {constraint}");
    }

    println!("\nAnalysis:");
    let (analysis_cache, analysis_env) = analyse(&program, &constraints);
    for label in labels {
        let terms = analysis_cache[&label]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        println!(
            "  {rowlabel:<7} {}",
            terms.join(", "),
            rowlabel = format!("C({label}):")
        );
    }
    println!();
    for variable in variables {
        let terms = analysis_env[&variable]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        println!(
            "  {rowlabel:<7} {}",
            terms.join(", "),
            rowlabel = format!("r({variable}):")
        );
    }
    println!();
}
