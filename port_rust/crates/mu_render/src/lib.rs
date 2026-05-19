pub mod assets;
pub mod camera;
pub mod effects;
pub mod entities;
pub mod materials;
pub mod models;
pub mod particles;
pub mod terrain;
pub mod textures;

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

pub use assets::{RenderAssets, RenderAssetsError, RenderAssetsPlugin, RenderAssetsState};
pub use camera::{hfov_to_vfov, CameraConfig, REFERENCE_ASPECT_RATIO, RENDER_DISTANCE_MULTIPLIER};
pub use effects::{SkillEffectEvent, SkillEffectQueue, SkillParticleCue};
pub use entities::{
    collect_render_entities, format_render_entity_catalog, RenderEntities, RenderEntitiesPlugin,
    RenderEntitiesState, RenderEntityCatalog, RenderEntityEntry, RenderEntityFamily,
};
pub use materials::{
    build_render_material_catalog, format_render_material_catalog, RenderMaterialBlendMode,
    RenderMaterialCatalog, RenderMaterialEntry,
};
pub use models::{
    collect_render_models, format_render_model_catalog, RenderModelCatalog, RenderModelEntry,
    RenderModelFamily,
};
pub use particles::{
    SkillParticleEvent, SkillParticlePlugin, SkillParticleQueue, SkillParticleState,
};
pub use terrain::{TerrainPlugin, TerrainRenderer, TerrainState};
pub use textures::{
    collect_render_textures, format_render_texture_catalog, RenderTextureCatalog,
    RenderTextureEntry, RenderTextureFamily,
};
