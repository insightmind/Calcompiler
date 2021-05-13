use crate::parser::expressions::expression::Expression;

pub struct AddOp {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for AddOp {
    // Nothing to implement
}