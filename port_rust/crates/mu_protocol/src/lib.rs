pub mod chat;
pub mod codec;
pub mod connect;
pub mod connect_server;
pub mod error;
pub mod frame;
pub mod login;
pub mod session;
pub mod wire;

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub use codec::{decode_packet, encode_packet};
pub use connect::ServerEntry;
pub use error::PacketCodecError;
pub use frame::PacketFrame;
