use bevy::prelude::{App, Plugin, Resource};

use crate::entities::WorldEntityPose;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WorldNpcKind {
    #[default]
    Other,
    Merchant,
    QuestGiver,
    Gatekeeper,
    Guide,
}

impl WorldNpcKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Other => "other",
            Self::Merchant => "merchant",
            Self::QuestGiver => "quest-giver",
            Self::Gatekeeper => "gatekeeper",
            Self::Guide => "guide",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorldNpcSpawn {
    pub kind: WorldNpcKind,
    pub label: String,
    pub key: u32,
    pub dialogue_index: u32,
    pub model: String,
    pub pose: WorldEntityPose,
}

impl WorldNpcSpawn {
    pub fn new(
        kind: WorldNpcKind,
        label: impl Into<String>,
        key: u32,
        dialogue_index: u32,
        model: impl Into<String>,
        pose: WorldEntityPose,
    ) -> Self {
        Self {
            kind,
            label: label.into(),
            key,
            dialogue_index,
            model: model.into(),
            pose,
        }
    }

    pub fn snapshot(&self) -> String {
        format!(
            "kind={}|label={}|key={}|dialogue_index={}|model={}|{}",
            self.kind.as_str(),
            self.label,
            self.key,
            self.dialogue_index,
            self.model,
            self.pose.snapshot(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct WorldNpcManager {
    spawns: Vec<WorldNpcSpawn>,
}

impl Resource for WorldNpcManager {}

#[derive(Debug, Default, Clone, Copy)]
pub struct WorldNpcPlugin;

impl Plugin for WorldNpcPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldNpcManager>();
    }
}

impl WorldNpcManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.spawns.clear();
    }

    pub fn state(&self) -> WorldNpcState {
        if self.spawns.is_empty() {
            WorldNpcState::Inactive
        } else {
            WorldNpcState::Ready
        }
    }

    pub fn spawns(&self) -> &[WorldNpcSpawn] {
        &self.spawns
    }

    pub fn count(&self) -> usize {
        self.spawns.len()
    }

    pub fn load_fixture<I>(&mut self, spawns: I)
    where
        I: IntoIterator<Item = WorldNpcSpawn>,
    {
        self.spawns = spawns.into_iter().collect();
    }

    pub fn push_spawn(&mut self, spawn: WorldNpcSpawn) {
        self.spawns.push(spawn);
    }

    pub fn snapshot(&self) -> String {
        format!(
            "state={}|count={}|spawns={}",
            self.state().as_str(),
            self.spawns.len(),
            snapshot_list(&self.spawns, 3, WorldNpcSpawn::snapshot),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WorldNpcState {
    #[default]
    Inactive,
    Ready,
}

impl WorldNpcState {
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

    use super::{WorldNpcKind, WorldNpcManager, WorldNpcPlugin, WorldNpcSpawn, WorldNpcState};
    use crate::entities::WorldEntityPose;

    #[test]
    fn world_npc_plugin_registers_resource() {
        let mut app = App::new();
        app.add_plugins(WorldNpcPlugin);

        let manager = app.world().resource::<WorldNpcManager>();
        assert_eq!(manager.state(), WorldNpcState::Inactive);
        assert!(manager.spawns().is_empty());
    }

    #[test]
    fn world_npc_manager_tracks_fixture_spawns_and_snapshot() {
        let mut manager = WorldNpcManager::new();
        manager.load_fixture([
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

        assert_eq!(manager.state(), WorldNpcState::Ready);
        assert_eq!(manager.count(), 2);
        assert_eq!(
            manager.snapshot(),
            "state=ready|count=2|spawns=[kind=quest-giver|label=Marlon|key=236|dialogue_index=18|model=data/object_1/npc_quest.glb|pose=position=12,0,24|rotation=0,180,0|scale=1,1,1;kind=merchant|label=Potion Merchant|key=237|dialogue_index=0|model=data/object_1/npc_merchant.glb|pose=position=18,0,30|rotation=0,90,0|scale=1,1,1]"
        );
    }
}
