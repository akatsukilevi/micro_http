use clap::Parser;
use std::io::Error;

use micro_http::{cli::*, config::init_config, web_server};

#[tokio::main]
async fn main() -> Result<(), Error> {
  // * Parse arguments and initialize settings
  let args = Args::parse();
  let settings = init_config(args.configuration);

  // * Run subcommand if has one
  if let Some(cmd) = &args.command {
    parse_commands(commands).await;
  }

  // * Spin up the server
  log::info!("Starting server");
  web_server::listen(&settings).await
}
