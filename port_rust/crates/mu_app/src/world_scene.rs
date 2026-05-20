use std::fs;

use bevy::asset::RenderAssetUsages;
use bevy::gltf::GltfAssetLabel;
use bevy::math::primitives::Cuboid;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::{
    AlphaMode, App, AssetServer, Assets, Camera3d, Color, Commands, Component, DirectionalLight,
    Entity, EulerRot, IntoScheduleConfigs, Mesh, Mesh3d, MeshMaterial3d, Name, Plugin, Query, Res,
    ResMut, Resource, SceneRoot, StandardMaterial, Transform, Update, Vec3,
};
use camino::Utf8Path;
use mu_assets::TerrainWorldSummary;
use mu_render::{RenderEntityCatalog, RenderEntityEntry, RenderEntityFamily};
use mu_ui::{UiRoute, UiShellState};

use crate::{ClientRuntime, GraphicalRuntimeConfig};

const WORLD_POSITION_SCALE: f32 = 0.05;
const WORLD_TERRAIN_AMPLITUDE: f32 = 1.0;
const WORLD_TERRAIN_FALLBACK_THICKNESS: f32 = 0.2;
const WORLD_TERRAIN_TEXTURE_REPEAT: f32 = 8.0;
const WORLD_TERRAIN_BLEND_OFFSET: f32 = 0.01;
const WORLD_TERRAIN_LIGHTMAP_OFFSET: f32 = 0.02;
const WORLD_TERRAIN_LIGHTMAP_OPACITY: f32 = 0.65;
const WORLD_CAMERA_HEIGHT_FACTOR: f32 = 1.8;
const WORLD_CAMERA_DISTANCE_FACTOR: f32 = 2.2;
const WORLD_RENDER_ENTITY_LIMIT: usize = 8;
const WORLD_LIGHT_LEVEL: f32 = 30_000.0;

#[derive(Debug, Default, Resource)]
struct WorldSceneState {
    world_id: Option<u32>,
    entities: Vec<Entity>,
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
struct WorldSceneMarker {
    family: RenderEntityFamily,
    key: String,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct WorldSceneCamera {
    follow_offset: Vec3,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
struct WorldSceneTerrain;

#[derive(Debug, Default, Clone, Copy)]
pub struct WorldScenePlugin;

impl Plugin for WorldScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldSceneState>().add_systems(
            Update,
            (
                sync_world_scene_system,
                sync_world_scene_transforms_system
                    .after(crate::world_motion::apply_world_motion_system),
                sync_world_scene_camera_system.after(sync_world_scene_transforms_system),
            )
                .chain(),
        );
    }
}

fn sync_world_scene_system(
    mut commands: Commands,
    mut state: ResMut<WorldSceneState>,
    ui_shell: Res<UiShellState>,
    client_runtime: Res<ClientRuntime>,
    config: Res<GraphicalRuntimeConfig>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if ui_shell.current() != UiRoute::World
        || !client_runtime.world_ready()
        || !client_runtime.render_entities_ready()
    {
        clear_world_scene(&mut commands, &mut state);
        return;
    }

    let Some(world_bundle) = client_runtime.world().bundle() else {
        clear_world_scene(&mut commands, &mut state);
        return;
    };

    let summary = world_bundle.summary();
    if state.world_id == Some(summary.world) && !state.entities.is_empty() {
        return;
    }

    clear_world_scene(&mut commands, &mut state);
    spawn_world_scene(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        world_bundle,
        config.asset_root.as_deref(),
        client_runtime.render_entities().catalog(),
        &mut state,
    );
    state.world_id = Some(summary.world);
}

fn sync_world_scene_transforms_system(
    ui_shell: Res<UiShellState>,
    client_runtime: Res<ClientRuntime>,
    state: Res<WorldSceneState>,
    mut markers: Query<(&WorldSceneMarker, &mut Transform)>,
) {
    if ui_shell.current() != UiRoute::World
        || !client_runtime.world_ready()
        || !client_runtime.render_entities_ready()
        || state.entities.is_empty()
    {
        return;
    }

    let catalog = client_runtime.render_entities().catalog();

    for (marker, mut transform) in markers.iter_mut() {
        if let Some(entry) = render_entity_for_marker(catalog, marker) {
            *transform = world_transform(&entry.pose);
        }
    }
}

fn sync_world_scene_camera_system(
    ui_shell: Res<UiShellState>,
    client_runtime: Res<ClientRuntime>,
    state: Res<WorldSceneState>,
    mut cameras: Query<(&WorldSceneCamera, &mut Transform)>,
) {
    if ui_shell.current() != UiRoute::World
        || !client_runtime.world_ready()
        || !client_runtime.render_entities_ready()
        || state.entities.is_empty()
    {
        return;
    }

    let Some(local_player) = client_runtime.world_entities().local_player() else {
        return;
    };

    let local_player_translation = world_transform(&local_player.pose).translation;

    for (camera, mut transform) in cameras.iter_mut() {
        *transform = Transform::from_translation(local_player_translation + camera.follow_offset)
            .looking_at(local_player_translation, Vec3::Y);
    }
}

fn clear_world_scene(commands: &mut Commands, state: &mut WorldSceneState) {
    for entity in state.entities.drain(..) {
        commands.entity(entity).despawn();
    }
    state.world_id = None;
}

fn spawn_world_scene(
    commands: &mut Commands,
    asset_server: &AssetServer,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    world_bundle: &mu_gameplay::TerrainWorldBundle,
    asset_root: Option<&Utf8Path>,
    catalog: &RenderEntityCatalog,
    state: &mut WorldSceneState,
) {
    let summary = world_bundle.summary();
    state.entities.push(spawn_world_camera(commands, &summary));
    state.entities.push(spawn_world_light(commands));
    state.entities.extend(spawn_world_terrain(
        commands,
        asset_server,
        meshes,
        materials,
        world_bundle,
        asset_root,
    ));
    spawn_world_entities(commands, asset_server, meshes, materials, catalog, state);
}

fn spawn_world_camera(commands: &mut Commands, summary: &TerrainWorldSummary) -> Entity {
    let follow_offset = world_camera_follow_offset(summary);

    commands
        .spawn((
            Camera3d::default(),
            Transform::from_translation(follow_offset).looking_at(Vec3::ZERO, Vec3::Y),
            WorldSceneCamera { follow_offset },
            Name::new(format!("world-camera-{}", summary.world)),
        ))
        .id()
}

fn world_camera_follow_offset(summary: &TerrainWorldSummary) -> Vec3 {
    let extent = summary.terrain_size as f32 * WORLD_POSITION_SCALE;
    Vec3::new(
        extent * WORLD_CAMERA_DISTANCE_FACTOR,
        extent * WORLD_CAMERA_HEIGHT_FACTOR,
        extent * WORLD_CAMERA_DISTANCE_FACTOR,
    )
}

fn spawn_world_light(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            DirectionalLight {
                illuminance: WORLD_LIGHT_LEVEL,
                shadows_enabled: false,
                ..Default::default()
            },
            Transform::from_rotation(bevy::prelude::Quat::from_euler(
                EulerRot::XYZ,
                -0.7,
                -0.4,
                0.0,
            )),
            Name::new("world-light"),
        ))
        .id()
}

