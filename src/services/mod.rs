use crate::core::routing::Router;

mod counter;
mod homepage;

mod r#static;

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

  router
    .insert("/{*path}", Box::new(r#static::serve))
    .unwrap();
  router
}
