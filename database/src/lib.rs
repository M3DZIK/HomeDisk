#![doc(html_root_url = "https://homedisk-doc.medzik.xyz")]

mod sqlite;

pub use homedisk_types::{
    database::User,
    errors::{DatabaseError as Error, DatabaseResult as Result},
};
pub use sqlite::*;
