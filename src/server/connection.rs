use std::{time::Duration};
use async_fn_traits::AsyncFn2;
use crate::{BodyReader, Request, RequestError, Response, TcpIO};
use tokio::{net::TcpStream, time::timeout};

pub struct Connection {
    keep_alive_timeout: u8,
    keep_alive_max: u8,
    io: TcpIO
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self { 
        Self { keep_alive_timeout: 5, keep_alive_max: 200, io: TcpIO::new(stream) } 
    }
    
    pub async fn handle_with<F>(self, handler: F)
    where 
        F: for<'a> AsyncFn2<&'a Request, &'a BodyReader, Output = Response>
    {
        let mut io = self.io;

        let mut handled_req_count: u8 = 0;

        // keep alive loop
        loop {
            handled_req_count += 1;
            
            let t_req = timeout(Duration::from_secs(self.keep_alive_timeout as u64), io.receive_request()).await;

            let req_or_early_res = match t_req {
                Ok(Ok(req)) => Either::Left(req),
                Ok(Err(err)) => match err {
                    RequestError::ConnectionClosed => break,
                    _ => {
                        let status = match err {
                            RequestError::InvalidHeader(_) => 400,
                            RequestError::InvalidContentLength(_) => 400,
                            RequestError::UnsupportedHttpVersion(_) => 505,
                            _ => 500,
                        };
                        let mut res = Response::build().status(status).body("500 Internal Server Error"); // TODO
                        res.headers.insert(("Connection", "close"));
                        Either::Right(res)
                    }
                },
                Err(_) => Either::Right(Response::build().status(408).header("Connection", "close").body("408 Request Timeout"))
            };

            let mut res: Response = match req_or_early_res {
                Either::Right(res) => res,
                Either::Left(req) => {
                    let payload = BodyReader::new(req.content_len().await.unwrap_or(0), io);
                    let mut res = handler(&req, &payload).await;
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
                        break;
                    }
                    io = payload.into_io();
                    res
                },
            };

            if res.send(&mut io).await.is_err() {
                break;
            }
            
            match res.headers.get("Keep-Alive") {
                _ if handled_req_count >= self.keep_alive_max => break,
                None => break,
                _ => {}
            };
        }
    }
}

enum Either<L, R> {
    Left(L),
    Right(R)
}
