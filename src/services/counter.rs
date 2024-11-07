use std::sync::atomic::{AtomicI32, Ordering};

use matchit::Params;
use tiny_http::Request;

use crate::core::routing::{AppState, HttpResponse, HttpResult};

lazy_static::lazy_static! {
  static ref COUNTER: AtomicI32 = AtomicI32::new(0);
}

pub fn current(_r: &Request, _p: Params, _s: AppState) -> HttpResult {
  let count = COUNTER.load(Ordering::Relaxed);
  HttpResponse::text(format!("{}", count), Some(200))
}

pub fn increment(_r: &Request, _p: Params, _s: AppState) -> HttpResult {
  let count = COUNTER.load(Ordering::Relaxed);
  COUNTER.store(count + 1, Ordering::Relaxed);

  HttpResponse::text(format!("{}", count + 1), Some(200))
}

pub fn decrement(_r: &Request, _p: Params, _s: AppState) -> HttpResult {
  let count = COUNTER.load(Ordering::Relaxed);
  COUNTER.store(count - 1, Ordering::Relaxed);

  HttpResponse::text(format!("{}", count - 1), Some(200))
}
