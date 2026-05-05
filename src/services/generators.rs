use crate::models::clusters::{Cluster, Clusters};
use crate::utils::converter::as_bool;
use crate::utils::evaluators::evaluator_str;
use crate::utils::parser::extract_parameters;
use serde_json::Value;
use std::collections::HashMap;

pub fn generate_fee(clusters: &Clusters, input: &Value) {
    let cluster = find_cluster(clusters, input);
    println!("cluster {:?}", cluster);
}
pub fn find_cluster<'a>(clusters: &'a Clusters, input: &'a Value) -> Option<&'a Cluster> {
    let mut parameters: HashMap<&'a str, evaluator_rs::Value> = HashMap::new();
    let c = look_up(&clusters.clusters, input, &mut parameters);
    if let Some(cluster) = c {
        if let Some(sub_clusters) = clusters.sub_clusters.get(&cluster.id)
            && let Some(sc) = look_up(sub_clusters, input, &mut parameters)
        {
            return Some(sc);
        }
        return Some(cluster);
    }
    clusters.default.as_ref()
}

fn look_up<'a>(clusters: &'a [Cluster],input: &'a Value, parameters: &mut HashMap<&'a str, evaluator_rs::Value>) -> Option<&'a Cluster> {
    for c in clusters.iter() {
        if let (Some(expr), Some(rule)) = (&c.expr, &c.rules) {
            extract_parameters(rule, input, parameters);
            if as_bool(evaluator_str(expr, parameters)) {
                return Some(c);
            }
        }
    }
    None
}
