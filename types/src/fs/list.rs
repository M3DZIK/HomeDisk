use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    pub files: Vec<FileInfo>,
    pub dirs: Vec<DirInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub name: String,
    pub size: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirInfo {
    pub name: String,
    pub size: String,
}
