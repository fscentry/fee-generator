use std::collections::HashMap;
use evaluator_rs::Expr;
use serde::{Deserialize};
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Cluster {
    pub name: String,
    pub id: String,
    pub priority: u32,
    pub sub_cluster: Option<String>,
    pub rules: Option<String>,
    /*ignored by serde*/
    #[derivative(Debug = "ignore")]
    pub expr: Option<Box<Expr>>,
}
#[derive(Deserialize)]
pub struct RawCluster {
    pub name: String,
    pub id: String,
    pub priority: u32,
    pub sub_cluster: Option<String>,
    pub rules: Option<String>,
}

#[derive(Debug)]
pub struct Clusters {
    pub clusters: Vec<Cluster>,
    pub sub_clusters: HashMap<String,Vec<Cluster>>,
    pub default : Option<Cluster>,
}

#[derive(Deserialize)]
pub struct RawClusters {
    pub clusters: Vec<RawCluster>,
    pub sub_clusters: Vec<RawCluster>
}