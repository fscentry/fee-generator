use fee_generator::utils::{configuration_loader, clusters_loader};
mod common;

#[test]
fn test_full_loading_flow() {
    // 1. Setup
    common::setup();

    // 2. Test Configuration Loader (TOML)
    let config = configuration_loader::get_config();
    assert!(!config.cluster_json_path.is_empty(), "Path JSON cannot be empty!");

    // 3. Test Cluster Loader (JSON)
    let clusters = clusters_loader::get_cluster(&config.cluster_json_path);

    // Validation
    assert!(!clusters.clusters.is_empty(), "there must be at least one cluster!");
    assert_eq!(clusters.clusters[0].id, "c-normal");
}
