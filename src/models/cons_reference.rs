use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DynamicValue {
    String(String),
    Float(f64),
    Int(i64),
    Bool(bool),
}

impl DynamicValue {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            DynamicValue::Float(v) => Some(*v),
            _ => None,
        }
    }
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            DynamicValue::Int(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            DynamicValue::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            DynamicValue::Bool(v) => Some(*v),
            _ => None,
        }
    }
    pub fn to_str(&self) -> String {
        match self{
            DynamicValue::Int(x) => x.to_string(),
            DynamicValue::Float(x) => x.to_string(),
            DynamicValue::Bool(x) => x.to_string(),
            DynamicValue::String(x) => format!("\"{}\"", x),
        }
    }
}