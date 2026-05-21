use bevy::prelude::{App, Plugin, Resource};

use crate::world::WorldManager;
use mu_assets::{TerrainCameraTour, TerrainWorldSummary};
use mu_input::{CameraBindings, CameraMode, CameraTourState, MovementBindings, MovementCommand};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MovementState {
    #[default]
    Inactive,
    Ready,
}

impl MovementState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inactive => "inactive",
            Self::Ready => "ready",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MovementActorState {
    pub label: String,
    pub key: u16,
    pub command: MovementCommand,
}

impl MovementActorState {
    pub fn new(label: impl Into<String>, key: u16, command: MovementCommand) -> Self {
        Self {
            label: label.into(),
            key,
            command,
        }
    }

    pub fn snapshot(&self) -> String {
        format!(
            "label={}|key={}|{}",
            self.label,
            self.key,
            self.command.snapshot()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MovementManager {
    movement_bindings: MovementBindings,
    camera_bindings: CameraBindings,
    world: Option<TerrainWorldSummary>,
    local: Option<MovementActorState>,
    remote: Vec<MovementActorState>,
    camera: CameraTourState,
}

impl Resource for MovementManager {}

#[derive(Debug, Default, Clone, Copy)]
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MovementManager>();
    }
}

impl MovementManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn state(&self) -> MovementState {
        if self.world.is_some() {
            MovementState::Ready
        } else {
            MovementState::Inactive
        }
    }

    pub fn movement_bindings(&self) -> &MovementBindings {
        &self.movement_bindings
    }

    pub fn camera_bindings(&self) -> &CameraBindings {
        &self.camera_bindings
    }

    pub fn world_summary(&self) -> Option<&TerrainWorldSummary> {
        self.world.as_ref()
    }

    pub fn local_actor(&self) -> Option<&MovementActorState> {
        self.local.as_ref()
    }

    pub fn remote_actors(&self) -> &[MovementActorState] {
        &self.remote
    }

    pub fn camera(&self) -> &CameraTourState {
        &self.camera
    }

    pub fn set_world_summary(&mut self, world: TerrainWorldSummary) {
        self.world = Some(world);
    }

    pub fn sync_world(&mut self, world: &WorldManager) {
        self.world = world.summary();

        if let Some(bundle) = world.bundle() {
            self.sync_camera_tour(&bundle.camera_tour);
        }
    }

    pub fn sync_camera_tour(&mut self, tour: &TerrainCameraTour) {
        let mut camera = CameraTourState::new(
            CameraMode::Default,
            tour.waypoints.len(),
            tour.blend_distance,
            tour.interpolation.clone(),
            tour.r#loop,
        );

        if let Some(waypoint) = tour.waypoints.first() {
            camera = camera.with_waypoint(
                waypoint.index as usize,
                waypoint.position,
                waypoint.look_at,
                waypoint.move_acceleration,
                waypoint.distance_level,
                waypoint.delay,
            );
        }

        self.camera = camera;
    }

    pub fn set_local_actor(&mut self, actor: MovementActorState) {
        self.local = Some(actor);
    }

    pub fn clear_local_actor(&mut self) {
        self.local = None;
    }

    pub fn push_remote_actor(&mut self, actor: MovementActorState) {
        self.remote.push(actor);
    }

    pub fn clear_remote_actors(&mut self) {
        self.remote.clear();
    }

    pub fn set_camera(&mut self, camera: CameraTourState) {
        self.camera = camera;
    }

