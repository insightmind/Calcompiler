use crate::parser::expressions::expression::Expression;

pub struct SubOp {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for SubOp {
    // Nothing to implement
}