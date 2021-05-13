use crate::parser::expressions::expression::Expression;

pub struct MultOp {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for MultOp {
    // Nothing to implement
}