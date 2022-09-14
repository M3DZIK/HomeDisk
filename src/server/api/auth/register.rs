use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

use crate::{
    database::{Database, User},
    server::error::*,
};

pub async fn register(
    Extension(db): Extension<Database>,
    request: Json<Request>,
) -> Result<Json<Response>> {
    // username must contain at least 4 characters
    if request.username.len() < 4 {
        return Err(Error::UsernameTooShort);
    }

    // username must be less than 25 characters
    if request.username.len() > 25 {
        return Err(Error::UsernameTooLong);
    }

    // password must contain at least 8 characters
    if request.password.len() < 8 {
        return Err(Error::PasswordTooShort);
    }

    let user = User::new(&request.username, &request.password, true);

    if let Err(err) = db.create_user(&user).await {
        if err.to_string().contains("UNIQUE constraint failed") {
            return Err(Error::UserAlreadyExists);
        }

        return Err(Error::Database);
    }

    Ok(Json(Response { success: true }))
}

#[derive(Debug, Deserialize)]
pub struct Request {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub success: bool,
}
