use std::io::{Error, ErrorKind};

use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct DatabaseConfiguration {
  pub filename: String,
  pub create_if_missing: Option<bool>,
  pub foreign_keys: Option<bool>,
  pub auto_vacuum: Option<bool>,
  pub optimize_on_close: Option<bool>,
}

pub async fn create_database(
  settings: &DatabaseConfiguration,
) -> Result<SqlitePool, Error> {
  // * Create the database options
  let options = SqliteConnectOptions::new()
    .filename(settings.filename.clone())
    .create_if_missing(settings.create_if_missing.unwrap_or(false))
    .foreign_keys(settings.foreign_keys.unwrap_or(true))
    .auto_vacuum(match settings.auto_vacuum.unwrap_or(true) {
      true => sqlx::sqlite::SqliteAutoVacuum::Full,
      false => sqlx::sqlite::SqliteAutoVacuum::None,
    })
    .optimize_on_close(settings.optimize_on_close.unwrap_or(true), None);

  // * Connect the database pool
  log::info!("Estabilishing connection with database");
  match SqlitePool::connect_with(options).await {
    Ok(x) => Ok(x),
    Err(e) => return Err(Error::new(ErrorKind::NotConnected, e)),
  }
}
