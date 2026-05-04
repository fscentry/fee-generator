use evaluator_rs::Value;

pub fn as_bool(val: Value) -> bool {
    match val {
        Value::Bool(b) => b,
        _ => false,
    }
}

pub fn as_f64(val: Value) -> f64 {
    match val {
        Value::Number(n) => n,
        _ => 0.0
    }
}

pub fn as_string(val: Value) -> String {
    match val {
        Value::String(s) => s,
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        evaluator_rs::Value::Array(_) => todo!(),
    }
}