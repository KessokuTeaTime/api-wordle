use std::time::Duration;

use crate::env::DATABASE_URL;

use api_framework::static_lazy_lock;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use tracing::level_filters::LevelFilter;

pub mod types;

pub mod puzzles;

static_lazy_lock! {
    pub OPTIONS: ConnectOptions = {
        let mut options = ConnectOptions::new(*DATABASE_URL);
        options.max_connections(100)
            .connect_timeout(Duration::from_secs(10))
            .idle_timeout(Duration::from_secs(5))
            .sqlx_logging(true)
            .sqlx_logging_level(LevelFilter::TRACE);
        options
    };
    "The connect options for PostgreSQL."
}

pub async fn setup() -> Result<(), DbErr> {
    let db = acquire().await?;
    Migrator::up(&db, None).await
}

pub async fn acquire() -> Result<DatabaseConnection, DbErr> {
    Database::connect(OPTIONS).await
}
