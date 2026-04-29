use std::fs;
use std::sync::OnceLock;
use crate::models::clusters::Clusters;

static CLUSTER_CACHE: OnceLock<Clusters> = OnceLock::new();
pub fn get_cluster(path: &str) -> &'static Clusters {
    CLUSTER_CACHE.get_or_init(|| {
        println!("Reading Configuration for Cluster from: {}", path);
        let raw_data = fs::read_to_string(path)
            .expect("Failed reading JSON File");

        serde_json::from_str(&raw_data)
            .expect("Failed Parse JSON")
    })
}