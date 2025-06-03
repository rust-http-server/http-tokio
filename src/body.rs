use bytes::Bytes;
use tokio_stream::Stream;

use crate::response::ResponseError;

pub enum Body {
    Bytes(Bytes),
    Stream(Box<dyn Stream<Item = Result<Bytes, ResponseError>> + Send + Sync + Unpin>),
}

impl From<Bytes> for Body {
    fn from(bytes: Bytes) -> Self {
        Body::Bytes(bytes)
    }
}