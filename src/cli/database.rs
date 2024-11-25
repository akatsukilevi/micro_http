use clap::*;
use sea_orm_migration::MigratorTrait;
use std::io::{Error, ErrorKind};

use crate::{config::Settings, database, providers};

#[derive(Args, Clone, Debug)]
pub struct MigrateOptions {
  #[command(subcommand)]
  command: MigrateMethod,
}

#[derive(Subcommand, Clone, Debug)]
pub enum MigrateMethod {
  /// Upgrade the database to the latest available version
  Upgrade,
  /// Rollback all applied migrations, then reapply all migrations
  Refresh,
  /// Rollback the last applied migration
  Rollback,
}

pub async fn migrate(
  settings: &Settings,
  options: &MigrateOptions,
) -> Result<(), Error> {
  log::info!("Initializing database");
  let db = providers::database::create_database(&settings.database).await?;

  let task = match options.command {
    MigrateMethod::Refresh => {
      log::info!("Refreshing database");
      database::migrations::Migrator::refresh(&db).await
    }
    MigrateMethod::Upgrade => {
      log::info!("Upgrading database");
      database::migrations::Migrator::up(&db, None).await
    }
    MigrateMethod::Rollback => {
      log::info!("Rolling-back database");
      database::migrations::Migrator::down(&db, None).await
    }
  };

  match task {
    Ok(..) => Ok(log::info!("Migration finished with success")),
    Err(e) => {
      log::error!("Migrations have failed: {:#?}", e);
      Err(Error::new(ErrorKind::Other, e))
    }
  }
}
