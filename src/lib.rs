pub mod models;
pub mod utils;
pub mod services;

use crate::services::generators::generate_fee;
use crate::utils::{
    clusters_loader::get_clusters, configuration_loader::get_config,
    input_loader::load_file_input_txt,
};
use crate::utils::clusters_loader::get_cluster_fee;

pub fn test_run() {
    let _config = get_config();
    let _clusters = get_clusters(&_config.cluster_json_path);
    let _clusters_fee =  get_cluster_fee(&_config.cluster_fee_json_path);
    let _input = load_file_input_txt(&_config.input_path);

    println!("Total Clusters: {}", _clusters.clusters.len());
    println!("Total Sub-Clusters: {}", _clusters.sub_clusters.len());
    
    println!("---------------------------------");
    //get cluster
    if let Ok(vec_data) = _input.as_ref() {
        if let Some(first_val) = vec_data.first() {
            generate_fee(_clusters, first_val);
        } else {
            println!("Vec kosong!");
        }
    }


}
