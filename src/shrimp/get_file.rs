use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::status::BadRequest;
use rocket::response::{self, Responder, Response};
use rocket::State;
use std::convert::TryInto;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use super::server_config::ServerConfig;
use super::server_state::ServerState;

pub struct RangeHeader(u64, u64);

#[derive(Debug)]
pub enum RangeHeaderError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RangeHeader {
    type Error = RangeHeaderError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn get_range_outcome(
            key: &str,
        ) -> Outcome<RangeHeader, <RangeHeader as FromRequest>::Error> {
            let tokens: Vec<&str> = key.split('=').collect();
            if tokens[0] != "bytes" {
                return Outcome::Failure((Status::BadRequest, RangeHeaderError::Invalid));
            }

            let tokens: Vec<&str> = tokens[1].split('-').collect();
            let first: u64 = tokens[0].parse().unwrap();
            let last: u64 = tokens[1].parse().unwrap();

            let range_header = RangeHeader(first, last);
            Outcome::Success(range_header)
        }

        match req.headers().get_one("Range") {
            None => Outcome::Failure((Status::BadRequest, RangeHeaderError::Missing)),
            Some(range_header) => get_range_outcome(range_header),
        }
    }
}

pub struct FileDataRange(Vec<u8>);

impl<'r> Responder<'r, 'static> for FileDataRange {
    fn respond_to(self, req: &Request) -> response::Result<'static> {
        Response::build_from(self.0.respond_to(req)?)
            .status(Status::PartialContent)
            .ok()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RangeNotSatisfiable(u64);

impl<'r, 'o: 'r> Responder<'r, 'o> for RangeNotSatisfiable {
    fn respond_to(self, _req: &'r Request<'_>) -> response::Result<'o> {
        Response::build()
            .raw_header("Accept-Ranges", "bytes */123455")
            .status(rocket::http::Status::RangeNotSatisfiable)
            .ok()
    }
}

pub enum GetFileError {
    RangeNotSatisfiable(RangeNotSatisfiable),
    BadRequest(BadRequest<String>),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for GetFileError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        let mut build = Response::build();
        build.merge(self.respond_to(req)?).ok()
    }
}

#[get("/file/<checksum>")]
pub fn get_file_from_checksum(
    checksum: String,
    state: &State<ServerState>,
    config: &State<ServerConfig>,
    range: RangeHeader,
) -> Result<FileDataRange, GetFileError> {
    let file_path = Path::new(&config.serve_path);
    let file_path = file_path.join(state.get_file(&checksum).unwrap());
    let mut f = File::open(&file_path).unwrap();

    let metadata = f.metadata();
    if metadata.is_err() {
        let bad_request = BadRequest(Some("WrongFile".to_string()));
        let error = GetFileError::BadRequest(bad_request);
        return Err(error);
    }
    let metadata = metadata.unwrap();
    if range.1 >= metadata.len() {
        let bad_range = RangeNotSatisfiable(metadata.len());
        let error = GetFileError::RangeNotSatisfiable(bad_range);
        return Err(error);
    }

    if f.seek(SeekFrom::Start(range.0)).is_err() {
        let bad_request = BadRequest(Some("Wrong file".to_string()));
        let error = GetFileError::BadRequest(bad_request);
        return Err(error);
    }

    let bytes_to_read = range.1 - range.0 + 1;
    let mut buf = vec![0u8; bytes_to_read.try_into().unwrap()];
    if f.read_exact(&mut buf).is_err() {
        let bad_range = RangeNotSatisfiable(metadata.len());
        let error = GetFileError::RangeNotSatisfiable(bad_range);
        return Err(error);
    }

    Ok(FileDataRange(buf))
}
