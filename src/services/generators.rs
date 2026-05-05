use std::collections::HashMap;
use crate::models::clusters::{Cluster, Clusters};
use crate::utils::parser::extract_parameters;
use crate::utils::evaluators::evaluator_str;
use serde_json::Value;
use crate::utils::converter::as_bool;

pub fn generate_fee(clusters: &Clusters, input: &Value){
    let cluster = get_cluster(clusters, input);
    println!("cluster: {:?}", cluster);
}
fn get_cluster<'a>(clusters: &'a Clusters, input: &'a Value) -> Option<&'a Cluster> {
    let mut parameters: HashMap<&'a str, evaluator_rs::Value> = HashMap::new();

    for c in clusters.clusters.iter() {
        if let (Some(expr), Some(rule)) = (&c.expr, &c.rules) {
            extract_parameters(rule, input, &mut parameters);
            if as_bool(evaluator_str(expr, &parameters)) {
                return Some(c);
            }
        }
    }
    None
}

