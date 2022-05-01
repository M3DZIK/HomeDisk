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

    // `path` cannot contain `..`
    // to prevent attack attempts because by using a `..` you can access the previous folder
    if request.path.contains("..") {
        return Err(ServerError::FsError(FsError::ReadDir(
            "the `path` must not contain `..`".to_string(),
        )));
    }

    let response = match db.find_user_by_id(token.claims.sub).await {
        Ok(res) => {
            // get file content
            let content = base64::decode(request.content)
                .map_err(|err| ServerError::FsError(FsError::Base64(err.to_string())))?;

            let path = format!(
                "{path}/{username}/{filepath}",
                path = config.storage.path,
                username = res.username,
                filepath = request.path
            );
            let path = Path::new(&path);

            // check if the file currently exists to avoid overwriting it
            if path.exists() {
                return Err(ServerError::FsError(FsError::FileAlreadyExists));
            }

            // create a directory where the file will be placed
            // e.g. path ==> `/secret/files/images/screenshot.png`
            // directories up to `/home/homedisk/{username}/secret/files/images/` will be created
            match path.parent() {
                Some(prefix) => fs::create_dir_all(&prefix).unwrap(),
                None => (),
            }

            // write file
            fs::write(&path, &content)
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
