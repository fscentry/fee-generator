pub mod models;
pub mod utils;
pub mod services;
mod constants;

use crate::services::generators::generate_fee;
use crate::utils::{
    clusters_loader::get_clusters, configuration_loader::get_config,
    input_loader::load_file_input_txt,
};
use crate::utils::clusters_loader::get_cluster_fee;

pub fn test_run() {
    let config = get_config();
    let clusters = get_clusters(&config.cluster_json_path);
    let clusters_fee =  get_cluster_fee(&config.cluster_fee_json_path);
    let input = load_file_input_txt(&config.input_path);

    println!("Total Clusters: {}", clusters.clusters.len());
    println!("Total Sub-Clusters: {}", clusters.sub_clusters.len());
    // println!("{:?}", clusters.sub_clusters.get("sub_cluster").unwrap().get(0).unwrap().expr);

    println!("---------------------------------");
    //get cluster
    if let Ok(vec_data) = input.as_ref() {
        if let Some(first_val) = vec_data.first() {
            generate_fee(clusters, first_val, clusters_fee);
        } else {
            println!("Vec kosong!");
        }
    }


}
