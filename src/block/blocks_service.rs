use thiserror::Error;

use reqwest::Client;
use serde_json::Value;

use super::{block::Block, blocks_repository::BlocksRepository};

#[derive(Error, Debug)]
pub enum BlocksServiceError {
    #[error("Failed to fetch block: {0}")]
    FetchBlockError(#[from] reqwest::Error),
    #[error("Failed to create block: {0}")]
    CreateBlockError(#[from] sqlx::Error),
}

#[derive()]
pub struct BlocksService {
    repository: BlocksRepository,
    http_client: Client,
    rpc_url: String,
}

impl BlocksService {
    pub fn new(repository: BlocksRepository, client: Client, rpc_url: String) -> Self {
        Self {
            repository,
            http_client: client,
            rpc_url,
        }
    }

    pub async fn fetch_block(&self, block_number: u64) -> Result<Block, BlocksServiceError> {
        let body = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "eth_getBlockByNumber",
        "params": [format!("0x{:x}", block_number), false],
        "id": 1,
            });

        let response = self
            .http_client
            .post(&self.rpc_url)
            .json(&body)
            .send()
            .await?;
        let data = response.json::<Value>().await?;
        let result: Value = serde_json::from_value(data["result"].clone()).unwrap();
        let new_block = Block {
            id: String::from("test"),
            number: String::from(result["number"].as_str().unwrap()),
            data: result,
        };

        let result = self.repository.create(&new_block).await?;
        Ok(result)
    }

    pub async fn get_latest(&self) -> Result<Block, sqlx::Error> {
        self.repository.get_latest().await
    }

    pub async fn get_all(&self) -> Result<Vec<Block>, sqlx::Error> {
        self.repository.get_all().await
    }
}
