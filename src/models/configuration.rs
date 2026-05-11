use serde::Deserialize;

#[derive(Deserialize)]
pub struct MainConfig {
    pub cluster_json_path: String,
    pub input_path: String,
    pub cluster_fee_json_path: String,
    pub cons_reference_path:String,
    pub output_path : String,
    pub backup : i64,
    pub backup_path : String,
    pub data_key : String,
}