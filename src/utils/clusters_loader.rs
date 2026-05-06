use std::collections::HashMap;
use crate::models::clusters::{Cluster, Clusters, RawCluster, RawClusters};
use serde::de::DeserializeOwned;
use std::fs;
use std::sync::OnceLock;
use evaluator_rs::{parse_expr_from_str};
use crate::models::clusters_fee::ClusterFee;
use crate::constants::DEFAULT;

static CLUSTER_CACHE: OnceLock<Clusters> = OnceLock::new();
static CLUSTER_FEE_CACHE: OnceLock<HashMap<String, ClusterFee>> = OnceLock::new();

pub fn get_clusters(path: &str) -> &'static Clusters {
    CLUSTER_CACHE.get_or_init(|| {

        let mut raw = extract::<RawClusters>(path, String::from("Reading Configuration for Cluster from: "))
            .expect("Failed to initialize cluster cache");

        let transform = |c: RawCluster| {
            let compiled_expr = c.rules.as_ref().map(|r| {
                parse_expr_from_str(r).expect("Failed to parse expression")
            });

            Cluster {
                name: c.name,
                id: c.id,
                priority: c.priority,
                sub_cluster: c.sub_cluster,
                rules: c.rules,
                expr: compiled_expr
            }
        };
        /*default*/
        let default_index = raw
            .clusters
            .iter()
            .position(|x| x.id == DEFAULT)
            .expect("default cluster not found");

        let default_cluster = transform(raw.clusters.remove(default_index));

        /*grouping sub-clusters*/
        let mut sub_clusters: HashMap<String, Vec<Cluster>> = HashMap::new();
        for item in raw.sub_clusters{
            let cluster = transform(item);
            if let Some(key) = cluster.sub_cluster.clone() {
                sub_clusters.entry(key).or_default().push(cluster);
            }
        }
        Clusters {
            clusters: raw.clusters.into_iter().filter(|item| item.id != DEFAULT ).map(transform).collect(),
            sub_clusters,
            default : Some(default_cluster),
        }
    })
}

pub fn get_cluster_fee(path: &str) -> &'static HashMap<String, ClusterFee> {
    CLUSTER_FEE_CACHE.get_or_init(|| {
        let raw_vec = extract::<Vec<ClusterFee>>(
            path,
            String::from("Reading Configuration for Cluster Fee from: ")
        ).expect("Failed to initialize cluster fee cache");

        raw_vec
            .into_iter()
            .map(|item| (item.cluster_id.clone(), item))
            .collect::<HashMap<String, ClusterFee>>()
    })
}

fn extract<T>(path: &str, who : String) -> Result<T, serde_json::Error> where
    T: DeserializeOwned,
{
    println!("{} {}", who,path);
    let raw_data = fs::read_to_string(path).expect("Failed reading JSON File");
    serde_json::from_str(&raw_data)
}
