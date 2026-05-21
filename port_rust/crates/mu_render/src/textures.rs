use camino::Utf8Path;
use mu_assets::{AssetManifest, AssetManifestEntry};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RenderTextureFamily {
    Interface,
    World,
    Effect,
    Logo,
    Object,
    #[default]
    Other,
}

impl RenderTextureFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Interface => "interface",
            Self::World => "world",
            Self::Effect => "effect",
            Self::Logo => "logo",
            Self::Object => "object",
            Self::Other => "other",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderTextureEntry {
    pub family: RenderTextureFamily,
    pub key: String,
    pub entry: AssetManifestEntry,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RenderTextureCatalog {
    pub interface: Vec<RenderTextureEntry>,
    pub world: Vec<RenderTextureEntry>,
    pub effect: Vec<RenderTextureEntry>,
    pub logo: Vec<RenderTextureEntry>,
    pub object: Vec<RenderTextureEntry>,
    pub other: Vec<RenderTextureEntry>,
}

impl RenderTextureCatalog {
    pub fn total_textures(&self) -> usize {
        self.interface.len()
            + self.world.len()
            + self.effect.len()
            + self.logo.len()
            + self.object.len()
            + self.other.len()
    }

    pub fn is_empty(&self) -> bool {
        self.total_textures() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &RenderTextureEntry> + '_ {
        self.interface
            .iter()
            .chain(self.world.iter())
            .chain(self.effect.iter())
            .chain(self.logo.iter())
            .chain(self.object.iter())
            .chain(self.other.iter())
    }
}

pub fn collect_render_textures(manifest: &AssetManifest) -> RenderTextureCatalog {
    let mut catalog = RenderTextureCatalog::default();

    for entry in &manifest.entries {
        if !is_render_texture(entry) {
            continue;
        }

        let family = classify_texture_family(&entry.converted_path);
        let render_entry = RenderTextureEntry {
            family,
            key: texture_key(entry),
            entry: entry.clone(),
        };
        push_render_texture(&mut catalog, render_entry);
    }

    sort_render_texture_catalog(&mut catalog);
    catalog
}

pub fn format_render_texture_catalog(catalog: &RenderTextureCatalog) -> String {
    let mut out = String::new();
    out.push_str(&format!("total_textures: {}\n", catalog.total_textures()));
    write_family_section(&mut out, "interface", &catalog.interface);
    write_family_section(&mut out, "world", &catalog.world);
    write_family_section(&mut out, "effect", &catalog.effect);
    write_family_section(&mut out, "logo", &catalog.logo);
    write_family_section(&mut out, "object", &catalog.object);
    write_family_section(&mut out, "other", &catalog.other);
    out
}

fn push_render_texture(catalog: &mut RenderTextureCatalog, entry: RenderTextureEntry) {
    match entry.family {
        RenderTextureFamily::Interface => catalog.interface.push(entry),
        RenderTextureFamily::World => catalog.world.push(entry),
        RenderTextureFamily::Effect => catalog.effect.push(entry),
        RenderTextureFamily::Logo => catalog.logo.push(entry),
        RenderTextureFamily::Object => catalog.object.push(entry),
        RenderTextureFamily::Other => catalog.other.push(entry),
    }
}

fn sort_render_texture_catalog(catalog: &mut RenderTextureCatalog) {
    let sort_entries = |entries: &mut Vec<RenderTextureEntry>| {
        entries.sort_by(|left, right| {
            left.entry
                .converted_path
                .cmp(&right.entry.converted_path)
                .then_with(|| left.entry.source_path.cmp(&right.entry.source_path))
        });
    };

    sort_entries(&mut catalog.interface);
    sort_entries(&mut catalog.world);
    sort_entries(&mut catalog.effect);
    sort_entries(&mut catalog.logo);
    sort_entries(&mut catalog.object);
    sort_entries(&mut catalog.other);
}

fn write_family_section(out: &mut String, label: &str, entries: &[RenderTextureEntry]) {
    out.push('\n');
    out.push_str(&format!("[{label}]\n"));
    for entry in entries {
        out.push_str(&format!(
            "{}: {} -> {}\n",
            entry.key, entry.entry.source_path, entry.entry.converted_path
        ));
    }
}

fn is_render_texture(entry: &AssetManifestEntry) -> bool {
    entry.kind == "texture" && has_png_extension(&entry.converted_path)
}

fn has_png_extension(path: &str) -> bool {
    Utf8Path::new(path)
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("png"))
}

fn texture_key(entry: &AssetManifestEntry) -> String {
    file_stem(&entry.converted_path)
}

fn classify_texture_family(path: &str) -> RenderTextureFamily {
    let lower = path.to_ascii_lowercase();

    if lower.contains("interface") {
        return RenderTextureFamily::Interface;
    }

    if lower.contains("world") {
        return RenderTextureFamily::World;
    }

    if lower.contains("effect") {
        return RenderTextureFamily::Effect;
    }

    if lower.contains("logo") {
        return RenderTextureFamily::Logo;
    }

    if lower.contains("object") {
        return RenderTextureFamily::Object;
    }

    RenderTextureFamily::Other
}

fn file_stem(path: &str) -> String {
    Utf8Path::new(path)
        .file_stem()
        .map(str::to_owned)
        .unwrap_or_else(|| path.to_owned())
}

#[cfg(test)]
mod tests {
    use super::{
        collect_render_textures, format_render_texture_catalog, RenderTextureCatalog,
        RenderTextureFamily,
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
    fn collect_render_textures_groups_entries_by_family() {
        let manifest = manifest_fixture(vec![
            manifest_entry("Interface/icon.bmp", "Interface/icon.png", "texture"),
            manifest_entry(
                "World1/TileGround01.bmp",
                "data/world_1/TileGround01.png",
                "texture",
            ),
            manifest_entry("Effect/rain01.ozj", "Effect/rain01.png", "texture"),
            manifest_entry("Logo/logo.bmp", "Logo/logo.png", "texture"),
            manifest_entry("Object1/tree.bmp", "data/object_1/tree.png", "texture"),
            manifest_entry("Notes/readme.txt", "Notes/readme.txt", "data"),
        ]);

        let catalog = collect_render_textures(&manifest);

        assert_eq!(catalog.total_textures(), 5);
        assert_eq!(catalog.interface[0].family, RenderTextureFamily::Interface);
        assert_eq!(catalog.interface[0].key, "icon");
        assert_eq!(catalog.world[0].key, "TileGround01");
        assert_eq!(catalog.effect[0].key, "rain01");
        assert_eq!(catalog.logo[0].key, "logo");
        assert_eq!(catalog.object[0].key, "tree");
        assert!(catalog.other.is_empty());
    }

    #[test]
    fn render_texture_catalog_formats_with_sections() {
        let manifest = manifest_fixture(vec![manifest_entry(
            "Interface/icon.bmp",
            "Interface/icon.png",
            "texture",
        )]);

        let catalog: RenderTextureCatalog = collect_render_textures(&manifest);
        let formatted = format_render_texture_catalog(&catalog);

        assert!(formatted.contains("total_textures: 1"));
        assert!(formatted.contains("[interface]"));
        assert!(formatted.contains("Interface/icon.bmp -> Interface/icon.png"));
    }
}
