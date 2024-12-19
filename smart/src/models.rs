use std::sync::Arc;

use anyhow::Result;
use serde::Serialize;
use sqlx::SqlitePool;
use tokio::sync::OnceCell;

use crate::config::get_global_config;

pub struct ModelsManager {
    pool: SqlitePool,
}
impl ModelsManager {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

pub static GLOBAL_MANAGER: OnceCell<Arc<ModelsManager>> = OnceCell::const_new();

pub async fn get_global_manager() -> &'static Arc<ModelsManager> {
    GLOBAL_MANAGER
        .get_or_init(|| async {
            let config = get_global_config().await;
            let pool = SqlitePool::connect(&config.database_url)
                .await
                .expect("Failed to connect to database");

            Arc::new(ModelsManager::new(pool))
        })
        .await
}

// -- create spl_token table in sqlite3
// CREATE TABLE spl_token (
//     mint TEXT PRIMARY KEY, -- mint address
//     smart_address TEXT NOT NULL, -- smart address, who related to this token
//     monitor_status TEXT NOT NULL DEFAULT 'active', -- monitor status
//     strategy_name TEXT NOT NULL, -- strategy name
//     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- created at
//     updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- updated at
// );

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct SplToken {
    pub mint: String,
    pub smart_address: String,
    pub monitor_status: String,
    pub strategy_name: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl ModelsManager {
    pub async fn add_new_spl_token(
        &self,
        mint: &str,
        smart_address: &str,
        strategy_name: &str,
    ) -> Result<()> {
        // judge if the spl token exists
        let sql_str = format!(
            "SELECT * FROM spl_token WHERE mint = '{}' 
            AND monitor_status = 'active' 
            AND strategy_name = '{}'
            AND smart_address = '{}'",
            mint, strategy_name, smart_address
        );
        let row = sqlx::query_as::<_, SplToken>(&sql_str)
            .fetch_optional(&self.pool)
            .await?;
        if row.is_some() {
            return Ok(());
        }
        // insert new spl token
        let sql_str = format!(
            "INSERT INTO spl_token (mint, smart_address, strategy_name) 
            VALUES ('{}', '{}', '{}')",
            mint, smart_address, strategy_name
        );
        sqlx::query(&sql_str).execute(&self.pool).await?;
        Ok(())
    }
}
