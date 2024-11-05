use clap::*;

pub mod config;

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
  /// Generate a default configuration file to set-up the program
  GenerateConfig(config::GenerateConfigOptions),
}