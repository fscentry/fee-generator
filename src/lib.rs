pub mod utils;
pub mod models;

use crate::utils::clusters_loader::get_cluster;
use crate::utils::configuration_loader::get_config;

pub fn test_run(){
    let config = get_config();
    let clusters = get_cluster(&config);

    println!("Total Clusters: {}", clusters.clusters.len());
    println!("Total Sub-Clusters: {}", clusters.sub_clusters.len());

    // example
    if let Some(first_cluster) = clusters.clusters.get(0) {
        println!("First Cluster : {} (ID: {}) priority : {} , sub_cluster :  {:?}", first_cluster.name, first_cluster.id, first_cluster.priority, first_cluster.sub_cluster);
        println! ("rules: {:?}", first_cluster.rules);
    }
}


#[cfg(test)]
mod tests {


}