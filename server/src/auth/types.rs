#[derive(Debug, Clone)]
pub struct Request {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub enum ResponseLogin {
    LoggedIn { access_token: String },
}
