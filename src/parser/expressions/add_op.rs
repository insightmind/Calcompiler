use crate::parser::expressions::expression::Expression;

#[derive(Debug)]
pub struct AddOp {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for AddOp {
    fn evaluate(&mut self) -> f64 {
        return self.left.evaluate() + self.right.evaluate();
    }
}