//! KessokuTeaTime API backend for the wordle game.

use crate::env::{
    DATABASE_URL, PORT,
    info::{BUILD_TIMESTAMP, GIT_HASH},
};

use std::net::SocketAddr;

use anyhow::{Error, anyhow};
use api_framework::{shutdown, static_lazy_lock};
use axum::Router;
use tokio::net::TcpListener;
use tracing::{info, trace};

pub mod env;
pub mod trace;

pub mod database;
pub mod endpoint;
pub mod middleware;

static_lazy_lock! {
    pub WORDS: &[&str] = random_word::all_len(5, random_word::Lang::En).unwrap();
}

#[tokio::main]
async fn main() {
    env::setup().unwrap();
    trace::setup().unwrap();
    trace!("loaded environment: {:#?}", std::env::vars());

    database::setup().await.unwrap();
    trace!("set up database at {}", *DATABASE_URL);

    info!(
        "binary {} version {}",
        clap::crate_name!(),
        clap::crate_version!()
    );
    info!("compiled from commit {GIT_HASH} at {BUILD_TIMESTAMP}");
    info!("starting server on port {}…", *PORT);

    serve().await.unwrap();

    info!("stopping…");
}

async fn serve() -> Result<(), Error> {
    let mut app = Router::new();
    app = endpoint::route_from(app);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", *PORT)).await?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown::signal())
    .await
    .map_err(|e| anyhow!(e))
}
