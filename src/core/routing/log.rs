use tiny_http::Request;

use super::HttpResponse;

pub fn log_response(
  req: &Request,
  res: &HttpResponse,
  start_time: chrono::DateTime<chrono::Local>,
) {
  let now = chrono::offset::Local::now();

  let res = res.into_http();
  let msg = format!(
    "{} \"{} {} {}\" {} {} {:.6}",
    req.remote_addr().unwrap(),
    req.method(),
    req.url(),
    req.http_version(),
    res.status_code().0,
    res.data_length().unwrap_or(0),
    ((now - start_time).num_microseconds().unwrap_or(0) as f32 / 1000.0)
  );

  match res.status_code().0 {
    x if x >= 200 && x < 400 => log::info!("{msg}"),
    x if x >= 400 && x < 500 => log::warn!("{msg}"),
    _ => log::error!("{msg}"),
  }
}
