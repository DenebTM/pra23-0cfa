use peg::{self, error::ParseError, str::LineCol};

use crate::{
    types::{Constant, Label, Variable},
    Expression, Term,
};

fn expr(term: Term) -> Box<Expression> {
    Box::new(Expression { label: 0, term })
}

peg::parser!(grammar func() for str {
    rule alpha() -> char = ['a'..='z' | 'A'..='Z']
    rule digit() -> char = ['0'..='9']

    rule _ = [' ' | '\n']*

    rule constant() -> Constant
        = n:$("-"? digit()+) {? n.parse().or(Err("i32")) }
        / "(" _ c:constant() _ ")" { c }

    rule variable() -> Variable
        = x:alpha() { x }
        / "(" _ v:variable() _ ")" { v }

    rule closure() -> Term
        = "fn" _ x:variable() _ "->" _ t:term() {
            Term::Closure(x, expr(t))
        }
        / "(" _ c:closure() _ ")" { c }

    rule recursive_closure() -> Term
        = "fun" _ f:variable() _ x:variable() _ "->" _ t:term() {
            Term::RecursiveClosure(f, x, expr(t))
        }
        / "(" _ r:recursive_closure() _ ")" { r }

    #[cache_left_rec]
    rule application() -> Term
        = t1:term() _ t2:term() { Term::Application(expr(t1), expr(t2)) }
        / "(" _ a:application() _ ")" { a }

    rule if_then_else() -> Term
        = "if" t0:term() _ "then" _ t1:term() _ "else" _ t2:term() {
            Term::IfThenElse(expr(t0), expr(t1), expr(t2))
        }
        / "(" _ i:if_then_else() _ ")" { i }

    rule let() -> Term
        = "let" _ x:variable() _ "=" _ t1:term() _ "in" _ t2:term() {
            Term::Let(x, expr(t1), expr(t2))
        }
        / "(" _ l:let() _ ")" { l }

    #[cache_left_rec]
    rule add_sub() -> Term
        = t1:term() _ op:['+' | '-'] _ t2:term() { Term::BinaryOp(expr(t1), op, expr(t2)) }

    #[cache_left_rec]
    rule mul_div() -> Term
        = t1:term() _ op:['*' | '/'] _ t2:term() { Term::BinaryOp(expr(t1), op, expr(t2)) }

    rule binary_op() -> Term = precedence!{
        x:(@) _ op:['+' | '-'] _ y:@ { Term::BinaryOp(expr(x), op, expr(y)) }
        --
        x:(@) _ op:['*' | '/'] _ y:@ { Term::BinaryOp(expr(x), op, expr(y)) }
        --
        x:(@) _ op:['^'] _ y:@ { Term::BinaryOp(expr(x), op, expr(y)) }
        --
        n:constant() { Term::Constant(n) }
        v:variable() { Term::Variable(v) }
        "(" e:binary_op() ")" { e }
    }

    #[cache_left_rec]
    pub rule term() -> Term
        = _ t:(
            t:(
                closure()
                / recursive_closure()

                / application()
                / if_then_else()
                / let()

                / binary_op()
            ) { t }
        ) _? { t }
});

fn relabel(expr: Expression, start: Label) -> (Expression, Label) {
    match expr.term {
        Term::Closure(x, e1) => {
            let (new_e1, next) = relabel(*e1, start);

            (
                Expression {
                    term: Term::Closure(x, Box::new(new_e1)),
                    label: next,
                },
                next + 1,
            )
        }

        Term::RecursiveClosure(f, x, e1) => {
            let (new_e1, next) = relabel(*e1, start);

            (
                Expression {
                    term: Term::RecursiveClosure(f, x, Box::new(new_e1)),
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
    let term = func::term(input)?;

    Ok(relabel(*expr(term), 1).0)
}
