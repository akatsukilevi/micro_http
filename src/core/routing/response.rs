use super::{HttpResponse, ResponseKind};

impl HttpResponse {
  pub fn into_http(&self) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    tiny_http::Response::from_string(self.data.clone())
      .with_status_code(self.status)
      .with_header(
        tiny_http::Header::from_bytes(
          &b"Content-Type"[..],
          match self.kind {
            ResponseKind::Text => &b"text/plain; charset=UTF-8"[..],
            ResponseKind::Json => &b"application/json; charset=UTF-8"[..],
            ResponseKind::Page => &b"text/html; charset=UTF-8"[..],
          },
        )
        .unwrap(),
      )
  }
}
