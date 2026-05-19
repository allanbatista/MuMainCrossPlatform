use std::io;
use std::net::SocketAddr;
use std::time::Duration;

use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinHandle;
use tokio::time::sleep;

use crate::transport::{read_packet, write_packet};

#[derive(Debug, Clone, Default)]
pub struct FakeServerScenario {
    connections: Vec<ConnectionScript>,
}

impl FakeServerScenario {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn single(connection: ConnectionScript) -> Self {
        Self {
            connections: vec![connection],
        }
    }

    pub fn connection(mut self, connection: ConnectionScript) -> Self {
        self.connections.push(connection);
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConnectionScript {
    steps: Vec<ConnectionStep>,
}

impl ConnectionScript {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn send_packet(mut self, packet: impl Into<Vec<u8>>) -> Self {
        self.steps.push(ConnectionStep::SendPacket(packet.into()));
        self
    }

    pub fn expect_packet(mut self, packet: impl Into<Vec<u8>>) -> Self {
        self.steps.push(ConnectionStep::ExpectPacket(packet.into()));
        self
    }

    pub fn delay(mut self, duration: Duration) -> Self {
        self.steps.push(ConnectionStep::Delay(duration));
        self
    }

    pub fn close(mut self) -> Self {
        self.steps.push(ConnectionStep::Close);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionStep {
    SendPacket(Vec<u8>),
    ExpectPacket(Vec<u8>),
    Delay(Duration),
    Close,
}

#[derive(Debug)]
pub struct FakeServer {
    address: SocketAddr,
    handle: Option<JoinHandle<io::Result<()>>>,
}

impl FakeServer {
    pub async fn spawn(bind_addr: SocketAddr, scenario: FakeServerScenario) -> io::Result<Self> {
        let listener = TcpListener::bind(bind_addr).await?;
        let address = listener.local_addr()?;
        let handle = tokio::spawn(async move { run(listener, scenario).await });

        Ok(Self {
            address,
            handle: Some(handle),
        })
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }

    pub async fn finish(mut self) -> io::Result<()> {
        let handle = self
            .handle
            .take()
            .ok_or_else(|| io::Error::other("fake server already finished"))?;

        handle.await.map_err(io::Error::other)?
    }
}

impl Drop for FakeServer {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
    }
}

async fn run(listener: TcpListener, scenario: FakeServerScenario) -> io::Result<()> {
    for connection in scenario.connections {
        let (stream, _) = listener.accept().await?;
        run_connection(stream, connection).await?;
    }

    Ok(())
}

async fn run_connection(mut stream: TcpStream, connection: ConnectionScript) -> io::Result<()> {
    for step in connection.steps {
        match step {
            ConnectionStep::SendPacket(packet) => {
                write_packet(&mut stream, &packet)
                    .await
                    .map_err(io::Error::other)?;
            }
            ConnectionStep::ExpectPacket(expected) => {
                let Some(actual) = read_packet(&mut stream, None)
                    .await
                    .map_err(io::Error::other)?
                else {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "client disconnected before sending expected packet",
                    ));
                };

                if actual != expected {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "client sent an unexpected packet",
                    ));
                }
            }
            ConnectionStep::Delay(duration) => {
                sleep(duration).await;
            }
            ConnectionStep::Close => {
                stream.shutdown().await?;
                break;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{ConnectionScript, FakeServer, FakeServerScenario};
    use mu_protocol::connect::{encode_server_list_response, server_list_request, ServerEntry};
    use std::time::Duration;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    #[tokio::test]
    async fn runs_a_scripted_connection_sequence() {
        let response = encode_server_list_response(&[ServerEntry::new(7, 42)]).unwrap();
        let request = server_list_request().unwrap();
        let server = FakeServer::spawn(
            "127.0.0.1:0".parse().unwrap(),
            FakeServerScenario::single(
                ConnectionScript::new()
                    .send_packet(response.clone())
                    .expect_packet(request.clone())
                    .send_packet(response.clone())
                    .close(),
            ),
        )
        .await
        .unwrap();

        let mut client = TcpStream::connect(server.address()).await.unwrap();
        let mut buffer = vec![0u8; response.len()];
        client.read_exact(&mut buffer).await.unwrap();
        assert_eq!(buffer, response);
        client.write_all(&request).await.unwrap();
        let mut buffer = vec![0u8; response.len()];
        client.read_exact(&mut buffer).await.unwrap();
        assert_eq!(buffer, response);

        server.finish().await.unwrap();
    }

    #[tokio::test]
    async fn delays_the_response_when_requested() {
        let response = encode_server_list_response(&[ServerEntry::new(7, 42)]).unwrap();
        let server = FakeServer::spawn(
            "127.0.0.1:0".parse().unwrap(),
            FakeServerScenario::single(
                ConnectionScript::new()
                    .delay(Duration::from_millis(25))
                    .send_packet(response.clone())
                    .close(),
            ),
        )
        .await
        .unwrap();

        let mut client = TcpStream::connect(server.address()).await.unwrap();
        let mut buffer = vec![0u8; response.len()];
        client.read_exact(&mut buffer).await.unwrap();
        assert_eq!(buffer, response);

        server.finish().await.unwrap();
    }
}
