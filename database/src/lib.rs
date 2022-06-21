#![doc(html_root_url = "https://homedisk-doc.medzik.xyz")]

mod sqlite;

pub use homedisk_types::database::User;
pub use homedisk_types::errors::DatabaseError as Error;
pub use sqlite::*;
