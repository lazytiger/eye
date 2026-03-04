use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use sqlx::{Pool, Sqlite, SqlitePool};

pub type DatabasePool = Pool<Sqlite>;
pub type DbConnectOptions = SqliteConnectOptions;

pub async fn init_database(url: &str) -> anyhow::Result<DatabasePool> {
    let pool_options = url
        .parse::<SqliteConnectOptions>()?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal);
    let pool = SqlitePool::connect_with(pool_options).await?;
    Ok(pool)
}

pub mod history;
