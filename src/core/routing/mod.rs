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
  Raw(String),
  Text,
  Json,
  Page,
}

pub struct HttpResponse {
  kind: ResponseKind,
  status: StatusCode,
  data: Vec<u8>,
}

impl HttpResponse {
  pub fn raw<T: Into<String>>(
    data: Vec<u8>,
    mime_type: T,
    status_code: Option<u16>,
  ) -> HttpResult {
    Ok(Self {
      kind: ResponseKind::Raw(mime_type.into()),
      data,
      status: StatusCode::from(status_code.unwrap_or(200)),
    })
  }

  pub fn text<T: Into<String>>(
    message: T,
    status_code: Option<u16>,
  ) -> HttpResult {
    Ok(Self {
      kind: ResponseKind::Text,
      data: message.into().into(),
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
      data: data.into(),
      status: StatusCode::from(status_code.unwrap_or(200)),
    })
  }

  pub fn json<T: serde::Serialize>(
    data: &T,
    status_code: Option<u16>,
  ) -> HttpResult {
    let data = match serde_json::to_vec(data) {
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
