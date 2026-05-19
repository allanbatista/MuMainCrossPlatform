use camino::Utf8Path;
use mu_assets::{AssetManifest, AssetManifestEntry};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RenderModelFamily {
    World,
    Object,
    SceneObjects,
    #[default]
    Other,
}

impl RenderModelFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::World => "world",
            Self::Object => "object",
            Self::SceneObjects => "scene-objects",
            Self::Other => "other",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderModelEntry {
    pub family: RenderModelFamily,
    pub key: String,
    pub entry: AssetManifestEntry,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RenderModelCatalog {
    pub world: Vec<RenderModelEntry>,
    pub object: Vec<RenderModelEntry>,
    pub scene_objects: Vec<RenderModelEntry>,
    pub other: Vec<RenderModelEntry>,
}

impl RenderModelCatalog {
    pub fn total_models(&self) -> usize {
        self.world.len() + self.object.len() + self.scene_objects.len() + self.other.len()
    }

    pub fn is_empty(&self) -> bool {
        self.total_models() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &RenderModelEntry> + '_ {
        self.world
            .iter()
            .chain(self.object.iter())
            .chain(self.scene_objects.iter())
            .chain(self.other.iter())
    }
}

pub fn collect_render_models(manifest: &AssetManifest) -> RenderModelCatalog {
    let mut catalog = RenderModelCatalog::default();

    for entry in &manifest.entries {
        if !is_render_model(entry) {
            continue;
        }

        let family = classify_model_family(&entry.converted_path);
        let render_entry = RenderModelEntry {
            family,
            key: model_key(entry),
            entry: entry.clone(),
        };
        push_render_model(&mut catalog, render_entry);
    }

    sort_render_model_catalog(&mut catalog);
    catalog
}

pub fn format_render_model_catalog(catalog: &RenderModelCatalog) -> String {
    let mut out = String::new();
    out.push_str(&format!("total_models: {}\n", catalog.total_models()));
    write_family_section(&mut out, "world", &catalog.world);
    write_family_section(&mut out, "object", &catalog.object);
    write_family_section(&mut out, "scene-objects", &catalog.scene_objects);
    write_family_section(&mut out, "other", &catalog.other);
    out
}

fn push_render_model(catalog: &mut RenderModelCatalog, entry: RenderModelEntry) {
    match entry.family {
        RenderModelFamily::World => catalog.world.push(entry),
        RenderModelFamily::Object => catalog.object.push(entry),
        RenderModelFamily::SceneObjects => catalog.scene_objects.push(entry),
        RenderModelFamily::Other => catalog.other.push(entry),
    }
}

fn sort_render_model_catalog(catalog: &mut RenderModelCatalog) {
    let sort_entries = |entries: &mut Vec<RenderModelEntry>| {
        entries.sort_by(|left, right| {
            left.entry
                .converted_path
                .cmp(&right.entry.converted_path)
                .then_with(|| left.entry.source_path.cmp(&right.entry.source_path))
        });
    };

    sort_entries(&mut catalog.world);
    sort_entries(&mut catalog.object);
    sort_entries(&mut catalog.scene_objects);
    sort_entries(&mut catalog.other);
}

fn write_family_section(out: &mut String, label: &str, entries: &[RenderModelEntry]) {
    out.push('\n');
    out.push_str(&format!("[{label}]\n"));
    for entry in entries {
        out.push_str(&format!(
            "{}: {} -> {}\n",
            entry.key, entry.entry.source_path, entry.entry.converted_path
        ));
    }
}

fn is_render_model(entry: &AssetManifestEntry) -> bool {
    entry.kind == "model" && is_model_output(&entry.converted_path)
}

fn is_model_output(path: &str) -> bool {
    Utf8Path::new(path).extension().is_some_and(|extension| {
        extension.eq_ignore_ascii_case("glb") || extension.eq_ignore_ascii_case("json")
    })
}

fn model_key(entry: &AssetManifestEntry) -> String {
    file_stem(&entry.converted_path)
}

fn classify_model_family(path: &str) -> RenderModelFamily {
    let lower = path.to_ascii_lowercase();

    if lower.contains("scene_objects") {
        return RenderModelFamily::SceneObjects;
    }

    if lower.contains("world") {
        return RenderModelFamily::World;
    }

    if lower.contains("object") {
        return RenderModelFamily::Object;
    }

    RenderModelFamily::Other
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
        collect_render_models, format_render_model_catalog, RenderModelCatalog, RenderModelFamily,
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
    fn collect_render_models_groups_world_object_and_scene_objects() {
        let manifest = manifest_fixture(vec![
            manifest_entry("World1/Terrain.bmd", "data/world_1/terrain.glb", "model"),
            manifest_entry("Object1/tree.bmd", "data/object_1/tree.glb", "model"),
            manifest_entry(
                "World1/scene.obj",
                "data/world_1/scene_objects.json",
                "model",
            ),
            manifest_entry("Interface/icon.bmp", "Interface/icon.png", "texture"),
        ]);

        let catalog = collect_render_models(&manifest);

        assert_eq!(catalog.total_models(), 3);
        assert_eq!(catalog.world[0].family, RenderModelFamily::World);
        assert_eq!(catalog.world[0].key, "terrain");
        assert_eq!(catalog.object[0].key, "tree");
        assert_eq!(catalog.scene_objects[0].key, "scene_objects");
        assert!(catalog.other.is_empty());
    }

    #[test]
    fn render_model_catalog_formats_with_sections() {
        let manifest = manifest_fixture(vec![manifest_entry(
            "World1/Terrain.bmd",
            "data/world_1/terrain.glb",
            "model",
        )]);

        let catalog: RenderModelCatalog = collect_render_models(&manifest);
        let formatted = format_render_model_catalog(&catalog);

        assert!(formatted.contains("total_models: 1"));
        assert!(formatted.contains("[world]"));
        assert!(formatted.contains("World1/Terrain.bmd -> data/world_1/terrain.glb"));
    }
}
