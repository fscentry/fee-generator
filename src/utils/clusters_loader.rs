use crate::models::clusters::Clusters;
use serde::de::DeserializeOwned;
use std::fs;
use std::sync::OnceLock;
use crate::models::clusters_fee::ClusterFee;

static CLUSTER_CACHE: OnceLock<Clusters> = OnceLock::new();
static CLUSTER_FEE_CACHE: OnceLock<Vec<ClusterFee>> = OnceLock::new();

pub fn get_cluster(path: &str) -> &'static Clusters {
    CLUSTER_CACHE.get_or_init(|| {
        extract::<Clusters>(path, String::from("Reading Configuration for Cluster from: "))
            .expect("Failed to initialize cluster cache")
    })
}

pub fn get_cluster_fee(path: &str) -> &'static Vec<ClusterFee> {
    CLUSTER_FEE_CACHE.get_or_init(|| {
        extract::<Vec<ClusterFee>>(path, String::from("Reading Configuration for Cluster Fee from: "))
            .expect("Failed to initialize cluster fee cache")
    })
}

fn extract<T>(path: &str, who : String) -> Result<T, serde_json::Error> where
    T: DeserializeOwned,
{
    println!("{} {}", who,path);
    let raw_data = fs::read_to_string(path).expect("Failed reading JSON File");
    serde_json::from_str(&raw_data)
}
