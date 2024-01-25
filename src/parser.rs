use peg::{self, error::ParseError, str::LineCol};

use crate::{
    types::{Constant, Label, Variable},
    Expression, Term,
};

fn expr(term: Term) -> Box<Expression> {
    Box::new(Expression { label: 0, term })
}

peg::parser!(grammar func() for str {
    rule __ = quiet!{ [' ' | '\n']+ }
    rule _  = quiet!{ [' ' | '\n']* }
    rule ws_or_eof() = &(_ / ![_])
    rule alpha() -> char = quiet!{ ['a'..='z' | 'A'..='Z'] }
    rule digit() -> char = quiet!{ ['0'..='9'] }
    rule neg() -> char = quiet!{ ['-'] }
    rule keyword() = quiet! { "fn" / "fun" / "if" / "then" / "else" / "let" / "in" }

    rule constant() -> Constant
        = n:$(neg()? digit()+) {? n.parse().or(Err("i32")) }
        / expected!("constant")

    rule variable() -> Variable
        = !keyword() x:alpha() ws_or_eof() { x }
        / expected!("variable")

    rule closure() -> Term
        = "fn" __ x:variable() _ ("->" / "=>") _ t:term() {
            Term::Closure(x, expr(t))
        }

    rule recursive_closure() -> Term
        = "fun" __ f:variable() __ x:variable() _ ("->" / "=>") _ t:term() {
            Term::RecursiveClosure(f, x, expr(t))
        }

    rule if_then_else() -> Term
        = "if" __ t0:term() __ "then" __ t1:term() __ "else" __ t2:term() {
            Term::IfThenElse(expr(t0), expr(t1), expr(t2))
        }

    rule let() -> Term
        = "let" __ x:variable() _ "=" _ t1:term() __ "in" __ t2:term() {
            Term::Let(x, expr(t1), expr(t2))
        }

    rule term() -> Term
        = _ t:precedence!{
                l:let() { l }
                --
                i:if_then_else() { i }
                --
                t1:@ __ t2:(@) { Term::Application(expr(t1), expr(t2)) }
                --
                c:closure() { c }
                r:recursive_closure() { r }
                --
                x:(@) _ op:$("||")  _ y:@ { Term::BinaryOp(expr(x), op.to_string(), expr(y)) }
                --
                x:(@) _ op:$("&&")  _ y:@ { Term::BinaryOp(expr(x), op.to_string(), expr(y)) }
                --
                x:(@) _ op:$("<=" / "==" / "!=" / ">=")  _ y:@ { Term::BinaryOp(expr(x), op.to_string(), expr(y)) }
                x:(@) _ op:$("<" / ">")  _ y:@ { Term::BinaryOp(expr(x), op.to_string(), expr(y)) }
                --
                x:(@) _ op:$("+" / "-") _ y:@ { Term::BinaryOp(expr(x), op.to_string(), expr(y)) }
                --
                x:(@) _ op:$("*" / "/") _ y:@ { Term::BinaryOp(expr(x), op.to_string(), expr(y)) }
                --
                n:constant() { Term::Constant(n) }
                v:variable() { Term::Variable(v) }
                --
                "(" _ t:term() _ ")" { t }
            }
        { t }

        pub rule program() -> Term = t:term() _ { t }
});

fn relabel(expr: Expression, start: Label) -> (Expression, Label) {
    match expr.term {
        Term::Closure(x, e0) => {
            let (new_e0, next) = relabel(*e0, start);

            (
                Expression {
                    term: Term::Closure(x, Box::new(new_e0)),
                    label: next,
                },
                next + 1,
            )
        }

        Term::RecursiveClosure(f, x, e0) => {
            let (new_e0, next) = relabel(*e0, start);

            (
                Expression {
                    term: Term::RecursiveClosure(f, x, Box::new(new_e0)),
                    label: next,
                },
                next + 1,
            )
        }

        Term::Application(e1, e2) => {
            let (new_e1, e2_start) = relabel(*e1, start);
            let (new_e2, next) = relabel(*e2, e2_start);

            (
                Expression {
                    term: Term::Application(Box::new(new_e1), Box::new(new_e2)),
                    label: next,
                },
                next + 1,
            )
        }

        Term::IfThenElse(e0, e1, e2) => {
            let (new_e0, e1_start) = relabel(*e0, start);
            let (new_e1, e2_start) = relabel(*e1, e1_start);
            let (new_e2, next) = relabel(*e2, e2_start);

            (
                Expression {
                    term: Term::IfThenElse(Box::new(new_e0), Box::new(new_e1), Box::new(new_e2)),
                    label: next,
                },
                next + 1,
            )
        }

        Term::Let(x, e1, e2) => {
            let (new_e1, e2_start) = relabel(*e1, start);
            let (new_e2, next) = relabel(*e2, e2_start);

            (
                Expression {
                    term: Term::Let(x, Box::new(new_e1), Box::new(new_e2)),
                    label: next,
                },
                next + 1,
            )
        }

        Term::BinaryOp(e1, op, e2) => {
            let (new_e1, e2_start) = relabel(*e1, start);
            let (new_e2, next) = relabel(*e2, e2_start);

            (
                Expression {
                    term: Term::BinaryOp(Box::new(new_e1), op, Box::new(new_e2)),
                    label: next,
                },
                next + 1,
            )
        }

        _ => (
            Expression {
                term: expr.term,
                label: start,
            },
            start + 1,
        ),
    }
}

pub fn parse(input: &str) -> Result<Expression, ParseError<LineCol>> {
    let program = func::program(input)?;

    Ok(relabel(*expr(program), 1).0)
}
