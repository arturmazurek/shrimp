use sha2::{Digest, Sha256};

pub mod server_state;
pub use server_state::ServerState;

pub mod server_config;
pub use server_config::ServerConfig;

pub mod file_types;
pub use file_types::FilesChecksums;

pub mod get_file;
pub mod get_file_metadata;
pub mod get_files;
pub mod head_file;

pub fn get_checksum(file_path: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(file_path);
    let hash = hasher.finalize();
    let result = format!("{:X}", hash);
    result
}
