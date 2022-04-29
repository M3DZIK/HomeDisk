use std::fs;

use axum::{extract::rejection::JsonRejection, Extension, Json};
use axum_auth::AuthBearer;
use byte_unit::Byte;
use homedisk_database::{Database, Error};
use homedisk_types::{
    config::types::Config,
    errors::{AuthError, FsError, ServerError},
    fs::list::{FileInfo, Request, Response},
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
            let user_path = format!(
                "{path}/{username}/{request_path}",
                path = config.storage.path,
                username = res.username,
                request_path = request.path,
            );

            let paths = fs::read_dir(&user_path)
                .map_err(|err| ServerError::FsError(FsError::ReadDir(err.to_string())))?;

            let mut files = vec![];
            let mut dirs = vec![];

            for path in paths {
                let path = path
                    .map_err(|err| ServerError::FsError(FsError::UnknowError(err.to_string())))?;
                let metadata = path
                    .metadata()
                    .map_err(|err| ServerError::FsError(FsError::UnknowError(err.to_string())))?;

                let name = path.path().display().to_string().replace(&user_path, "");
                let filesize = Byte::from_bytes(metadata.len().into()).get_appropriate_unit(true);

                if metadata.is_dir() {
                    dirs.push(name)
                } else {
                    files.push(FileInfo {
                        name,
                        size: filesize.to_string(),
                    })
                }
            }

            Response { files, dirs }
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
