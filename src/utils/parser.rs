use std::collections::HashMap;
use serde_json::Value as JsonValue;
use evaluator_rs::Value;
use regex::Regex;

pub fn extract_parameters(template: &str, data: &JsonValue) -> HashMap<&'static str, Value> {
    let mut parameters = HashMap::new();
    let re = Regex::new(r"\{(?P<key>[^}]+)}").unwrap();

    for caps in re.captures_iter(template) {
        let key_str = &caps["key"];

        let found_value = data.get(key_str)
            .or_else(|| data.pointer(key_str))
            .or_else(|| get_by_dot_notation(data, key_str));

        if let Some(v) = found_value {
            let eval_val = match v {
                JsonValue::Number(n) => {
                    Value::Number(n.as_f64().unwrap_or(0.0))
                }
                JsonValue::String(s) => {
                    Value::String(s.clone())
                }
                JsonValue::Bool(b) => {
                    Value::Bool(*b)
                }
                _ => continue,
            };

            let static_key: &'static str = Box::leak(key_str.to_string().into_boxed_str());
            parameters.insert(static_key, eval_val);
        }
    }
    parameters
}

fn get_by_dot_notation<'a>(data: &'a JsonValue, path: &str) -> Option<&'a JsonValue> {
    path.split('.').fold(Some(data), |acc, key| acc?.get(key))
}
