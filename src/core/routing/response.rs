use super::{HttpResponse, ResponseKind};

impl HttpResponse {
  pub fn into_http(&self) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    tiny_http::Response::from_data(self.data.clone())
      .with_status_code(self.status)
      .with_header(
        tiny_http::Header::from_bytes(
          &b"Content-Type"[..],
          match &self.kind {
            ResponseKind::Raw(mime) => {
              let kind: Vec<u8> = format!("{}", mime).into();
              kind
            }
            ResponseKind::Text => "text/plain; charset=UTF-8".into(),
            ResponseKind::Json => "application/json; charset=UTF-8".into(),
            ResponseKind::Page => "text/html; charset=UTF-8".into(),
          },
        )
        .unwrap(),
      )
  }
}
