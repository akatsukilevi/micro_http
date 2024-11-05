use matchit::Params;
use tiny_http::{Request, Response};

use crate::core::routing::HttpResponse;

pub fn homepage(_req: &Request, _params: Params) -> HttpResponse {
  Ok(Response::from_string("Welcome"))
}
