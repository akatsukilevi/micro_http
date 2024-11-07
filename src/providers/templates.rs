use std::io::{Error, ErrorKind};

use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct TemplatingSettings {
  pub path: String,
}

pub struct Templater {
  tera: Tera,
}

impl Templater {
  pub fn render(&self, name: &str) -> Result<String, tera::Error> {
    self.tera.render(name, &Context::new())
  }

  pub fn render_data<T: serde::Serialize>(
    &self,
    name: &str,
    data: &T,
  ) -> Result<String, tera::Error> {
    let ctx = &Context::from_serialize(&data)?;
    self.tera.render(name, &ctx)
  }

  pub fn render_context(
    &self,
    name: &str,
    ctx: &Context,
  ) -> Result<String, tera::Error> {
    self.tera.render(name, &ctx)
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

  Ok(Templater { tera })
}
