use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonInput;
use bevy::prelude::{App, Plugin, Res, ResMut, Time, Update, Vec3};

use crate::bootstrap_runtime::BootstrapRuntime;
use crate::ClientRuntime;
use crate::SessionState;
use mu_gameplay::{world_tile_from_position, MovementCommand};
use mu_ui::{UiRoute, UiShellState};

const LOCAL_PLAYER_MOVE_SPEED: f32 = 48.0;
const ROTATION_NORTH: u8 = 0;
const ROTATION_NORTH_EAST: u8 = 1;
const ROTATION_EAST: u8 = 2;
const ROTATION_SOUTH_EAST: u8 = 3;
const ROTATION_SOUTH: u8 = 4;
const ROTATION_SOUTH_WEST: u8 = 5;
const ROTATION_WEST: u8 = 6;
const ROTATION_NORTH_WEST: u8 = 7;

#[derive(Debug, Default, Clone, Copy)]
pub struct WorldMotionPlugin;

impl Plugin for WorldMotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_world_motion_system);
    }
}

pub(crate) fn apply_world_motion_system(
    mut client_runtime: ResMut<ClientRuntime>,
    ui_shell: Res<UiShellState>,
    session_state: Res<SessionState>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    bootstrap_runtime: Option<Res<BootstrapRuntime>>,
) {
    apply_world_motion(
        &mut client_runtime,
        &ui_shell,
        &session_state,
        &keys,
        time.delta_secs(),
        bootstrap_runtime.as_deref(),
    );
}

pub(crate) fn apply_world_motion(
    client_runtime: &mut ClientRuntime,
    ui_shell: &UiShellState,
    session_state: &SessionState,
    keys: &ButtonInput<KeyCode>,
    delta_secs: f32,
    bootstrap_runtime: Option<&BootstrapRuntime>,
) {
    if ui_shell.current() != UiRoute::World
        || !client_runtime.world_ready()
        || session_state.is_disconnected()
    {
        return;
    }

    let direction = local_player_motion_direction(keys);
    if direction == Vec3::ZERO {
        return;
    }

    let delta = direction.normalize() * LOCAL_PLAYER_MOVE_SPEED * delta_secs;
    if let Some(movement) = movement_command_for_local_player(client_runtime, direction) {
        let has_live_session = session_state.is_logged_in();
        if has_live_session {
            if let Some(bootstrap_runtime) = bootstrap_runtime {
                let _ = bootstrap_runtime.queue_movement_request(movement);
            }
        }
    }

    client_runtime.translate_local_player([delta.x as f64, 0.0, delta.z as f64]);
}

fn local_player_motion_direction(keys: &ButtonInput<KeyCode>) -> Vec3 {
    let mut direction = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
    }

    if keys.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }

    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }

    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    direction
}

fn movement_command_for_local_player(
    client_runtime: &ClientRuntime,
    direction: Vec3,
) -> Option<MovementCommand> {
    let local_player = client_runtime.world_entities().local_player()?;
    let [source_x, source_y] = world_tile_from_position(local_player.pose.position);

    Some(MovementCommand::new(
        source_x,
        source_y,
        0,
        movement_rotation_from_direction(direction),
        [],
    ))
}

fn movement_rotation_from_direction(direction: Vec3) -> u8 {
    let x_sign = direction.x.signum() as i8;
    let z_sign = direction.z.signum() as i8;

    match (x_sign, z_sign) {
        (0, -1) => ROTATION_NORTH,
        (1, -1) => ROTATION_NORTH_EAST,
        (1, 0) => ROTATION_EAST,
        (1, 1) => ROTATION_SOUTH_EAST,
        (0, 1) => ROTATION_SOUTH,
        (-1, 1) => ROTATION_SOUTH_WEST,
        (-1, 0) => ROTATION_WEST,
        (-1, -1) => ROTATION_NORTH_WEST,
        _ => ROTATION_NORTH,
    }
}

#[cfg(test)]
mod tests {
    use super::apply_world_motion;
    use crate::{ClientRuntime, SessionState};
    use bevy::input::keyboard::KeyCode;
    use bevy::input::ButtonInput;
    use camino::Utf8PathBuf;
    use mu_assets::load_terrain_world_bundle;
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

    fn runtime_with_world() -> ClientRuntime {
        let asset_root = repo_assets_root();
        let bundle = load_terrain_world_bundle(&asset_root, 1).unwrap();
        let mut runtime = ClientRuntime::new();
        runtime.load_world_bundle(bundle);
        runtime
    }

    #[test]
    fn world_motion_moves_the_local_avatar_in_the_world_route() {
        let mut runtime = runtime_with_world();
        let mut ui_shell = UiShellState::default();
        let session_state = SessionState::new();
        ui_shell.set_route(UiRoute::World);
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::KeyW);

        apply_world_motion(&mut runtime, &ui_shell, &session_state, &keys, 1.0, None);

        assert_eq!(
            runtime
                .world_entities()
                .local_player()
                .unwrap()
                .pose
                .position,
            [0.0, 1.0, -48.0]
        );
        assert_eq!(
            runtime
                .render_entities()
                .catalog()
                .local_player
                .as_ref()
                .unwrap()
                .pose
                .position,
            [0.0, 1.0, -48.0]
        );
    }

    #[test]
    fn world_motion_ignores_non_world_routes() {
        let mut runtime = runtime_with_world();
        let ui_shell = UiShellState::default();
        let session_state = SessionState::new();
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::KeyW);

        apply_world_motion(&mut runtime, &ui_shell, &session_state, &keys, 1.0, None);

        assert_eq!(
            runtime
                .world_entities()
                .local_player()
                .unwrap()
                .pose
                .position,
            [0.0, 1.0, 0.0]
        );
    }

    #[test]
    fn world_motion_ignores_a_reset_world_projection() {
        let mut runtime = runtime_with_world();
        let mut ui_shell = UiShellState::default();
        let session_state = SessionState::new();
        ui_shell.set_route(UiRoute::World);
        runtime.clear_world_projection();
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::KeyW);

        apply_world_motion(&mut runtime, &ui_shell, &session_state, &keys, 1.0, None);

        assert!(runtime.world_entities().local_player().is_none());
    }

    #[test]
    fn world_motion_ignores_disconnected_sessions() {
        let mut runtime = runtime_with_world();
        let mut ui_shell = UiShellState::default();
        let mut session_state = SessionState::new();
        ui_shell.set_route(UiRoute::World);
        session_state.disconnect();
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::KeyW);

        apply_world_motion(&mut runtime, &ui_shell, &session_state, &keys, 1.0, None);

        assert_eq!(
            runtime
                .world_entities()
                .local_player()
                .unwrap()
                .pose
                .position,
            [0.0, 1.0, 0.0]
        );
    }
}
