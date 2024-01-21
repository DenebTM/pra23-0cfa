use crate::term::Term;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expression {
    pub label: usize,
    pub term: Term,
}
