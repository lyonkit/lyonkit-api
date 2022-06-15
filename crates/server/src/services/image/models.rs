use crate::config::SETTINGS;
use crate::errors::utils::try_unwrap_active_value;
use crate::errors::ApiError;
use chrono::{DateTime, Utc};
use getset::Getters;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Clone, Getters)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct ImageOutput {
  id: i32,
  public_url: String,
  lazy_image: LazyImageOutput,
  alt: Option<String>,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
}

#[derive(Serialize, Clone, Getters)]
#[serde(rename_all = "camelCase")]
#[getset(get = "pub")]
pub struct LazyImageOutput {
  id: i32,
  public_url: String,
  alt: Option<String>,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
}

impl
  TryFrom<(
    Arc<String>,
    entity::image::ActiveModel,
    entity::image::ActiveModel,
  )> for ImageOutput
{
  type Error = ApiError;

  fn try_from(
    (bucket, image, lazy_image): (
      Arc<String>,
      entity::image::ActiveModel,
      entity::image::ActiveModel,
    ),
  ) -> Result<Self, Self::Error> {
    Ok(ImageOutput {
      id: try_unwrap_active_value(image.id)?,
      public_url: format!(
        "{base_url}/{bucket}/{id}",
        base_url = (*SETTINGS).s3().base_url(),
        id = try_unwrap_active_value(image.storage_key)?
      ),
      lazy_image: LazyImageOutput {
        id: try_unwrap_active_value(lazy_image.id)?,
        public_url: format!(
          "{base_url}/{bucket}/{id}",
          base_url = (*SETTINGS).s3().base_url(),
          id = try_unwrap_active_value(lazy_image.storage_key)?
        ),
        alt: try_unwrap_active_value(lazy_image.alt)?,
        created_at: try_unwrap_active_value(lazy_image.created_at)?,
        updated_at: try_unwrap_active_value(lazy_image.updated_at)?,
      },
      alt: try_unwrap_active_value(image.alt)?,
      created_at: try_unwrap_active_value(image.created_at)?,
      updated_at: try_unwrap_active_value(image.updated_at)?,
    })
  }
}

impl From<(Arc<String>, entity::image::Model, entity::image::Model)> for ImageOutput {
  fn from(
    (bucket, image, lazy_image): (Arc<String>, entity::image::Model, entity::image::Model),
  ) -> Self {
    ImageOutput {
      id: image.id,
      public_url: format!(
        "{base_url}/{bucket}/{id}",
        base_url = (*SETTINGS).s3().base_url(),
        id = image.storage_key
      ),
      lazy_image: LazyImageOutput {
        id: lazy_image.id,
        public_url: format!(
          "{base_url}/{bucket}/{id}",
          base_url = (*SETTINGS).s3().base_url(),
          id = lazy_image.storage_key
        ),
        alt: lazy_image.alt,
        created_at: lazy_image.created_at,
        updated_at: lazy_image.updated_at,
      },
      alt: image.alt,
      created_at: image.created_at,
      updated_at: image.updated_at,
    }
  }
}

#[derive(Deserialize, Serialize, Getters)]
#[getset(get = "pub")]
pub struct ImageUploadQuery {
  alt: Option<String>,
}
