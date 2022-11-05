use crate::{
    config::Settings,
    errors::{utils::MapApiError, ApiError},
    middlewares::{
        api_key::{ApiKey, WriteApiKey},
        s3::{S3ClientExt, S3ClientProvider},
    },
    server::AppState,
    services::image::models::{ImageOutput, ImageUploadQuery},
};
use actix_multipart::Multipart;
use actix_web::{delete, get, post, web, Error as ActixError, HttpResponse};
use aws_sdk_s3::{model::ObjectCannedAcl::PublicRead, types::ByteStream};
use aws_smithy_http::body::SdkBody;
use deunicode::deunicode;
use entity::image::{Column, Entity, LazyImageLink, Model};
use futures::{future::ready, StreamExt, TryFutureExt};
use image::{codecs::jpeg::JpegEncoder, imageops::FilterType, DynamicImage};
use sea_orm::{prelude::*, ActiveValue::Set};
use std::{ffi::OsStr, path::Path, sync::Arc};
use tracing::{error, info_span, warn, Instrument};
use uuid::Uuid;

#[get("")]
pub async fn list_images(
    data: web::Data<AppState>,
    api_key: ApiKey,
) -> Result<HttpResponse, ActixError> {
    let settings = data.settings();

    let images: Vec<ImageOutput> = Entity::find()
        .filter(Column::Namespace.eq(api_key.namespace().to_string()))
        .filter(Column::LazyImageId.is_not_null())
        .find_also_linked(LazyImageLink)
        .all(data.conn())
        .await
        .map_api_err()?
        .iter()
        .map(|(img, lz_img_opt): &(Model, Option<Model>)| {
            ImageOutput::from((
                Arc::new(settings.s3().buckets().image().to_string()),
                img.clone(),
                lz_img_opt
                    .clone()
                    .expect("Query should not return null lazy image"),
            ))
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(images))
}

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

    let resize = image.width() > width || image.height() > height;
    let image = image.resize_to_fill(
        if resize { width } else { image.width() },
        if resize { height } else { image.height() },
        filter,
    );

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

    let file_stem = p_filename
        .file_stem()
        .and_then(OsStr::to_str)
        .map(|v| {
            // Cleanup filename
            deunicode(v)
                .chars()
                .filter(|c| c.is_ascii())
                .map(|c| match c {
                    ' ' => '_',
                    _ => c,
                })
                .collect::<String>()
        })
        .unwrap_or_else(|| "unknown".to_string());

    let key = format!("{id}__{file_stem}{}.jpeg", if lazy { "__lazy" } else { "" },);

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
                // ["gif", "jpeg", "ico", "png", "pnm", "tga", "tiff", "webp", "bmp", "hdr",
                // "dxt", "dds", "farbfeld", "jpeg_rayon", "openexr"]
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

                let s3_id: Arc<String> = Uuid::new_v4().to_string().into();
                let arc_filename: Arc<String> = field
                    .content_disposition()
                    .get_filename()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "unknown".to_string())
                    .into();

                let arc_image = Arc::new(image);
                let arc_content_type = Arc::new(content_type);
                let result: (String, String) = tokio::try_join!(
                    tokio::spawn(
                        compress_and_upload(
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
                        )
                        .instrument(
                            info_span!(
                                "MAIN_IMAGE_PROCESSING",
                                image_bucket = format!("{:?}", s3_bucket).as_str(),
                                image_width = 1920_u32,
                                image_height = 1080_u32,
                                image_filter = format!("{:?}", FilterType::Triangle).as_str(),
                                image_lazy = false,
                                image_filename = format!("{:?}", arc_filename).as_str(),
                                image_content_type = format!("{:?}", arc_content_type).as_str()
                            )
                            .or_current()
                        )
                    )
                    .map_err(|e| {
                        error!(
                            error_message = format!("{:?}", e).as_str(),
                            "An error occured while joining async task compress and upload"
                        );
                        ApiError::InternalServerError
                    })
                    .and_then(ready),
                    tokio::spawn(
                        compress_and_upload(
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
                        )
                        .instrument(
                            info_span!(
                                "LAZY_IMAGE_PROCESSING",
                                image_bucket = format!("{:?}", s3_bucket).as_str(),
                                image_width = 64_u32,
                                image_height = 36_u32,
                                image_filter = format!("{:?}", FilterType::Nearest).as_str(),
                                image_lazy = true,
                                image_filename = format!("{:?}", arc_filename).as_str(),
                                image_content_type = format!("{:?}", arc_content_type).as_str()
                            )
                            .or_current()
                        )
                    )
                    .map_err(|e| {
                        error!(
                            error_message = format!("{:?}", e).as_str(),
                            "An error occured while joining async task compress and upload"
                        );
                        ApiError::InternalServerError
                    })
                    .and_then(ready)
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
    .map_api_err()?;

    let image = entity::image::ActiveModel {
        namespace: Set(api_key.namespace().to_string()),
        storage_key: Set(res),
        alt: Set(image_query.alt().clone()),
        lazy_image_id: Set(Some(image_lazy.id.clone().unwrap())),
        ..Default::default()
    }
    .save(data.conn())
    .await
    .map_api_err()?;

    Ok(HttpResponse::Ok().json(ImageOutput::try_from((s3_bucket, image, image_lazy))?))
}

#[delete("/{id}")]
pub async fn delete_image(
    data: web::Data<AppState>,
    api_key: WriteApiKey,
    path_id: web::Path<i32>,
) -> Result<HttpResponse, ActixError> {
    let settings: &Settings = data.settings();
    let id = path_id.into_inner();

    let (image, lz_image) = Entity::find()
        .filter(Column::Namespace.eq(api_key.namespace().to_string()))
        .filter(Column::Id.eq(id))
        .filter(Column::LazyImageId.is_not_null())
        .find_also_linked(LazyImageLink)
        .one(data.conn())
        .await
        .map_api_err()?
        .and_then(|(img, lz_img_opt): (Model, Option<Model>)| {
            lz_img_opt.map(|lz_img| (img, lz_img))
        })
        .ok_or(ApiError::NotFound)?;

    image.clone().delete(data.conn()).await.map_api_err()?;

    Ok(HttpResponse::Ok().json(ImageOutput::from((
        Arc::new(settings.s3().buckets().image().clone()),
        image,
        lz_image,
    ))))
}
