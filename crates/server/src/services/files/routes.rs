use std::time::Duration;

use actix_web::{get, post, web};
use aws_sdk_s3::{model::ObjectCannedAcl::PublicRead, presigning::config::PresigningConfig};

use crate::{
    errors::ApiError,
    middlewares::{
        api_key::{ApiKey, WriteApiKey},
        s3::S3ClientProvider,
    },
    server::AppState,
    services::files::{
        models::{FileFilter, FileInput, FileOutputList, IntoFileOutputList, UploadFileOutput},
        repository::FilesRepository,
    },
};

#[post("")]
pub async fn create_file(
    data: web::Data<AppState>,
    api_key: WriteApiKey,
    s3_provider: S3ClientProvider,
    file_data: web::Json<FileInput>,
) -> Result<UploadFileOutput, ApiError> {
    let maybe_content_type = file_data.content_type.clone();
    let content_length = *file_data.content_length();
    let max_file_size = 50_000_000_u32;
    if content_length > max_file_size {
        return Err(ApiError::FileTooBig(max_file_size, content_length));
    }

    let model = data
        .conn()
        .create_file(api_key.namespace().as_str(), file_data.into_inner())
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
        upload_url: presigned_url,
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
        .find_all_by_tag(api_key.namespace().as_str(), filter.tag())
        .await?;

    let s3_settings = data.settings().s3();
    let s3_base_url = format!(
        "{}/{}",
        s3_settings.base_url(),
        s3_settings.buckets().file()
    );

    Ok(files.into_file_output_list(s3_base_url.as_str()))
}
