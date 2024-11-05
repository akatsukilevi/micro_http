use std::{
  io::{Error, ErrorKind},
  sync::Arc,
};

use matchit::Params;
use tiny_http::{Request, StatusCode};

use super::error::HttpError;

mod log;

pub mod handle;
pub mod response;

pub enum ResponseKind {
  Text,
  Json,
  Page,
}

pub struct HttpResponse {
  kind: ResponseKind,
  status: StatusCode,
  data: String,
}

impl HttpResponse {
  pub fn text<T: Into<String>>(
    message: T,
    status_code: Option<u16>,
  ) -> HttpResult {
    Ok(Self {
      kind: ResponseKind::Text,
      data: message.into(),
      status: StatusCode::from(status_code.unwrap_or(200)),
    })
  }

  pub fn html(
    page: Result<String, tera::Error>,
    status_code: Option<u16>,
  ) -> HttpResult {
    let data = match page {
      Ok(x) => x,
      Err(e) => {
        return Err(HttpError::InternalServerError(Error::new(
          ErrorKind::Other,
          e,
        )))
      }
    };

    Ok(Self {
      kind: ResponseKind::Page,
      data,
      status: StatusCode::from(status_code.unwrap_or(200)),
    })
  }

  pub fn json<T: serde::Serialize>(
    data: &T,
    status_code: Option<u16>,
  ) -> HttpResult {
    let data = match serde_json::to_string(data) {
      Ok(x) => x,
      Err(e) => return Err(HttpError::InternalServerError(e.into())),
    };

    Ok(Self {
      kind: ResponseKind::Json,
      data,
      status: StatusCode::from(status_code.unwrap_or(200)),
    })
  }
}

pub type Router = matchit::Router<Box<Route>>;
pub type Route = dyn Fn(&Request, Params, AppState) -> HttpResult + Sync + Send;
pub type HttpResult = Result<HttpResponse, HttpError>;

pub type AppState = Arc<crate::web_server::AppState>;
