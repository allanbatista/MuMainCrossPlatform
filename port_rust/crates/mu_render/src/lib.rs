pub mod assets;
pub mod materials;
pub mod models;
pub mod terrain;
pub mod textures;

pub mod effects;

pub mod camera;

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub use assets::{RenderAssets, RenderAssetsError, RenderAssetsPlugin, RenderAssetsState};
pub use camera::{hfov_to_vfov, CameraConfig, REFERENCE_ASPECT_RATIO, RENDER_DISTANCE_MULTIPLIER};
pub use effects::{SkillEffectEvent, SkillEffectQueue};
pub use materials::{
    build_render_material_catalog, format_render_material_catalog, RenderMaterialBlendMode,
    RenderMaterialCatalog, RenderMaterialEntry,
};
pub use models::{
    collect_render_models, format_render_model_catalog, RenderModelCatalog, RenderModelEntry,
    RenderModelFamily,
};
pub use terrain::{TerrainPlugin, TerrainRenderer, TerrainState};
pub use textures::{
    collect_render_textures, format_render_texture_catalog, RenderTextureCatalog,
    RenderTextureEntry, RenderTextureFamily,
};
