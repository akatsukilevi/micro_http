use std::fs::read_to_string;

use serde::{Deserialize, Serialize};

use crate::providers;

pub mod web;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
  pub log_level: Option<String>,
  pub web: web::WebSettings,
  pub database: providers::database::DatabaseConfiguration,
  pub templates: providers::templates::TemplatingSettings,
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
  flexi_logger::Logger::try_with_env_or_str(log_level)
    .unwrap()
    .start()
    .unwrap();

  // * Return final settings
  settings
}
