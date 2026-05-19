use camino::Utf8PathBuf;
use mu_assets::load_terrain_world_bundle;
use mu_gameplay::{
    MovementActorState, MovementCommand, MovementManager, MovementState, WorldManager,
};
use mu_protocol::movement::walk_request;

fn repo_assets_root() -> Utf8PathBuf {
    Utf8PathBuf::from_path_buf(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../port_rust/assets")
            .canonicalize()
            .unwrap(),
    )
    .unwrap()
}

#[test]
fn world_fixture_replay_exposes_local_remote_movement_and_camera() {
    let asset_root = repo_assets_root();
    let bundle = load_terrain_world_bundle(&asset_root, 1).unwrap();
    let mut world = WorldManager::new();
    world.set_bundle(bundle);

    let mut movement = MovementManager::new();
    movement.sync_world(&world);
    movement.set_local_actor(MovementActorState::new(
        "hero",
        1001,
        MovementCommand::new(147, 128, 3, 4, [0x12, 0x34]),
    ));
    movement.push_remote_actor(MovementActorState::new(
        "remote-1",
        2002,
        MovementCommand::new(141, 132, 2, 1, [0x56]),
    ));

    assert_eq!(
        walk_request(147, 128, 3, 4, [0x12, 0x34]).unwrap(),
        vec![0xC1, 0x08, 0xD4, 147, 128, 0x43, 0x12, 0x34]
    );
    assert_eq!(movement.state(), MovementState::Ready);
    assert!(movement
        .snapshot()
        .contains("world_summary=world=1|world_directory=data/world_1"));
    assert!(movement.snapshot().contains(
        "local=label=hero|key=1001|source=147,128|steps=3|rotation=4|directions=[18,52]"
    ));
    assert!(movement.snapshot().contains("remote_count=1"));
    assert!(movement
        .snapshot()
        .contains("camera=mode=default|waypoint=0/8|loop=true|blend=300|interpolation=smooth"));
    assert!(movement
        .camera()
        .snapshot()
        .contains("position=14700,490,12800"));
}
