use std::fmt::{Debug, Display, Formatter};

use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use humansize::{FormatSize, DECIMAL};
use mime::Mime;
use sea_orm::DbErr;
use serde_json::json;

pub mod utils;

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
    PatchNotNullable(String),
    PatchAtLeastOneField,
    GitError,
    GitTokenMissing,
    GitBodyUnparseable,
    /// First is max size, second is actual size
    FileTooBig(u32, u32),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::ApiKeyNotProvided => write!(f, "ApiKeyError: API key was not provided"),
            ApiError::ApiKeyInvalid => write!(f, "ApiKeyError: Invalid API key"),
            ApiError::ApiKeyReadOnly => write!(f, "ApiKeyError: This API key is readonly"),
            ApiError::DbError
            | ApiError::DbDeserializeError
            | ApiError::GitError
            | ApiError::GitBodyUnparseable
            | ApiError::GitTokenMissing => write!(f, "Internal Server Error"),
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
            ApiError::ImageNotDecodable => write!(
                f,
                "Provide image is not decodable, make sure you provide a valid image or that you \
                 use a supported image format !"
            ),
            ApiError::InternalServerError => write!(f, "Internal Server Error"),
            ApiError::PatchNotNullable(field) => write!(f, "Field {field} is not nullable"),
            ApiError::PatchAtLeastOneField => write!(f, "You must patch at least one field"),
            ApiError::FileTooBig(max_size, actual_size) => write!(
                f,
                "The file you are trying to upload is too big (your file is {} but the maximum \
                 size is {})",
                actual_size.format_size(DECIMAL),
                max_size.format_size(DECIMAL)
            ),
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
            ApiError::PatchNotNullable(_) => String::from("PTHNN"),
            ApiError::PatchAtLeastOneField => String::from("PTHOF"),
            ApiError::GitError => String::from("GITER"),
            ApiError::GitBodyUnparseable => String::from("GITBU"),
            ApiError::GitTokenMissing => String::from("GITTM"),
            ApiError::FileTooBig(_, _) => String::from("FTBIG"),
        }
    }

    fn http_code(&self) -> StatusCode {
        match self {
            ApiError::ApiKeyNotProvided | ApiError::ApiKeyInvalid => StatusCode::FORBIDDEN,
            ApiError::ApiKeyReadOnly => StatusCode::UNAUTHORIZED,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::ReferenceNotFound(_)
            | ApiError::InvalidContentType(_, _)
            | ApiError::PatchNotNullable(_)
            | ApiError::PatchAtLeastOneField
            | ApiError::MissingField(_) => StatusCode::BAD_REQUEST,
            ApiError::ImageNotDecodable => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::FileTooBig(_, _) => StatusCode::PAYLOAD_TOO_LARGE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        self.http_response()
    }
}

impl From<DbErr> for ApiError {
    fn from(_value: DbErr) -> Self {
        ApiError::DbError
    }
}