fn spawn_world_terrain(
    commands: &mut Commands,
    asset_server: &AssetServer,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    world_bundle: &mu_gameplay::TerrainWorldBundle,
    asset_root: Option<&Utf8Path>,
) -> Vec<Entity> {
    let summary = world_bundle.summary();
    let extent = summary.terrain_size as f32 * WORLD_POSITION_SCALE;
    let Some(base_mesh) = build_world_terrain_mesh(&summary, &world_bundle.map) else {
        return vec![spawn_world_terrain_surface(
            commands,
            meshes,
            materials,
            Mesh::from(Cuboid::new(
                extent,
                WORLD_TERRAIN_FALLBACK_THICKNESS,
                extent,
            )),
            solid_world_terrain_material(),
            Vec3::new(0.0, -WORLD_TERRAIN_FALLBACK_THICKNESS * 0.5, 0.0),
            format!("world-terrain-{}", summary.world),
        )];
    };

    let (base_texture_path, blend_texture_path) = world_terrain_texture_paths(world_bundle);
    let mut entities = Vec::new();
    let base_material =
        build_world_terrain_material(asset_server, base_texture_path, AlphaMode::Opaque);
    entities.push(spawn_world_terrain_surface(
        commands,
        meshes,
        materials,
        base_mesh,
        base_material,
        Vec3::new(0.0, -WORLD_TERRAIN_FALLBACK_THICKNESS * 0.5, 0.0),
        format!("world-terrain-base-{}", summary.world),
    ));

    if let Some(lightmap_path) = world_terrain_lightmap_path(asset_root, world_bundle) {
        if let Some(lightmap_mesh) = build_world_terrain_mesh(&summary, &world_bundle.map) {
            let lightmap_material =
                build_world_terrain_lightmap_material(asset_server, Some(lightmap_path.as_str()));
            entities.push(spawn_world_terrain_surface(
                commands,
                meshes,
                materials,
                lightmap_mesh,
                lightmap_material,
                Vec3::new(
                    0.0,
                    -WORLD_TERRAIN_FALLBACK_THICKNESS * 0.5 + WORLD_TERRAIN_LIGHTMAP_OFFSET,
                    0.0,
                ),
                format!("world-terrain-lightmap-{}", summary.world),
            ));
        }
    }

    if let (Some(blend_texture_path), Some(blend_mesh)) = (
        blend_texture_path,
        build_world_terrain_blend_mesh(&summary, &world_bundle.map),
    ) {
        let blend_material =
            build_world_terrain_material(asset_server, Some(blend_texture_path), AlphaMode::Blend);
        entities.push(spawn_world_terrain_surface(
            commands,
            meshes,
            materials,
            blend_mesh,
            blend_material,
            Vec3::new(
                0.0,
                -WORLD_TERRAIN_FALLBACK_THICKNESS * 0.5 + WORLD_TERRAIN_BLEND_OFFSET,
                0.0,
            ),
            format!("world-terrain-blend-{}", summary.world),
        ));
    }

    entities
}

