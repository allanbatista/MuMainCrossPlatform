use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};
use mu_editor_admin::{AdminAccessState, AdminAuthState};

const ADMIN_TITLE: &str = "Editor Admin";
const HIDDEN_ACTIONS: &[AdminAction] = &[];
const BLOCKED_ACTIONS: &[AdminAction] = &[AdminAction::Close];
const READY_ACTIONS: &[AdminAction] = &[AdminAction::OpenEditor, AdminAction::Close];

pub use mu_editor_admin::AdminAccessState as AdminScreenState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdminAction {
    OpenEditor,
    Close,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdminScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: AdminScreenState,
    pub title: &'static str,
    pub notice: Option<&'static str>,
    pub release_player_mode: bool,
    pub authorized: bool,
    pub entry_visible: bool,
    pub actions: &'static [AdminAction],
}

pub fn admin_screen(auth: &AdminAuthState) -> AdminScreen {
    let state = auth.access_state();
    let actions = match state {
        AdminAccessState::Hidden => HIDDEN_ACTIONS,
        AdminAccessState::Blocked => BLOCKED_ACTIONS,
        AdminAccessState::Ready => READY_ACTIONS,
    };

    AdminScreen {
        route: UiRoute::EditorAdmin,
        layout: UiShellLayout::for_route(UiRoute::EditorAdmin),
        widgets: UiShellWidgetSet::for_route(UiRoute::EditorAdmin),
        state,
        title: ADMIN_TITLE,
        notice: state.notice(),
        release_player_mode: auth.is_release_player_mode(),
        authorized: auth.is_authorized(),
        entry_visible: auth.entry_visible(),
        actions,
    }
}

impl AdminScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|notice={:?}|release_player_mode={}|authorized={}|entry_visible={}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.notice,
            self.release_player_mode,
            self.authorized,
            self.entry_visible,
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{admin_screen, AdminAuthState, AdminScreenState, BLOCKED_ACTIONS, READY_ACTIONS};

    #[test]
    fn editor_admin_hidden_snapshot() {
        let screen = admin_screen(&AdminAuthState::new());

        assert_eq!(
            screen.clone().snapshot(),
            "route=editor-admin|group=admin|state=hidden|title=Editor Admin|notice=Some(\"Editor/Admin entry is hidden in release player mode.\")|release_player_mode=true|authorized=false|entry_visible=false|actions=[]|layout=UiShellLayout { group: Admin, outer_margin: 24.0, panel_gap: 18.0, sidebar_width: 420.0, content_max_width: 1280.0, footer_height: 32.0 }|widgets=[Header, Body, Sidebar, Footer]"
        );
        assert_eq!(screen.state, AdminScreenState::Hidden);
        assert!(screen.actions.is_empty());
    }

    #[test]
    fn editor_admin_blocked_snapshot() {
        let auth = AdminAuthState::with_release_player_mode(false);
        let screen = admin_screen(&auth);

        assert_eq!(
            screen.clone().snapshot(),
            "route=editor-admin|group=admin|state=blocked|title=Editor Admin|notice=Some(\"Admin permission is required.\")|release_player_mode=false|authorized=false|entry_visible=true|actions=[Close]|layout=UiShellLayout { group: Admin, outer_margin: 24.0, panel_gap: 18.0, sidebar_width: 420.0, content_max_width: 1280.0, footer_height: 32.0 }|widgets=[Header, Body, Sidebar, Footer]"
        );
        assert_eq!(screen.state, AdminScreenState::Blocked);
        assert_eq!(screen.actions, BLOCKED_ACTIONS);
    }

    #[test]
    fn editor_admin_ready_snapshot() {
        let mut auth = AdminAuthState::with_release_player_mode(false);
        auth.authorize();

        let screen = admin_screen(&auth);

        assert_eq!(
            screen.clone().snapshot(),
            "route=editor-admin|group=admin|state=ready|title=Editor Admin|notice=Some(\"Open the editor/admin tools.\")|release_player_mode=false|authorized=true|entry_visible=true|actions=[OpenEditor, Close]|layout=UiShellLayout { group: Admin, outer_margin: 24.0, panel_gap: 18.0, sidebar_width: 420.0, content_max_width: 1280.0, footer_height: 32.0 }|widgets=[Header, Body, Sidebar, Footer]"
        );
        assert_eq!(screen.state, AdminScreenState::Ready);
        assert_eq!(screen.actions, READY_ACTIONS);
    }
}
