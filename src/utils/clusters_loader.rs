use std::fs;
use std::sync::OnceLock;
use crate::models::clusters::Clusters;
use crate::models::configuration::MainConfig;

static CLUSTER_CACHE: OnceLock<Clusters> = OnceLock::new();
pub fn get_cluster(config: &MainConfig) -> &'static Clusters {
    CLUSTER_CACHE.get_or_init(|| {
        println!("Reading Configuration for Cluster from: {}", &config.cluster_json_path);
        let raw_data = fs::read_to_string(&config.cluster_json_path)
            .expect("Failed reading JSON File");

        serde_json::from_str(&raw_data)
            .expect("Failed Parse JSON")
    })
}