pub mod bindings;
pub mod camera;
pub mod movement;

pub use bindings::Bindings;
pub use camera::{CameraBindings, CameraMode, CameraTourState};
pub use movement::{MovementBindings, MovementCommand};

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
