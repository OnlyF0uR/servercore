use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::sync::Arc;
use tokio::sync::OnceCell;

use crate::config::get_config;

#[derive(Debug)]
pub struct DB {
    pub pool: SqlitePool,
}

impl DB {
    pub async fn init(path: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut requires_setup = false;
        if !Sqlite::database_exists(&path).await? {
            Sqlite::create_database(&path).await?;
            requires_setup = true;
        } else {
            log::info!("Loading database.");
        }

        let pool = SqlitePool::connect(&path).await?;

        if requires_setup {
            log::info!("Setting up database.");
            sqlx::query(
                "
                CREATE TABLE players (
                    id INTEGER PRIMARY KEY,
                    uuid TEXT NOT NULL,
                    nickname TEXT NOT NULL,
                    money REAL NOT NULL DEFAULT 0
                )",
            )
            .execute(&pool)
            .await?;

            sqlx::query(
                "
                CREATE TABLE homes (
                    id INTEGER PRIMARY KEY,
                    user_id INTEGER NOT NULL,
                    name TEXT NOT NULL,
                    x REAL NOT NULL,
                    y REAL NOT NULL,
                    z REAL NOT NULL
                )",
            )
            .execute(&pool)
            .await?;

            sqlx::query(
                "
                CREATE TABLE warps (
                    id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL,
                    x REAL NOT NULL,
                    y REAL NOT NULL,
                    z REAL NOT NULL
                )",
            )
            .execute(&pool)
            .await?;
        }

        Ok(DB { pool })
    }
}

static DB_INSTANCE: OnceCell<Arc<DB>> = OnceCell::const_new();

pub async fn setup_db(path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let path = format!("{}/{}", path, get_config().await.value.db_path);

    let db = DB::init(&path).await?;
    if let Err(e) = DB_INSTANCE.set(Arc::new(db)) {
        return Err(e.into());
    };

    Ok(())
}

pub async fn get_db() -> Arc<DB> {
    DB_INSTANCE.get().unwrap().clone()
}
