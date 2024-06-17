mod block;

use std::env;

use block::{blocks_repository, blocks_service};
use dotenv::dotenv;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let infura_project_id = env::var("INFURA_PROJECT_ID").expect("INFURA_PROJECT_ID must be set");
    let client = Client::new();
    let rpc_url = format!("https://mainnet.infura.io/v3/{}", infura_project_id);

    let db_pool = sqlx::PgPool::connect(&database_url).await?;
    let blocks_repository = blocks_repository::BlocksRepository::new(db_pool);
    let blocks_service = blocks_service::BlocksService::new(blocks_repository, client, rpc_url);

    let latest_block = blocks_service.get_latest().await?;
    blocks_service
        .fetch_block(latest_block.number.parse::<u64>().unwrap() + 1)
        .await?;

    let all_blocks = blocks_service.get_all().await?;

    for block in all_blocks {
        println!("Block: {:?}", block);
    }

    Ok(())
}