fn world_terrain_lightmap_path(
    asset_root: Option<&Utf8Path>,
    world_bundle: &mu_gameplay::TerrainWorldBundle,
) -> Option<String> {
    let asset_root = asset_root?;
    resolve_world_asset_path(asset_root, &world_bundle.config.lightmap)
}

fn build_world_terrain_mesh(
    summary: &TerrainWorldSummary,
    map: &mu_assets::TerrainMapJson,
) -> Option<Mesh> {
    let terrain_size = summary.terrain_size as usize;
    if terrain_size < 2 {
        return None;
    }

    let height_rows = map.layer1.as_slice();
    if height_rows.len() != terrain_size || height_rows.iter().any(|row| row.len() != terrain_size)
    {
        return None;
    }

    let half_extent = (terrain_size as f32 - 1.0) * WORLD_POSITION_SCALE * 0.5;
    let height_scale = WORLD_TERRAIN_AMPLITUDE / f32::from(summary.layer_stats.layer1.max.max(1));

    let mut positions = Vec::with_capacity(terrain_size * terrain_size);
    for (z, row) in height_rows.iter().enumerate() {
        for (x, sample) in row.iter().enumerate() {
            positions.push([
                x as f32 * WORLD_POSITION_SCALE - half_extent,
                f32::from(*sample) * height_scale,
                z as f32 * WORLD_POSITION_SCALE - half_extent,
            ]);
        }
    }
    let uvs = build_world_terrain_uvs(terrain_size);

    let mut indices = Vec::with_capacity((terrain_size - 1) * (terrain_size - 1) * 6);
    for z in 0..(terrain_size - 1) {
        for x in 0..(terrain_size - 1) {
            let top_left = (z * terrain_size + x) as u32;
            let top_right = top_left + 1;
            let bottom_left = top_left + terrain_size as u32;
            let bottom_right = bottom_left + 1;
            indices.extend_from_slice(&[
                top_left,
                bottom_left,
                top_right,
                top_right,
                bottom_left,
                bottom_right,
            ]);
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh.compute_smooth_normals();
    Some(mesh)
}

fn build_world_terrain_uvs(terrain_size: usize) -> Vec<[f32; 2]> {
    let tile_step = WORLD_TERRAIN_TEXTURE_REPEAT / (terrain_size as f32 - 1.0);
    let mut uvs = Vec::with_capacity(terrain_size * terrain_size);

    for z in 0..terrain_size {
        for x in 0..terrain_size {
            uvs.push([x as f32 * tile_step, z as f32 * tile_step]);
        }
    }

    uvs
}

fn build_world_terrain_material(
    asset_server: &AssetServer,
    texture_path: Option<&str>,
    alpha_mode: AlphaMode,
) -> StandardMaterial {
    let Some(texture_path) = texture_path else {
        return solid_world_terrain_material();
    };

    StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(asset_server.load(texture_path.to_owned())),
        perceptual_roughness: 1.0,
        cull_mode: None,
        alpha_mode,
        ..Default::default()
    }
}

fn build_world_terrain_lightmap_material(
    asset_server: &AssetServer,
    texture_path: Option<&str>,
) -> StandardMaterial {
    let Some(texture_path) = texture_path else {
        return solid_world_terrain_material();
    };

    StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, WORLD_TERRAIN_LIGHTMAP_OPACITY),
        base_color_texture: Some(asset_server.load(texture_path.to_owned())),
        perceptual_roughness: 1.0,
        cull_mode: None,
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    }
}

fn solid_world_terrain_material() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgb(0.12, 0.28, 0.12),
        perceptual_roughness: 1.0,
        cull_mode: None,
        ..Default::default()
    }
}

fn world_terrain_texture_paths(
    world_bundle: &mu_gameplay::TerrainWorldBundle,
) -> (Option<&str>, Option<&str>) {
    let mut texture_paths = world_bundle.texture_slots.slots.values();
    let base = texture_paths.next().map(String::as_str);
    let blend = texture_paths.next().map(String::as_str);
    (base, blend)
}

