use api_framework::static_lazy_lock;
use diesel::{
    Connection as _, PgConnection,
    backend::Backend,
    prelude::{Insertable, Queryable},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    database::types::{PuzzleDate, PuzzleWord},
    env::DATABASE_URL,
    schema::puzzles,
};

pub mod types;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

static_lazy_lock! {
    pub CONNECTION: Mutex<PgConnection> = Mutex::new(establish_connection());
    "The connection to PostgreSQL."
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = puzzles)]
#[diesel(check_for_backend(Pg))]
pub struct Puzzle {
    pub date: PuzzleDate,
    pub puzzle: PuzzleWord,
    pub is_deleted: bool,
}

fn establish_connection() -> PgConnection {
    info!("establishing connection with {}…", *DATABASE_URL);
    let mut connection = PgConnection::establish(&DATABASE_URL)
        .unwrap_or_else(|e| panic!("failed to establish connection with {}: {e}", *DATABASE_URL));

    info!("running database migrations…");
    run_migrations(&mut connection).unwrap_or_else(|e| panic!("failed to run migrations: {e}"));

    info!("established connection with {}", *DATABASE_URL);
    connection
}

pub fn run_migrations<DB>(
    connection: &mut impl MigrationHarness<DB>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>
where
    DB: Backend,
{
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}
