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
    async fn read_all(mut self: Box<Self>) -> io::Result<String> {
        let mut buffer = String::new();
        self.file.read_to_string(&mut buffer).await?;
        Ok(buffer)
    }

    async fn copy_to<'a>(&mut self, writer: &'a mut OwnedWriteHalf) -> io::Result<()> {
        ::io::copy_file(&mut self.file, writer).await?;
        Ok(())
    }
}
