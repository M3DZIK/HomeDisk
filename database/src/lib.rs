#![doc(html_root_url = "https://homedisk-doc.medzik.xyz")]

mod sqlite;

/// Imported from [homedisk_types::database::User].
pub use homedisk_types::database::User;
/// Imported from [homedisk_types::errors::DatabaseError].
pub use homedisk_types::errors::DatabaseError as Error;
/// Imported from [homedisk_types::errors::DatabaseResult].
pub use homedisk_types::errors::DatabaseResult as Result;
pub use sqlite::*;
