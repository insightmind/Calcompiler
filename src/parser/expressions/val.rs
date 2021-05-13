use crate::parser::expressions::expression::Expression;

#[derive(Debug)]
pub struct Val {
    pub value: f64,
}

impl Expression for Val {
    fn evaluate(&mut self) -> f64 {
        return self.value;
    }
}