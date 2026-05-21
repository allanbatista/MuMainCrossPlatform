use bevy::prelude::{App, Plugin, Resource};
use mu_assets::TerrainWorldSummary;
use mu_gameplay::{
    WorldEntitiesManager, WorldMonsterManager, WorldMonsterSpawn, WorldNpcManager, WorldNpcSpawn,
    WorldObjectSpawn, WorldPlayerSpawn,
};

const SNAPSHOT_SAMPLE_LIMIT: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderEntitiesState {
    #[default]
    Inactive,
    Ready,
}

impl RenderEntitiesState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inactive => "inactive",
            Self::Ready => "ready",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderEntityFamily {
    LocalPlayer,
    RemotePlayer,
    Object,
    Npc,
    Monster,
    #[default]
    Other,
}

impl RenderEntityFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalPlayer => "local-player",
            Self::RemotePlayer => "remote-player",
            Self::Object => "object",
            Self::Npc => "npc",
            Self::Monster => "monster",
            Self::Other => "other",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RenderEntityEntry {
    pub family: RenderEntityFamily,
    pub label: String,
    pub key: String,
    pub model: String,
    pub pose: mu_gameplay::WorldEntityPose,
}

impl RenderEntityEntry {
    pub fn snapshot(&self) -> String {
        format!(
            "family={}|label={}|key={}|model={}|{}",
            self.family.as_str(),
            self.label,
            self.key,
            self.model,
            self.pose.snapshot(),
        )
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RenderEntityCatalog {
    pub local_player: Option<RenderEntityEntry>,
    pub remote_players: Vec<RenderEntityEntry>,
    pub objects: Vec<RenderEntityEntry>,
    pub npcs: Vec<RenderEntityEntry>,
    pub monsters: Vec<RenderEntityEntry>,
}

impl RenderEntityCatalog {
    pub fn total_entities(&self) -> usize {
        self.local_player.iter().count()
            + self.remote_players.len()
            + self.objects.len()
            + self.npcs.len()
            + self.monsters.len()
    }

    pub fn is_empty(&self) -> bool {
        self.total_entities() == 0
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RenderEntities {
    world: Option<TerrainWorldSummary>,
    catalog: RenderEntityCatalog,
}

impl Resource for RenderEntities {}

#[derive(Debug, Default, Clone, Copy)]
pub struct RenderEntitiesPlugin;

impl Plugin for RenderEntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RenderEntities>();
    }
}

impl RenderEntities {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn state(&self) -> RenderEntitiesState {
        if self.world.is_some() || !self.catalog.is_empty() {
            RenderEntitiesState::Ready
        } else {
            RenderEntitiesState::Inactive
        }
    }

    pub fn world_summary(&self) -> Option<&TerrainWorldSummary> {
        self.world.as_ref()
    }

    pub fn catalog(&self) -> &RenderEntityCatalog {
        &self.catalog
    }

    pub fn sync_world(
        &mut self,
        world: &WorldEntitiesManager,
        npcs: &WorldNpcManager,
        monsters: &WorldMonsterManager,
    ) {
        self.world = world.world_summary().cloned();
        self.catalog = collect_render_entities(world, npcs, monsters);
    }

    pub fn snapshot(&self) -> String {
        let world_summary = self
            .world
            .as_ref()
            .map(TerrainWorldSummary::snapshot)
            .unwrap_or_else(|| "none".to_string());
        let local_player = self
            .catalog
            .local_player
            .as_ref()
            .map(RenderEntityEntry::snapshot)
            .unwrap_or_else(|| "none".to_string());
        let remote_players = snapshot_list(
            &self.catalog.remote_players,
            SNAPSHOT_SAMPLE_LIMIT,
            RenderEntityEntry::snapshot,
        );
        let objects = snapshot_list(
            &self.catalog.objects,
            SNAPSHOT_SAMPLE_LIMIT,
            RenderEntityEntry::snapshot,
        );
        let npcs = snapshot_list(
            &self.catalog.npcs,
            SNAPSHOT_SAMPLE_LIMIT,
            RenderEntityEntry::snapshot,
        );
        let monsters = snapshot_list(
            &self.catalog.monsters,
            SNAPSHOT_SAMPLE_LIMIT,
            RenderEntityEntry::snapshot,
        );

        format!(
            "state={}|world_summary={}|local={}|remote_count={}|remote={}|object_count={}|objects={}|npc_count={}|npcs={}|monster_count={}|monsters={}",
            self.state().as_str(),
            world_summary,
            local_player,
            self.catalog.remote_players.len(),
            remote_players,
            self.catalog.objects.len(),
            objects,
            self.catalog.npcs.len(),
            npcs,
            self.catalog.monsters.len(),
            monsters,
        )
    }
}

pub fn collect_render_entities(
    world: &WorldEntitiesManager,
    npcs: &WorldNpcManager,
    monsters: &WorldMonsterManager,
) -> RenderEntityCatalog {
    RenderEntityCatalog {
        local_player: world
            .local_player()
            .map(|spawn| render_player(spawn, RenderEntityFamily::LocalPlayer)),
        remote_players: world
            .remote_players()
            .iter()
            .map(|spawn| render_player(spawn, RenderEntityFamily::RemotePlayer))
            .collect(),
        objects: world.objects().iter().map(render_object).collect(),
        npcs: npcs.spawns().iter().map(render_npc).collect(),
        monsters: monsters.spawns().iter().map(render_monster).collect(),
    }
}

pub fn format_render_entity_catalog(catalog: &RenderEntityCatalog) -> String {
    let mut out = String::new();
    out.push_str(&format!("total_entities: {}\n", catalog.total_entities()));
    write_section(&mut out, "local-player", catalog.local_player.as_slice());
    write_section(&mut out, "remote-player", &catalog.remote_players);
    write_section(&mut out, "object", &catalog.objects);
    write_section(&mut out, "npc", &catalog.npcs);
    write_section(&mut out, "monster", &catalog.monsters);
    out
}

fn render_player(spawn: &WorldPlayerSpawn, family: RenderEntityFamily) -> RenderEntityEntry {
    RenderEntityEntry {
        family,
        label: spawn.label.clone(),
        key: spawn.key.to_string(),
        model: spawn.model.clone(),
        pose: spawn.pose,
    }
}

fn render_object(spawn: &WorldObjectSpawn) -> RenderEntityEntry {
    RenderEntityEntry {
        family: RenderEntityFamily::Object,
        label: spawn.label.clone(),
        key: spawn.id.clone(),
        model: spawn.model.clone(),
        pose: spawn.pose,
    }
}

fn render_npc(spawn: &WorldNpcSpawn) -> RenderEntityEntry {
    RenderEntityEntry {
        family: RenderEntityFamily::Npc,
        label: spawn.label.clone(),
        key: spawn.key.to_string(),
        model: spawn.model.clone(),
        pose: spawn.pose,
    }
}

fn render_monster(spawn: &WorldMonsterSpawn) -> RenderEntityEntry {
    RenderEntityEntry {
        family: RenderEntityFamily::Monster,
        label: spawn.label.clone(),
        key: spawn.key.to_string(),
        model: spawn.model.clone(),
        pose: spawn.pose,
    }
}

fn write_section(out: &mut String, label: &str, entries: &[RenderEntityEntry]) {
    out.push('\n');
    out.push_str(&format!("[{label}]\n"));
    for entry in entries {
        out.push_str(&format!(
            "{}|{}|{}|{}\n",
            entry.key,
            entry.label,
            entry.model,
            entry.pose.snapshot()
        ));
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
        collect_render_entities, format_render_entity_catalog, RenderEntities,
        RenderEntitiesPlugin, RenderEntitiesState,
    };
    use mu_gameplay::{
        WorldEntitiesManager, WorldEntityPose, WorldMonsterKind, WorldMonsterManager,
        WorldMonsterSpawn, WorldNpcKind, WorldNpcManager, WorldNpcSpawn, WorldObjectSpawn,
        WorldPlayerRole, WorldPlayerSpawn,
    };

    fn world_entities_fixture() -> (WorldEntitiesManager, WorldNpcManager, WorldMonsterManager) {
        let mut world_entities = WorldEntitiesManager::new();
        world_entities.set_local_player(WorldPlayerSpawn::new(
            WorldPlayerRole::Local,
            "Hero",
            17,
            "data/character/dark_knight.glb",
            WorldEntityPose::new([1.0, 2.0, 3.0], [0.0, 90.0, 0.0], [1.0, 1.0, 1.0]),
        ));
        world_entities.set_remote_players([WorldPlayerSpawn::new(
            WorldPlayerRole::Remote,
            "Ally",
            23,
            "data/character/wizard.glb",
            WorldEntityPose::new([4.0, 5.0, 6.0], [0.0, 180.0, 0.0], [1.0, 1.0, 1.0]),
        )]);
        world_entities.set_objects([
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
        ]);

        let mut npcs = WorldNpcManager::new();
        npcs.load_fixture([
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

        let mut monsters = WorldMonsterManager::new();
        monsters.load_fixture([
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

        (world_entities, npcs, monsters)
    }

    #[test]
    fn render_entities_plugin_registers_resource() {
        let mut app = App::new();
        app.add_plugins(RenderEntitiesPlugin);

        let entities = app.world().resource::<RenderEntities>();
        assert_eq!(entities.state(), RenderEntitiesState::Inactive);
        assert!(entities.catalog().is_empty());
        assert!(entities.world_summary().is_none());
    }

    #[test]
    fn render_entities_formats_player_object_npc_and_monster_catalogs() {
        let (world_entities, npcs, monsters) = world_entities_fixture();

        let catalog = collect_render_entities(&world_entities, &npcs, &monsters);
        let formatted = format_render_entity_catalog(&catalog);

        assert_eq!(catalog.total_entities(), 8);
        assert!(formatted.contains("[local-player]"));
        assert!(formatted.contains("Hero|data/character/dark_knight.glb"));
        assert!(formatted.contains("[npc]"));
        assert!(formatted.contains("[monster]"));
        assert!(formatted.contains("obj_00000|stone_01|data/object_1/stone_01.glb"));
        assert!(formatted.contains("Marlon|data/object_1/npc_quest.glb"));
        assert!(formatted.contains("Death King|data/object_1/death_king.glb"));

        let mut render_entities = RenderEntities::new();
        render_entities.sync_world(&world_entities, &npcs, &monsters);

        assert_eq!(render_entities.state(), RenderEntitiesState::Ready);
        assert!(render_entities.snapshot().contains("world_summary=none"));
        assert!(render_entities.snapshot().contains("remote_count=1"));
        assert!(render_entities.snapshot().contains("object_count=2"));
        assert!(render_entities.snapshot().contains("npc_count=2"));
        assert!(render_entities.snapshot().contains("monster_count=2"));
    }
}
