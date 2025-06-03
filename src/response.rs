use std::path::Path;

use bytes::Bytes;
use httpdate::HttpDate;
use thiserror::Error;
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_stream::{Stream, StreamExt};
use tokio_util::io::ReaderStream;

use crate::body::Body;

use super::{extensions::Extensions, headers::Headers, status_code::StatusCode, TcpIO};

pub struct Response<T> {
    pub status: StatusCode,
    pub headers: Headers,
    pub extensions: Extensions,
    pub body: Option<T>,
}

impl<T> Response<T> {
    fn new() -> Self {
        Response {
            body: None,
            status: StatusCode::OK,
            headers: Headers::new(),
            extensions: Extensions::new(),
        }
    }

    fn fmt_head(&self) -> String {
        format!("HTTP/1.1 {} \r\n{}\r\n\r\n",self.status, self.headers.to_string())
    }
}

pub type HttpResponse = Response<Body>;

impl HttpResponse {
    pub fn build() -> ResponseBuilder {
        ResponseBuilder::new()
    }

    pub fn into_builder(self) -> ResponseBuilder {
        ResponseBuilder { inner: self }
    }

    pub async fn send(&mut self, io: &mut TcpIO) -> Result<(), ResponseError> {
        let mut payload = self.fmt_head().into_bytes();

        if let Some(body) = self.body.take() {
            match body {
                Body::Bytes(bytes) => {
                    payload.extend_from_slice(&bytes);
                    io.writer().write_all(&payload).await?;
                    io.writer().flush().await?;
                },
                Body::Stream(mut stream) => {
                    io.writer().write_all(&payload).await?;
                    
                    while let Some(chunk) = stream.next().await {
                        let chunk = chunk?;
                        let chunk_len = format!("{:X}\r\n", chunk.len());
                        io.writer().write_all(chunk_len.as_bytes()).await?;
                        io.writer().write_all(&chunk).await?;
                        io.writer().write_all(b"\r\n").await?;
                    }
                    
                    io.writer().write_all(b"0\r\n\r\n").await?; // End of stream
                    io.writer().flush().await?;
                },
            }
        } else {
            io.writer().write_all(&payload).await?;
            io.writer().flush().await?;
        }

        Ok(())
    }
}

pub struct ResponseBuilder {
    inner: HttpResponse,
}

impl ResponseBuilder {
    fn new() -> Self {
        let mut res = Response::new();
        res.headers.insert("Date", &HttpDate::from(std::time::SystemTime::now()).to_string());
        Self { inner: res }
    }

    pub fn body<I: Into<Bytes>>(mut self, body: I) -> HttpResponse {
        let body = body.into();
        if !self.inner.headers.contains_key("Content-Type") {
            self.inner.headers.insert("Content-Type", "text/plain; charset=utf-8");
        }
        self.inner.headers.insert("Content-Length", &body.len().to_string());
        self.inner.headers.remove("Transfer-Encoding");
        self.inner.body = Some(body.into());
        self.inner
    }

    pub fn stream<S: Stream<Item = Result<Bytes, ResponseError>> + Send + Sync + Unpin + 'static>(mut self, body: S) -> HttpResponse {
        if !self.inner.headers.contains_key("Content-Type") {
            self.inner.headers.insert("Content-Type", "application/octet-stream");
        }
        self.inner.headers.remove("Content-Length");
        self.inner.headers.insert("Transfer-Encoding", "chunked");
        self.inner.body = Some(Body::Stream(Box::new(body)));
        self.inner
    }

    pub async fn file<P: AsRef<Path>>(mut self, path: P) -> Result<HttpResponse, std::io::Error> {
        if !self.inner.headers.contains_key("Content-Type") {
            let content_type = mime_guess::from_path(&path).first_or_octet_stream().to_string();
            self.inner.headers.insert("Content-Type", &content_type);
        }
        let file = File::open(&path).await?;
        let stream = ReaderStream::new(file).map(|res| res.map(Bytes::from).map_err(Into::into));
        Ok(self.stream(stream))
    }

    pub fn end(self) -> HttpResponse {
        self.inner
    }

    pub fn status<S: Into<StatusCode>>(mut self, status: S) -> Self {
        self.inner.status = status.into();
        self
    }

    pub fn header(mut self, header: &str, value: &str) -> Self {
        self.inner.headers.insert(header, value);
        self
    }

    pub fn cookie(mut self, key: &str, value: &str) -> Self {
        self.inner.headers.add_set_cookie(key, value);
        self
    }
}

#[derive(Error, Debug)]
pub enum ResponseError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    // #[error("Invalid status line: {0}")]
    // InvalidStatusLine(String),

    // #[error("Invalid status code: {0}")]
    // InvalidStatusCode(String),

    // #[error("Invalid header: {0}")]
    // InvalidHeader(String),

    // #[error("Invalid Content-Length: {0}")]
    // InvalidContentLength(String),

    // #[error("Invalid json body")]
    // Json(#[from] serde_json::Error),
}