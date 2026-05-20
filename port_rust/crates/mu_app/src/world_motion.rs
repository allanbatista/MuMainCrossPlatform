use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonInput;
use bevy::prelude::{App, Plugin, Res, ResMut, Time, Update, Vec3};

use crate::ClientRuntime;
use mu_ui::{UiRoute, UiShellState};

const LOCAL_PLAYER_MOVE_SPEED: f32 = 48.0;

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
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    apply_world_motion(&mut client_runtime, &ui_shell, &keys, time.delta_secs());
}

pub(crate) fn apply_world_motion(
    client_runtime: &mut ClientRuntime,
    ui_shell: &UiShellState,
    keys: &ButtonInput<KeyCode>,
    delta_secs: f32,
) {
    if ui_shell.current() != UiRoute::World || !client_runtime.world_ready() {
        return;
    }

    let direction = local_player_motion_direction(keys);
    if direction == Vec3::ZERO {
        return;
    }

    let delta = direction.normalize() * LOCAL_PLAYER_MOVE_SPEED * delta_secs;
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

#[cfg(test)]
mod tests {
    use super::apply_world_motion;
    use crate::ClientRuntime;
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
        ui_shell.set_route(UiRoute::World);
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::KeyW);

        apply_world_motion(&mut runtime, &ui_shell, &keys, 1.0);

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
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::KeyW);

        apply_world_motion(&mut runtime, &ui_shell, &keys, 1.0);

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
        ui_shell.set_route(UiRoute::World);
        runtime.clear_world_projection();
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::KeyW);

        apply_world_motion(&mut runtime, &ui_shell, &keys, 1.0);

        assert!(runtime.world_entities().local_player().is_none());
    }
}
