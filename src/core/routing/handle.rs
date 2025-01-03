use tiny_http::Request;

use super::{log::log_response, AppState, HttpError};

pub async fn handle_request(req: Request, state: AppState) {
  let start = chrono::offset::Local::now();

  let Ok(handler) = state.router.at(&req.url()) else {
    let res = HttpError::NotFound.to_response().unwrap();
    log_response(&req, &res, start);

    let res = res.into_http();
    req.respond(res).expect("Should be able to respond");
    return;
  };

  let res = match (handler.value)(&req, handler.params, state.clone()) {
    Ok(x) => x,
    Err(e) => {
      let res = e.to_response().unwrap();
      log_response(&req, &res, start);

      let res = res.into_http();
      req.respond(res).expect("Should be able to respond");
      return;
    }
  };

  log_response(&req, &res, start);
  let res = res.into_http();
  req.respond(res).expect("Should be able to respond");
}
