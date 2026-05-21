use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonInput;
use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Res, ResMut, Update};
use mu_ui::{UiRoute, UiShellState};

use crate::{ClientRuntime, SessionState};

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct InventoryRoutePlugin;

impl Plugin for InventoryRoutePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            toggle_inventory_route_system.before(crate::world_motion::apply_world_motion_system),
        );
    }
}

pub(crate) fn toggle_inventory_route_system(
    mut ui_shell: ResMut<UiShellState>,
    client_runtime: Res<ClientRuntime>,
    session_state: Res<SessionState>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    toggle_inventory_route(&mut ui_shell, &client_runtime, &session_state, &keys);
}

pub(crate) fn toggle_inventory_route(
    ui_shell: &mut UiShellState,
    client_runtime: &ClientRuntime,
    session_state: &SessionState,
    keys: &ButtonInput<KeyCode>,
) {
    if !keys.just_pressed(KeyCode::Tab) {
        return;
    }

    match ui_shell.current() {
        UiRoute::World if client_runtime.world_ready() && session_state.is_logged_in() => {
            ui_shell.set_route(UiRoute::Inventory);
        }
        UiRoute::Inventory => {
            if client_runtime.world_ready() && session_state.is_logged_in() {
                ui_shell.set_route(UiRoute::World);
            } else {
                ui_shell.set_route(UiRoute::Login);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::toggle_inventory_route;
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

    fn logged_in_session() -> SessionState {
        let mut session_state = SessionState::new();
        assert!(session_state.login_success());
        session_state
    }

    #[test]
    fn tab_toggles_inventory_from_the_world_route() {
        let runtime = runtime_with_world();
        let session_state = logged_in_session();
        let mut ui_shell = UiShellState::default();
        ui_shell.set_route(UiRoute::World);
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::Tab);

        toggle_inventory_route(&mut ui_shell, &runtime, &session_state, &keys);

        assert_eq!(ui_shell.current(), UiRoute::Inventory);
    }

    #[test]
    fn tab_returns_from_inventory_to_the_world_route() {
        let runtime = runtime_with_world();
        let session_state = logged_in_session();
        let mut ui_shell = UiShellState::default();
        ui_shell.set_route(UiRoute::World);
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::Tab);

        toggle_inventory_route(&mut ui_shell, &runtime, &session_state, &keys);

        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::Tab);

        toggle_inventory_route(&mut ui_shell, &runtime, &session_state, &keys);

        assert_eq!(ui_shell.current(), UiRoute::World);
    }

    #[test]
    fn tab_ignores_non_world_routes_and_disconnected_sessions() {
        let runtime = runtime_with_world();
        let mut ui_shell = UiShellState::default();
        let mut session_state = SessionState::new();
        ui_shell.set_route(UiRoute::Login);
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::Tab);

        toggle_inventory_route(&mut ui_shell, &runtime, &session_state, &keys);

        assert_eq!(ui_shell.current(), UiRoute::Login);

        session_state.login_success();
        ui_shell.set_route(UiRoute::World);
        keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::Tab);
        toggle_inventory_route(&mut ui_shell, &runtime, &session_state, &keys);
        assert_eq!(ui_shell.current(), UiRoute::Inventory);

        session_state.disconnect();
        keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::Tab);
        toggle_inventory_route(&mut ui_shell, &runtime, &session_state, &keys);
        assert_eq!(ui_shell.current(), UiRoute::Login);
    }
}
