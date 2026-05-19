use mu_assets::AssetManifestEntry;

use crate::textures::{RenderTextureCatalog, RenderTextureEntry, RenderTextureFamily};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RenderMaterialBlendMode {
    #[default]
    Opaque,
    AlphaMask,
}

impl RenderMaterialBlendMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Opaque => "opaque",
            Self::AlphaMask => "alpha-mask",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderMaterialEntry {
    pub key: String,
    pub texture_path: String,
    pub blend_mode: RenderMaterialBlendMode,
    pub texture_family: RenderTextureFamily,
    pub entry: AssetManifestEntry,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RenderMaterialCatalog {
    pub opaque: Vec<RenderMaterialEntry>,
    pub alpha_mask: Vec<RenderMaterialEntry>,
}

impl RenderMaterialCatalog {
    pub fn total_materials(&self) -> usize {
        self.opaque.len() + self.alpha_mask.len()
    }

    pub fn is_empty(&self) -> bool {
        self.total_materials() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &RenderMaterialEntry> + '_ {
        self.opaque.iter().chain(self.alpha_mask.iter())
    }
}

pub fn build_render_material_catalog(textures: &RenderTextureCatalog) -> RenderMaterialCatalog {
    let mut catalog = RenderMaterialCatalog::default();

    for texture in textures.iter() {
        let entry = RenderMaterialEntry {
            key: texture.key.clone(),
            texture_path: texture.entry.converted_path.clone(),
            blend_mode: classify_material_blend_mode(texture),
            texture_family: texture.family,
            entry: texture.entry.clone(),
        };
        push_render_material(&mut catalog, entry);
    }

    sort_render_material_catalog(&mut catalog);
    catalog
}

pub fn format_render_material_catalog(catalog: &RenderMaterialCatalog) -> String {
    let mut out = String::new();
    out.push_str(&format!("total_materials: {}\n", catalog.total_materials()));
    write_family_section(&mut out, "opaque", &catalog.opaque);
    write_family_section(&mut out, "alpha-mask", &catalog.alpha_mask);
    out
}

fn push_render_material(catalog: &mut RenderMaterialCatalog, entry: RenderMaterialEntry) {
    match entry.blend_mode {
        RenderMaterialBlendMode::Opaque => catalog.opaque.push(entry),
        RenderMaterialBlendMode::AlphaMask => catalog.alpha_mask.push(entry),
    }
}

fn sort_render_material_catalog(catalog: &mut RenderMaterialCatalog) {
    let sort_entries = |entries: &mut Vec<RenderMaterialEntry>| {
        entries.sort_by(|left, right| {
            left.texture_path
                .cmp(&right.texture_path)
                .then_with(|| left.key.cmp(&right.key))
        });
    };

    sort_entries(&mut catalog.opaque);
    sort_entries(&mut catalog.alpha_mask);
}

fn write_family_section(out: &mut String, label: &str, entries: &[RenderMaterialEntry]) {
    out.push('\n');
    out.push_str(&format!("[{label}]\n"));
    for entry in entries {
        out.push_str(&format!(
            "{}: {} -> {}\n",
            entry.key,
            entry.texture_path,
            entry.blend_mode.as_str()
        ));
    }
}

fn classify_material_blend_mode(texture: &RenderTextureEntry) -> RenderMaterialBlendMode {
    let key = texture.key.to_ascii_lowercase();
    let path = texture.entry.converted_path.to_ascii_lowercase();

    if key.contains("alpha") || path.contains("alpha") {
        RenderMaterialBlendMode::AlphaMask
    } else {
        RenderMaterialBlendMode::Opaque
    }
}

#[cfg(test)]
mod tests {
    use super::{build_render_material_catalog, format_render_material_catalog};
    use crate::textures::{
        collect_render_textures, RenderTextureCatalog, RenderTextureEntry, RenderTextureFamily,
    };
    use mu_assets::{sha256_hex, AssetManifest, AssetManifestEntry, SUPPORTED_SCHEMA_VERSION};

    fn manifest_entry(source_path: &str, converted_path: &str, kind: &str) -> AssetManifestEntry {
        let bytes = converted_path.as_bytes();
        AssetManifestEntry {
            source_path: source_path.to_string(),
            converted_path: converted_path.to_string(),
            kind: kind.to_string(),
            source_hash: sha256_hex(source_path.as_bytes()),
            converted_hash: sha256_hex(bytes),
            dependencies: Vec::new(),
        }
    }

    fn manifest_fixture(entries: Vec<AssetManifestEntry>) -> AssetManifest {
        AssetManifest {
            schema_version: SUPPORTED_SCHEMA_VERSION,
            source_client_version: "1.0.0".to_string(),
            content_hash: String::new(),
            generated_at: "2026-05-19T00:00:00Z".to_string(),
            entries,
        }
        .with_computed_content_hash()
    }

    #[test]
    fn build_render_material_catalog_marks_alpha_textures() {
        let manifest = manifest_fixture(vec![
            manifest_entry(
                "World1/TileGround01.bmp",
                "data/world_1/TileGround01.png",
                "texture",
            ),
            manifest_entry(
                "World1/TileGrass01_alpha.bmp",
                "data/world_1/TileGrass01_alpha.png",
                "texture",
            ),
        ]);

        let textures = collect_render_textures(&manifest);
        let catalog = build_render_material_catalog(&textures);

        assert_eq!(catalog.total_materials(), 2);
        assert_eq!(catalog.opaque[0].key, "TileGround01");
        assert_eq!(catalog.alpha_mask[0].key, "TileGrass01_alpha");
    }

    #[test]
    fn render_material_catalog_formats_with_sections() {
        let textures = RenderTextureCatalog {
            interface: vec![RenderTextureEntry {
                family: RenderTextureFamily::Interface,
                key: "icon".to_string(),
                entry: manifest_entry("Interface/icon.bmp", "Interface/icon.png", "texture"),
            }],
            ..RenderTextureCatalog::default()
        };

        let catalog = build_render_material_catalog(&textures);
        let formatted = format_render_material_catalog(&catalog);

        assert!(formatted.contains("total_materials: 1"));
        assert!(formatted.contains("[opaque]"));
        assert!(formatted.contains("icon: Interface/icon.png -> opaque"));
    }
}
