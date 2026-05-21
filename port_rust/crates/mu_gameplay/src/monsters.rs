use bevy::prelude::{App, Plugin, Resource};

use crate::entities::WorldEntityPose;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WorldMonsterKind {
    #[default]
    Common,
    Elite,
    Boss,
    Summoned,
    Other,
}

impl WorldMonsterKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Common => "common",
            Self::Elite => "elite",
            Self::Boss => "boss",
            Self::Summoned => "summoned",
            Self::Other => "other",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorldMonsterSpawn {
    pub kind: WorldMonsterKind,
    pub label: String,
    pub key: u32,
    pub level: u16,
    pub model: String,
    pub pose: WorldEntityPose,
}

impl WorldMonsterSpawn {
    pub fn new(
        kind: WorldMonsterKind,
        label: impl Into<String>,
        key: u32,
        level: u16,
        model: impl Into<String>,
        pose: WorldEntityPose,
    ) -> Self {
        Self {
            kind,
            label: label.into(),
            key,
            level,
            model: model.into(),
            pose,
        }
    }

    pub fn snapshot(&self) -> String {
        format!(
            "kind={}|label={}|key={}|level={}|model={}|{}",
            self.kind.as_str(),
            self.label,
            self.key,
            self.level,
            self.model,
            self.pose.snapshot(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct WorldMonsterManager {
    spawns: Vec<WorldMonsterSpawn>,
}

impl Resource for WorldMonsterManager {}

#[derive(Debug, Default, Clone, Copy)]
pub struct WorldMonsterPlugin;

impl Plugin for WorldMonsterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldMonsterManager>();
    }
}

impl WorldMonsterManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.spawns.clear();
    }

    pub fn state(&self) -> WorldMonsterState {
        if self.spawns.is_empty() {
            WorldMonsterState::Inactive
        } else {
            WorldMonsterState::Ready
        }
    }

    pub fn spawns(&self) -> &[WorldMonsterSpawn] {
        &self.spawns
    }

    pub fn count(&self) -> usize {
        self.spawns.len()
    }

    pub fn load_fixture<I>(&mut self, spawns: I)
    where
        I: IntoIterator<Item = WorldMonsterSpawn>,
    {
        self.spawns = spawns.into_iter().collect();
    }

    pub fn push_spawn(&mut self, spawn: WorldMonsterSpawn) {
        self.spawns.push(spawn);
    }

    pub fn snapshot(&self) -> String {
        format!(
            "state={}|count={}|spawns={}",
            self.state().as_str(),
            self.spawns.len(),
            snapshot_list(&self.spawns, 3, WorldMonsterSpawn::snapshot),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WorldMonsterState {
    #[default]
    Inactive,
    Ready,
}

impl WorldMonsterState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inactive => "inactive",
            Self::Ready => "ready",
        }
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

#[cfg(test)]
mod tests {
    use bevy::prelude::App;

    use super::{
        WorldMonsterKind, WorldMonsterManager, WorldMonsterPlugin, WorldMonsterSpawn,
        WorldMonsterState,
    };
    use crate::entities::WorldEntityPose;

    #[test]
    fn world_monster_plugin_registers_resource() {
        let mut app = App::new();
        app.add_plugins(WorldMonsterPlugin);

        let manager = app.world().resource::<WorldMonsterManager>();
        assert_eq!(manager.state(), WorldMonsterState::Inactive);
        assert!(manager.spawns().is_empty());
    }

    #[test]
    fn world_monster_manager_tracks_fixture_spawns_and_snapshot() {
        let mut manager = WorldMonsterManager::new();
        manager.load_fixture([
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

        assert_eq!(manager.state(), WorldMonsterState::Ready);
        assert_eq!(manager.count(), 2);
        assert_eq!(
            manager.snapshot(),
            "state=ready|count=2|spawns=[kind=common|label=Bull Fighter|key=401|level=32|model=data/object_1/bull_fighter.glb|pose=position=42,0,88|rotation=0,0,0|scale=1,1,1;kind=boss|label=Death King|key=777|level=120|model=data/object_1/death_king.glb|pose=position=64,0,96|rotation=0,45,0|scale=1.2,1.2,1.2]"
        );
    }
}
