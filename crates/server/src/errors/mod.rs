pub mod utils;

use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use mime::Mime;
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
  DbDeserializeError,
  NotFound,
  ReferenceNotFound(String),
  InvalidContentType(Vec<Mime>, Mime),
  MissingField(String),
  ImageNotDecodable,
  InternalServerError,
}

impl Display for ApiError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ApiError::ApiKeyNotProvided => write!(f, "ApiKeyError: API key was not provided"),
      ApiError::ApiKeyInvalid => write!(f, "ApiKeyError: Invalid API key"),
      ApiError::ApiKeyReadOnly => write!(f, "ApiKeyError: This API key is readonly"),
      ApiError::DbError | ApiError::DbDeserializeError => write!(f, "Internal Server Error"),
      ApiError::NotFound => write!(f, "Not found"),
      ApiError::ReferenceNotFound(reference) => {
        write!(f, "Reference to \"{}\" not found", reference)
      }
      ApiError::InvalidContentType(expected, actual) => {
        write!(
          f,
          "Invalid content-type \"{}\", expected one of \"{}\"",
          actual,
          expected
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("\", \"")
        )
      }
      ApiError::MissingField(field) => write!(f, "Missing field \"{}\"", field),
      ApiError::ImageNotDecodable => write!(f, "Provide image is not decodable, make sure you provide a valid image or that you use a supported image format !"),
      ApiError::InternalServerError => write!(f, "Internal Server Error"),
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
      ApiError::DbDeserializeError => String::from("DBDSE"),
      ApiError::NotFound => String::from("NTFND"),
      ApiError::ReferenceNotFound(_) => String::from("REFNF"),
      ApiError::InvalidContentType(_, _) => String::from("BADCT"),
      ApiError::MissingField(_) => String::from("FLMIS"),
      ApiError::ImageNotDecodable => String::from("IMGND"),
      ApiError::InternalServerError => String::from("INTSE"),
    }
  }

  fn http_code(&self) -> StatusCode {
    match self {
      ApiError::ApiKeyNotProvided | ApiError::ApiKeyInvalid => StatusCode::FORBIDDEN,
      ApiError::ApiKeyReadOnly => StatusCode::UNAUTHORIZED,
      ApiError::NotFound => StatusCode::NOT_FOUND,
      ApiError::ReferenceNotFound(_)
      | ApiError::InvalidContentType(_, _)
      | ApiError::MissingField(_) => StatusCode::BAD_REQUEST,
      ApiError::ImageNotDecodable => StatusCode::UNPROCESSABLE_ENTITY,
      _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

impl ResponseError for ApiError {
  fn error_response(&self) -> HttpResponse<BoxBody> {
    self.http_response()
  }
}
