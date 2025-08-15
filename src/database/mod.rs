//! The database access.

use std::time::Duration;

use crate::env::DATABASE_URL;

use api_framework::static_lazy_lock;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use tracing::log::LevelFilter;

pub mod tables;

static_lazy_lock! {
    pub OPTIONS: ConnectOptions = {
        let mut options = ConnectOptions::new(&*DATABASE_URL);
        options.max_connections(100)
            .connect_timeout(Duration::from_secs(10))
            .idle_timeout(Duration::from_secs(5))
            .sqlx_logging(true)
            .sqlx_logging_level(LevelFilter::Trace);
        options
    };
    "The connect options for PostgreSQL."
}

/// Acquires a database connection, or runs an expression.
///
/// See: [`acquire`]
#[macro_export]
macro_rules! acquire_or {
    (|$name:ident| $expr:expr) => {
        match $crate::database::acquire().await {
            Ok(db) => db,
            Err($name) => $expr,
        }
    };
}

pub use acquire_or;

/// Acquires a database connection, or responses with a 500 Internal Server Error and the corresponding error description.
///
/// See: [`acquire`], [`acquire_or`]
#[macro_export]
macro_rules! acquire_or_response {
    () => {
        $crate::database::acquire_or!(|err| return (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            err.to_string()
        )
            .into_response())
    };
}

pub use acquire_or_response;

/// Sets up the database and run necessary migrations.
///
/// # Errors
///
/// Returns a [`DbErr`] if the setup process fails.
pub async fn setup() -> Result<(), DbErr> {
    let db = acquire().await?;
    Migrator::up(&db, None).await
}

/// Acquires a database connection.
///
/// # Errors
///
/// Returns a [`DbErr`] if unable to acquire a connection.
///
/// See: [`OPTIONS`]
pub async fn acquire() -> Result<DatabaseConnection, DbErr> {
    Database::connect(OPTIONS.clone()).await
}
