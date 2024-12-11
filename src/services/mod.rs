use crate::core::{r#static, routing::Router};

mod counter;
mod homepage;

pub fn router() -> Router {
  let mut router = Router::new();
  router.insert("/", Box::new(homepage::homepage)).unwrap();

  router
    .insert("/counter/get", Box::new(counter::current))
    .unwrap();

  router
    .insert("/counter/increment", Box::new(counter::increment))
    .unwrap();

  router
    .insert("/counter/decrement", Box::new(counter::decrement))
    .unwrap();

  r#static::with_static(router)
}
