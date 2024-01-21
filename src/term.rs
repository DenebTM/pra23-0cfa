use crate::{
    expression::Expression,
    types::{Operator, Variable},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Term {
    Constant,
    Variable(Variable),
    Closure(Variable, Box<Expression>),
    RecursiveClosure(Variable, Variable, Box<Expression>),
    Application(Box<Expression>, Box<Expression>),
    IfThenElse(Box<Expression>, Box<Expression>, Box<Expression>),
    Let(Variable, Box<Expression>, Box<Expression>),
    BinaryOp(Box<Expression>, Operator, Box<Expression>),
}
