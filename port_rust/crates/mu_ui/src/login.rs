use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};

const LOGIN_TITLE: &str = "Login";
const LOGIN_PROMPT: &str = "Enter your account credentials.";
const AUTHENTICATING_NOTICE: &str = "Authenticating...";
const INVALID_CREDENTIALS_NOTICE: &str = "Invalid account name or password.";
const SERVER_UNAVAILABLE_NOTICE: &str = "The selected server is unavailable.";

const LOGIN_FIELDS: &[LoginField] = &[
    LoginField::Username,
    LoginField::Password,
    LoginField::RememberUsername,
];

const LOGIN_ACTIONS: &[LoginAction] = &[
    LoginAction::Connect,
    LoginAction::Options,
    LoginAction::Exit,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginScreenState {
    Idle,
    Authenticating,
    InvalidCredentials,
    ServerUnavailable,
}

impl LoginScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Authenticating => "authenticating",
            Self::InvalidCredentials => "invalid-credentials",
            Self::ServerUnavailable => "server-unavailable",
        }
    }

    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Self::Idle => None,
            Self::Authenticating => Some(AUTHENTICATING_NOTICE),
            Self::InvalidCredentials => Some(INVALID_CREDENTIALS_NOTICE),
            Self::ServerUnavailable => Some(SERVER_UNAVAILABLE_NOTICE),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginField {
    Username,
    Password,
    RememberUsername,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginAction {
    Connect,
    Options,
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoginScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: LoginScreenState,
    pub title: &'static str,
    pub prompt: &'static str,
    pub notice: Option<&'static str>,
    pub fields: &'static [LoginField],
    pub actions: &'static [LoginAction],
}

pub fn login_screen(state: LoginScreenState) -> LoginScreen {
    LoginScreen {
        route: UiRoute::Login,
        layout: UiShellLayout::for_route(UiRoute::Login),
        widgets: UiShellWidgetSet::for_route(UiRoute::Login),
        state,
        title: LOGIN_TITLE,
        prompt: LOGIN_PROMPT,
        notice: state.notice(),
        fields: LOGIN_FIELDS,
        actions: LOGIN_ACTIONS,
    }
}

impl LoginScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|prompt={}|notice={:?}|fields={:?}|actions={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.prompt,
            self.notice,
            self.fields,
            self.actions,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{login_screen, LoginScreenState};

    #[test]
    fn login_idle_snapshot() {
        assert_eq!(
            login_screen(LoginScreenState::Idle).snapshot(),
            "route=login|group=authentication|state=idle|title=Login|prompt=Enter your account credentials.|notice=None|fields=[Username, Password, RememberUsername]|actions=[Connect, Options, Exit]|layout=UiShellLayout { group: Authentication, outer_margin: 40.0, panel_gap: 20.0, sidebar_width: 0.0, content_max_width: 1024.0, footer_height: 36.0 }|widgets=[Header, Body, MessageRail, Footer]"
        );
    }

    #[test]
    fn login_error_states_snapshot() {
        assert_eq!(
            login_screen(LoginScreenState::InvalidCredentials).snapshot(),
            "route=login|group=authentication|state=invalid-credentials|title=Login|prompt=Enter your account credentials.|notice=Some(\"Invalid account name or password.\")|fields=[Username, Password, RememberUsername]|actions=[Connect, Options, Exit]|layout=UiShellLayout { group: Authentication, outer_margin: 40.0, panel_gap: 20.0, sidebar_width: 0.0, content_max_width: 1024.0, footer_height: 36.0 }|widgets=[Header, Body, MessageRail, Footer]"
        );
        assert_eq!(
            login_screen(LoginScreenState::ServerUnavailable).snapshot(),
            "route=login|group=authentication|state=server-unavailable|title=Login|prompt=Enter your account credentials.|notice=Some(\"The selected server is unavailable.\")|fields=[Username, Password, RememberUsername]|actions=[Connect, Options, Exit]|layout=UiShellLayout { group: Authentication, outer_margin: 40.0, panel_gap: 20.0, sidebar_width: 0.0, content_max_width: 1024.0, footer_height: 36.0 }|widgets=[Header, Body, MessageRail, Footer]"
        );
    }
}
