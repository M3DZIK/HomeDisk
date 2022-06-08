//! Error types

mod auth;
#[cfg(feature = "database")]
mod database;
mod fs;
mod server;

pub use auth::Error as AuthError;
pub use database::Error as DatabaseError;
pub use fs::Error as FsError;
pub use server::Error as ServerError;
