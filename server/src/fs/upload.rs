use std::{fs, io::Write, path::Path};

use axum::{
    extract::{Multipart, Query},
    Extension,
};
use axum_auth::AuthBearer;
use futures::TryStreamExt;
use homedisk_database::Database;
use homedisk_types::{
    config::Config,
    errors::{FsError, ServerError},
    fs::upload::Pagination,
};

use crate::middleware::{find_user, validate_jwt, validate_path};

pub async fn handle(
    Extension(db): Extension<Database>,
    Extension(config): Extension<Config>,
    AuthBearer(token): AuthBearer,
    mut multipart: Multipart,
    query: Query<Pagination>,
) -> Result<(), ServerError> {
    // validate user token
    let token = validate_jwt(config.jwt.secret.as_bytes(), &token)?;

    // validate the `path` can be used
    validate_path(&query.path)?;

    // search for a user by UUID from a token
    let user = find_user(&db, &token.claims.sub).await?;

    // path to the file
    let file_path = format!(
        "{user_dir}/{request_path}",
        user_dir = user.user_dir(&config.storage.path),
        request_path = query.path
    );
    let file_path = Path::new(&file_path);

    // check if the file currently exists to avoid overwriting it
    if file_path.exists() {
        return Err(ServerError::FsError(FsError::FileAlreadyExists));
    }

    // create a directory where the file will be placed
    // e.g. path ==> `/secret/files/images/screenshot.png`
    // directories up to `{storage dir}/{username}/secret/files/images/` will be created
    if let Some(prefix) = file_path.parent() {
        fs::create_dir_all(&prefix)
            .map_err(|err| ServerError::FsError(FsError::CreateFile(err.to_string())))?
    }

    // get multipart field
    let field = multipart
        .next_field()
        .await
        .map_err(|_| ServerError::FsError(FsError::MultipartError))?
        .ok_or(ServerError::FsError(FsError::MultipartError))?;

    // create file
    let file = std::fs::File::create(&file_path)
        .map_err(|err| ServerError::FsError(FsError::CreateFile(err.to_string())))?;

    // write file
    field
        .try_fold((file, 0u64), |(mut file, written_len), bytes| async move {
            file.write_all(bytes.as_ref()).expect("write file error");

            Ok((file, written_len + bytes.len() as u64))
        })
        .await
        .map_err(|err| ServerError::FsError(FsError::WriteFile(err.to_string())))?;

    // send an empty response
    Ok(())
}
