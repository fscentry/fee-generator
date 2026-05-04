
extern crate evaluator_rs;
use std::collections::HashMap;
use evaluator_rs::{evaluate, Expr, Value};

pub fn evaluator_str(expr: &Expr, parameters: &HashMap<&str, Value>) -> Value {
    evaluate(expr, parameters).unwrap()
}