fn resolve_world_asset_path(asset_root: &Utf8Path, relative_path: &str) -> Option<String> {
    let relative_path = Utf8Path::new(relative_path);
    if asset_root.join(relative_path).exists() {
        return Some(relative_path.to_string());
    }

    let parent = relative_path.parent().unwrap_or_else(|| Utf8Path::new(""));
    let checked_in_fallback = parent.join("TerrainLight.png");
    if asset_root.join(&checked_in_fallback).exists() {
        return Some(checked_in_fallback.to_string());
    }

    let file_name = relative_path.file_name()?;
    let directory = asset_root.join(parent);
    let actual_name = fs::read_dir(&directory)
        .ok()?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .find(|entry| entry.eq_ignore_ascii_case(file_name))?;

    Some(parent.join(actual_name).to_string())
}

fn build_world_terrain_blend_mesh(
    summary: &TerrainWorldSummary,
    map: &mu_assets::TerrainMapJson,
) -> Option<Mesh> {
    let terrain_size = summary.terrain_size as usize;
    let alpha_rows = map.alpha.as_slice();
    if alpha_rows.len() != terrain_size || alpha_rows.iter().any(|row| row.len() != terrain_size) {
        return None;
    }

    let mut mesh = build_world_terrain_mesh(summary, map)?;
    let colors = build_world_terrain_blend_colors(alpha_rows)?;
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    Some(mesh)
}

fn build_world_terrain_blend_colors(alpha_rows: &[Vec<u8>]) -> Option<Vec<[f32; 4]>> {
    let Some(terrain_size) = alpha_rows.first().map(Vec::len) else {
        return None;
    };

    if alpha_rows.len() != terrain_size || alpha_rows.iter().any(|row| row.len() != terrain_size) {
        return None;
    }

    let mut colors = Vec::with_capacity(terrain_size * terrain_size);
    for row in alpha_rows {
        for sample in row {
            let alpha = f32::from(*sample) / 255.0;
            colors.push([1.0, 1.0, 1.0, alpha]);
        }
    }

    Some(colors)
}

fn spawn_world_terrain_surface(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    mesh: Mesh,
    material: StandardMaterial,
    translation: Vec3,
    name: String,
) -> Entity {
    commands
        .spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(material)),
            Transform::from_translation(translation),
            WorldSceneTerrain,
            Name::new(name),
        ))
        .id()
}

fn spawn_world_entities(
    commands: &mut Commands,
    asset_server: &AssetServer,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    catalog: &RenderEntityCatalog,
    state: &mut WorldSceneState,
) {
    if let Some(entry) = catalog.local_player.as_ref() {
        state.entities.push(spawn_world_marker(
            commands,
            meshes,
            materials,
            entry,
            marker_style(RenderEntityFamily::LocalPlayer),
            "local-player",
        ));
    }

    spawn_world_marker_family(
        commands,
        meshes,
        materials,
        &catalog.remote_players,
        RenderEntityFamily::RemotePlayer,
        state,
        "remote-player",
    );
    spawn_world_scene_family(
        commands,
        asset_server,
        &catalog.objects,
        RenderEntityFamily::Object,
        state,
        "object",
    );
    spawn_world_scene_family(
        commands,
        asset_server,
        &catalog.npcs,
        RenderEntityFamily::Npc,
        state,
        "npc",
    );
    spawn_world_scene_family(
        commands,
        asset_server,
        &catalog.monsters,
        RenderEntityFamily::Monster,
        state,
        "monster",
    );
}

fn spawn_world_marker_family(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    entries: &[RenderEntityEntry],
    family: RenderEntityFamily,
    state: &mut WorldSceneState,
    label_prefix: &str,
) {
    for entry in entries.iter().take(WORLD_RENDER_ENTITY_LIMIT) {
        state.entities.push(spawn_world_marker(
            commands,
            meshes,
            materials,
            entry,
            marker_style(family),
            label_prefix,
        ));
    }
}

fn spawn_world_scene_family(
    commands: &mut Commands,
    asset_server: &AssetServer,
    entries: &[RenderEntityEntry],
    family: RenderEntityFamily,
    state: &mut WorldSceneState,
    label_prefix: &str,
) {
    for entry in entries.iter().take(WORLD_RENDER_ENTITY_LIMIT) {
        state.entities.push(spawn_world_scene_entity(
            commands,
            asset_server,
            entry,
            family,
            label_prefix,
        ));
    }
}

fn spawn_world_scene_entity(
    commands: &mut Commands,
    asset_server: &AssetServer,
    entry: &RenderEntityEntry,
    family: RenderEntityFamily,
    label_prefix: &str,
) -> Entity {
    commands
        .spawn((
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(entry.model.clone()))),
            world_transform(&entry.pose),
            WorldSceneMarker {
                family,
                key: entry.key.clone(),
            },
            Name::new(format!("{label_prefix}-{}-{}", entry.key, entry.label)),
        ))
        .id()
}

