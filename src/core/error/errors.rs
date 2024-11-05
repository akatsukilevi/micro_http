use super::ErrorResponse;

pub fn bad_request(err: Option<String>) -> ErrorResponse {
  ErrorResponse {
    error: "Bad request".into(),
    description: "The request you passed is missing fields or bad formatted"
      .into(),
    context: err,
    status_code: 400,
  }
}

pub fn unauthorized() -> ErrorResponse {
  ErrorResponse {
    error: "Unauthorized".into(),
    description: "Insufficient authentication for this request.".into(),
    context: None,
    status_code: 401,
  }
}

pub fn forbidden() -> ErrorResponse {
  ErrorResponse {
    error: "Forbidden".into(),
    description: "Insuficient permission to access this resource.".into(),
    context: None,
    status_code: 403,
  }
}

pub fn not_found() -> ErrorResponse {
  ErrorResponse {
    error: "Page not found".into(),
    description: "The page you have requested could not be found.".into(),
    context: None,
    status_code: 404,
  }
}

pub fn method_not_allowed() -> ErrorResponse {
  ErrorResponse {
    error: "Method Not Allowed".into(),
    description: "The requested method is not supported by this resource"
      .into(),
    context: None,
    status_code: 405,
  }
}

pub fn not_acceptable(err: Option<String>) -> ErrorResponse {
  ErrorResponse {
    error: "Not acceptable".into(),
    description: "The server has refused your request".into(),
    context: err,
    status_code: 406,
  }
}

pub fn internal_server_error(err: &std::io::Error) -> ErrorResponse {
  ErrorResponse {
    error: "Internal Server Error".into(),
    description: "The server encountered an error processing your request."
      .into(),
    context: Some(format!("{}: {}", err.kind(), err.to_string())),
    status_code: 500,
  }
}
