use peg::{self, error::ParseError, str::LineCol};

use crate::{
    types::{Constant, Label, Variable},
    Expression, Term,
};

fn expr(term: Term) -> Box<Expression> {
    Box::new(Expression { label: 0, term })
}

peg::parser!(grammar func() for str {
    rule _ = [' ' | '\n']*
    rule ws_or_eof() = &(_ / ![_])
    rule alpha() -> char = ['a'..='z' | 'A'..='Z']
    rule digit() -> char = ['0'..='9']
    rule keyword() = "if" / "then" / "else" / "let" / "in"

    rule constant() -> Constant
        = n:$("-"? digit()+) {? n.parse().or(Err("i32")) }

    rule variable() -> Variable
        = !keyword() x:alpha() ws_or_eof() { x }

    rule closure() -> Term
        = "fn" _ x:variable() _ "->" _ t:term() {
            Term::Closure(x, expr(t))
        }

    rule recursive_closure() -> Term
        = "fun" _ f:variable() _ x:variable() _ "->" _ t:term() {
            Term::RecursiveClosure(f, x, expr(t))
        }

    rule if_then_else() -> Term
        = "if" t0:term() _ "then" _ t1:term() _ "else" _ t2:term() {
            Term::IfThenElse(expr(t0), expr(t1), expr(t2))
        }

    rule let() -> Term
        = "let" _ x:variable() _ "=" _ t1:term() _ "in" _ t2:term() {
            Term::Let(x, expr(t1), expr(t2))
        }

    rule binary_op() -> Term = precedence!{
        x:(@) _ "<"  _ y:@ { Term::BinaryOp(expr(x), "<" .to_string(), expr(y)) }
        x:(@) _ "<=" _ y:@ { Term::BinaryOp(expr(x), "<=".to_string(), expr(y)) }
        x:(@) _ "==" _ y:@ { Term::BinaryOp(expr(x), "==".to_string(), expr(y)) }
        x:(@) _ "!=" _ y:@ { Term::BinaryOp(expr(x), "!=".to_string(), expr(y)) }
        x:(@) _ ">=" _ y:@ { Term::BinaryOp(expr(x), ">=".to_string(), expr(y)) }
        x:(@) _ ">"  _ y:@ { Term::BinaryOp(expr(x), ">" .to_string(), expr(y)) }
        --
        x:(@) _ "+" _ y:@ { Term::BinaryOp(expr(x), "+".to_string(), expr(y)) }
        x:(@) _ "-" _ y:@ { Term::BinaryOp(expr(x), "-".to_string(), expr(y)) }
        --
        x:(@) _ "*" _ y:@ { Term::BinaryOp(expr(x), "*".to_string(), expr(y)) }
        x:(@) _ "/" _ y:@ { Term::BinaryOp(expr(x), "/".to_string(), expr(y)) }
        --
        x:(@) _ "^" _ y:@ { Term::BinaryOp(expr(x), "^".to_string(), expr(y)) }
        --
        n:constant() { Term::Constant(n) }
        v:variable() { Term::Variable(v) }
        "(" b:binary_op() ")" { b }
    }

    #[cache_left_rec]
    pub rule term() -> Term
        = _ t:precedence!{
                l:let() { l }
                --
                i:if_then_else() { i }
                --
                t1:@ _ t2:(@) { Term::Application(expr(t1), expr(t2)) }
                --
                c:closure() { c }
                r:recursive_closure() { r }
                --
                b:binary_op() { b }
                --
                "(" _ t:term() _ ")" { t }
            }
        _ { t }
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
