use evaluator_rs::Expr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterFee {
    pub cluster_id: String,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    pub name: String,
    pub rule: Option<String>,
    pub calculation: Vec<Calculation>,
    #[serde(skip)]
    pub expr: Option<Box<Expr>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Calculation {
    pub key: String,
    pub exp: String,
    #[serde(skip)]
    pub expr: Option<Box<Expr>>,
}

