use super::services::fetch_git_json_file;
use crate::errors::utils::MapApiError;
use crate::errors::ApiError;
use crate::middlewares::api_key::WriteApiKey;
use crate::server::AppState;
use crate::services::git_json_file::models::GitCommitPayload;
use crate::services::git_json_file::services::GITHUB_CLIENT;
use actix_web::{get, put, web, Error as ActixError, HttpResponse};
use entity::git_auth::{Column, Entity};
use sea_orm::prelude::*;
use tracing::{error, info};

#[get("/{path}*")]
pub async fn get_git_json_file(
    app_data: web::Data<AppState>,
    path: web::Path<String>,
    api_key: WriteApiKey,
) -> Result<HttpResponse, ActixError> {
    // On récupère l'org et le repos depuis l'API KEY
    let git_auth = Entity::find()
        .filter(Column::Namespace.eq(api_key.namespace().to_owned()))
        .one(app_data.conn())
        .await
        .map_api_err()?
        .ok_or(ApiError::GitTokenMissing)?;

    let org = &git_auth.organisation;
    let repo = &git_auth.repository;
    let github_token = &git_auth.github_token;

    // On vérifie que le fichier fait partie des editables_files
    let filepath = format!("/{path}");
    if !git_auth.editable_files.inner().contains(&filepath) {
        info!(
            authorized_files = git_auth.editable_files.inner().join("///"),
            requested_file = &filepath,
            "Someone tried to access some unwanted file"
        );
        return Err(ApiError::NotFound.into());
    }

    // On requete l'api github
    let file = fetch_git_json_file(
        &path.into_inner(),
        api_key.namespace(),
        org,
        repo,
        github_token,
    )
    .await?;

    // On retourne
    Ok(HttpResponse::Ok().json(file.content()))
}

#[put("/{path}*")]
pub async fn update_git_json_file(
    app_data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<serde_json::Value>,
    api_key: WriteApiKey,
) -> Result<HttpResponse, ActixError> {
    // On récupère le git auth associé à la clé d'API
    let git_auth = Entity::find()
        .filter(Column::Namespace.eq(api_key.namespace().to_owned()))
        .one(app_data.conn())
        .await
        .map_api_err()?
        .ok_or(ApiError::GitTokenMissing)?;

    let org = &git_auth.organisation;
    let repo = &git_auth.repository;
    let github_token = &git_auth.github_token;

    // On vérifie que le fichier fait partie des editables_files
    let filepath = format!("/{path}");
    if !git_auth.editable_files.inner().contains(&filepath) {
        info!(
            authorized_files = git_auth.editable_files.inner().join("///"),
            requested_file = &filepath,
            "Someone tried to access some unwanted file"
        );
        return Err(ApiError::NotFound.into());
    }

    // On requete l'api github
    let inner_path = path.into_inner();
    let content =
        fetch_git_json_file(&inner_path, api_key.namespace(), org, repo, github_token).await?;

    let updated_content = body.into_inner();
    let commit = GitCommitPayload::builder()
        .content(base64::encode(format!("{:#}\n", &updated_content)))
        .message(format!("chore: API update of {inner_path}"))
        .branch("main".to_string())
        .sha(content.sha().clone())
        .build();

    let url = format!("https://api.github.com/repos/{org}/{repo}/contents/{inner_path}");
    let response = GITHUB_CLIENT
        .put(&url)
        .header("Authorization", github_token)
        .header(
            "User-Agent",
            format!("LyonkitApi ({})", api_key.namespace()),
        )
        .json(&commit)
        .send()
        .await
        .map_err(|err| {
            error!(
                url = &url,
                error = format!("{:?}", &err),
                "An error occured while updating git file"
            );
            ApiError::GitError
        })?;

    let text = response.text().await.unwrap();
    dbg!(text);

    Ok(HttpResponse::Ok().json(updated_content))
}
