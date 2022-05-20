use crate::errors::ApiError;
use sea_orm::{ActiveValue, DbErr, Value};
use tracing::error;

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
