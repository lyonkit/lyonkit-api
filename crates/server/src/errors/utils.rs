use crate::errors::ApiError;
use sea_orm::{ActiveValue, DbErr, Value};
use tracing::error;

pub trait IntoApiError {
    fn into_api_err(self) -> ApiError;
}

pub trait MapApiError<T> {
    fn map_api_err(self) -> Result<T, ApiError>;
}

impl IntoApiError for DbErr {
    fn into_api_err(self) -> ApiError {
        error!(
            error_message = format!("{:?}", self).as_str(),
            "An error occured while saving page"
        );
        ApiError::DbError
    }
}

impl<T, E: IntoApiError> MapApiError<T> for Result<T, E> {
    fn map_api_err(self) -> Result<T, ApiError> {
        self.map_err(IntoApiError::into_api_err)
    }
}

pub fn db_err_into_api_err(e: DbErr) -> ApiError {
    error!(
        error_message = format!("{:?}", e).as_str(),
        "An error occured while saving page"
    );
    ApiError::DbError
}

pub fn try_unwrap_active_value<T: Into<Value>>(value: ActiveValue<T>) -> Result<T, ApiError> {
    match value {
        ActiveValue::Set(v) | ActiveValue::Unchanged(v) => Ok(v),
        ActiveValue::NotSet => {
            error!("DB Deserialize Error : encountered NotSet value");
            Err(ApiError::DbDeserializeError)
        }
    }
}

pub trait TryUnwrapActiveValue<T> {
    fn try_unwrap_av(self) -> Result<T, ApiError>;
}

impl<T: Into<Value>> TryUnwrapActiveValue<T> for ActiveValue<T> {
    fn try_unwrap_av(self) -> Result<T, ApiError> {
        try_unwrap_active_value(self)
    }
}
