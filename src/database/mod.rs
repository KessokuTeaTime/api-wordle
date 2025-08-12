use crate::env::DATABASE_URL;

use api_framework::static_lazy_lock;
use diesel::{PgConnection, r2d2::ConnectionManager};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use tracing::info;

pub mod types;

pub mod puzzles;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

static_lazy_lock! {
    pub POOL: Pool = establish_pool();
    "The connection pool for PostgreSQL."
}

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn establish_pool() -> Pool {
    info!("establishing connection pool with {}…", *DATABASE_URL);
    let manager = ConnectionManager::<PgConnection>::new(&*DATABASE_URL);
    let pool = Pool::builder()
        .max_size(15)
        .build(manager)
        .unwrap_or_else(|e| {
            panic!(
                "failed to establish connection pool with {}: {e}",
                *DATABASE_URL
            )
        });

    {
        info!("running database migrations…");
        let mut conn = pool
            .get()
            .unwrap_or_else(|e| panic!("failed to get connection: {e}"));

        conn.run_pending_migrations(MIGRATIONS)
            .unwrap_or_else(|e| panic!("failed to run migrations: {e}"));
    }

    info!("established connection pool with {}", *DATABASE_URL);
    pool
}
