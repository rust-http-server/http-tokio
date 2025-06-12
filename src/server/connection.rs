use std::{future::Future, net::SocketAddr, time::Duration};
use crate::{status_code::StatusCode, BodyReader, Request, RequestError, Response, TcpIO};
use tokio::{net::{TcpStream}, time::timeout};
use tracing::{info, instrument, warn};

pub struct Connection {
    io: TcpIO,
    #[allow(unused)]
    addr: SocketAddr,
    keep_alive_timeout: usize,
    keep_alive_max: usize,
    events_handler: Box<dyn ConnectionEventsHandler>,
}

impl Connection {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Self { 
        Self { keep_alive_timeout: 5, keep_alive_max: 200, io: TcpIO::new(stream), addr, events_handler: Box::new(DefaultConncetionEventsHandler) } 
    }

    /// Sets the keep-alive timeout in seconds.
    /// 
    /// Default is 5 seconds.
    pub fn keep_alive_timeout(mut self, timeout: usize) -> Self {
        self.keep_alive_timeout = timeout;
        self
    }

    /// Sets the maximum number of requests to handle in a keep-alive connection.
    /// 
    /// Default is 200 requests.
    pub fn keep_alive_max(mut self, max: usize) -> Self {
        self.keep_alive_max = max;
        self
    }

    /// Sets the events handler for the connection.
    /// 
    /// Code example:
    /// ```rust
    /// struct MyEventsHandler;
    /// impl ConnectionEventsHandler for MyEventsHandler {
    ///     fn handle_client_error(&self, err: RequestError, status_code: StatusCode) -> Pin<Box<dyn Future<Output = Response> + Send>> {
    ///         Box::pin(async move {
    ///             Response::build().status(status_code).body(format!("invalid request: {err}"))
    ///         })
    ///     }
    /// 
    ///     fn handle_timeout(&self) -> Pin<Box<dyn Future<Output = Response> + Send>> {
    ///         Box::pin(async move {
    ///             Response::build().status(StatusCode::REQUEST_TIMEOUT).header(("Connection", "close")).body("Request Timeout")
    ///         })
    ///     }
    /// }
    /// 
    /// let connection = Connection::new(stream, addr)
    ///     .events_handler(MyEventsHandler);
    /// ```
    pub fn events_handler(mut self, handler: impl ConnectionEventsHandler + 'static) -> Self {
        self.events_handler = Box::new(handler);
        self
    }
    
    #[instrument(skip_all, "new connection", fields(client_address = %self.addr))]
    pub async fn handle_with(self, handler: impl for<'a> ConnectionHandler<'a>) {
        let mut io = self.io;

        let mut handled_req_count: usize = 0;

        // keep alive loop
        loop {
            handled_req_count += 1;
            
            let t_req = timeout(Duration::from_secs(self.keep_alive_timeout as u64), io.receive_request()).await;

            let req_or_early_res = match t_req {
                Ok(Ok(req)) => RequestOutcome::EarlyResponse(req),
                Ok(Err(err)) => match err {
                    RequestError::ConnectionClosed => {
                        info!("Connection closed by client, stopping keep-alive loop");
                        break;
                    },
                    _ => {
                        let status = match err {
                            RequestError::InvalidHeader(_) => StatusCode::BAD_REQUEST,
                            RequestError::InvalidContentLength(_) => StatusCode::BAD_REQUEST,
                            RequestError::UnsupportedHttpVersion(_) => StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                            _ => StatusCode::INTERNAL_SERVER_ERROR,
                        };
                        warn!(error = %err, "Error receiving request, sending error response with status");
                        let mut res = self.events_handler.handle_client_error(err, status).await;
                        res.headers.insert(("Connection", "close"));
                        RequestOutcome::ValidRequest(res)
                    }
                },
                Err(_) => {
                    info!("Request timed out after {} seconds, sending timeout response", self.keep_alive_timeout);
                    RequestOutcome::ValidRequest(self.events_handler.handle_timeout().await)
                }
            };

            let mut res: Response = match req_or_early_res {
                RequestOutcome::ValidRequest(res) => res,
                RequestOutcome::EarlyResponse(req) => {
                    let payload = BodyReader::new(req.content_len().await.unwrap_or(0), io);
                    let mut res = handler.handle(&req, &payload).await;
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
                        warn!("Error draining request body, closing connection");
                        break;
                    }
                    io = payload.into_io();
                    res
                },
            };

            if res.send(&mut io).await.is_err() {
                warn!("Error sending response, closing connection");
                break;
            }
            
            match res.headers.get("Keep-Alive") {
                _ if handled_req_count >= self.keep_alive_max => {
                    info!("Max keep-alive requests reached, closing connection");
                    break
                },
                None => {
                    info!("Found \"Connection: close\" header, closing connection");
                    break
                },
                _ => {}
            };
        }
    }
}

pub trait ConnectionHandler<'a>: Clone + Send + 'static {
    fn handle(&'a self, request: &'a Request, payload: &'a BodyReader) -> std::pin::Pin<Box<dyn Future<Output = Response> + Send + 'a>>;
}

impl<'a, Fut, F> ConnectionHandler<'a> for F
where 
    Fut: Future<Output = Response> + Send,
    F: FnOnce(&'a Request, &'a BodyReader) -> Fut + Clone + Send + Sync + 'static
{
    /// Handles a request and returns a response.
    fn handle(&'a self, request: &'a Request, payload: &'a BodyReader) -> std::pin::Pin<Box<dyn Future<Output = Response> + Send + 'a>> {
        Box::pin(async move {
            self.clone()(request, payload).await
        })
    }
}

pub trait ConnectionEventsHandler: Send + 'static {
    /// Triggered when an invalid request is received;
    /// 
    /// should return a response with the suggested status code
    fn handle_client_error(&self, err: RequestError, status_code: StatusCode) -> std::pin::Pin<Box<dyn Future<Output = Response> + Send>> {
        Box::pin(async move {
            Response::build().status(status_code).body(format!("invalid request: {err}"))
        })
    }

    /// Triggered when a request times out;
    /// 
    /// should return a response with status code 408 `StatusCode::REQUEST_TIMEOUT` and a "Connection: close" header
    fn handle_timeout(&self) -> std::pin::Pin<Box<dyn Future<Output = Response> + Send>> {
        Box::pin(async move {
            Response::build().status(StatusCode::REQUEST_TIMEOUT).header(("Connection", "close")).body("Request Timeout")
        })
    }
}

#[derive(Clone)]
struct DefaultConncetionEventsHandler;
impl ConnectionEventsHandler for DefaultConncetionEventsHandler {}

enum RequestOutcome<L, R> {
    EarlyResponse(L),
    ValidRequest(R)
}