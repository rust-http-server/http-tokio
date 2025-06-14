use std::future::Future;

use tokio::{net::{TcpListener, ToSocketAddrs}, task};
use tracing::warn;
use crate::{server::{Connection, ConnectionHandler}};

pub async fn run_server<A: ToSocketAddrs>(addr: A, handler: impl for<'a> ServerHandler<'a>) -> tokio::io::Result<()> {
    let server = TcpListener::bind(addr).await?;
    loop {
        match server.accept().await {
            Ok((stream, addr)) => {
                let conn = Connection::new(stream, addr);
                task::spawn(conn.handle_with(handler.clone()));
            }
            Err(err) => {
                warn!( error = %err, kind = ?err.kind(), "Failed to accept incoming connection");
                handler.clone().handle_connection_error(err).await;
            },
        }
    }
}

pub trait ServerHandler<'a>: ConnectionHandler<'a> {
    #[allow(unused_variables)]
    fn handle_connection_error(&'a self, err: tokio::io::Error) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {})
    }
}

impl<'a, Fut, F> ServerHandler<'a> for F
where 
    Fut: Future<Output = crate::Response> + Send,
    F: FnOnce(&'a crate::Request, &'a crate::BodyReader) -> Fut + Clone + Send + Sync + 'static
{
}