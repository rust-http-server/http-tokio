use super::{extensions::Extensions, headers::Headers, tcp_io::TcpIO};

pub struct Request<T> {
    pub method: String,
    pub path: String,
    pub headers: Headers,
    pub extensions: Extensions,
    pub body: Option<T>,
}

pub type IncomingRequest = Request<()>;

impl<T> Request<T> {
    pub async fn receive(io: &mut TcpIO) -> Result<IncomingRequest, RequestError> {
        let (first_line_len, first_line) = io.read_line().await?;
        if first_line_len == 0 { return Err(RequestError::ConnectionClosed) }
        let mut parts = first_line.split_whitespace();
        let method = parts
            .next()
            .ok_or_else(|| RequestError::InvalidRequestLine(first_line.clone()))?
            .to_string();
        let full_path = parts
            .next()
            .ok_or_else(|| RequestError::InvalidRequestLine(first_line.clone()))?
            .to_string();

        // TODO: URI Struct
        let mut full_path = full_path.split("?");
        let path = "/".to_owned() + full_path.next().unwrap_or("/").trim_matches('/');
        let _query_string = full_path.next().unwrap_or("");

        let http_version = parts
            .next()
            .ok_or_else(|| RequestError::InvalidRequestLine(first_line.clone()))?
            .to_string();

        if !http_version.eq("HTTP/1.1") {
            return Err(RequestError::UnsupportedHttpVersion(http_version));
        }

        // parsing headers
        let mut headers = Headers::new();
        let extensions = Extensions::new();
        loop {
            let (len, line) = io.read_line().await?;
            if len <= 2 {
                break; // Empty line signals end of headers
            }
            if let Some((key, value)) = line.split_once(":") {
                let key = key.trim();
                let value = value.trim();
                if key.eq_ignore_ascii_case("content-length") {
                    match value.parse::<usize>() {
                        Ok(length) => {
                            headers.append(key, value);
                            extensions.insert(ContentLength(length)).await;
                        }
                        Err(_) => {
                            return Err(RequestError::InvalidContentLength(value.to_string()));
                        }
                    }
                }
                headers.append(key, value);
            } else {
                return Err(RequestError::InvalidHeader(line));
            }
        }

        Ok(IncomingRequest {
            headers,
            method,
            path,
            extensions,
            body: None,
        })
    }
}

impl IncomingRequest {
    pub fn content_len(&self) -> Option<usize> {
        // FIXME: non so quanto questo vada bene...
        self.extensions.get_sync_unsafe::<ContentLength>().map(|cl| cl.0)
    }
}

struct ContentLength(usize);

#[derive(thiserror::Error, Debug)]
pub enum RequestError {
    #[error("could not read from TcpStream: {0}")]
    Read(#[from] tokio::io::Error),

    #[error("Tcp client closed connection")]
    ConnectionClosed,

    #[error("invalid request line: \"{0}\"")]
    InvalidRequestLine(String),

    #[error("unsupported http version: \"{0}\"")]
    UnsupportedHttpVersion(String),

    #[error("invalid header: {0:?}")]
    InvalidHeader(String),

    #[error("invalid content length header: {0:?}")]
    InvalidContentLength(String),
    // #[error("body has already been consumed")]
    // BodyAlreadyConsumed,

    // #[error("invalid json body")]
    // Json(#[from] serde_json::Error),
}
