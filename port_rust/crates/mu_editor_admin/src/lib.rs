pub mod auth;

pub use auth::{AdminAccessState, AdminAuthState, AdminPlugin};

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
