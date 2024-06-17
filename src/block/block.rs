use serde::{self, Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub id: String,
    pub number: String,
    #[serde(flatten)]
    pub data: Value,
}
