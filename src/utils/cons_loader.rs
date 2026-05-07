use std::collections::HashMap;
use std::fs;
use std::sync::OnceLock;
use crate::models::cons_reference::{DynamicValue};

static CONS_REFERENCE_CACHE: OnceLock<HashMap<String, DynamicValue>> = OnceLock::new();
pub fn get_cons_reference(path: &str) ->&'static HashMap<String, DynamicValue>{
    CONS_REFERENCE_CACHE.get_or_init(|| {
        let json = fs::read_to_string(path).expect("Failed reading Constant Reference file");
        serde_json::from_str(&json).unwrap()
    })
}