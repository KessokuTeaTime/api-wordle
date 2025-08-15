#![cfg(test)]

use std::env;

use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

mod submit_history;

async fn setup() -> DatabaseConnection {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in environment");
    let db = Database::connect(ConnectOptions::new(&db_url))
        .await
        .unwrap_or_else(|_| panic!("cannot connect to {db_url}"));

    Migrator::refresh(&db).await.unwrap();
    db
}
