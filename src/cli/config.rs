use clap::*;

use std::fs::File;
use std::io::{Error, Write};

use crate::config::Settings;

#[derive(Args, Clone, Debug)]
pub struct GenerateConfigOptions {
  #[arg(short, long)]
  /// Where to write the configuration file
  target: String,
}

pub fn generate_config(options: &GenerateConfigOptions) -> Result<(), Error> {
  let settings = Settings::default();
  let serialized = toml::to_string(&settings).unwrap();

  let mut output = File::create(options.target.clone())?;
  match write!(output, "{}", serialized) {
    Ok(..) => log::info!("Configuration file saved successfully"),
    Err(e) => return Err(e),
  }

  Ok(())
}
