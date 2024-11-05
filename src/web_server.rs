use std::{
  io::{Error, ErrorKind},
  sync::Arc,
};
use tera::{Context, Tera};
use tiny_http::Server;

use crate::{config::Settings, core::routing, services};

#[derive(Clone, Debug)]
pub struct AppState {
  tera: Tera,
}

impl AppState {
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

pub fn listen(settings: &Settings) -> Result<(), Error> {
  let router = Arc::new(services::router());

  log::info!("Starting server on http://{}", &settings.listen.address);
  let server = match Server::http(&settings.listen.address) {
    Ok(x) => x,
    Err(e) => {
      return Err(Error::new(ErrorKind::AddrNotAvailable, e.to_string()))
    }
  };

  let mut tera = match Tera::new("views/**/*.html") {
    Ok(t) => t,
    Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
  };
  tera.autoescape_on(vec![".html", ".sql"]);

  let app_state = Arc::new(AppState { tera });
  for req in server.incoming_requests() {
    let state = app_state.clone();
    let route = router.clone();

    tokio::spawn(async {
      routing::handle::handle_request(req, route, state).await;
    });
  }

  Ok(())
}
