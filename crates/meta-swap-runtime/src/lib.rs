#![forbid(unsafe_code)]
//! Runtime configuration validation. This crate intentionally does not open sockets or log
//! secrets; transport/runtime adapters may consume the validated configuration.

use std::{env, fmt, net::SocketAddr};

pub const DEFAULT_BIND_ADDRESS: &str = "127.0.0.1:3000";
pub const MIN_WEBHOOK_SECRET_LENGTH: usize = 32;

#[derive(Clone, Eq, PartialEq)]
pub struct AppConfig {
    database_url: String,
    telegram_webhook_secret: String,
    bind_address: SocketAddr,
}

impl AppConfig {
    /// Loads the process environment without emitting any secret values.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError`] when required settings are absent, blank, malformed, or the
    /// webhook secret is too short for a production authentication boundary.
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_values(
            env::var("DATABASE_URL").ok(),
            env::var("TELEGRAM_WEBHOOK_SECRET").ok(),
            env::var("BIND_ADDR").ok(),
        )
    }

    /// # Errors
    ///
    /// Returns [`ConfigError`] for the same validation conditions as [`Self::from_env`].
    pub fn from_values(
        database_url: Option<String>,
        telegram_webhook_secret: Option<String>,
        bind_address: Option<String>,
    ) -> Result<Self, ConfigError> {
        let database_url = required("DATABASE_URL", database_url)?;
        let telegram_webhook_secret = required("TELEGRAM_WEBHOOK_SECRET", telegram_webhook_secret)?;
        if telegram_webhook_secret.len() < MIN_WEBHOOK_SECRET_LENGTH {
            return Err(ConfigError::WebhookSecretTooShort);
        }
        let bind_address = bind_address
            .unwrap_or_else(|| String::from(DEFAULT_BIND_ADDRESS))
            .parse()
            .map_err(|_| ConfigError::InvalidBindAddress)?;
        Ok(Self {
            database_url,
            telegram_webhook_secret,
            bind_address,
        })
    }

    #[must_use]
    pub fn database_url(&self) -> &str {
        &self.database_url
    }
    #[must_use]
    pub fn telegram_webhook_secret(&self) -> &str {
        &self.telegram_webhook_secret
    }
    #[must_use]
    pub const fn bind_address(&self) -> SocketAddr {
        self.bind_address
    }
}

impl fmt::Debug for AppConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AppConfig")
            .field("database_url", &"[REDACTED]")
            .field("telegram_webhook_secret", &"[REDACTED]")
            .field("bind_address", &self.bind_address)
            .finish()
    }
}

fn required(name: &'static str, value: Option<String>) -> Result<String, ConfigError> {
    value
        .filter(|setting| !setting.trim().is_empty())
        .ok_or(ConfigError::Missing(name))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigError {
    Missing(&'static str),
    WebhookSecretTooShort,
    InvalidBindAddress,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing(name) => write!(formatter, "missing required configuration: {name}"),
            Self::WebhookSecretTooShort => {
                formatter.write_str("Telegram webhook secret is too short")
            }
            Self::InvalidBindAddress => formatter.write_str("BIND_ADDR is not a socket address"),
        }
    }
}
impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;
    fn secret() -> String {
        "a".repeat(MIN_WEBHOOK_SECRET_LENGTH)
    }
    #[test]
    fn valid_configuration_uses_safe_default_bind_address() {
        let config = AppConfig::from_values(
            Some(String::from("postgres://example")),
            Some(secret()),
            None,
        )
        .unwrap();
        assert_eq!(config.bind_address().to_string(), DEFAULT_BIND_ADDRESS);
    }
    #[test]
    fn configuration_debug_output_redacts_secrets() {
        let secret = secret();
        let config = AppConfig::from_values(
            Some(String::from("postgres://username:password@host/db")),
            Some(secret.clone()),
            None,
        )
        .unwrap();
        let rendered = format!("{config:?}");
        assert!(!rendered.contains(&secret));
        assert!(!rendered.contains("password"));
    }
    #[test]
    fn short_or_blank_critical_configuration_is_rejected() {
        assert_eq!(
            AppConfig::from_values(Some(String::new()), Some(secret()), None),
            Err(ConfigError::Missing("DATABASE_URL"))
        );
        assert_eq!(
            AppConfig::from_values(
                Some(String::from("postgres://example")),
                Some(String::from("short")),
                None
            ),
            Err(ConfigError::WebhookSecretTooShort)
        );
    }
}
