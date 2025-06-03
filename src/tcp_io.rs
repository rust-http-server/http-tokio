use std::pin::Pin;
use tokio::{
    io::{AsyncBufReadExt, BufReader, BufWriter},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream, ToSocketAddrs,
    },
};

// pinned heap pointer for Send enabled cheap ownership passing (maybe?)
pub struct TcpIO(Pin<Box<InnerTcpIO>>);

struct InnerTcpIO {
    reader: BufReader<OwnedReadHalf>,
    writer: BufWriter<OwnedWriteHalf>,
}

impl TcpIO {
    pub fn new(stream: TcpStream) -> Self {
        let (read_half, write_half) = stream.into_split();
        let reader = BufReader::new(read_half);
        let writer = BufWriter::new(write_half);
        Self(Box::pin(InnerTcpIO { reader, writer }))
    }

    pub async fn connect<A>(addr: A) -> tokio::io::Result<Self>
    where
        A: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addr).await?;
        Ok(Self::new(stream))
    }

    pub fn reader(&mut self) -> &mut BufReader<OwnedReadHalf> {
        &mut self.0.reader
    }

    pub fn writer(&mut self) -> &mut BufWriter<OwnedWriteHalf> {
        &mut self.0.writer
    }

    pub async fn read_line(&mut self) -> Result<(usize, String), tokio::io::Error> {
        let mut buf = String::new();
        let len = self.0.reader.read_line(&mut buf).await?;
        let parsed = buf.trim_end().to_string(); // remove line terminators \r\n
        Ok((len, parsed))
    }
}
