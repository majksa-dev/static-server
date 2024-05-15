use std::{fs::File, io::Write, net::TcpStream, sync::Arc};

use essentials::{error, warn};
use snedfile::send_file;

use crate::utils::etag;

use super::{
    builder::Builder,
    request::{Error, ReadRequest, Request},
};

#[derive(Debug)]
struct Response {
    file: File,
    len: u64,
    etag: String,
}

#[derive(Debug)]
enum ErrorResponse {
    NotModified,
    BadRequest,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    InternalServerError,
}

impl From<Error> for ErrorResponse {
    fn from(e: Error) -> Self {
        match e {
            Error::Io(e) => {
                warn!("I/O error: {e}");
                ErrorResponse::InternalServerError
            }
            Error::InvalidRequestLine => ErrorResponse::BadRequest,
            Error::MethodNotAllowed => ErrorResponse::MethodNotAllowed,
        }
    }
}

fn handle_request(mut request: Request, ctx: Arc<Builder>) -> Result<Response, ErrorResponse> {
    let path = ctx.server_root.join(request.path.split_off(1));
    let file = File::open(path.clone()).map_err(|e| {
        error!("{:?}: {e}", &path);
        ErrorResponse::NotFound
    })?;
    let metadata = file.metadata().map_err(|_| ErrorResponse::Forbidden)?;
    let len = metadata.len();
    let last_access = metadata
        .accessed()
        .map_err(|_| ErrorResponse::InternalServerError)?;
    let etag = etag::generate(&last_access, len);
    if let Some(if_none_match) = request.headers.if_none_match {
        if etag == if_none_match {
            Err(ErrorResponse::NotModified)?
        }
    }
    Ok(Response { file, etag, len })
}

macro_rules! create_response {
    ($status:expr) => {
        concat!("HTTP/1.1 ", $status, "\r\nContent-Length: 0\r\n\r\n")
    };
}

pub fn handle_connection(mut stream: TcpStream, ctx: Arc<Builder>) {
    let response = stream
        .read_request()
        .map_err(ErrorResponse::from)
        .and_then(|req| handle_request(req, ctx));
    if let Err(error) = &response {
        if let Err(e) = stream.write_all(
            match error {
                ErrorResponse::NotModified => create_response!("304 Not Modified"),
                ErrorResponse::BadRequest => create_response!("400 Bad Request"),
                ErrorResponse::Forbidden => create_response!("403 Forbidden"),
                ErrorResponse::NotFound => create_response!("404 Not Found"),
                ErrorResponse::MethodNotAllowed => create_response!("405 Method Not Allowed"),
                ErrorResponse::InternalServerError => {
                    create_response!("500 Internal Server Error")
                }
            }
            .as_bytes(),
        ) {
            warn!("Error sending error response: {}", e);
        } else {
            return;
        }
    }
    let Response {
        mut file,
        etag,
        len,
    } = response.unwrap();
    if let Err(e) = stream.write_all(
        format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nETag: {}\r\n\r\n",
            len, etag,
        )
        .as_bytes(),
    ) {
        warn!("Error sending response: {}", e);
    }
    if let Err(e) = send_file(&mut file, &mut stream) {
        warn!("Error sending file: {}", e);
    }
}
