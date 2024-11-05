use clap::Parser;
use std::io::Error;

use mocha_manager::{cli::*, config::init_config, web_server};

#[tokio::main]
async fn main() -> Result<(), Error> {
  // * Load environment variables
  match dotenvy::dotenv() {
    Ok(..) => {}
    Err(..) => {}
  }

  // * Parse arguments and initialize settings
  let args = Args::parse();
  let settings = init_config(args.configuration);

  // * Run subcommand if has one
  if let Some(cmd) = &args.command {
    match cmd {
      Commands::GenerateConfig(x) => return config::generate_config(x),
    }
  }

  // * Spin up the server
  log::info!("Starting server");
  web_server::listen(&settings)
}