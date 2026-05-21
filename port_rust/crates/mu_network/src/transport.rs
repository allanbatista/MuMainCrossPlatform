use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::time::Duration;

use mu_protocol::{decode_packet, PacketCodecError, PacketFrame};
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("connect timed out after {timeout:?}")]
    ConnectTimeout { timeout: Duration },
    #[error("read timed out after {timeout:?}")]
    ReadTimeout { timeout: Duration },
    #[error("malformed packet: {0}")]
    MalformedPacket(#[from] PacketCodecError),
    #[error(transparent)]
    Io(#[from] io::Error),
}

#[derive(Debug)]
pub struct TcpTransport {
    stream: TcpStream,
    read_timeout: Duration,
}

impl TcpTransport {
    pub async fn connect(
        address: SocketAddr,
        connect_timeout: Duration,
        read_timeout: Duration,
    ) -> Result<Self, TransportError> {
        let stream = connect_with(address, connect_timeout, TcpStream::connect).await?;
        stream.set_nodelay(true)?;
        Ok(Self {
            stream,
            read_timeout,
        })
    }

    pub async fn send(&mut self, packet: impl AsRef<[u8]>) -> Result<(), TransportError> {
        write_packet(&mut self.stream, packet.as_ref()).await
    }

    pub async fn receive(&mut self) -> Result<Option<Vec<u8>>, TransportError> {
        read_packet(&mut self.stream, Some(self.read_timeout)).await
    }
}

pub(crate) async fn connect_with<F, Fut>(
    address: SocketAddr,
    connect_timeout: Duration,
    connector: F,
) -> Result<TcpStream, TransportError>
where
    F: FnOnce(SocketAddr) -> Fut,
    Fut: Future<Output = io::Result<TcpStream>>,
{
    match timeout(connect_timeout, connector(address)).await {
        Ok(Ok(stream)) => Ok(stream),
        Ok(Err(error)) => Err(TransportError::Io(error)),
        Err(_) => Err(TransportError::ConnectTimeout {
            timeout: connect_timeout,
        }),
    }
}

pub(crate) async fn read_packet(
    stream: &mut TcpStream,
    read_timeout: Option<Duration>,
) -> Result<Option<Vec<u8>>, TransportError> {
    let mut code = [0u8; 1];
    if !read_exact(stream, &mut code, read_timeout).await? {
        return Ok(None);
    }

    let header_len = PacketFrame::header_len(code[0]);
    let mut header = vec![0u8; header_len - 1];
    if !read_exact(stream, &mut header, read_timeout).await? {
        return Ok(None);
    }

    let size = if header_len == 4 {
        usize::from(header[0])
    } else {
        usize::from(u16::from_be_bytes([header[0], header[1]]))
    };

    let mut packet = Vec::with_capacity(size);
    packet.push(code[0]);
    packet.extend_from_slice(&header);

    if size > header_len {
        let body_len = size - header_len;
        let mut body = vec![0u8; body_len];
        if !read_exact(stream, &mut body, read_timeout).await? {
            return Ok(None);
        }
        packet.extend_from_slice(&body);
    }

    decode_packet(&packet)?;
    Ok(Some(packet))
}

pub(crate) async fn write_packet(
    stream: &mut TcpStream,
    packet: &[u8],
) -> Result<(), TransportError> {
    stream.write_all(packet).await?;
    stream.flush().await?;
    Ok(())
}

async fn read_exact(
    stream: &mut TcpStream,
    buffer: &mut [u8],
    read_timeout: Option<Duration>,
) -> Result<bool, TransportError> {
    let read_future = stream.read_exact(buffer);
    let result = match read_timeout {
        Some(timeout_duration) => match timeout(timeout_duration, read_future).await {
            Ok(result) => result,
            Err(_) => {
                return Err(TransportError::ReadTimeout {
                    timeout: timeout_duration,
                });
            }
        },
        None => read_future.await,
    };

    match result {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => Ok(false),
        Err(error) => Err(TransportError::Io(error)),
    }
}

#[cfg(test)]
mod tests {
    use super::{connect_with, TransportError};
    use std::time::Duration;
    use tokio::net::TcpStream;

    #[tokio::test]
    async fn connect_with_times_out_for_a_slow_connector() {
        let error = connect_with(
            "127.0.0.1:1".parse().unwrap(),
            Duration::from_millis(10),
            |_| async {
                tokio::time::sleep(Duration::from_millis(50)).await;
                Ok(TcpStream::connect("127.0.0.1:1").await.unwrap())
            },
        )
        .await
        .unwrap_err();

        assert!(matches!(error, TransportError::ConnectTimeout { .. }));
    }

    #[tokio::test]
    async fn connect_with_returns_io_errors() {
        let error = connect_with(
            "127.0.0.1:1".parse().unwrap(),
            Duration::from_millis(50),
            |_| async {
                Err(std::io::Error::new(
                    std::io::ErrorKind::ConnectionRefused,
                    "refused",
                ))
            },
        )
        .await
        .unwrap_err();

        assert!(matches!(
            error,
            TransportError::Io(error) if error.kind() == std::io::ErrorKind::ConnectionRefused
        ));
    }
}
