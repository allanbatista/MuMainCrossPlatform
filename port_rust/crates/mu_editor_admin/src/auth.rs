use bevy::prelude::{App, Plugin, Resource};

const HIDDEN_NOTICE: &str = "Editor/Admin entry is hidden in release player mode.";
const BLOCKED_NOTICE: &str = "Admin permission is required.";
const READY_NOTICE: &str = "Open the editor/admin tools.";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AdminAccessState {
    #[default]
    Hidden,
    Blocked,
    Ready,
}

impl AdminAccessState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hidden => "hidden",
            Self::Blocked => "blocked",
            Self::Ready => "ready",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Hidden => Some(HIDDEN_NOTICE),
            Self::Blocked => Some(BLOCKED_NOTICE),
            Self::Ready => Some(READY_NOTICE),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdminAuthState {
    release_player_mode: bool,
    authorized: bool,
}

impl Default for AdminAuthState {
    fn default() -> Self {
        Self {
            release_player_mode: true,
            authorized: false,
        }
    }
}

impl Resource for AdminAuthState {}

#[derive(Debug, Default, Clone, Copy)]
pub struct AdminPlugin;

impl Plugin for AdminPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AdminAuthState>();
    }
}

impl AdminAuthState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_release_player_mode(release_player_mode: bool) -> Self {
        Self {
            release_player_mode,
            ..Self::default()
        }
    }

    pub fn is_release_player_mode(&self) -> bool {
        self.release_player_mode
    }

    pub fn is_authorized(&self) -> bool {
        self.authorized
    }

    pub fn entry_visible(&self) -> bool {
        !self.release_player_mode
    }

    pub fn access_state(&self) -> AdminAccessState {
        if self.release_player_mode {
            return AdminAccessState::Hidden;
        }

        if self.authorized {
            AdminAccessState::Ready
        } else {
            AdminAccessState::Blocked
        }
    }

    pub fn set_release_player_mode(&mut self, release_player_mode: bool) {
        self.release_player_mode = release_player_mode;
    }

    pub fn set_authorized(&mut self, authorized: bool) {
        self.authorized = authorized;
    }

    pub fn authorize(&mut self) {
        self.authorized = true;
    }

    pub fn revoke(&mut self) {
        self.authorized = false;
    }
}

#[cfg(test)]
mod tests {
    use super::{AdminAccessState, AdminAuthState, AdminPlugin};
    use bevy::prelude::App;

    #[test]
    fn release_player_mode_hides_the_entry() {
        let state = AdminAuthState::new();

        assert_eq!(state.access_state(), AdminAccessState::Hidden);
        assert!(state.is_release_player_mode());
        assert!(!state.entry_visible());
    }

    #[test]
    fn unauthorized_users_are_blocked_outside_release_mode() {
        let state = AdminAuthState::with_release_player_mode(false);

        assert_eq!(state.access_state(), AdminAccessState::Blocked);
        assert!(!state.is_authorized());
        assert!(state.entry_visible());
    }

    #[test]
    fn authorization_unlocks_the_entry_until_revoked() {
        let mut state = AdminAuthState::with_release_player_mode(false);

        state.authorize();
        assert_eq!(state.access_state(), AdminAccessState::Ready);

        state.revoke();
        assert_eq!(state.access_state(), AdminAccessState::Blocked);
    }

    #[test]
    fn plugin_registers_the_resource() {
        let mut app = App::new();
        app.add_plugins(AdminPlugin);

        let state = app.world().resource::<AdminAuthState>();

        assert_eq!(state.access_state(), AdminAccessState::Hidden);
        assert!(state.is_release_player_mode());
    }
}
