use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{
  io::{Error, ErrorKind},
  time::Duration,
};

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct DatabaseConfiguration {
  pub connection_string: String,
  pub min_connections: Option<u32>,
  pub max_connections: Option<u32>,
  pub connection_timeout: Option<u64>,
  pub acquire_timeout: Option<u64>,
  pub idle_timeout: Option<u64>,
  pub max_lifetime: Option<u64>,
  pub logging: Option<bool>,
  pub slow_query_threshold: Option<u64>,
}

pub async fn create_database(
  settings: &DatabaseConfiguration,
) -> Result<DatabaseConnection, Error> {
  let mut options = ConnectOptions::new(settings.connection_string.clone());
  options
    .min_connections(settings.min_connections.unwrap_or(5))
    .max_connections(settings.max_connections.unwrap_or(100))
    .connect_timeout(Duration::from_secs(
      settings.connection_timeout.unwrap_or(8),
    ))
    .acquire_timeout(Duration::from_secs(settings.acquire_timeout.unwrap_or(8)))
    .idle_timeout(Duration::from_secs(settings.idle_timeout.unwrap_or(8)))
    .max_lifetime(Duration::from_secs(settings.max_lifetime.unwrap_or(8)))
    .sqlx_logging(settings.logging.unwrap_or(false))
    .sqlx_logging_level(log::LevelFilter::Info)
    .sqlx_slow_statements_logging_settings(
      log::LevelFilter::Warn,
      Duration::from_secs(settings.slow_query_threshold.unwrap_or(8)),
    );

  Database::connect(options)
    .await
    .map_err(|e| Error::new(ErrorKind::NotConnected, e))
}
