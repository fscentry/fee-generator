use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Cluster {
    pub name: String,
    pub id: String,
    pub priority: u32,
    pub sub_cluster: Option<String>,
    pub rules: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Clusters {
    pub clusters: Vec<Cluster>,
    #[serde(rename = "sub_clusters")]
    pub sub_clusters: Vec<Cluster>,
}