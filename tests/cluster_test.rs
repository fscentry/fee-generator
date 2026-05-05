use fee_generator::services::generators::{find_cluster};
use fee_generator::utils::clusters_loader::{get_clusters};
use fee_generator::utils::configuration_loader::get_config;
use serde_json::Value;

#[test]
fn get_one_cluster() {
    let config = get_config();
    let clusters = get_clusters(&config.cluster_json_path);
    let raw = r#"{"issuer": "BNI","acquirer" : "MDR", "destination" : "-", "code" : "'01'"}"#;
    let input: Value = serde_json::from_str(raw).unwrap();
    let res = find_cluster(clusters, &input);
    assert!(res.is_some());
}

#[test]
fn get_default_cluster() {
    let config = get_config();
    let clusters = get_clusters(&config.cluster_json_path);
    let raw = r#"{"issuer": "BNI","acquirer" : "BNI", "destination" : "-", "code" : "'01'"}"#;
    let input: Value = serde_json::from_str(raw).unwrap();
    let res = find_cluster(clusters, &input);
    assert!(res.is_some() && res.unwrap().id == "default");
}
