use std::fmt;

pub trait Expression: fmt::Debug {
    // Because our grammar is pretty simple we
    // allow generating the result directly by traversing throught the
    // expression tree.
    fn evaluate(&mut self) -> f64;
}