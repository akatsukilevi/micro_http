use crate::core::routing::Router;

mod homepage;

pub fn router() -> Router {
  let mut router = Router::new();
  router.insert("/", Box::new(homepage::homepage)).unwrap();

  router
}
