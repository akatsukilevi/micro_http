use crate::core::routing::{AppState, HttpResponse, HttpResult};

pub fn homepage(
  _req: &tiny_http::Request,
  _params: matchit::Params,
  state: AppState,
) -> HttpResult {
  HttpResponse::html(state.templates.render("homepage.html"), None)
}
