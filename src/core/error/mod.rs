use serde::{Deserialize, Serialize};
use tiny_http::{Header, Response, StatusCode};

use super::routing::HttpResponseInner;

mod errors;

#[derive(Debug)]
pub enum HttpError {
  BadRequest(Option<String>),
  Unauthorized,
  Forbidden,
  NotFound,
  MethodNotAllowed,
  NotAcceptable(Option<String>),
  InternalServerError(std::io::Error),
}

impl HttpError {
  pub fn to_response(&self) -> HttpResponseInner {
    let error = match self {
      HttpError::BadRequest(x) => errors::bad_request(x.clone()),
      HttpError::Unauthorized => errors::unauthorized(),
      HttpError::Forbidden => errors::forbidden(),
      HttpError::NotFound => errors::not_found(),
      HttpError::MethodNotAllowed => errors::method_not_allowed(),
      HttpError::NotAcceptable(x) => errors::not_acceptable(x.clone()),
      HttpError::InternalServerError(x) => errors::internal_server_error(x),
    };

    Response::from_string(serde_json::to_string(&error).unwrap())
      .with_status_code(StatusCode::from(error.status_code))
      .with_header(
        Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..])
          .unwrap(),
      )
  }
}

#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
  pub context: Option<String>,
  pub description: String,
  pub error: String,
  #[serde(skip)]
  pub status_code: u16,
}
