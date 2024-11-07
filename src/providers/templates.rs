use std::io::{Error, ErrorKind};
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct TemplatingSettings {
  pub path: String,
}

pub struct Templater {
  tera: Arc<Mutex<Tera>>,
}

impl Templater {
  pub fn render(&self, name: &str) -> Result<String, tera::Error> {
    let tera = match self.tera.lock() {
      Ok(x) => x,
      Err(e) => {
        log::error!("Failed to obtain Tera lock: {:?}", e);
        return Err(tera::Error::io_error(Error::new(
          ErrorKind::Other,
          e.to_string(),
        )));
      }
    };

    tera.render(name, &Context::new())
  }

  pub fn render_data<T: serde::Serialize>(
    &self,
    name: &str,
    data: &T,
  ) -> Result<String, tera::Error> {
    let tera = match self.tera.lock() {
      Ok(x) => x,
      Err(e) => {
        log::error!("Failed to obtain Tera lock: {:?}", e);
        return Err(tera::Error::io_error(Error::new(
          ErrorKind::Other,
          e.to_string(),
        )));
      }
    };

    let ctx = &Context::from_serialize(&data)?;
    tera.render(name, &ctx)
  }

  pub fn render_context(
    &self,
    name: &str,
    ctx: &Context,
  ) -> Result<String, tera::Error> {
    let tera = match self.tera.lock() {
      Ok(x) => x,
      Err(e) => {
        log::error!("Failed to obtain Tera lock: {:?}", e);
        return Err(tera::Error::io_error(Error::new(
          ErrorKind::Other,
          e.to_string(),
        )));
      }
    };

    tera.render(name, &ctx)
  }
}

pub fn create_templater(
  settings: &TemplatingSettings,
) -> Result<Templater, Error> {
  let mut tera = match Tera::new(&format!("{}/**/*.html", settings.path)) {
    Ok(t) => t,
    Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
  };
  tera.autoescape_on(vec![".html", ".sql"]);

  let tera = Arc::new(Mutex::new(tera));

  Ok(Templater { tera })
}
