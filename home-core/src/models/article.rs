use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: u64,
    pub title: String,
    pub url: Option<String>,
    pub score: i64,
}
