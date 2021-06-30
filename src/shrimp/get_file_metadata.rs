use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use std::fs;
use std::path::Path;

use super::server_config::ServerConfig;
use super::server_state::ServerState;

#[derive(Serialize, Deserialize)]
pub struct FileMetadata {
    pub file_name: String,
    pub file_size: u64,
}

pub fn do_get_file_metadata(file_name: &str, directory: &str) -> FileMetadata {
    let file_path = Path::new(directory);
    let file_path = file_path.join(file_name);
    let fs_metadata = fs::metadata(file_path);
    if fs_metadata.is_err() {
        panic!("File doesn't exist for metadata")
    }

    let fs_metadata = fs_metadata.unwrap();

    FileMetadata {
        file_name: String::from(file_name),
        file_size: fs_metadata.len(),
    }
}

#[get("/file/metadata/<checksum>")]
pub fn get_file_metadata(
    checksum: String,
    state: &State<ServerState>,
    config: &State<ServerConfig>,
) -> String {
    let find_result = state.get_file(&checksum);
    if find_result.is_none() {
        return "{}".to_string(); // an empty json
    }

    let file_name = find_result.unwrap();
    let metadata = do_get_file_metadata(file_name, &config.serve_path);

    serde_json::to_string(&metadata).unwrap()
}
