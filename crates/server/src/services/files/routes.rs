use std::time::Duration;

use actix_web::{delete, get, post, put, web};
use aws_sdk_s3::{model::ObjectCannedAcl::PublicRead, presigning::config::PresigningConfig};

use crate::{
    errors::ApiError,
    middlewares::{
        api_key::{ApiKey, WriteApiKey},
        s3::S3ClientProvider,
    },
    server::AppState,
    services::files::{
        models::{
            FileDeleteResponse, FileFilter, FileInput, FileOutputList, FileUpdateInput,
            IntoFileOutputList, UploadFileOutput,
        },
        repository::FilesRepository,
    },
};

#[post("")]
pub async fn create_file(
    data: web::Data<AppState>,
    api_key: WriteApiKey,
    s3_provider: S3ClientProvider,
    input: web::Json<FileInput>,
) -> Result<UploadFileOutput, ApiError> {
    let maybe_content_type = input.file().content_type.clone();
    let content_length = *input.file().content_length();
    let max_file_size = 50_000_000_u32;
    if content_length > max_file_size {
        return Err(ApiError::FileTooBig(max_file_size, content_length));
    }

    let model = data
        .conn()
        .create_file(api_key.namespace().as_str(), input.into_inner())
        .await?;

    let settings = data.settings();
    let s3_client = s3_provider.provide();

    let mut s3_request = s3_client
        .put_object()
        .content_length(content_length.into())
        .bucket(settings.s3().buckets().file())
        .key(model.storage_key())
        .acl(PublicRead);

    if let Some(content_type) = maybe_content_type {
        s3_request = s3_request.content_type(content_type)
    }

    let presigned_url = s3_request
        .presigned(
            PresigningConfig::builder()
                .expires_in(Duration::from_secs(180))
                .build()
                .unwrap(),
        )
        .await
        .map_err(|_e| ApiError::InternalServerError)?
        .uri()
        .to_string();

    let public_url = format!(
        "{}/{}/{}",
        settings.s3().base_url(),
        settings.s3().buckets().file(),
        model.storage_key()
    );

    Ok(UploadFileOutput {
        id: model.id,
        upload_url: Some(presigned_url),
        key: model.storage_key,
        tags: model.tags,
        metadata: model.metadata,
        public_url,
        created_at: model.created_at,
        updated_at: model.updated_at,
    })
}

#[get("")]
pub async fn list_files(
    data: web::Data<AppState>,
    api_key: ApiKey,
    filter: web::Query<FileFilter>,
) -> Result<FileOutputList, ApiError> {
    let files = data
        .conn()
        .find_files_by_tag(api_key.namespace().as_str(), filter.tag())
        .await?;

    let s3_settings = data.settings().s3();
    let s3_base_url = format!(
        "{}/{}",
        s3_settings.base_url(),
        s3_settings.buckets().file()
    );

    Ok(files.into_file_output_list(s3_base_url.as_str()))
}

#[put("/{id}")]
pub async fn update_file(
    data: web::Data<AppState>,
    api_key: WriteApiKey,
    path: web::Path<i32>,
    s3_provider: S3ClientProvider,
    payload: web::Json<FileUpdateInput>,
) -> Result<UploadFileOutput, ApiError> {
    let payload = payload.into_inner();
    let model = data
        .conn()
        .update_file(api_key.namespace(), &path.into_inner(), payload.clone())
        .await?;

    let settings = data.settings();
    let s3_client = s3_provider.provide();

    let mut presigned_url = None;
    if let Some(file) = payload.file() {
        let maybe_content_type = file.content_type.clone();
        let content_length = *file.content_length();

        let mut s3_request = s3_client
            .put_object()
            .content_length(content_length.into())
            .bucket(settings.s3().buckets().file())
            .key(model.storage_key())
            .acl(PublicRead);

        if let Some(content_type) = maybe_content_type {
            s3_request = s3_request.content_type(content_type)
        }

        presigned_url = Some(
            s3_request
                .presigned(
                    PresigningConfig::builder()
                        .expires_in(Duration::from_secs(180))
                        .build()
                        .unwrap(),
                )
                .await
                .map_err(|_e| ApiError::InternalServerError)?
                .uri()
                .to_string(),
        );
    }

    let public_url = format!(
        "{}/{}/{}",
        settings.s3().base_url(),
        settings.s3().buckets().file(),
        model.storage_key()
    );

    Ok(UploadFileOutput {
        id: model.id,
        upload_url: presigned_url,
        key: model.storage_key,
        tags: model.tags,
        metadata: model.metadata,
        public_url,
        created_at: model.created_at,
        updated_at: model.updated_at,
    })
}

#[delete("/{id}")]
pub async fn delete_file(
    data: web::Data<AppState>,
    api_key: WriteApiKey,
    path: web::Path<i32>,
    s3_provider: S3ClientProvider,
) -> Result<FileDeleteResponse, ApiError> {
    let model = data
        .conn()
        .delete_file(api_key.namespace(), &path.into_inner())
        .await?;

    let s3_client = s3_provider.provide();

    s3_client
        .delete_object()
        .bucket(data.settings().s3().buckets().file())
        .key(model.storage_key())
        .send()
        .await
        .map_err(|_e| ApiError::InternalServerError)?;

    return Ok(FileDeleteResponse { id: *model.id() });
}
