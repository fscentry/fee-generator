pub mod models;
pub mod utils;
pub mod services;

use crate::services::generators::generate_fee;
use crate::utils::{
    clusters_loader::get_cluster, configuration_loader::get_config,
    input_loader::load_file_input_txt,
};
use crate::utils::clusters_loader::get_cluster_fee;

pub fn test_run() {
    let config = get_config();
    let clusters = get_cluster(&config.cluster_json_path);
    let clusters_fee =  get_cluster_fee(&config.cluster_fee_json_path);
    let input = load_file_input_txt(&config.input_path);

    println!("Total Clusters: {}", clusters.clusters.len());
    println!("Total Sub-Clusters: {}", clusters.sub_clusters.len());

    /*
    example
    if let Some(first_cluster) = clusters.clusters.first() {
        println!(
            "First Cluster : {} (ID: {}) priority : {} , sub_cluster :  {:?}",
            first_cluster.name, first_cluster.id, first_cluster.priority, first_cluster.sub_cluster
        );
        println!("rules: {:?}", first_cluster.rules);
    }

    check input
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

    check fee
    println!("{:?}", clusters_fee.first().unwrap().rules.first().unwrap().calculation.first().unwrap());
   */
    println!("---------------------------------");
    //get cluster
    if let Ok(vec_data) = input.as_ref() {
        if let Some(first_val) = vec_data.first() {
            generate_fee(clusters, first_val);
        } else {
            println!("Vec kosong!");
        }
    }


}
