pub mod models;
pub mod utils;
pub mod services;
mod constants;

use serde_json::json;
use crate::services::generators::generate_fee;
use crate::utils::{
    clusters_loader::get_clusters, configuration_loader::get_config,
    input_loader::load_file_input_txt,
};
use crate::utils::clusters_loader::get_cluster_fee;
use crate::utils::cons_loader::get_cons_reference;
use crate::utils::writer::write_into_file;

pub fn run_calculation() {
    let config = get_config();
    let cons_ref = get_cons_reference(&config.cons_reference_path);
    let clusters = get_clusters(&config.cluster_json_path);
    let clusters_fee =  get_cluster_fee(&config.cluster_fee_json_path,cons_ref);
    let input = load_file_input_txt(&config.input_path);


    println!("Total Cons Ref {}",cons_ref.len());
    println!("Total Clusters: {}", clusters.clusters.len());
    println!("Total Sub-Clusters: {}", clusters.sub_clusters.len());
    println!("Total Fees List: {}", clusters_fee.len());
    // println!("Total input: {}",  input.as_ref().map(|v| v.len()).unwrap_or(0));
    println!("---------------------------------");

    let mut results : Vec<String> = Vec::new();
    if let Ok(datas) = input {
        for data in datas.iter() {
            let rs = generate_fee(clusters, data, clusters_fee);
            let json_string = json!(rs).to_string();
            results.push(json_string);
        }
    }
    let file_name = String::from("results_fee");
    write_into_file(&config.output_path,&file_name , &results).expect("Failed to Write into file");

    /*test*/
    for result in &results {
        println!("{}", result);
    }
}
