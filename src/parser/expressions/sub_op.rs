use crate::parser::expressions::expression::Expression;

#[derive(Debug)]
pub struct SubOp {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for SubOp {
    fn evaluate(&mut self) -> f64 {
        return self.left.evaluate() - self.right.evaluate();
    }
}