use crate::errors::utils::db_err_into_api_err;
use crate::errors::ApiError;
use crate::middlewares::api_key::WriteApiKey;
use crate::middlewares::s3::{S3ClientExt, S3ClientProvider};
use crate::server::AppState;
use crate::services::image::models::{ImageOutput, ImageUploadQuery};
use actix_multipart::Multipart;
use actix_web::{post, web, Error as ActixError, HttpResponse};
use aws_sdk_s3::model::ObjectCannedAcl::PublicRead;
use aws_sdk_s3::types::ByteStream;
use aws_smithy_http::body::SdkBody;
use futures::future::ready;
use futures::{StreamExt, TryFutureExt};
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::DynamicImage;
use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;
use std::ffi::OsStr;
use std::path::Path;
use std::sync::Arc;
use tracing::{error, warn, Instrument};
use uuid::Uuid;

#[allow(clippy::too_many_arguments)]
async fn compress_and_upload(
  s3: S3ClientExt,
  bucket: Arc<String>,
  image: Arc<DynamicImage>,
  width: u32,
  height: u32,
  filter: FilterType,
  lazy: bool,
  id: Arc<String>,
  filename: Arc<String>,
  content_type: Arc<mime::Mime>,
) -> Result<String, ApiError> {
  let p_filename = Path::new(filename.as_str());

  let image = image.resize_to_fill(width, height, filter);

  let image = {
    let mut image_output: Vec<u8> = Vec::new();
    JpegEncoder::new_with_quality(&mut image_output, 75)
      .encode_image(&image)
      .map_err(|e| {
        error!(
          error_message = format!("{:?}", e).as_str(),
          "An error occured while encoding image to json"
        );
        ApiError::InternalServerError
      })?;
    image_output
  };

  let key = format!(
    "{}__{}{}.{}",
    id,
    p_filename
      .file_stem()
      .and_then(OsStr::to_str)
      .unwrap_or("unknown"),
    if lazy { "__lazy" } else { "" },
    "jpeg"
  );

  s3.put_object()
    .bucket(bucket.to_string())
    .content_type(content_type.to_string())
    .metadata("s3_id", id.to_string())
    .metadata("filename", filename.to_string())
    .metadata("lazy", if lazy { "true" } else { "false" })
    .key(&key)
    .body(ByteStream::new(SdkBody::from(image)))
    .acl(PublicRead)
    .send()
    .await
    .map(|_res| key)
    .map_err(|e| {
      error!(
        error_message = format!("{:?}", e).as_str(),
        "An error occured while uploading object to S3"
      );
      ApiError::InternalServerError
    })
}

#[post("")]
pub async fn upload_image(
  data: web::Data<AppState>,
  s3_provider: S3ClientProvider,
  query: web::Query<ImageUploadQuery>,
  mut payload: Multipart,
  api_key: WriteApiKey,
) -> Result<HttpResponse, ActixError> {
  let settings = data.settings();
  let s3_client = s3_provider.provide();
  let s3_bucket = Arc::new(settings.s3().buckets().image().clone());
  let mut image_upload_result: Option<(String, String)> = None;

  while let Some(Ok(field)) = payload.next().await {
    match field.name() {
      "image" => {
        let content_type = field.content_type().clone();
        // ["gif", "jpeg", "ico", "png", "pnm", "tga", "tiff", "webp", "bmp", "hdr", "dxt", "dds", "farbfeld", "jpeg_rayon", "openexr"]
        let supported_mime = vec![
          mime::IMAGE_JPEG,
          mime::IMAGE_PNG,
          mime::IMAGE_SVG,
          mime::IMAGE_GIF,
          mime::IMAGE_BMP,
        ];

        let mut field = match supported_mime.contains(&content_type) {
          true => Ok(field),
          false => Err(ApiError::InvalidContentType(
            supported_mime,
            content_type.clone(),
          )),
        }?;

        let mut image: Vec<u8> = Vec::new();
        while let Some(bytes) = field.next().await {
          for byte in bytes? {
            image.push(byte)
          }
        }

        let image = image::load_from_memory(image.as_slice()).map_err(|e| {
          warn!(
            error_message = format!("{:?}", e).as_str(),
            "Cannot load image"
          );
          ApiError::ImageNotDecodable
        })?;

        let s3_id = Arc::new(Uuid::new_v4().to_string());
        let arc_filename = Arc::new(
          field
            .content_disposition()
            .get_filename()
            .map(ToString::to_string)
            .unwrap_or_else(|| "unknown".to_string()),
        );

        let arc_image = Arc::new(image);
        let arc_content_type = Arc::new(content_type);
        let result: (String, String) = tokio::try_join!(
          tokio::spawn(compress_and_upload(
            s3_client.clone(),
            s3_bucket.clone(),
            arc_image.clone(),
            1920,
            1080,
            FilterType::Triangle,
            false,
            s3_id.clone(),
            arc_filename.clone(),
            arc_content_type.clone(),
          ))
          .map_err(|e| {
            error!(
              error_message = format!("{:?}", e).as_str(),
              "An error occured while joining async task compress and upload"
            );
            ApiError::InternalServerError
          })
          .and_then(ready)
          .instrument(tracing::debug_span!("image-processing").or_current()),
          tokio::spawn(compress_and_upload(
            s3_client.clone(),
            s3_bucket.clone(),
            arc_image.clone(),
            64,
            36,
            FilterType::Nearest,
            true,
            s3_id.clone(),
            arc_filename.clone(),
            arc_content_type.clone(),
          ))
          .map_err(|e| {
            error!(
              error_message = format!("{:?}", e).as_str(),
              "An error occured while joining async task compress and upload"
            );
            ApiError::InternalServerError
          })
          .and_then(ready)
          .instrument(tracing::debug_span!("image-processing").or_current())
        )?;
        image_upload_result = Some(result);
      }
      _ => continue,
    }
  }

  let (res, res_lazy) =
    image_upload_result.ok_or_else(|| ApiError::MissingField("image".to_string()))?;

  let image_query = query.into_inner();
  let image_lazy = entity::image::ActiveModel {
    namespace: Set(api_key.namespace().to_string()),
    storage_key: Set(res_lazy),
    alt: Set(image_query.alt().clone()),
    ..Default::default()
  }
  .save(data.conn())
  .await
  .map_err(db_err_into_api_err)?;

  let image = entity::image::ActiveModel {
    namespace: Set(api_key.namespace().to_string()),
    storage_key: Set(res),
    alt: Set(image_query.alt().clone()),
    lazy_image_id: Set(Some(image_lazy.id.clone().unwrap())),
    ..Default::default()
  }
  .save(data.conn())
  .await
  .map_err(db_err_into_api_err)?;

  Ok(HttpResponse::Ok().json(ImageOutput::try_from((s3_bucket, image, image_lazy))?))
}
