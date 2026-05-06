use fee_generator::services::generators::{find_cluster};
use fee_generator::utils::clusters_loader::{get_clusters};
use fee_generator::utils::configuration_loader::get_config;
use serde_json::Value;
use fee_generator::utils::parser::build_parameter_map;

#[test]
fn get_one_cluster() {
    let config = get_config();
    let clusters = get_clusters(&config.cluster_json_path);
    let raw = r#"{"issuer": "BNI","acquirer" : "MDR", "destination" : "-", "code" : "'01'", "channel" : 62000}"#;

    let input: Value = serde_json::from_str(raw).unwrap();
    let parameters= build_parameter_map(&input);
    let borrow = parameters
        .iter()
        .map(|(k, v)| (k.as_str(), v.clone()))
        .collect();
    let res = find_cluster(clusters, &borrow);
    assert!(res.is_some() && res.unwrap().id == "c-normal");
}

#[test]
fn get_default_cluster() {
    let config = get_config();
    let clusters = get_clusters(&config.cluster_json_path);
    let raw = r#"{"issuer": "BNI","acquirer" : "BNI", "destination" : "-", "code" : "'01'"}"#;
    let input: Value = serde_json::from_str(raw).unwrap();

    let parameters= build_parameter_map(&input);
    let borrow = parameters
        .iter()
        .map(|(k, v)| (k.as_str(), v.clone()))
        .collect();

    let res = find_cluster(clusters, &borrow);
    assert!(res.is_some() && res.unwrap().id == "default");
}


#[test]
fn get_sub_cluster() {
    let config = get_config();
    let clusters = get_clusters(&config.cluster_json_path);
    let raw = r#"{"issuer": "BNI","acquirer" : "BNI", "destination" : "-", "code" : "'01'", "channel" : 62001}"#;
    let input: Value = serde_json::from_str(raw).unwrap();
    let parameters= build_parameter_map(&input);
    let borrow = parameters
        .iter()
        .map(|(k, v)| (k.as_str(), v.clone()))
        .collect();
    let res = find_cluster(clusters, &borrow);
    assert!(res.is_some());
}


