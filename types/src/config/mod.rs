//! # Configuration file
//!
//! Path to a config file is `config.toml` in the system configuration directory (`~/.config/homedisk/config.toml`).
//!
//! Example config:
//!
//! ```toml
//! [http]
//! host = "0.0.0.0" # HTTP Host
//! port = 8080 # HTTP Port
//! cors = ["homedisk.medzik.xyz"] # Domains allowed for CORS
//!
//! [jwt]
//! secret = "secret key used to sign tokens" # JWT Secret string (used to sign tokens)
//! expires = 24 # Token expiration time (in hours)
//!
//! [storage]
//! path = "/home/homedisk" # # Directory where user files will be stored
//! ```
//!
//! ## External docs
//! - [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS)

mod types;

pub use types::*;
