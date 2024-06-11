use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;
use essentials::{error, warn};
use gateway::{Context, Error, OriginResponse, OriginServer, Request, Response, Result};
use http::{header, StatusCode};
use tokio::{fs::File, io::ReadHalf, net::TcpStream};

use crate::utils::etag;

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
        _: Arc<Context>,
        mut request: Request,
        _: ReadHalf<TcpStream>,
        _: Vec<u8>,
    ) -> Result<(Response, OriginResponse, Vec<u8>)> {
        let path = self.0.join(request.path.split_off(1));
        if !path.starts_with(&self.0) {
            Err(Error::status(StatusCode::FORBIDDEN))?;
        }
        let metadata = path.metadata().map_err(|e| {
            warn!("{:?}: {e}", &path);
            Error::status(StatusCode::NOT_FOUND)
        })?;
        if !metadata.is_file() {
            Err(Error::status(StatusCode::FORBIDDEN))?;
        }
        let file = File::open(path.clone()).await.map_err(|e| {
            warn!("{:?}: {e}", &path);
            Error::status(StatusCode::NOT_FOUND)
        })?;
        let len = metadata.len();
        let last_access = metadata.accessed().map_err(|e| {
            error!("{:?}: {e}", &path);
            Error::status(StatusCode::INTERNAL_SERVER_ERROR)
        })?;
        let etag = etag::generate(&last_access, len);
        if let Some(if_none_match) = request.headers.get(header::IF_NONE_MATCH) {
            if etag == if_none_match.to_str().unwrap() {
                Err(Error::status(StatusCode::NOT_MODIFIED))?;
            }
        }
        let mut response = Response::new(StatusCode::OK);
        response.insert_header(header::ETAG, etag);
        response.insert_header(header::CONTENT_LENGTH, len);
        Ok((response, Box::new(file), Vec::new()))
    }
}
