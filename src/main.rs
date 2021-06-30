use rocket::{Build, Rocket, State};
use std::{env, fs};

mod shrimp;

// GET /files - list of filenames and their checksums
// GET /file/metadata/<checksum> - metadata for a specific file
// GET /file/<checksum> - downloads a specific file

#[macro_use]
extern crate rocket;

#[get("/")]
fn get_index(config: &State<shrimp::ServerConfig>) -> String {
    format!("Hello World! Serving from: {}", config.serve_path)
}

fn make_server_state(config: &shrimp::ServerConfig) -> shrimp::ServerState {
    let mut result = shrimp::ServerState::new();

    let paths = fs::read_dir(&config.serve_path).unwrap();
    for path in paths {
        let p = path.unwrap();
        let file_path = String::from(p.path().to_str().unwrap());
        let file_name = String::from(p.file_name().to_str().unwrap());
        let checksum = shrimp::get_checksum(&file_path);

        result.add(file_name, checksum);
    }

    result
}

fn make_rocket(config: shrimp::ServerConfig) -> Rocket<Build> {
    let state = make_server_state(&config);

    let paths = fs::read_dir(&config.serve_path).unwrap();
    for path in paths {
        let file_name = String::from(path.unwrap().file_name().to_str().unwrap());
        println!("File in serve directory: {}", file_name);
    }

    let root_routes = routes![
        get_index,
        shrimp::get_files::get_files,
        shrimp::get_file_metadata::get_file_metadata,
        shrimp::head_file::head_file_checksum,
        shrimp::get_file::get_file_from_checksum,
    ];

    rocket::build()
        .mount("/", root_routes)
        .manage(config)
        .manage(state)
}

#[rocket::main]
async fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Requiring source path to check for files");
        return;
    }

    let mut config = shrimp::ServerConfig::new();
    config.serve_path = args.remove(1);
    if let Err(e) = make_rocket(config).launch().await {
        drop(e)
    }
}
