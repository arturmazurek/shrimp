pub struct ServerConfig {
    pub serve_path: String,
}

impl ServerConfig {
    pub fn new() -> ServerConfig {
        ServerConfig {
            serve_path: String::new(),
        }
    }
}
