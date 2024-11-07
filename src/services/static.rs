use std::{fs::File, io::Read, path::Path};

use crate::core::{
  error::HttpError,
  routing::{AppState, HttpResponse, HttpResult},
};

pub fn serve(
  _req: &tiny_http::Request,
  params: matchit::Params,
  state: AppState,
) -> HttpResult {
  let Some(path) = params.get("path") else {
    return Err(HttpError::NotFound);
  };

  if path.contains("..") {
    return Err(HttpError::NotFound);
  }

  let path = format!(
    "{}{}",
    &state.settings.web.static_dir,
    path.replace("assets/", "/")
  );
  let path = Path::new(&path);

  let mut f = match File::open(path) {
    Ok(x) => x,
    Err(e) => return Err(HttpError::InternalServerError(e)),
  };

  let mime_type = match path.extension().unwrap_or_default() {
    x if x == "png" || x == "webp" => format!("image/{}", &x.to_string_lossy()),
    x if x == "css" => "text/css".into(),
    x if x == "js" => "text/javascript".into(),
    _ => "application/octet-stream".into(),
  };

  let mut buffer = Vec::new();
  match f.read_to_end(&mut buffer) {
    Ok(..) => HttpResponse::raw(buffer, mime_type, Some(200)),
    Err(e) => Err(HttpError::InternalServerError(e)),
  }
}
