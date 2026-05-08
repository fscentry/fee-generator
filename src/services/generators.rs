use crate::models::clusters::{Cluster, Clusters};
use crate::utils::converter::{as_bool, as_f64};
use crate::utils::evaluators::{evaluator_f64, evaluator_str};
use crate::utils::parser::{build_parameter_map};
use serde_json::Value;
use std::collections::HashMap;
use crate::constants::DEFAULT;
use crate::models::clusters_fee::{Calculation, ClusterFee, Rule};

pub fn generate_fee(clusters: &Clusters, input: &Value, fees : &HashMap<String, ClusterFee>) -> HashMap<String, f64> {
    let parameters= build_parameter_map(input);
    let borrowed: HashMap<&str, evaluator_rs::Value> = parameters
        .iter()
        .map(|(k, v)| (k.as_str(), v.clone()))
        .collect();

    let cluster = find_cluster(clusters, &borrowed);
    let fee = find_fee(&cluster.unwrap().id, fees, &borrowed);
    calculation(&fee.unwrap().calculation, &borrowed)
}
pub fn find_cluster<'a>(clusters: &'a Clusters, parameters: &HashMap<&str, evaluator_rs::Value>) -> Option<&'a Cluster> {
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
fn look_up<'a>(clusters: &'a [Cluster], parameters: &HashMap<&str, evaluator_rs::Value>) -> Option<&'a Cluster> {
    for c in clusters.iter() {
        if let Some(expr) = &c.expr && as_bool(evaluator_str(expr, parameters)) {
            return Some(c);
        }
    }
    None
}
fn find_fee<'a>(name : &str, fees : &'a HashMap<String, ClusterFee>, parameters: & HashMap<&str, evaluator_rs::Value >)-> Option<&'a Rule> {
    if let Some(f) = fees.get(name) {
        for r in f.rules.iter() {
            if let Some(expr) = &r.expr && as_bool(evaluator_str(expr, parameters)) {
                return Some(r);
            }
        }
    }
    /*Default*/
    fees.get(DEFAULT)?.rules.first()
}

fn calculation(calculation : &[Calculation], parameters: & HashMap<&str, evaluator_rs::Value >) -> HashMap<String, f64>{
    let mut results : HashMap<String, f64> = HashMap::new();
    for cal in calculation.iter() {
        if let Some(expr) = &cal.expr {
            let res = as_f64(evaluator_f64(expr, parameters));
            results.insert(cal.key.clone(), res);
        }
    }   
    results
}

