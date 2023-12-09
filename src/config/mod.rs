pub mod tracing;

use std::path::PathBuf;

use chrono::Duration;
use config_types::chrono_types::duration::ConfigDuration;
use digestible::Digestible;
use sea_orm::ConnectOptions;
use serde::{Deserialize, Serialize};

use crate::recaptcha::GoogleRecaptcha;

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct ServerConfig {
    pub bind_address: String,
    pub home_url: Option<String>,
    pub workers: Option<usize>,
    pub tls: Option<TlsConfig>,
    pub database: Database,
    pub session: SessionConfigFull,
    pub tracing: tracing::TracingConfiguration,
    pub public_registration: bool,
    pub recaptcha: Option<GoogleRecaptcha>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TlsConfig {
    pub private_key: PathBuf,
    pub certificate_chain: PathBuf,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0:5312".to_string(),
            home_url: None,
            workers: None,
            tls: None,
            database: Database::default(),
            session: SessionConfigFull::default(),
            tracing: Default::default(),
            public_registration: true,
            recaptcha: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Digestible)]
#[serde(default)]
pub struct SessionConfig {
    pub cookie_name: String,
    pub allow_in_header: bool,
    pub session_lifetime: ConfigDuration,
}
impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            cookie_name: "session".to_string(),
            allow_in_header: true,
            session_lifetime: ConfigDuration {
                duration: Duration::days(1),
                unit: config_types::chrono_types::duration::Unit::Days,
            },
        }
    }
}
// TODO. Add SessionCleaner, and session life.
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct SessionConfigFull {
    pub manager: SessionManagerConfig,
    #[serde(default, flatten)]
    pub session_config: SessionConfig,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "settings")]
pub enum SessionManagerConfig {
    Memory {
        #[serde(default)]
        start_size: usize,
    },
    Redb {
        file: PathBuf,
    },
}
impl Default for SessionManagerConfig {
    fn default() -> Self {
        // TODO: Default Redb
        Self::Memory { start_size: 100 }
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Database {
    pub user: String,
    pub password: String,
    pub host: String,
    pub database: String,
}
impl Default for Database {
    fn default() -> Self {
        Self {
            user: "".to_string(),
            password: "".to_string(),
            host: "localhost:5432".to_string(),
            database: "codi-time".to_string(),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<ConnectOptions> for Database {
    fn into(self) -> ConnectOptions {
        ConnectOptions::new(format!(
            "postgres://{}:{}@{}/{}",
            self.user, self.password, self.host, self.database
        ))
    }
}
