#![doc(html_root_url = "https://homedisk-doc.medzik.xyz")]

pub mod auth;
pub mod config;
#[cfg(feature = "database")]
pub mod database;
pub mod errors;
pub mod fs;

mod macros;
