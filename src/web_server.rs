use std::{
  io::{Error, ErrorKind},
  sync::Arc,
};
use tiny_http::Server;

use crate::{config::Settings, core::routing::handle_request, services};

pub fn listen(settings: &Settings) -> Result<(), Error> {
  let router = Arc::new(services::router());

  log::info!("Starting server on http://{}", &settings.listen.address);
  let server = match Server::http(&settings.listen.address) {
    Ok(x) => x,
    Err(e) => {
      return Err(Error::new(ErrorKind::AddrNotAvailable, e.to_string()))
    }
  };

  for req in server.incoming_requests() {
    let route = router.clone();
    tokio::spawn(async {
      handle_request(req, route).await;
    });
  }

  Ok(())
}
