
extern crate evaluator_rs;
use std::collections::HashMap;
use evaluator_rs::{evaluate, parse_expr_from_str, Value};

pub fn evaluator_str(expression : &str, parameters : &HashMap<&str, Value>) -> Value {
    // from str expression
    let expr = parse_expr_from_str(expression).unwrap();
    let result = evaluate(&expr,parameters).unwrap();
    result
}