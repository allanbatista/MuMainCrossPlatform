use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};

const SERVER_SELECT_TITLE: &str = "Server Select";
const LOADING_NOTICE: &str = "Loading server list...";
const EMPTY_NOTICE: &str = "No servers are available.";
const ERROR_NOTICE: &str = "Failed to load the server list.";

const READY_SERVERS: &[ServerEntry] = &[
    ServerEntry::new("Alpha", "Online", Some(248)),
    ServerEntry::new("Bravo", "Maintenance", None),
];

const SERVER_ACTIONS: &[ServerSelectAction] = &[
    ServerSelectAction::Refresh,
    ServerSelectAction::Connect,
    ServerSelectAction::Back,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerSelectScreenState {
    Loading,
    Empty,
    Ready,
    Error,
}

impl ServerSelectScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Loading => "loading",
            Self::Empty => "empty",
            Self::Ready => "ready",
            Self::Error => "error",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Loading => Some(LOADING_NOTICE),
            Self::Empty => Some(EMPTY_NOTICE),
            Self::Ready => None,
            Self::Error => Some(ERROR_NOTICE),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerEntry {
    pub name: &'static str,
    pub status: &'static str,
    pub players: Option<u16>,
}

impl ServerEntry {
    pub const fn new(name: &'static str, status: &'static str, players: Option<u16>) -> Self {
        Self {
            name,
            status,
            players,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerSelectAction {
    Refresh,
    Connect,
    Back,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ServerSelectScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: ServerSelectScreenState,
    pub title: &'static str,
    pub notice: Option<&'static str>,
    pub servers: &'static [ServerEntry],
    pub actions: &'static [ServerSelectAction],
}

pub fn server_select_screen(state: ServerSelectScreenState) -> ServerSelectScreen {
    let servers = match state {
        ServerSelectScreenState::Ready => READY_SERVERS,
        _ => &[],
    };

    ServerSelectScreen {
        route: UiRoute::ServerSelect,
        layout: UiShellLayout::for_route(UiRoute::ServerSelect),
        widgets: UiShellWidgetSet::for_route(UiRoute::ServerSelect),
        state,
        title: SERVER_SELECT_TITLE,
        notice: state.notice(),
        servers,
        actions: SERVER_ACTIONS,
    }
}

impl ServerSelectScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|notice={:?}|servers={:?}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.notice,
            self.servers,
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{server_select_screen, ServerSelectScreenState};

    #[test]
    fn server_select_loading_and_empty_snapshots() {
        assert_eq!(
            server_select_screen(ServerSelectScreenState::Loading).snapshot(),
            "route=server-select|group=authentication|state=loading|title=Server Select|notice=Some(\"Loading server list...\")|servers=[]|actions=[Refresh, Connect, Back]|layout=UiShellLayout { group: Authentication, outer_margin: 40.0, panel_gap: 20.0, sidebar_width: 0.0, content_max_width: 1024.0, footer_height: 36.0 }|widgets=[Header, Body, MessageRail, Footer]"
        );
        assert_eq!(
            server_select_screen(ServerSelectScreenState::Empty).snapshot(),
            "route=server-select|group=authentication|state=empty|title=Server Select|notice=Some(\"No servers are available.\")|servers=[]|actions=[Refresh, Connect, Back]|layout=UiShellLayout { group: Authentication, outer_margin: 40.0, panel_gap: 20.0, sidebar_width: 0.0, content_max_width: 1024.0, footer_height: 36.0 }|widgets=[Header, Body, MessageRail, Footer]"
        );
    }

    #[test]
    fn server_select_ready_snapshot_lists_servers() {
        assert_eq!(
            server_select_screen(ServerSelectScreenState::Ready).snapshot(),
            "route=server-select|group=authentication|state=ready|title=Server Select|notice=None|servers=[ServerEntry { name: \"Alpha\", status: \"Online\", players: Some(248) }, ServerEntry { name: \"Bravo\", status: \"Maintenance\", players: None }]|actions=[Refresh, Connect, Back]|layout=UiShellLayout { group: Authentication, outer_margin: 40.0, panel_gap: 20.0, sidebar_width: 0.0, content_max_width: 1024.0, footer_height: 36.0 }|widgets=[Header, Body, MessageRail, Footer]"
        );
    }

    #[test]
    fn server_select_error_snapshot() {
        assert_eq!(
            server_select_screen(ServerSelectScreenState::Error).snapshot(),
            "route=server-select|group=authentication|state=error|title=Server Select|notice=Some(\"Failed to load the server list.\")|servers=[]|actions=[Refresh, Connect, Back]|layout=UiShellLayout { group: Authentication, outer_margin: 40.0, panel_gap: 20.0, sidebar_width: 0.0, content_max_width: 1024.0, footer_height: 36.0 }|widgets=[Header, Body, MessageRail, Footer]"
        );
    }
}
