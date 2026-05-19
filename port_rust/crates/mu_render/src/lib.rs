pub mod effects;

pub mod camera;

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub use camera::{hfov_to_vfov, CameraConfig, REFERENCE_ASPECT_RATIO, RENDER_DISTANCE_MULTIPLIER};
pub use effects::{SkillEffectEvent, SkillEffectQueue};
