use bevy::prelude::{App, Plugin, Resource};
use mu_assets::{TerrainSceneObject, TerrainWorldSummary};

use crate::world::WorldManager;

const SNAPSHOT_OBJECT_SAMPLE_LIMIT: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WorldEntitiesState {
    #[default]
    Inactive,
    Ready,
}

impl WorldEntitiesState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inactive => "inactive",
            Self::Ready => "ready",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WorldEntityPose {
    pub position: [f64; 3],
    pub rotation: [f64; 3],
    pub scale: [f64; 3],
}

impl WorldEntityPose {
    pub const fn new(position: [f64; 3], rotation: [f64; 3], scale: [f64; 3]) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub const fn identity() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        }
    }

    pub fn snapshot(&self) -> String {
        format!(
            "pose=position={}|rotation={}|scale={}",
            format_triplet(self.position),
            format_triplet(self.rotation),
            format_triplet(self.scale),
        )
    }
}

impl Default for WorldEntityPose {
    fn default() -> Self {
        Self::identity()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WorldPlayerRole {
    #[default]
    Local,
    Remote,
}

impl WorldPlayerRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Remote => "remote",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorldPlayerSpawn {
    pub role: WorldPlayerRole,
    pub label: String,
    pub key: u32,
    pub model: String,
    pub pose: WorldEntityPose,
}

impl WorldPlayerSpawn {
    pub fn new(
        role: WorldPlayerRole,
        label: impl Into<String>,
        key: u32,
        model: impl Into<String>,
        pose: WorldEntityPose,
    ) -> Self {
        Self {
            role,
            label: label.into(),
            key,
            model: model.into(),
            pose,
        }
    }

    pub fn snapshot(&self) -> String {
        format!(
            "role={}|label={}|key={}|model={}|{}",
            self.role.as_str(),
            self.label,
            self.key,
            self.model,
            self.pose.snapshot(),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorldObjectSpawn {
    pub id: String,
    pub label: String,
    pub object_type: u32,
    pub model: String,
    pub pose: WorldEntityPose,
}

impl WorldObjectSpawn {
    pub fn new(
        id: impl Into<String>,
        object_type: u32,
        model: impl Into<String>,
        pose: WorldEntityPose,
    ) -> Self {
        let model = model.into();
        Self {
            id: id.into(),
            label: file_stem(&model),
            object_type,
            model,
            pose,
        }
    }

    pub fn from_scene_object(scene_object: &TerrainSceneObject) -> Self {
        Self {
            id: scene_object.id.clone(),
            label: file_stem(&scene_object.model),
            object_type: scene_object.object_type,
            model: scene_object.model.clone(),
            pose: WorldEntityPose::new(
                scene_object.position,
                scene_object.rotation,
                scene_object.scale,
            ),
        }
    }

    pub fn snapshot(&self) -> String {
        format!(
            "id={}|label={}|type={}|model={}|{}",
            self.id,
            self.label,
            self.object_type,
            self.model,
            self.pose.snapshot(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct WorldEntitiesManager {
    world: Option<TerrainWorldSummary>,
    local_player: Option<WorldPlayerSpawn>,
    remote_players: Vec<WorldPlayerSpawn>,
    objects: Vec<WorldObjectSpawn>,
}

impl Resource for WorldEntitiesManager {}

#[derive(Debug, Default, Clone, Copy)]
pub struct WorldEntitiesPlugin;

impl Plugin for WorldEntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldEntitiesManager>();
    }
}

impl WorldEntitiesManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn state(&self) -> WorldEntitiesState {
        if self.world.is_some()
            || self.local_player.is_some()
            || !self.remote_players.is_empty()
            || !self.objects.is_empty()
        {
            WorldEntitiesState::Ready
        } else {
            WorldEntitiesState::Inactive
        }
    }

    pub fn world_summary(&self) -> Option<&TerrainWorldSummary> {
        self.world.as_ref()
    }

    pub fn local_player(&self) -> Option<&WorldPlayerSpawn> {
        self.local_player.as_ref()
    }

    pub fn remote_players(&self) -> &[WorldPlayerSpawn] {
        &self.remote_players
    }

    pub fn objects(&self) -> &[WorldObjectSpawn] {
        &self.objects
    }

    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    pub fn set_world_summary(&mut self, world: TerrainWorldSummary) {
        self.world = Some(world);
    }

    pub fn sync_world(&mut self, world: &WorldManager) {
        self.world = world.summary();
        self.objects = world
            .bundle()
            .map(|bundle| {
                bundle
                    .scene_objects
                    .objects
                    .iter()
                    .map(WorldObjectSpawn::from_scene_object)
                    .collect()
            })
            .unwrap_or_default();
    }

    pub fn set_local_player(&mut self, local_player: WorldPlayerSpawn) {
        self.local_player = Some(local_player);
    }

    pub fn clear_local_player(&mut self) {
        self.local_player = None;
    }

    pub fn set_remote_players<I>(&mut self, remote_players: I)
    where
        I: IntoIterator<Item = WorldPlayerSpawn>,
    {
        self.remote_players = remote_players.into_iter().collect();
    }

    pub fn push_remote_player(&mut self, remote_player: WorldPlayerSpawn) {
        self.remote_players.push(remote_player);
    }

    pub fn clear_remote_players(&mut self) {
        self.remote_players.clear();
    }

    pub fn set_objects<I>(&mut self, objects: I)
    where
        I: IntoIterator<Item = WorldObjectSpawn>,
    {
        self.objects = objects.into_iter().collect();
    }

    pub fn clear_objects(&mut self) {
        self.objects.clear();
    }

    pub fn snapshot(&self) -> String {
        let world_summary = self
            .world
            .as_ref()
            .map(TerrainWorldSummary::snapshot)
            .unwrap_or_else(|| "none".to_string());
        let local_player = self
            .local_player
            .as_ref()
            .map(WorldPlayerSpawn::snapshot)
            .unwrap_or_else(|| "none".to_string());
        let remote_players = snapshot_list(
            &self.remote_players,
            SNAPSHOT_OBJECT_SAMPLE_LIMIT,
            WorldPlayerSpawn::snapshot,
        );
        let objects = snapshot_list(
            &self.objects,
            SNAPSHOT_OBJECT_SAMPLE_LIMIT,
            WorldObjectSpawn::snapshot,
        );

        format!(
            "state={}|world_summary={}|local={}|remote_count={}|remote={}|object_count={}|objects={}",
            self.state().as_str(),
            world_summary,
            local_player,
            self.remote_players.len(),
            remote_players,
            self.objects.len(),
            objects,
        )
    }
}

fn snapshot_list<T, F>(entries: &[T], sample_limit: usize, snapshot: F) -> String
where
    F: Fn(&T) -> String,
{
    if entries.is_empty() {
        return "[]".to_string();
    }

    let mut parts = entries
        .iter()
        .take(sample_limit)
        .map(snapshot)
        .collect::<Vec<_>>();
    if entries.len() > sample_limit {
        parts.push("...".to_string());
    }

    format!("[{}]", parts.join(";"))
}

fn format_triplet(values: [f64; 3]) -> String {
    [
        format_float(values[0]),
        format_float(values[1]),
        format_float(values[2]),
    ]
    .join(",")
}

fn format_float(value: f64) -> String {
    let mut text = format!("{value:.3}");
    while text.contains('.') && text.ends_with('0') {
        text.pop();
    }
    if text.ends_with('.') {
        text.pop();
    }
    text
}

fn file_stem(path: &str) -> String {
    std::path::Path::new(path)
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string())
}

#[cfg(test)]
mod tests {
    use bevy::prelude::App;

    use super::{
        WorldEntitiesManager, WorldEntitiesPlugin, WorldEntitiesState, WorldEntityPose,
        WorldObjectSpawn, WorldPlayerRole, WorldPlayerSpawn,
    };

    #[test]
    fn world_entities_plugin_registers_resource() {
        let mut app = App::new();
        app.add_plugins(WorldEntitiesPlugin);

        let manager = app.world().resource::<WorldEntitiesManager>();
        assert_eq!(manager.state(), WorldEntitiesState::Inactive);
        assert!(manager.world_summary().is_none());
        assert!(manager.local_player().is_none());
        assert!(manager.remote_players().is_empty());
        assert!(manager.objects().is_empty());
    }

    #[test]
    fn world_entities_manager_tracks_players_objects_and_snapshot() {
        let mut manager = WorldEntitiesManager::new();
        manager.set_local_player(WorldPlayerSpawn::new(
            WorldPlayerRole::Local,
            "Hero",
            17,
            "data/character/dark_knight.glb",
            WorldEntityPose::new([1.0, 2.0, 3.0], [0.0, 90.0, 0.0], [1.0, 1.0, 1.0]),
        ));
        manager.set_remote_players([WorldPlayerSpawn::new(
            WorldPlayerRole::Remote,
            "Ally",
            23,
            "data/character/wizard.glb",
            WorldEntityPose::new([4.0, 5.0, 6.0], [0.0, 180.0, 0.0], [1.0, 1.0, 1.0]),
        )]);
        manager.set_objects([
            WorldObjectSpawn::new(
                "obj_00000",
                30,
                "data/object_1/stone_01.glb",
                WorldEntityPose::new(
                    [1397.10498046875, 44.230224609375, 682.5578002929688],
                    [0.0, 0.0, -60.0],
                    [0.940000057220459, 0.940000057220459, 0.940000057220459],
                ),
            ),
            WorldObjectSpawn::new(
                "obj_00001",
                30,
                "data/object_1/stone_01.glb",
                WorldEntityPose::new(
                    [784.9892578125, -20.855173110961914, 643.00341796875],
                    [0.0, 0.0, 150.0],
                    [1.0, 1.0, 1.0],
                ),
            ),
            WorldObjectSpawn::new(
                "obj_00002",
                33,
                "data/object_1/stone_04.glb",
                WorldEntityPose::new(
                    [950.0, 79.48616027832031, 750.0],
                    [-25.0, 0.0, -30.0],
                    [1.0, 1.0, 1.0],
                ),
            ),
        ]);

        assert_eq!(manager.state(), WorldEntitiesState::Ready);
        assert_eq!(manager.remote_players().len(), 1);
        assert_eq!(manager.object_count(), 3);
        assert_eq!(
            manager.snapshot(),
            "state=ready|world_summary=none|local=role=local|label=Hero|key=17|model=data/character/dark_knight.glb|pose=position=1,2,3|rotation=0,90,0|scale=1,1,1|remote_count=1|remote=[role=remote|label=Ally|key=23|model=data/character/wizard.glb|pose=position=4,5,6|rotation=0,180,0|scale=1,1,1]|object_count=3|objects=[id=obj_00000|label=stone_01|type=30|model=data/object_1/stone_01.glb|pose=position=1397.105,44.23,682.558|rotation=0,0,-60|scale=0.94,0.94,0.94;id=obj_00001|label=stone_01|type=30|model=data/object_1/stone_01.glb|pose=position=784.989,-20.855,643.003|rotation=0,0,150|scale=1,1,1;...]"
        );
    }
}
