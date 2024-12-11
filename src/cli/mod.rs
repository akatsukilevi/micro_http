use clap::*;

pub mod config;
pub mod database;

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
  #[arg(short, long, env = "CONFIG_FILE")]
  /// The configuration file to use
  pub configuration: Option<String>,

  #[command(subcommand)]
  pub command: Option<Commands>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
  /// Manage the connected database
  Database(database::MigrateOptions),
  /// Generate a default configuration file to set-up the program
  GenerateConfig(config::GenerateConfigOptions),
}

pub async fn parse_commands(commands: &Commands) {
  match cmd {
    Commands::GenerateConfig(x) => return config::generate_config(&x),
    Commands::Database(x) => return database::migrate(&settings, &x).await,
  }
}