fn spawn_world_marker(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    entry: &RenderEntityEntry,
    (color, marker_size): (Color, f32),
    label_prefix: &str,
) -> Entity {
    let mesh = meshes.add(Mesh::from(Cuboid::new(
        marker_size,
        marker_size,
        marker_size,
    )));
    let material = materials.add(StandardMaterial {
        base_color: color,
        perceptual_roughness: 0.9,
        ..Default::default()
    });

    commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            world_transform(&entry.pose),
            WorldSceneMarker {
                family: entry.family,
                key: entry.key.clone(),
            },
            Name::new(format!("{label_prefix}-{}-{}", entry.key, entry.label)),
        ))
        .id()
}

fn world_transform(pose: &mu_gameplay::WorldEntityPose) -> Transform {
    let translation = Vec3::new(
        pose.position[0] as f32,
        pose.position[1] as f32,
        pose.position[2] as f32,
    ) * WORLD_POSITION_SCALE;
    let rotation = bevy::prelude::Quat::from_euler(
        EulerRot::XYZ,
        (pose.rotation[0] as f32).to_radians(),
        (pose.rotation[1] as f32).to_radians(),
        (pose.rotation[2] as f32).to_radians(),
    );
    let scale = Vec3::new(
        pose.scale[0] as f32,
        pose.scale[1] as f32,
        pose.scale[2] as f32,
    );

    Transform {
        translation,
        rotation,
        scale,
    }
}

fn marker_style(family: RenderEntityFamily) -> (Color, f32) {
    match family {
        RenderEntityFamily::LocalPlayer => (Color::srgb(0.95, 0.80, 0.20), 1.6),
        RenderEntityFamily::RemotePlayer => (Color::srgb(0.20, 0.70, 1.00), 1.3),
        RenderEntityFamily::Object => (Color::srgb(0.55, 0.50, 0.45), 0.9),
        RenderEntityFamily::Npc => (Color::srgb(0.65, 0.40, 0.85), 1.1),
        RenderEntityFamily::Monster => (Color::srgb(0.95, 0.25, 0.25), 1.4),
        RenderEntityFamily::Other => (Color::srgb(0.80, 0.80, 0.80), 1.0),
    }
}

