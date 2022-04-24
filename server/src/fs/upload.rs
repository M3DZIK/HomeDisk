use std::{fs, path::Path};

use axum::{extract::rejection::JsonRejection, Extension, Json};
use axum_auth::AuthBearer;
use homedisk_database::{Database, Error};
use homedisk_types::{
    config::types::Config,
    errors::{AuthError, FsError, ServerError},
    fs::upload::{Request, Response},
};

use crate::middleware::{validate_json, validate_jwt};

pub async fn handle(
    db: Extension<Database>,
    config: Extension<Config>,
    AuthBearer(token): AuthBearer,
    request: Result<Json<Request>, JsonRejection>,
) -> Result<Json<Response>, ServerError> {
    let Json(request) = validate_json::<Request>(request)?;
    let token = validate_jwt(config.jwt.secret.as_bytes(), &token)?;

    let response = match db.find_user_by_id(token.claims.sub).await {
        Ok(res) => {
            // get file content
            let content = base64::decode(request.content)
                .map_err(|err| ServerError::FsError(FsError::Base64(err.to_string())))?;

            // check if file already exists
            if Path::new(&request.path).exists() {
                return Err(ServerError::FsError(FsError::FileAlreadyExists));
            }

            let folder_path = format!(
                "{path}/{username}",
                path = config.storage.path,
                username = res.username,
            );
            let path = format!("{folder_path}/{filepath}", filepath = request.path);

            // create user directory
            fs::create_dir_all(&folder_path).unwrap();

            // write file
            fs::write(&path, content)
                .map_err(|err| ServerError::FsError(FsError::WriteFile(err.to_string())))?;

            Response { uploaded: true }
        }

        Err(err) => match err {
            Error::UserNotFound => return Err(ServerError::AuthError(AuthError::UserNotFound)),
            _ => {
                return Err(ServerError::AuthError(AuthError::UnknowError(
                    err.to_string(),
                )))
            }
        },
    };

    Ok(Json(response))
}
