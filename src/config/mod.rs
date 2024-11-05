use std::fs::read_to_string;

use env_logger::Env;
use serde::{Deserialize, Serialize};

pub mod web;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
  pub log_level: Option<String>,
  pub listen: web::ListenSettings,
}

pub fn init_config(config_path: Option<String>) -> Settings {
  // * Parse configuration
  let settings: Settings = {
    let path = match config_path {
      Some(x) => x,
      None => "data/config.toml".into(),
    };

    match read_to_string(path) {
      Ok(x) => toml::from_str(&x).unwrap(),
      Err(e) => panic!("Failed to load settings: {}", e),
    }
  };

  let log_level = settings.log_level.clone().unwrap_or("info".into());
  env_logger::Builder::from_env(Env::default().default_filter_or(log_level))
    .init();

  // * Return final settings
  settings
}