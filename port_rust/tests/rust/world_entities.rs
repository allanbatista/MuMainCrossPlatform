use camino::Utf8PathBuf;
use mu_assets::load_terrain_world_bundle;
use mu_gameplay::{
    WorldEntitiesManager, WorldMonsterKind, WorldMonsterManager, WorldMonsterSpawn, WorldNpcKind,
    WorldNpcManager, WorldNpcSpawn, WorldPlayerRole, WorldPlayerSpawn, WorldEntityPose,
    WorldManager,
};
use mu_render::{RenderEntities, RenderEntitiesPlugin};

fn repo_assets_root() -> Utf8PathBuf {
    Utf8PathBuf::from_path_buf(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../port_rust/assets")
            .canonicalize()
            .unwrap(),
    )
    .unwrap()
}

fn world_entities_fixture() -> (
    WorldEntitiesManager,
    WorldNpcManager,
    WorldMonsterManager,
) {
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
fn world_minimal_fixture_spawns_local_remote_objects_npcs_and_monsters() {
    let asset_root = repo_assets_root();
    let bundle = load_terrain_world_bundle(&asset_root, 1).unwrap();
    let mut world = WorldManager::new();
    world.set_bundle(bundle);

    let (mut world_entities, npcs, monsters) = world_entities_fixture();
    world_entities.sync_world(&world);

    assert_eq!(world_entities.object_count(), 2_870);
    assert_eq!(world_entities.remote_players().len(), 1);
    assert!(world_entities
        .snapshot()
        .contains("object_count=2870|objects=[id=obj_00000|label=stone_01|type=30|model=data/object_1/stone_01.glb"));
    assert!(npcs.snapshot().contains("count=2"));
    assert!(monsters.snapshot().contains("count=2"));

    let mut render_entities = RenderEntities::new();
    render_entities.sync_world(&world_entities, &npcs, &monsters);

    assert!(render_entities.snapshot().contains("state=ready"));
    assert!(render_entities
        .snapshot()
        .contains("world_summary=world=1|world_directory=data/world_1"));
    assert!(render_entities.snapshot().contains("remote_count=1"));
    assert!(render_entities.snapshot().contains("npc_count=2"));
    assert!(render_entities.snapshot().contains("monster_count=2"));
}

#[test]
fn render_entities_plugin_exposes_initial_resource() {
    let mut app = bevy::prelude::App::new();
    app.add_plugins(RenderEntitiesPlugin);

    let entities = app.world().resource::<RenderEntities>();
    assert!(entities.catalog().is_empty());
    assert!(entities.world_summary().is_none());
}
