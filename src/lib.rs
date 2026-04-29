pub mod models;
pub mod utils;

use crate::utils::{
    clusters_loader::get_cluster, configuration_loader::get_config,
    input_loader::load_file_input_txt,
};
pub fn test_run() {
    let config = get_config();
    let clusters = get_cluster(&config.cluster_json_path);
    let input = load_file_input_txt(&config.input_path);

    println!("Total Clusters: {}", clusters.clusters.len());
    println!("Total Sub-Clusters: {}", clusters.sub_clusters.len());

    // example
    if let Some(first_cluster) = clusters.clusters.first() {
        println!(
            "First Cluster : {} (ID: {}) priority : {} , sub_cluster :  {:?}",
            first_cluster.name, first_cluster.id, first_cluster.priority, first_cluster.sub_cluster
        );
        println!("rules: {:?}", first_cluster.rules);
    }

    //check input
    match input {
        Ok(bodies) => {
            for (index, body) in bodies.iter().enumerate() {
                println!("Proses data ke-{}: {}", index, body);
            }
        }
        Err(error) => {
            eprintln!("Waduh, gagal load file: {}", error);
        }
    }
}