fn render_entity_for_marker<'a>(
    catalog: &'a RenderEntityCatalog,
    marker: &WorldSceneMarker,
) -> Option<&'a RenderEntityEntry> {
    match marker.family {
        RenderEntityFamily::LocalPlayer => catalog
            .local_player
            .as_ref()
            .filter(|entry| entry.key == marker.key),
        RenderEntityFamily::RemotePlayer => catalog
            .remote_players
            .iter()
            .find(|entry| entry.key == marker.key),
        RenderEntityFamily::Object => catalog.objects.iter().find(|entry| entry.key == marker.key),
        RenderEntityFamily::Npc => catalog.npcs.iter().find(|entry| entry.key == marker.key),
        RenderEntityFamily::Monster => catalog
            .monsters
            .iter()
            .find(|entry| entry.key == marker.key),
        RenderEntityFamily::Other => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        build_world_terrain_blend_mesh, build_world_terrain_mesh, world_terrain_lightmap_path,
        world_terrain_texture_paths, WorldSceneCamera, WorldSceneMarker, WorldScenePlugin,
        WorldSceneState, WorldSceneTerrain, WORLD_POSITION_SCALE, WORLD_TERRAIN_TEXTURE_REPEAT,
    };
    use crate::ClientRuntime;
    use crate::GraphicalRuntimeConfig;
    use bevy::asset::AssetPlugin;
    use bevy::mesh::VertexAttributeValues;
    use bevy::prelude::{App, AssetApp, Camera3d, Mesh3d, Name, SceneRoot};
    use bevy::{gltf::GltfPlugin, scene::ScenePlugin};
    use camino::Utf8PathBuf;
    use mu_assets::load_terrain_world_bundle;
    use mu_gameplay::{
        WorldEntityPose, WorldMonsterKind, WorldMonsterSpawn, WorldNpcKind, WorldNpcSpawn,
    };
    use mu_render::RenderEntityFamily;
    use mu_ui::{UiRoute, UiShellState};

    fn repo_assets_root() -> Utf8PathBuf {
        Utf8PathBuf::from_path_buf(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../../port_rust/assets")
                .canonicalize()
                .unwrap(),
        )
        .unwrap()
    }

    fn spawn_ready_app() -> App {
        bevy::tasks::IoTaskPool::get_or_init(bevy::tasks::TaskPool::new);

        let mut app = App::new();
        let asset_root = repo_assets_root();
        app.add_plugins((
            mu_ui::UiShellPlugin,
            AssetPlugin {
                file_path: asset_root.to_string(),
                ..Default::default()
            },
            GltfPlugin::default(),
            ScenePlugin::default(),
            WorldScenePlugin,
        ));
        app.init_asset::<bevy::prelude::Image>();
        app.insert_resource(GraphicalRuntimeConfig {
            asset_root: Some(asset_root.clone()),
            server: None,
            config_path: Utf8PathBuf::from("config/client.toml"),
            editor_admin: false,
            offline_fixture: None,
            evidence_dir: None,
        });
        app.insert_resource(ClientRuntime::new());
        app.insert_resource(bevy::prelude::Assets::<bevy::prelude::Mesh>::default());
        app.insert_resource(bevy::prelude::Assets::<bevy::prelude::StandardMaterial>::default());
        app
    }

    fn load_world(app: &mut App) {
        let world_root = repo_assets_root();
        let bundle = load_terrain_world_bundle(&world_root, 1).unwrap();
        {
            let mut runtime = app.world_mut().resource_mut::<ClientRuntime>();
            runtime.load_world_bundle(bundle);
            runtime.world_npcs_mut().load_fixture([
                WorldNpcSpawn::new(
                    WorldNpcKind::QuestGiver,
                    "Marlon",
                    236,
                    18,
                    "data/object_1/npc_quest.glb",
                    WorldEntityPose::new([12.0, 0.0, 24.0], [0.0, 180.0, 0.0], [1.0, 1.0, 1.0]),
                ),
                WorldNpcSpawn::new(
                    WorldNpcKind::Merchant,
                    "Potion Merchant",
                    237,
                    0,
                    "data/object_1/npc_merchant.glb",
                    WorldEntityPose::new([18.0, 0.0, 30.0], [0.0, 90.0, 0.0], [1.0, 1.0, 1.0]),
                ),
            ]);
            runtime.world_monsters_mut().load_fixture([
                WorldMonsterSpawn::new(
                    WorldMonsterKind::Common,
                    "Bull Fighter",
                    401,
                    32,
                    "data/object_1/bull_fighter.glb",
                    WorldEntityPose::new([42.0, 0.0, 88.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]),
                ),
                WorldMonsterSpawn::new(
                    WorldMonsterKind::Boss,
                    "Death King",
                    777,
                    120,
                    "data/object_1/death_king.glb",
                    WorldEntityPose::new([64.0, 0.0, 96.0], [0.0, 45.0, 0.0], [1.2, 1.2, 1.2]),
                ),
            ]);
            runtime.sync_world_projection();
        }
        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::World);
        app.update();
    }

    #[test]
    fn world_scene_plugin_registers_state() {
        let mut app = App::new();
        app.add_plugins(WorldScenePlugin);

        let state = app.world().resource::<WorldSceneState>();
        assert!(state.entities.is_empty());
        assert!(state.world_id.is_none());
    }

    #[test]
    fn world_scene_spawns_visible_shell_when_world_route_is_ready() {
        let mut app = spawn_ready_app();
        load_world(&mut app);

        let state = app.world().resource::<WorldSceneState>();
        assert_eq!(state.world_id, Some(1));
        assert!(!state.entities.is_empty());

        let world = app.world();
        assert!(state
            .entities
            .iter()
            .any(|entity| world.entity(*entity).contains::<Camera3d>()));
        assert!(state
            .entities
            .iter()
            .any(|entity| world.entity(*entity).contains::<WorldSceneCamera>()));
        assert!(state
            .entities
            .iter()
            .any(|entity| world.entity(*entity).contains::<WorldSceneTerrain>()));
        assert_eq!(
            state
                .entities
                .iter()
                .filter(|entity| world.entity(**entity).contains::<WorldSceneTerrain>())
                .count(),
            3
        );
        assert!(state.entities.iter().any(|entity| {
            world
                .entity(*entity)
                .get::<Name>()
                .map(|name| name.as_str().contains("lightmap"))
                .unwrap_or(false)
        }));
        assert!(state
            .entities
            .iter()
            .any(|entity| world.entity(*entity).contains::<WorldSceneMarker>()));
        assert!(state
            .entities
            .iter()
            .any(|entity| world.entity(*entity).contains::<SceneRoot>()));
        assert!(state.entities.iter().any(|entity| {
            world
                .entity(*entity)
                .get::<WorldSceneMarker>()
                .map(|marker| marker.family == RenderEntityFamily::Object)
                .unwrap_or(false)
        }));
        assert!(state.entities.iter().any(|entity| {
            world
                .entity(*entity)
                .get::<WorldSceneMarker>()
                .map(|marker| {
                    marker.family == RenderEntityFamily::Object
                        && world.entity(*entity).contains::<SceneRoot>()
                        && !world.entity(*entity).contains::<Mesh3d>()
                })
                .unwrap_or(false)
        }));
    }

    #[test]
    fn world_scene_builds_a_heightfield_mesh_from_the_bundle_samples() {
        let world_root = repo_assets_root();
        let bundle = load_terrain_world_bundle(&world_root, 1).unwrap();
        let summary = bundle.summary();
        let mesh = build_world_terrain_mesh(&summary, &bundle.map).unwrap();

        let positions = match mesh
            .attribute(bevy::prelude::Mesh::ATTRIBUTE_POSITION)
            .expect("mesh should have positions")
        {
            VertexAttributeValues::Float32x3(values) => values,
            other => panic!("unexpected position attribute: {other:?}"),
        };
        let uvs = match mesh
            .attribute(bevy::prelude::Mesh::ATTRIBUTE_UV_0)
            .expect("mesh should have uvs")
        {
            VertexAttributeValues::Float32x2(values) => values,
            other => panic!("unexpected uv attribute: {other:?}"),
        };

        let terrain_size = summary.terrain_size as usize;
        assert_eq!(positions.len(), terrain_size * terrain_size);
        assert_eq!(uvs.len(), terrain_size * terrain_size);

        let max_height = f32::from(summary.layer_stats.layer1.max.max(1));
        let expected_first = f32::from(bundle.map.layer1[0][0]) / max_height;
        let expected_last = f32::from(
            *bundle
                .map
                .layer1
                .last()
                .and_then(|row| row.last())
                .expect("bundle should include the last terrain sample"),
        ) / max_height;
        let half_extent = (terrain_size as f32 - 1.0) * WORLD_POSITION_SCALE * 0.5;

        assert!((positions[0][0] + half_extent).abs() < f32::EPSILON);
        assert!((positions[0][1] - expected_first).abs() < f32::EPSILON);
        assert!((positions[0][2] + half_extent).abs() < f32::EPSILON);
        assert!((uvs[0][0]).abs() < f32::EPSILON);
        assert!((uvs[0][1]).abs() < f32::EPSILON);

        let last = positions
            .last()
            .expect("mesh should include the last vertex");
        assert!((last[0] - half_extent).abs() < f32::EPSILON);
        assert!((last[1] - expected_last).abs() < f32::EPSILON);
        assert!((last[2] - half_extent).abs() < f32::EPSILON);

        let last_uv = uvs.last().expect("mesh should include the last uv");
        assert!((last_uv[0] - WORLD_TERRAIN_TEXTURE_REPEAT).abs() < f32::EPSILON);
        assert!((last_uv[1] - WORLD_TERRAIN_TEXTURE_REPEAT).abs() < f32::EPSILON);
    }

    #[test]
    fn world_terrain_layer_paths_use_the_first_two_available_slots() {
        let world_root = repo_assets_root();
        let bundle = load_terrain_world_bundle(&world_root, 1).unwrap();

        assert_eq!(
            world_terrain_texture_paths(&bundle),
            (
                Some("data/world_1/TileGrass01.png"),
                Some("data/world_1/TileGrass02.png"),
            )
        );

        let mut no_texture_bundle = bundle.clone();
        no_texture_bundle.texture_slots.slots.clear();
        assert_eq!(
            world_terrain_texture_paths(&no_texture_bundle),
            (None, None)
        );
    }

    #[test]
    fn world_terrain_blend_mesh_uses_alpha_colors() {
        let world_root = repo_assets_root();
        let bundle = load_terrain_world_bundle(&world_root, 1).unwrap();
        let summary = bundle.summary();
        let mesh = build_world_terrain_blend_mesh(&summary, &bundle.map).unwrap();

        let colors = match mesh
            .attribute(bevy::prelude::Mesh::ATTRIBUTE_COLOR)
            .expect("mesh should have colors")
        {
            VertexAttributeValues::Float32x4(values) => values,
            other => panic!("unexpected color attribute: {other:?}"),
        };

        let terrain_size = summary.terrain_size as usize;
        assert_eq!(colors.len(), terrain_size * terrain_size);
        assert!((colors[0][3] - (f32::from(bundle.map.alpha[0][0]) / 255.0)).abs() < f32::EPSILON);

        let last_color = colors.last().expect("mesh should include the last color");
        let last_alpha = f32::from(
            *bundle
                .map
                .alpha
                .last()
                .and_then(|row| row.last())
                .expect("bundle should include the last alpha sample"),
        ) / 255.0;
        assert!((last_color[3] - last_alpha).abs() < f32::EPSILON);
    }

    #[test]
    fn world_terrain_lightmap_path_resolves_the_checked_in_asset() {
        let world_root = repo_assets_root();
        let bundle = load_terrain_world_bundle(&world_root, 1).unwrap();

        assert_eq!(
            world_terrain_lightmap_path(Some(world_root.as_ref()), &bundle),
            Some("data/world_1/TerrainLight.png".to_string())
        );
    }

    #[test]
    fn world_scene_reconciles_the_local_player_marker_after_runtime_motion() {
        let mut app = spawn_ready_app();
        load_world(&mut app);

        let (local_marker, initial_x) = {
            let state = app.world().resource::<WorldSceneState>();
            let world = app.world();
            let local_marker = state
                .entities
                .iter()
                .copied()
                .find(|entity| {
                    world
                        .entity(*entity)
                        .get::<WorldSceneMarker>()
                        .map(|marker| {
                            marker.family == RenderEntityFamily::LocalPlayer && marker.key == "0"
                        })
                        .unwrap_or(false)
                })
                .expect("local player marker should exist");
            let initial_x = world
                .entity(local_marker)
                .get::<bevy::prelude::Transform>()
                .unwrap()
                .translation
                .x;
            (local_marker, initial_x)
        };

        assert!(app.world().entity(local_marker).contains::<Mesh3d>());
        assert!(!app.world().entity(local_marker).contains::<SceneRoot>());

        app.world_mut()
            .resource_mut::<ClientRuntime>()
            .translate_local_player([20.0, 0.0, 0.0]);
        app.update();

        let updated_x = app
            .world()
            .entity(local_marker)
            .get::<bevy::prelude::Transform>()
            .unwrap()
            .translation
            .x;

        assert!(updated_x > initial_x);
    }

    #[test]
    fn world_scene_camera_follows_the_local_player_marker() {
        let mut app = spawn_ready_app();
        load_world(&mut app);

        let (camera_entity, local_marker_entity, initial_offset) = {
            let state = app.world().resource::<WorldSceneState>();
            let world = app.world();

            let camera_entity = state
                .entities
                .iter()
                .copied()
                .find(|entity| world.entity(*entity).contains::<Camera3d>())
                .expect("world camera should exist");
            let local_marker_entity = state
                .entities
                .iter()
                .copied()
                .find(|entity| {
                    world
                        .entity(*entity)
                        .get::<WorldSceneMarker>()
                        .map(|marker| {
                            marker.family == RenderEntityFamily::LocalPlayer && marker.key == "0"
                        })
                        .unwrap_or(false)
                })
                .expect("local player marker should exist");
            let camera_translation = world
                .entity(camera_entity)
                .get::<bevy::prelude::Transform>()
                .unwrap()
                .translation;
            let local_translation = world
                .entity(local_marker_entity)
                .get::<bevy::prelude::Transform>()
                .unwrap()
                .translation;

            (
                camera_entity,
                local_marker_entity,
                camera_translation - local_translation,
            )
        };

        app.world_mut()
            .resource_mut::<ClientRuntime>()
            .translate_local_player([20.0, 0.0, 0.0]);
        app.update();

        let world = app.world();
        let camera_translation = world
            .entity(camera_entity)
            .get::<bevy::prelude::Transform>()
            .unwrap()
            .translation;
        let local_translation = world
            .entity(local_marker_entity)
            .get::<bevy::prelude::Transform>()
            .unwrap()
            .translation;
        let updated_offset = camera_translation - local_translation;

        assert!((updated_offset - initial_offset).length_squared() < f32::EPSILON);
    }

    #[test]
    fn world_scene_spawns_scene_roots_for_objects_npcs_and_monsters() {
        let mut app = spawn_ready_app();
        load_world(&mut app);

        let state = app.world().resource::<WorldSceneState>();
        let world = app.world();

        for family in [
            RenderEntityFamily::Object,
            RenderEntityFamily::Npc,
            RenderEntityFamily::Monster,
        ] {
            let entity = state
                .entities
                .iter()
                .copied()
                .find(|entity| {
                    world
                        .entity(*entity)
                        .get::<WorldSceneMarker>()
                        .map(|marker| marker.family == family)
                        .unwrap_or(false)
                })
                .unwrap_or_else(|| panic!("missing spawned entity for family {family:?}"));

            assert!(world.entity(entity).contains::<SceneRoot>());
            assert!(!world.entity(entity).contains::<Mesh3d>());
        }
    }

    #[test]
    fn world_scene_clears_when_route_leaves_world() {
        let mut app = spawn_ready_app();
        load_world(&mut app);

        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::Login);
        app.update();

        let state = app.world().resource::<WorldSceneState>();
        assert!(state.entities.is_empty());
        assert!(state.world_id.is_none());
    }

    #[test]
    fn world_scene_waits_for_world_projection() {
        let mut app = spawn_ready_app();
        app.world_mut()
            .resource_mut::<UiShellState>()
            .set_route(UiRoute::World);
        app.update();

        let state = app.world().resource::<WorldSceneState>();
        assert!(state.entities.is_empty());
        assert!(state.world_id.is_none());
    }
}
