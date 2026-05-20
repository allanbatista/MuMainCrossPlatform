use bevy::math::primitives::Cuboid;
use bevy::prelude::{
    App, Assets, Camera3d, Color, Commands, Component, DirectionalLight, Entity, EulerRot,
    IntoScheduleConfigs, Mesh, Mesh3d, MeshMaterial3d, Name, Plugin, Query, Res, ResMut, Resource,
    StandardMaterial, Transform, Update, Vec3,
};
use mu_assets::TerrainWorldSummary;
use mu_render::{RenderEntityCatalog, RenderEntityEntry, RenderEntityFamily};
use mu_ui::{UiRoute, UiShellState};

use crate::ClientRuntime;

const WORLD_POSITION_SCALE: f32 = 0.05;
const WORLD_TERRAIN_HEIGHT: f32 = 0.2;
const WORLD_CAMERA_HEIGHT_FACTOR: f32 = 1.8;
const WORLD_CAMERA_DISTANCE_FACTOR: f32 = 2.2;
const WORLD_MARKER_LIMIT: usize = 8;
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

    let Some(summary) = client_runtime.terrain().summary() else {
        clear_world_scene(&mut commands, &mut state);
        return;
    };

    if state.world_id == Some(summary.world) && !state.entities.is_empty() {
        return;
    }

    clear_world_scene(&mut commands, &mut state);
    spawn_world_scene(
        &mut commands,
        &mut meshes,
        &mut materials,
        summary,
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

fn clear_world_scene(commands: &mut Commands, state: &mut WorldSceneState) {
    for entity in state.entities.drain(..) {
        commands.entity(entity).despawn();
    }
    state.world_id = None;
}

fn spawn_world_scene(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    summary: &TerrainWorldSummary,
    catalog: &RenderEntityCatalog,
    state: &mut WorldSceneState,
) {
    state.entities.push(spawn_world_camera(commands, summary));
    state.entities.push(spawn_world_light(commands));
    state
        .entities
        .push(spawn_world_terrain(commands, meshes, materials, summary));
    spawn_world_entities(commands, meshes, materials, catalog, state);
}

fn spawn_world_camera(commands: &mut Commands, summary: &TerrainWorldSummary) -> Entity {
    let extent = summary.terrain_size as f32 * WORLD_POSITION_SCALE;
    let camera_position = Vec3::new(
        extent * WORLD_CAMERA_DISTANCE_FACTOR,
        extent * WORLD_CAMERA_HEIGHT_FACTOR,
        extent * WORLD_CAMERA_DISTANCE_FACTOR,
    );

    commands
        .spawn((
            Camera3d::default(),
            Transform::from_translation(camera_position).looking_at(Vec3::ZERO, Vec3::Y),
            Name::new(format!("world-camera-{}", summary.world)),
        ))
        .id()
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
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    summary: &TerrainWorldSummary,
) -> Entity {
    let extent = summary.terrain_size as f32 * WORLD_POSITION_SCALE;
    let mesh = meshes.add(Mesh::from(Cuboid::new(
        extent,
        WORLD_TERRAIN_HEIGHT,
        extent,
    )));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.28, 0.12),
        perceptual_roughness: 1.0,
        ..Default::default()
    });

    commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(Vec3::new(0.0, -WORLD_TERRAIN_HEIGHT * 0.5, 0.0)),
            WorldSceneTerrain,
            Name::new(format!("world-terrain-{}", summary.world)),
        ))
        .id()
}

fn spawn_world_entities(
    commands: &mut Commands,
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
    spawn_world_marker_family(
        commands,
        meshes,
        materials,
        &catalog.objects,
        RenderEntityFamily::Object,
        state,
        "object",
    );
    spawn_world_marker_family(
        commands,
        meshes,
        materials,
        &catalog.npcs,
        RenderEntityFamily::Npc,
        state,
        "npc",
    );
    spawn_world_marker_family(
        commands,
        meshes,
        materials,
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
    for entry in entries.iter().take(WORLD_MARKER_LIMIT) {
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
    use super::{WorldSceneMarker, WorldScenePlugin, WorldSceneState, WorldSceneTerrain};
    use crate::ClientRuntime;
    use bevy::prelude::{App, Camera3d};
    use camino::Utf8PathBuf;
    use mu_assets::load_terrain_world_bundle;
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
        let mut app = App::new();
        app.add_plugins((mu_ui::UiShellPlugin, WorldScenePlugin));
        app.insert_resource(ClientRuntime::new());
        app.insert_resource(bevy::prelude::Assets::<bevy::prelude::Mesh>::default());
        app.insert_resource(bevy::prelude::Assets::<bevy::prelude::StandardMaterial>::default());
        app
    }

    fn load_world(app: &mut App) {
        let world_root = repo_assets_root();
        let bundle = load_terrain_world_bundle(&world_root, 1).unwrap();
        app.world_mut()
            .resource_mut::<ClientRuntime>()
            .load_world_bundle(bundle);
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
            .any(|entity| world.entity(*entity).contains::<WorldSceneTerrain>()));
        assert!(state
            .entities
            .iter()
            .any(|entity| world.entity(*entity).contains::<WorldSceneMarker>()));
        assert!(state.entities.iter().any(|entity| {
            world
                .entity(*entity)
                .get::<WorldSceneMarker>()
                .map(|marker| marker.family == RenderEntityFamily::Object)
                .unwrap_or(false)
        }));
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
