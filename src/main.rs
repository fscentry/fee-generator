extern crate evaluator_rs;

use std::collections::HashMap;

use evaluator_rs::{evaluate, parse_expr_from_str, Value};


fn main() {
    // from str expression
    let expr = parse_expr_from_str("{a} + 30 + 100").unwrap();
    let parameters = HashMap::from([
        ("a", Value::from(30)),
    ]);
    println!("{:#?}", expr);
    println!("params : {:?}", &parameters);
    let rs = evaluate(&expr,&parameters).unwrap();
    println!("rs: {:?}", rs);

    // let expr = parse_expr_from_str("{a} in [1, 2 , 3]").unwrap();
    // let parameters = HashMap::from([("a", Value::from(1))]);
    // let rs = evaluate(&expr, &parameters).unwrap();
    // assert_eq!(rs, Value::from(true));
    //
    // // from json expression
    // let json_expr = r#"{
    //     "lhs": "{a}",
    //     "op": "in",
    //     "rhs": [4, 5, 6]
    // }"#;
    // let expr = parse_expr_from_json(json_expr).unwrap();
    // let parameters = HashMap::from([("a", Value::from(4))]);
    // let rs = evaluate(&expr, &parameters).unwrap();
    // assert_eq!(rs, Value::from(true));
}
