use std::{
  io::{Error, ErrorKind},
  sync::Arc,
};
use tiny_http::Server;

use crate::{config::Settings, core::routing, providers, services};

pub struct AppState {
  pub templates: providers::templates::Templater,
  pub router: routing::Router,
  pub settings: Settings,
}

pub fn listen(settings: &Settings) -> Result<(), Error> {
  let server = Server::http(&settings.web.listen)
    .map_err(|e| Error::new(ErrorKind::AddrNotAvailable, e.to_string()))?;

  log::info!("Initializing dependencies");
  let templates = providers::templates::create_templater(&settings.templates)?;
  let app_state = Arc::new(AppState {
    router: services::router(),
    settings: settings.clone(),
    templates,
  });

  log::info!("Starting server on http://{}", &settings.web.listen);
  for req in server.incoming_requests() {
    let state = app_state.clone();

    tokio::spawn(async {
      routing::handle::handle_request(req, state).await;
    });
  }

  Ok(())
}
