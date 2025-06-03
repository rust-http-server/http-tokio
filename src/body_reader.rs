use tokio::{io::AsyncReadExt, sync::Mutex};
use std::io;
use super::tcp_io::TcpIO;

pub struct BodyReader(Mutex<InnerBodyReader>);

struct InnerBodyReader {
    io: TcpIO,
    remaining: usize,
}

impl BodyReader {
    pub fn new(c_len: usize, io: TcpIO) -> Self {
        Self(Mutex::new(InnerBodyReader { io, remaining: c_len }))
    }

    pub fn into_io(self) -> TcpIO {
        self.0.into_inner().io
    }

    pub async fn next(&self) -> io::Result<Option<Vec<u8>>> {
        let mut inner = self.0.lock().await;

        if inner.remaining == 0 {
            return Ok(None);
        }

        let to_read = 1024.min(inner.remaining);
        let mut buf = vec![0u8; to_read];
        let read = inner.io.reader().read(&mut buf).await?;

        if read == 0 {
            inner.remaining = 0;
            return Ok(None);
        }

        inner.remaining -= read;
        buf.truncate(read);
        Ok(Some(buf))
    }

    pub async fn read_all(&self) -> io::Result<Vec<u8>> {
        let mut result = Vec::with_capacity(1024);
        while let Some(chunk) = self.next().await? {
            result.extend_from_slice(&chunk);
        }
        Ok(result)
    }

    pub async fn drain(&self) -> io::Result<()> {
        let mut buf = vec![0u8; 1024];
        let mut inner = self.0.lock().await;
        loop {
            if inner.remaining == 0 {
                break;
            }

            let to_read = 1024.min(inner.remaining);
            let read = inner.io.reader().read(&mut buf[..to_read]).await?;
            if read == 0 {
                break;
            }
            inner.remaining -= read;
        }
        Ok(())
    }
}