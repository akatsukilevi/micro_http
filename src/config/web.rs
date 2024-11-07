use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct WebSettings {
  pub listen: String,
  pub static_dir: String,
}
