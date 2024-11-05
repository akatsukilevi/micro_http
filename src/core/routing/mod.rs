use std::sync::Arc;

use matchit::Params;
use tiny_http::{Request, Response};

use super::error::HttpError;

pub async fn handle_request(req: Request, router: Arc<Router>) {
  let start = chrono::offset::Local::now();

  let Ok(handler) = router.at(&req.url()) else {
    let res = HttpError::NotFound.to_response();
    log_response(&req, &res, start);
    req.respond(res).expect("Should be able to respond");
    return;
  };

  let res = match (handler.value)(&req, handler.params) {
    Ok(x) => x,
    Err(e) => {
      let res = e.to_response();
      log_response(&req, &res, start);
      req.respond(res).expect("Should be able to respond");
      return;
    }
  };

  log_response(&req, &res, start);
  req.respond(res).expect("Should be able to respond");
}

fn log_response(
  req: &Request,
  res: &HttpResponseInner,
  start_time: chrono::DateTime<chrono::Local>,
) {
  let now = chrono::offset::Local::now();

  let msg = format!(
    "{} \"{} {} {}\" {} {} {:.6}",
    req.remote_addr().unwrap(),
    req.method(),
    req.url(),
    req.http_version(),
    res.status_code().0,
    res.data_length().unwrap_or(0),
    ((now - start_time).num_microseconds().unwrap_or(0) as f32 / 1000.0)
  );

  match res.status_code().0 {
    x if x >= 200 && x < 400 => log::info!("{msg}"),
    x if x >= 400 && x < 500 => log::warn!("{msg}"),
    _ => log::error!("{msg}"),
  }
}

pub type Router = matchit::Router<Box<Route>>;
pub type Route = dyn Fn(&Request, Params) -> HttpResponse + Sync + Send;

pub type HttpResponse = Result<HttpResponseInner, HttpError>;
pub type HttpResponseInner = Response<std::io::Cursor<Vec<u8>>>;
