use crate::parser::expressions::expression::Expression;

#[derive(Debug)]
pub struct DivOp {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for DivOp {
    fn evaluate(&mut self) -> f64 {
        return self.left.evaluate() / self.right.evaluate();
    }
}