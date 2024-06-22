use super::body::FileBody;
use crate::utils::etag;
use async_trait::async_trait;
use essentials::warn;
use gateway::{http::HeaderMapExt, Context, Error, OriginServer, Request, Response, Result};
use http::{header, StatusCode};
use std::path::PathBuf;
use tokio::{fs::File, net::tcp::OwnedReadHalf};

pub struct FileServer(PathBuf);

impl FileServer {
    pub fn new(server_root: PathBuf) -> Self {
        Self(server_root)
    }
}

#[async_trait]
impl OriginServer for FileServer {
    async fn connect(
        &self,
        _: &Context,
        request: Request,
        _: OwnedReadHalf,
        _: Vec<u8>,
    ) -> Result<Response> {
        let path = self.0.join(&request.path.as_str()[1..]);
        if !path.starts_with(&self.0) {
            return Ok(Response::new(StatusCode::FORBIDDEN));
        }
        let metadata = match path.metadata() {
            Ok(metadata) => metadata,
            Err(e) => {
                warn!("{:?}: {e}", &path);
                return Ok(Response::new(StatusCode::NOT_FOUND));
            }
        };
        if !metadata.is_file() {
            return Ok(Response::new(StatusCode::FORBIDDEN));
        }
        let file = match File::open(path.clone()).await {
            Ok(file) => file,
            Err(e) => {
                warn!("{:?}: {e}", &path);
                return Ok(Response::new(StatusCode::NOT_FOUND));
            }
        };
        let len = metadata.len();
        let last_access = metadata.accessed().map_err(Error::io)?;
        let etag = etag::generate(&last_access, len);
        if let Some(if_none_match) = request.header(header::IF_NONE_MATCH) {
            if etag == if_none_match.to_str().unwrap() {
                return Ok(Response::new(StatusCode::NOT_MODIFIED));
            }
        }
        let mut response = Response::new(StatusCode::OK);
        response.set_body(FileBody::new(file));
        response.insert_header(header::ETAG, etag);
        response.insert_header(header::CONTENT_LENGTH, len);
        Ok(response)
    }
}
