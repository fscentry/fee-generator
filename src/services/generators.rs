use crate::models::clusters::{Cluster, Clusters};
use crate::utils::converter::as_bool;
use crate::utils::evaluators::evaluator_str;
use crate::utils::parser::{build_parameter_map};
use serde_json::Value;
use std::collections::HashMap;
use crate::models::clusters_fee::ClusterFee;

pub fn generate_fee(clusters: &Clusters, input: &Value, _fees : &HashMap<String, ClusterFee>) {
    let parameters= build_parameter_map(input);
    let borrowed: HashMap<&str, evaluator_rs::Value> = parameters
        .iter()
        .map(|(k, v)| (k.as_str(), v.clone()))
        .collect();

    let cluster = find_cluster(clusters, &borrowed);
    println!("cluster {:?}", cluster);
}
pub fn find_cluster<'a>(clusters: &'a Clusters, parameters: &HashMap<&'a str, evaluator_rs::Value>) -> Option<&'a Cluster> {
    let c = look_up(&clusters.clusters, parameters);
    if let Some(cluster) = c {
        if let Some(sub_clusters) = clusters.sub_clusters.get(&cluster.id)
            && let Some(sc) = look_up(sub_clusters, parameters)
        {
            return Some(sc);
        }
        return Some(cluster);
    }
    clusters.default.as_ref()
}

fn look_up<'a>(clusters: &'a [Cluster], parameters: &HashMap<&'a str, evaluator_rs::Value>) -> Option<&'a Cluster> {
    for c in clusters.iter() {
        if let Some(expr) = &c.expr {
            if as_bool(evaluator_str(expr, parameters)) {
                return Some(c);
            }
        }
    }
    None
}

// fn find_fee<'a>(name : &str, fees : &HashMap<String, ClusterFee>, parameters: &mut HashMap<&'a str, evaluator_rs::Value >) {
//     if let Some(f) = fees.get(name) {
//         for r in f.rules.iter() {
//             if let Some(rr) = &r.rule {
//                 extract_parameters(rule, input, parameters);
//                 if as_bool(evaluator_str(expr, parameters)) {
//                     return Some(c);
//                 }
//             }
//         }
//     }
//
// }


