
extern crate evaluator_rs;
use std::collections::HashMap;
use evaluator_rs::{evaluate, parse_expr_from_str, Value};

pub fn _evaluator_str(){

    // from str expression
    let expr = parse_expr_from_str("{a} + 30 + 100").unwrap();
    let parameters = HashMap::from([
        ("a", Value::from(30)),
    ]);
    println!("{:#?}", expr);
    println!("params : {:?}", &parameters);
    let rs = evaluate(&expr,&parameters).unwrap();
    println!("rs: {:?}", rs);



}