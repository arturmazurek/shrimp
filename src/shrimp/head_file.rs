use rocket::fs::NamedFile;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::State;
use std::path::Path;

use super::server_config::ServerConfig;
use super::server_state::ServerState;

pub struct AcceptRangesFileWrapper(NamedFile);

impl<'r> Responder<'r, 'static> for AcceptRangesFileWrapper {
    fn respond_to(self, req: &Request) -> response::Result<'static> {
        Response::build_from(self.0.respond_to(req)?)
            .raw_header("Accept-Ranges", "bytes")
            .ok()
    }
}

#[head("/file/<checksum>")]
pub async fn head_file_checksum(
    checksum: String,
    state: &State<ServerState>,
    config: &State<ServerConfig>,
) -> Option<AcceptRangesFileWrapper> {
    let file_path = Path::new(&config.serve_path);
    let file_path = file_path.join(state.get_file(&checksum).unwrap());

    NamedFile::open(file_path)
        .await
        .ok()
        .map(AcceptRangesFileWrapper)
}
