use rocket::State;

use super::server_state::ServerState;

#[get("/files")]
pub fn get_files(state: &State<ServerState>) -> String {
    let file_infos = state.get_file_checksums();

    serde_json::to_string(&file_infos).unwrap()
}
