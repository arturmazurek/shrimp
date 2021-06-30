use std::collections::HashMap;

use super::file_types::{FileChecksum, FilesChecksums};

pub struct ServerState {
    files_to_checksums: HashMap<String, String>,
    checksums_to_files: HashMap<String, String>,
}

impl ServerState {
    pub fn new() -> ServerState {
        ServerState {
            files_to_checksums: HashMap::new(),
            checksums_to_files: HashMap::new(),
        }
    }

    pub fn add(&mut self, file_name: String, checksum: String) {
        self.files_to_checksums
            .insert(file_name.clone(), checksum.clone());
        self.checksums_to_files.insert(checksum, file_name);
    }

    pub fn get_file(&self, checksum: &str) -> Option<&String> {
        self.checksums_to_files.get(checksum)
    }

    pub fn get_file_checksums(&self) -> FilesChecksums {
        let mut file_infos = FilesChecksums::new();

        for (file, hash) in &self.files_to_checksums {
            let file_info = FileChecksum {
                name: String::from(file),
                checksum: String::from(hash),
            };
            file_infos.files.push(file_info);
        }

        file_infos
    }
}
