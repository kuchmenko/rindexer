use sqlx::PgPool;

use super::block::Block;

#[derive()]
pub struct BlocksRepository {
    db_pool: PgPool,
}

impl BlocksRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { db_pool: pool }
    }

    pub async fn create(&self, block: &Block) -> Result<Block, sqlx::Error> {
        let trimmed_number = block.number.trim_start_matches("0x");
        let block_number = i64::from_str_radix(&trimmed_number, 16).unwrap();
        let insert_query = sqlx::query!(
            "INSERT INTO blocks (number, data) VALUES ($1, $2) ON CONFLICT (number) DO NOTHING RETURNING id, number, data",
            block_number,
            serde_json::to_value(&block.data).unwrap()
        ).fetch_one(&self.db_pool).await?;

        Ok(Block {
            id: insert_query.id.to_string(),
            number: insert_query.number.to_string(),
            data: insert_query.data,
        })
    }

    pub async fn get_all(&self) -> Result<Vec<Block>, sqlx::Error> {
        let rows = sqlx::query!("SELECT * FROM blocks")
            .fetch_all(&self.db_pool)
            .await?;

        Ok(rows
            .iter()
            .map(|row| Block {
                id: row.id.to_string(),
                number: row.number.to_string(),
                data: row.data.clone(),
            })
            .collect())
    }

    pub async fn get_latest(&self) -> Result<Block, sqlx::Error> {
        let row = sqlx::query!("SELECT * FROM blocks ORDER BY number DESC LIMIT 1")
            .fetch_one(&self.db_pool)
            .await?;
        Ok(Block {
            id: row.id.to_string(),
            number: row.number.to_string(),
            data: row.data.clone(),
        })
    }
}
