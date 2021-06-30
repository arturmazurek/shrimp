use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FileChecksum {
    pub name: String,
    pub checksum: String,
}

#[derive(Serialize, Deserialize)]
pub struct FilesChecksums {
    pub files: Vec<FileChecksum>,
}

impl FilesChecksums {
    pub fn new() -> FilesChecksums {
        FilesChecksums { files: Vec::new() }
    }
}
