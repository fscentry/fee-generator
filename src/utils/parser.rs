use evaluator_rs::Value;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

pub fn build_parameter_map(data: &JsonValue) -> HashMap<String, Value> {
    let mut output = HashMap::new();
    flatten_json("", data, &mut output);
    output
}

fn flatten_json(prefix: &str,value: &JsonValue,output: &mut HashMap<String, Value>) {
    match value {
        JsonValue::Object(map) => {
            for (key, val) in map {
                let full_key = if prefix.is_empty() {
                    key.to_string()
                } else {
                    format!("{prefix}.{key}")
                };

                flatten_json(&full_key, val, output);
            }
        }
        JsonValue::Number(n) => {
            output.insert(
                prefix.to_string(),
                Value::Number(n.as_f64().unwrap_or(0.0)),
            );
        }
        JsonValue::String(s) => {
            output.insert(
                prefix.to_string(),
                Value::String(s.clone()),
            );
        }
        JsonValue::Bool(b) => {
            output.insert(
                prefix.to_string(),
                Value::Bool(*b),
            );
        }

        _ => {}
    }
}