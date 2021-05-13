use crate::parser::expressions::expression::Expression;

pub struct DivOp {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for DivOp {
    // Nothing to implement
}