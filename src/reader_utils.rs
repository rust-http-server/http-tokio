use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader, Error};

pub async fn read_line<T>(buf_reader: &mut BufReader<T>) -> Result<(usize, String), Error>
where
    T: AsyncRead + Unpin,
{
    let mut buf = String::new();
    let len = buf_reader.read_line(&mut buf).await?;
    let parsed = buf.trim_end().to_string(); // remove line terminators \r\n
    Ok((len, parsed))
}

// pub async fn read_len<T>(buf_reader: &mut BufReader<T>, len: usize) -> Result<String, Error>
// where
//     T: AsyncRead + Unpin + Send,
// {
//     let mut buf = vec![0 as u8; len];
//     buf_reader.read_exact(&mut buf).await?;
//     let utf = String::from_utf8(buf)
//         .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
//     Ok(utf)
// }