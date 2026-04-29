use serde::Deserialize;

#[derive(Deserialize)]
pub struct MainConfig {
    pub cluster_json_path: String,
    pub input_path: String,
}