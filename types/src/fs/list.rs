//! HTTP `/fs/list` Request, Response, FileInfo and DirInfo types

use serde::{Deserialize, Serialize};

/// `/fs/list` Request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    pub path: String,
}

/// `/fs/list` Response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    pub files: Vec<FileInfo>,
    pub dirs: Vec<DirInfo>,
}

/// Info about a file
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    /// File name
    pub name: String,
    /// File size
    pub size: String,
    /// Latest modification of this file
    pub modified: String,
}

/// Info about a directory
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirInfo {
    /// Directory name
    pub name: String,
    /// Directory size (size of all files in directory)
    pub size: String,
}