    pub fn snapshot(&self) -> String {
        let world = self
            .world
            .as_ref()
            .map(TerrainWorldSummary::snapshot)
            .unwrap_or_else(|| "none".to_string());
        let local = self
            .local
            .as_ref()
            .map(MovementActorState::snapshot)
            .unwrap_or_else(|| "none".to_string());
        let remote = if self.remote.is_empty() {
            "[]".to_string()
        } else {
            format!(
                "[{}]",
                self.remote
                    .iter()
                    .map(MovementActorState::snapshot)
                    .collect::<Vec<_>>()
                    .join(";")
            )
        };

        format!(
            "state={}|world_summary={}|movement_bindings={}|camera_bindings={}|local={}|remote_count={}|remote={}|camera={}",
            self.state().as_str(),
            world,
            self.movement_bindings.snapshot(),
            self.camera_bindings.snapshot(),
            local,
            self.remote.len(),
            remote,
            self.camera.snapshot(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{MovementActorState, MovementManager, MovementPlugin, MovementState};
    use bevy::prelude::App;
    use mu_assets::{TerrainLayerStats, TerrainLayerStatsSet, TerrainWorldSummary};
    use mu_input::{CameraMode, CameraTourState, MovementCommand};

    fn sample_world_summary() -> TerrainWorldSummary {
        TerrainWorldSummary {
            world: 1,
            world_directory: "data/world_1".to_string(),
            map_number: 1,
            terrain_size: 256,
            texture_layers: vec![
                "grass01".to_string(),
                "ground01".to_string(),
                "rock01".to_string(),
            ],
            texture_slots: 17,
            scene_objects: 2_870,
            camera_waypoints: 8,
            layer_stats: TerrainLayerStatsSet {
                layer1: TerrainLayerStats {
                    min: 0,
                    max: 8,
                    mean: 0.76103,
                    unique_values: 9,
                },
                layer2: TerrainLayerStats {
                    min: 0,
                    max: 255,
                    mean: 171.43759,
                    unique_values: 8,
                },
                alpha: TerrainLayerStats {
                    min: 0,
                    max: 255,
                    mean: 38.53535,
                    unique_values: 17,
                },
            },
            height_map_path: "data/world_1/terrain_height.json".to_string(),
            alpha_map_path: "data/world_1/alpha_tile01.png".to_string(),
            lightmap_path: "data/world_1/terrain_light.png".to_string(),
        }
    }

    #[test]
    fn movement_plugin_registers_manager_resource() {
        let mut app = App::new();
        app.add_plugins(MovementPlugin);

        let movement = app.world().resource::<MovementManager>();
        assert_eq!(movement.state(), MovementState::Inactive);
        assert_eq!(movement.movement_bindings().get("move_forward"), Some("W"));
        assert_eq!(movement.camera_bindings().get("cycle_camera"), Some("F9"));
        assert_eq!(movement.camera().mode, CameraMode::Default);
    }

    #[test]
    fn movement_manager_tracks_world_local_remote_and_camera() {
        let mut movement = MovementManager::new();
        movement.set_world_summary(sample_world_summary());
        movement.set_local_actor(MovementActorState::new(
            "hero",
            17,
            MovementCommand::new(1, 2, 3, 4, [0x12, 0x34]),
        ));
        movement.push_remote_actor(MovementActorState::new(
            "remote",
            23,
            MovementCommand::new(5, 6, 1, 2, [0x56]),
        ));
        movement.set_camera(
            CameraTourState::new(CameraMode::Orbital, 8, 300.0, "smooth", true).with_waypoint(
                0,
                [1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0],
                16.0,
                8.0,
                0.0,
            ),
        );

        assert_eq!(movement.state(), MovementState::Ready);
        assert_eq!(movement.world_summary().unwrap().world, 1);
        assert_eq!(movement.local_actor().unwrap().label, "hero");
        assert_eq!(movement.remote_actors().len(), 1);
        assert_eq!(
            movement.snapshot(),
            "state=ready|world_summary=world=1|world_directory=data/world_1|map_number=1|terrain_size=256|texture_layers=[grass01,ground01,rock01]|layer_stats=layer1[min=0|max=8|mean=0.76103|unique=9];layer2[min=0|max=255|mean=171.43759|unique=8];alpha[min=0|max=255|mean=38.53535|unique=17]|texture_slots=17|scene_objects=2870|camera_waypoints=8|height_map_path=data/world_1/terrain_height.json|alpha_map_path=data/world_1/alpha_tile01.png|lightmap_path=data/world_1/terrain_light.png|movement_bindings=move_forward=W,move_backward=S,move_left=A,move_right=D,jump=Space,attack=Mouse1,interact=E|camera_bindings=cycle_camera=F9,toggle_zoom_lock=F10,reset_camera=F11,zoom=MouseWheel,rotate_orbital=MouseDrag|local=label=hero|key=17|source=1,2|steps=3|rotation=4|directions=[18,52]|remote_count=1|remote=[label=remote|key=23|source=5,6|steps=1|rotation=2|directions=[86]]|camera=mode=orbital|waypoint=0/8|loop=true|blend=300|interpolation=smooth|position=1,2,3|look_at=4,5,6|move_acceleration=16|distance_level=8|delay=0"
        );
    }
}
