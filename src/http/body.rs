use async_trait::async_trait;
use gateway::http::response::ResponseBody;
use tokio::{
    fs::File,
    io::{self, AsyncReadExt},
    net::tcp::OwnedWriteHalf,
};

#[derive(Debug)]
pub struct FileBody {
    file: File,
}

impl FileBody {
    pub fn new(file: File) -> Self {
        Self { file }
    }
}

#[async_trait]
impl ResponseBody for FileBody {
    async fn read_all(mut self: Box<Self>, len: usize) -> io::Result<String> {
        let mut buf = vec![0; len];
        self.file.read_exact(&mut buf).await?;
        String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    async fn copy_to<'a>(&mut self, writer: &'a mut OwnedWriteHalf) -> io::Result<()> {
        ::io::copy_file(&mut self.file, writer).await?;
        Ok(())
    }
}
