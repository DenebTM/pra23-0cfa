use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{
    block::AssignmentBlock,
    expression::{AExp, Value, Variable},
    program::Program,
};

#[derive(Parser)]
#[grammar = "parser/while.pest"]
pub struct WhileParser;

type ParseError = String;

fn parse_op(pair: Pair<'_, Rule>) -> String {
    match pair.as_rule() {
        Rule::AOp | Rule::BOp | Rule::ROp => pair.get_input().to_string(),

        _ => unreachable!(),
    }
}

fn parse_aexp(pair: Pair<'_, Rule>) -> Result<AExp, ParseError> {
    match pair.as_rule() {
        Rule::Var => {
            let inner = pair.into_inner().take(1).collect::<Vec<_>>();
            Ok(AExp::Variable(inner[0].as_str().to_string()))
        }

        Rule::Num => {
            let inner = pair.into_inner().take(1).collect::<Vec<_>>();
            Ok(AExp::Number(
                Value::from_str_radix(inner[0].as_str(), 10)
                    .map_err(|_| "Invalid integer".to_string())?,
            ))
        }

        Rule::AOp => {
            let inner = pair.into_inner().take(3).collect::<Vec<_>>();
            assert!(inner.len() == 3);

            let lhs = parse_aexp(inner[0].clone())?;
            let op = parse_op(inner[1].clone());
            let rhs = parse_aexp(inner[2].clone())?;

            Ok(AExp::ArithmeticOp(Box::new(lhs), op, Box::new(rhs)))
        }

        Rule::AAtom => {
            let inner = pair.into_inner().take(1).collect::<Vec<_>>();
            parse_aexp(inner[0].clone())
        }
        _ => unreachable!(),
    }
}

fn parse_ass(pair: Pair<'_, Rule>) -> Result<(Variable, AExp), ParseError> {
    let inner = pair.into_inner().take(2).collect::<Vec<_>>();
    assert!(inner.len() == 2);

    let var = inner[0].as_str().to_string();
    let expr = parse_aexp(inner[1].clone())?;

    Ok((var, expr))
}

fn parse_inner(pair: Pair<'_, Rule>) {
    match pair.as_rule() {
        // Rule::AExp => {
        //     parse_aexp(pair);
        // }
        Rule::Assignment => {
            let (var, expr) = parse_ass(pair).unwrap();

            println!("{var} := {expr}");
        }

        Rule::Statement | Rule::TermStmt => {
            for subpair in pair.into_inner() {
                parse_inner(subpair)
            }
        }

        _ => println!("{pair}"),
    }
}

pub fn parse(input: &str) -> Program {
    //

    let wp_res = WhileParser::parse(Rule::Statement, input);

    if let Ok(wp) = wp_res {
        for p in wp {
            parse_inner(p)
        }
    }

    todo!()
}
