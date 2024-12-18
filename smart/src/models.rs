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

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Coin {
    pub id: i64,
    pub account: String,
    pub token: String,
    pub created_at: i64,
    pub deleted: i64,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Account {
    pub id: i64,
    pub account: String,
    pub created_at: i64,
    pub deleted: i64,
}

impl ModelsManager {
    pub async fn add_new_account(&self, mint: String) -> Result<()> {
        // judge if the account exists
        let sql_str = format!(
            "SELECT * FROM accounts WHERE account = '{}' AND DELETED = 0;",
            mint
        );
        let account = sqlx::query_as::<_, Account>(&sql_str)
            .fetch_one(&self.pool)
            .await
            .ok();
        if account.is_some() {
            return Ok(());
        }

        // insert new account
        let sql_str = format!(
            "INSERT INTO accounts (account, created_at, deleted) VALUES ('{}', {}, 0);",
            mint,
            chrono::Local::now().timestamp()
        );
        sqlx::query(&sql_str).execute(&self.pool).await?;

        Ok(())
    }
}
