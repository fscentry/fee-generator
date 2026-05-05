use evaluator_rs::Value;
use regex::Regex;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::LazyLock;

static PARAM_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\{(?P<key>[^}]+)}").unwrap()
});

pub fn extract_parameters<'a>(template: &'a str,data: &'a JsonValue,parameters: &mut HashMap<&'a str, Value>) {

    for caps in PARAM_REGEX.captures_iter(template) {
        let key_str = caps.name("key").unwrap().as_str();

        match parameters.entry(key_str) {
            Entry::Occupied(_) => {}
            Entry::Vacant(entry) => {
                let found_value = data
                    .get(key_str)
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
                    entry.insert(eval_val);
                }
            }
        }
    }
}

fn get_by_dot_notation<'a>(data: &'a JsonValue, path: &str) -> Option<&'a JsonValue> {
    path.split('.').try_fold(data, |acc, key| acc.get(key))
}
