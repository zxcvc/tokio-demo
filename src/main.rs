use std::{
    self,
    fmt::Result,
    net::SocketAddr,
    str::FromStr,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    self, fs,
    io::AsyncReadExt,
    io::{self, copy, copy_buf, stdin, stdout, AsyncWriteExt, BufReader, BufWriter, ReadBuf},
    net::{self, tcp, TcpStream},
};

async unsafe fn handler(mut tcp_stream: TcpStream) -> std::io::Result<()> {
    let mut buf = vec![];
    let mut f_buf = vec![];
    let file = fs::File::open("./a.html").await?;
    let mut read = BufReader::new(file);
    read.read_to_end(&mut f_buf).await?;
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n", f_buf.len());
    buf.extend_from_slice(response.as_bytes());
    buf.extend(f_buf);
    let mut wbuf = BufWriter::new(&mut tcp_stream);
    copy(&mut buf.as_slice(), &mut wbuf).await?;
    Ok(())
}
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr = SocketAddr::from_str("0.0.0.0:8000").unwrap();
    let lser = net::TcpListener::bind(addr).await?;
    let mut builder = tokio::runtime::Builder::new_multi_thread();
    builder.max_blocking_threads(5);
    builder.worker_threads(4);
    let rt = builder.build()?;

    while let Ok((tcp_stream, _)) = lser.accept().await {
        unsafe {
            rt.spawn(handler(tcp_stream));
        }
    }

    Ok(())
}
