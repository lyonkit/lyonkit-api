use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use serde_json::json;
use std::fmt::{Debug, Display, Formatter};

pub trait ApiErrorTrait: Display + Debug {
  fn error_code(&self) -> String;
  fn http_code(&self) -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
  }
  fn http_response(&self) -> HttpResponse<BoxBody> {
    HttpResponse::build(self.http_code()).json(json!({
      "code": self.error_code(),
      "message": format!("{}", self),
    }))
  }
}

#[derive(Debug, Clone)]
pub enum ApiError {
  ApiKeyNotProvided,
  ApiKeyInvalid,
  ApiKeyReadOnly,
  DbError,
}

impl Display for ApiError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ApiError::ApiKeyNotProvided => write!(f, "ApiKeyError: API key was not provided"),
      ApiError::ApiKeyInvalid => write!(f, "ApiKeyError: Invalid API key"),
      ApiError::ApiKeyReadOnly => write!(f, "ApiKeyError: This API key is readonly"),
      ApiError::DbError => write!(f, "Internal Server Error"),
    }
  }
}

impl ApiErrorTrait for ApiError {
  fn error_code(&self) -> String {
    match self {
      ApiError::ApiKeyNotProvided => String::from("AKNPV"),
      ApiError::ApiKeyInvalid => String::from("AKINV"),
      ApiError::ApiKeyReadOnly => String::from("AKIRO"),
      ApiError::DbError => String::from("DBERR"),
    }
  }

  fn http_code(&self) -> StatusCode {
    match self {
      ApiError::ApiKeyNotProvided => StatusCode::FORBIDDEN,
      ApiError::ApiKeyInvalid => StatusCode::FORBIDDEN,
      ApiError::ApiKeyReadOnly => StatusCode::UNAUTHORIZED,
      _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

impl ResponseError for ApiError {
  fn error_response(&self) -> HttpResponse<BoxBody> {
    self.http_response()
  }
}
