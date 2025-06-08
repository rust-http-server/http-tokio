use std::{future::Future, net::SocketAddr, time::Duration};
use crate::{status_code::StatusCode, BodyReader, Request, RequestError, Response, TcpIO};
use tokio::{net::{TcpStream}, time::timeout};

pub struct Connection {
    keep_alive_timeout: u8,
    keep_alive_max: u8,
    io: TcpIO,
    #[allow(unused)]
    addr: SocketAddr,
}

impl Connection {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Self { 
        Self { keep_alive_timeout: 5, keep_alive_max: 200, io: TcpIO::new(stream), addr } 
    }
    
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(peer = %self.addr)))]
    pub async fn handle_with(self, handler: impl for<'a> ConnectionHandler<'a>) {
        let mut io = self.io;

        let mut handled_req_count: u8 = 0;

        // keep alive loop
        loop {
            handled_req_count += 1;
            
            let t_req = timeout(Duration::from_secs(self.keep_alive_timeout as u64), io.receive_request()).await;

            let req_or_early_res = match t_req {
                Ok(Ok(req)) => Either::Left(req),
                Ok(Err(err)) => match err {
                    RequestError::ConnectionClosed => {
                        crate::log_info!("Connection closed by client, stopping keep-alive loop");
                        break;
                    },
                    _ => {
                        let status = match err {
                            RequestError::InvalidHeader(_) => StatusCode::BAD_REQUEST,
                            RequestError::InvalidContentLength(_) => StatusCode::BAD_REQUEST,
                            RequestError::UnsupportedHttpVersion(_) => StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                            _ => StatusCode::INTERNAL_SERVER_ERROR,
                        };
                        crate::log_warn!("Error receiving request, sending error response with status {}", error = %err);
                        let mut res = handler.handle_client_error(err, status).await;
                        res.headers.insert(("Connection", "close"));
                        Either::Right(res)
                    }
                },
                Err(_) => {
                    crate::log_info!("Request timed out after {} seconds, sending timeout response", self.keep_alive_timeout);
                    Either::Right(handler.handle_timeout().await)
                }
            };

            let mut res: Response = match req_or_early_res {
                Either::Right(res) => res,
                Either::Left(req) => {
                    let payload = BodyReader::new(req.content_len().await.unwrap_or(0), io);
                    let mut res = handler.handle_connection(&req, &payload).await;
                    if !res.headers.contains_key("Connection") {
                        let connection = req.headers.get("Connection").cloned().unwrap_or("keep-alive".to_string());
                        if connection.eq_ignore_ascii_case("close") {
                            res.headers.insert(("Connection", "close"));
                            res.headers.remove("Keep-Alive");
                        } else {
                            res.headers.insert(("Connection", "keep-alive"));
                            res.headers.insert(("Keep-Alive", &format!("timeout={}, max={}", self.keep_alive_timeout, self.keep_alive_max)));
                        }
                    }
                    if payload.drain().await.is_err() {
                        crate::log_warn!("Error draining request body, closing connection");
                        break;
                    }
                    io = payload.into_io();
                    res
                },
            };

            if res.send(&mut io).await.is_err() {
                crate::log_warn!("Error sending response, closing connection");
                break;
            }
            
            match res.headers.get("Keep-Alive") {
                _ if handled_req_count >= self.keep_alive_max => {
                    crate::log_info!("Max keep-alive requests reached, closing connection");
                    break
                },
                None => {
                    crate::log_info!("Found \"Connection: close\" header, closing connection");
                    break
                },
                _ => {}
            };
        }
    }
}

pub trait ConnectionHandler<'a>: Clone + Send + 'static {
    fn handle_connection(&'a self, request: &'a Request, payload: &'a BodyReader) -> std::pin::Pin<Box<dyn Future<Output = Response> + Send + 'a>>;
    /// should use the suggested status code
    fn handle_client_error(&'a self, err: RequestError, status_code: StatusCode) -> std::pin::Pin<Box<dyn Future<Output = Response> + Send + 'a>> {
        Box::pin(async move {
            Response::build().status(status_code).body(format!("invalid request: {err}"))
        })
    }
    /// should return a response with status 408
    fn handle_timeout(&'a self) -> std::pin::Pin<Box<dyn Future<Output = Response> + Send + 'a>> {
        Box::pin(async move {
            Response::build().status(StatusCode::REQUEST_TIMEOUT).header(("Connection", "close")).body("Request Timeout")
        })
    }
}

impl<'a, Fut, F> ConnectionHandler<'a> for F
where 
    Fut: Future<Output = Response> + Send,
    F: Fn(&'a Request, &'a BodyReader) -> Fut + Clone + Send + Sync + 'static
{
    fn handle_connection(&'a self, request: &'a Request, payload: &'a BodyReader) -> std::pin::Pin<Box<dyn Future<Output = Response> + Send + 'a>> {
        Box::pin(async move {
            self(request, payload).await
        })
    }
}

enum Either<L, R> {
    Left(L),
    Right(R)
}