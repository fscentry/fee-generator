use std::collections::HashMap;
use evaluator_rs::{evaluate, parse_expr_from_str, Value};
use fee_generator::utils::{configuration_loader, clusters_loader};
use fee_generator::utils::converter::as_bool;

mod common;

#[test]
fn test_full_loading_flow() {
    common::setup();

    let config = configuration_loader::get_config();
    assert!(!config.cluster_json_path.is_empty(), "Path JSON cannot be empty!");
    let clusters = clusters_loader::get_cluster(&config.cluster_json_path);

    assert!(!clusters.clusters.is_empty(), "there must be at least one cluster!");
    assert_eq!(clusters.clusters[0].id, "c-normal");
}

#[test]
fn test_evaluator_logic(){
    let str = "({issuer} != {acquirer}) && {issuer} != {destination} && {code} in ['02','23','04','01']";
    let parameters = HashMap::from([
        ("issuer", Value::from("BNI")),
        ("acquirer", Value::from("BTN")),
        ("destination", Value::from("BCA")),
        ("code", Value::from("'01'")),
        ("decode", Value::from("'01'")),
    ]);
    println!("{:?}", &parameters);
    let expr = parse_expr_from_str(str).expect("Gagal parsing ekspresi");
    println!("{:?}", &expr);
    let is_valid = evaluate(&expr, &parameters)
        .map_or(false, |v| as_bool(v));

    assert!(is_valid, "Hasil evaluasi harus true");
}
