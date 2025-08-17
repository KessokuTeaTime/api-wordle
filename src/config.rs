//! Defines runtime configs.

use std::{path::PathBuf, sync::LazyLock};

use axum::http::HeaderValue;
use serde::{Deserialize, de::DeserializeOwned};

pub trait RuntimeConfig {
    const PATH: LazyLock<PathBuf>;

    fn path() -> PathBuf {
        let path = Self::PATH;
        PathBuf::from("config")
            .join(clap::crate_name!())
            .join(path.to_owned())
    }

    async fn load() -> Self
    where
        Self: DeserializeOwned + Sized,
    {
        let config_str = tokio::fs::read_to_string(Self::path())
            .await
            .unwrap_or_else(|e| {
                panic!(
                    "failed to read config file from {}: {e}",
                    Self::path().to_str().unwrap()
                )
            });
        toml::from_str(&config_str).unwrap_or_else(|e| {
            panic!(
                "failed to parse config file from {}: {e}",
                Self::path().to_str().unwrap()
            )
        })
    }

    async fn load_or_default() -> Self
    where
        Self: Default + DeserializeOwned + Sized,
    {
        let config_str = tokio::fs::read_to_string(Self::path()).await.ok();
        match config_str {
            Some(config_str) => toml::from_str(&config_str).unwrap_or_default(),
            None => Default::default(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CorsRuntimeConfig {
    pub origins: Vec<String>,
}

impl CorsRuntimeConfig {
    pub fn contains(&self, origin: &HeaderValue) -> bool {
        let origins: Vec<HeaderValue> = self.origins.iter().flat_map(|s| s.parse().ok()).collect();
        origins.contains(origin)
    }
}

impl RuntimeConfig for CorsRuntimeConfig {
    const PATH: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("cors").with_extension("toml"));
}
