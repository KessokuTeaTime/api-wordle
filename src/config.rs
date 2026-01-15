//! Reads configs.

use std::{fmt::Debug, path::PathBuf};

use config_file::FromConfigFile;
use serde::Deserialize;

use crate::env::CONFIG_DIR;

/// The available config files.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigFile {
    /// The CORS configuration.
    Cors,
}

impl ConfigFile {
    /// The file name of the config file.
    pub fn file_name(&self) -> &'static str {
        match self {
            Self::Cors => "cors.toml",
        }
    }

    /// The path to the config file, respecting [`CONFIG_DIR`].
    pub fn path(&self) -> PathBuf {
        CONFIG_DIR.join(self.file_name())
    }
}

/// A trait for configs that can be deserialized.
pub trait Config<'de>: Deserialize<'de> + FromConfigFile {
    /// The [`ConfigFile`] this config corresponds to.
    fn file() -> ConfigFile;

    /// Reads the config from the config file. This function wraps the error logging and returns
    /// `None` on failure.
    ///
    /// See: [`FromConfigFile::from_config_file`]
    fn read() -> Option<Self>
    where
        Self: Debug,
    {
        match Self::from_config_file(Self::file().path()) {
            Ok(c) => {
                tracing::trace!("read config from file {:?}: {:#?}", Self::file().path(), c);
                Some(c)
            }
            Err(e) => {
                tracing::error!(
                    "failed to read config from file {:?}: {:?}",
                    Self::file().path(),
                    e
                );
                None
            }
        }
    }
}

/// The services config.
pub mod services {
    use std::collections::HashSet;

    use axum::http::HeaderValue;

    use super::*;

    /// Defines a service to update.
    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    pub struct CorsConfig {
        /// The allowed origins.
        pub origins: HashSet<HeaderValue>,
    }

    impl<'de> Deserialize<'de> for CorsConfig {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            #[derive(Deserialize)]
            struct RawCorsConfig {
                origins: Vec<String>,
            }

            let raw = RawCorsConfig::deserialize(deserializer)?;
            let origins = raw
                .origins
                .into_iter()
                .filter_map(|s| s.parse::<HeaderValue>().ok())
                .collect();

            Ok(Self { origins })
        }
    }

    impl CorsConfig {
        /// Checks whether the given origin is allowed.
        pub fn contains(&self, origin: &HeaderValue) -> bool {
            self.origins.contains(origin)
        }
    }

    impl Config<'_> for CorsConfig {
        fn file() -> ConfigFile {
            ConfigFile::Cors
        }
    }
}
