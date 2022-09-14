use axum::Json;
use serde::Serialize;

use crate::server::{error::*, utils::token::Token};

pub async fn whoami(Token(user): Token) -> Result<Json<Response>> {
    Ok(Json(Response {
        user: user.username,
    }))
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub user: String,
}
