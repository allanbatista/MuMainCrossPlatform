use std::net::SocketAddr;
use std::time::Duration;

use thiserror::Error;

use crate::transport::{TcpTransport, TransportError};

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("client is not connected")]
    NotConnected,
    #[error(transparent)]
    Transport(#[from] TransportError),
}

#[derive(Debug)]
pub struct Client {
    endpoint: SocketAddr,
    connect_timeout: Duration,
    read_timeout: Duration,
    transport: Option<TcpTransport>,
}

impl Client {
    pub async fn connect(
        endpoint: SocketAddr,
        connect_timeout: Duration,
        read_timeout: Duration,
    ) -> Result<Self, ClientError> {
        let transport = TcpTransport::connect(endpoint, connect_timeout, read_timeout).await?;
        Ok(Self {
            endpoint,
            connect_timeout,
            read_timeout,
            transport: Some(transport),
        })
    }

    pub fn is_connected(&self) -> bool {
        self.transport.is_some()
    }

    pub fn disconnect(&mut self) {
        self.transport = None;
    }

    pub async fn reconnect(&mut self) -> Result<(), ClientError> {
        let transport =
            TcpTransport::connect(self.endpoint, self.connect_timeout, self.read_timeout).await?;
        self.transport = Some(transport);
        Ok(())
    }

    pub async fn send(&mut self, packet: impl AsRef<[u8]>) -> Result<(), ClientError> {
        let transport = self.transport.as_mut().ok_or(ClientError::NotConnected)?;
        transport.send(packet).await?;
        Ok(())
    }

    pub async fn receive(&mut self) -> Result<Option<Vec<u8>>, ClientError> {
        let result = {
            let transport = self.transport.as_mut().ok_or(ClientError::NotConnected)?;
            transport.receive().await?
        };

        if result.is_none() {
            self.transport = None;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::Client;
    use crate::fake_server::{ConnectionScript, FakeServer, FakeServerScenario};
    use mu_protocol::connect::{encode_server_list_response, server_list_request, ServerEntry};
    use std::time::Duration;

    #[tokio::test]
    async fn reconnects_after_disconnect() {
        let response = encode_server_list_response(&[ServerEntry::new(7, 42)]).unwrap();
        let request = server_list_request().unwrap();
        let server = FakeServer::spawn(
            "127.0.0.1:0".parse().unwrap(),
            FakeServerScenario::new()
                .connection(
                    ConnectionScript::new()
                        .send_packet(response.clone())
                        .expect_packet(request.clone())
                        .send_packet(response.clone())
                        .close(),
                )
                .connection(
                    ConnectionScript::new()
                        .send_packet(response.clone())
                        .close(),
                ),
        )
        .await
        .unwrap();

        let mut client = Client::connect(
            server.address(),
            Duration::from_millis(250),
            Duration::from_millis(250),
        )
        .await
        .unwrap();

        assert_eq!(client.receive().await.unwrap(), Some(response.clone()));
        assert!(client.is_connected());
        client.send(&request).await.unwrap();
        assert_eq!(client.receive().await.unwrap(), Some(response.clone()));
        assert_eq!(client.receive().await.unwrap(), None);
        assert!(!client.is_connected());

        client.reconnect().await.unwrap();
        assert!(client.is_connected());
        assert_eq!(client.receive().await.unwrap(), Some(response));

        server.finish().await.unwrap();
    }

    #[tokio::test]
    async fn disconnect_clears_the_active_transport() {
        let server = FakeServer::spawn(
            "127.0.0.1:0".parse().unwrap(),
            FakeServerScenario::single(ConnectionScript::new().close()),
        )
        .await
        .unwrap();

        let mut client = Client::connect(
            server.address(),
            Duration::from_millis(250),
            Duration::from_millis(250),
        )
        .await
        .unwrap();
        client.disconnect();

        assert!(!client.is_connected());
        assert!(matches!(
            client.send([0xC1, 0x03, 0xF4, 0x06]).await,
            Err(super::ClientError::NotConnected)
        ));

        server.finish().await.unwrap();
    }
}
