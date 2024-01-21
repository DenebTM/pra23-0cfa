use crate::term::Term;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Expression {
    pub label: usize,
    pub term: Term,
}
