//! Defines environment variables.

use std::{env, path::PathBuf};

use api_framework::{env::parse_env, static_lazy_lock};
use sha2::Digest;

/// Sets up environment variables from `.env` and `{crate_name}.env`.
pub fn setup() {
    dotenvy::dotenv().ok();
    dotenvy::from_filename_override(format!("{}.env", clap::crate_name!())).ok();
}

/// The info generated during build.
pub mod info {
    /// The latest Git commit hash.
    pub const GIT_HASH: &str = env!("GIT_HASH");
    /// The build timestamp.
    pub const BUILD_TIMESTAMP: &str = env!("VERGEN_BUILD_TIMESTAMP");
}

static_lazy_lock! {
    pub PORT: u16 = parse_env!("PORT" => |s| s.parse::<u16>(); anyhow).expect("PORT not set in environment");
    "The port to listen to."
}

static_lazy_lock! {
    pub KTT_API_USERNAME: String = env::var("KTT_API_USERNAME").expect("KTT_API_USERNAME not set in environment");
    "The username of the API key."
}

static_lazy_lock! {
    pub KTT_API_PASSWORD: String = env::var("KTT_API_PASSWORD").expect("KTT_API_PASSWORD not set in environment");
    "The password of the API key."
}

static_lazy_lock! {
    pub DATABASE_URL: String = env::var("DATABASE_URL").expect("DATABASE_URL not set in environment");
    "The url to connect the database."
}

static_lazy_lock! {
    pub ADMIN_PASSWORD: [u8; 32] = parse_env!("ADMIN_PASSWORD" => |k| Ok(sha2::Sha256::digest(k.into_bytes()))).expect("ADMIN_PASSWORD not set in environment").into();
    "The administration password. Hashed using SHA256 to generate a 32 bytes long key."
}

static_lazy_lock! {
    pub PASETO_SYMMETRIC_KEY: [u8; 32] = parse_env!("PASETO_SYMMETRIC_KEY" => |k| Ok(sha2::Sha256::digest(k.into_bytes()))).expect("PASETO_SYMMETRIC_KEY not set in environment").into();
    "The PASETO symmetric key. Hashed using SHA256 to generate a 32 bytes long key."
}

static_lazy_lock! {
    pub SESSION_SYMMETRIC_KEY: [u8; 32] = parse_env!("SESSION_SYMMETRIC_KEY" => |k| Ok(sha2::Sha256::digest(k.into_bytes()))).expect("SESSION_SYMMETRIC_KEY not set in environment").into();
    "The session symmetric key. Hashed using SHA256 to generate a 32 bytes long key."
}

static_lazy_lock! {
    pub TRACING_MAX_FILES: usize = parse_env!("TRACING_MAX_FILES" => |s| s.parse::<usize>(); anyhow).unwrap_or(5);
    "The maximum file count to use for tracing."
}

static_lazy_lock! {
    pub TRACING_DIR: PathBuf = parse_env!("TRACING_DIR" => |s| Ok(PathBuf::from(s))).unwrap_or(PathBuf::from("/tmp/tracing")).join(clap::crate_name!());
    "The directory for tracing files. Defaults to `/tmp/tracing/wordle` if not specified."
}

static_lazy_lock! {
    pub CONFIG_DIR: PathBuf = parse_env!("CONFIG_DIR" => |s| Ok(PathBuf::from(s))).expect("CONFIG_DIR not set in environment").join(clap::crate_name!());
    "The directory for config files."
}
