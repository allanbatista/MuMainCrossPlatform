pub mod client;
pub mod fake_server;
pub mod redaction;
pub mod transport;

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub use client::{Client, ClientError};
pub use fake_server::{ConnectionScript, ConnectionStep, FakeServer, FakeServerScenario};
pub use redaction::{redact, Redacted, REDACTED};
pub use transport::{TcpTransport, TransportError};
