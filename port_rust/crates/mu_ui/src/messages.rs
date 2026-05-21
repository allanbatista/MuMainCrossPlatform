use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};

const INFO_TITLE: &str = "Message";
const WARNING_TITLE: &str = "Warning";
const ERROR_TITLE: &str = "Error";
const BLOCKING_TITLE: &str = "Blocking Message";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageSeverity {
    Info,
    Warning,
    Error,
    Blocking,
}

impl MessageSeverity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Blocking => "blocking",
        }
    }

    pub const fn title(self) -> &'static str {
        match self {
            Self::Info => INFO_TITLE,
            Self::Warning => WARNING_TITLE,
            Self::Error => ERROR_TITLE,
            Self::Blocking => BLOCKING_TITLE,
        }
    }

    pub const fn blocking(self) -> bool {
        matches!(self, Self::Blocking)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageAction {
    Acknowledge,
    Retry,
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MessagesScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub severity: MessageSeverity,
    pub title: &'static str,
    pub body: &'static str,
    pub actions: &'static [MessageAction],
    pub blocking: bool,
}

const INFO_ACTIONS: &[MessageAction] = &[MessageAction::Acknowledge];
const RETRY_ACTIONS: &[MessageAction] = &[MessageAction::Retry, MessageAction::Exit];

pub fn messages_screen(severity: MessageSeverity, body: &'static str) -> MessagesScreen {
    let actions = match severity {
        MessageSeverity::Info | MessageSeverity::Warning => INFO_ACTIONS,
        MessageSeverity::Error | MessageSeverity::Blocking => RETRY_ACTIONS,
    };

    MessagesScreen {
        route: UiRoute::Messages,
        layout: UiShellLayout::for_route(UiRoute::Messages),
        widgets: UiShellWidgetSet::for_route(UiRoute::Messages),
        severity,
        title: severity.title(),
        body,
        actions,
        blocking: severity.blocking(),
    }
}

impl MessagesScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|severity={}|title={}|body={}|actions={:?}|blocking={}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.severity.as_str(),
            self.title,
            self.body,
            self.actions,
            self.blocking,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{messages_screen, MessageSeverity};

    #[test]
    fn message_severity_titles_snapshot() {
        assert_eq!(
            messages_screen(MessageSeverity::Info, "Welcome back.").snapshot(),
            "route=messages|group=authentication|severity=info|title=Message|body=Welcome back.|actions=[Acknowledge]|blocking=false|layout=UiShellLayout { group: Authentication, outer_margin: 40.0, panel_gap: 20.0, sidebar_width: 0.0, content_max_width: 1024.0, footer_height: 36.0 }|widgets=[Header, Body, MessageRail, Footer]"
        );
        assert_eq!(
            messages_screen(MessageSeverity::Warning, "Your changes are unsaved.").snapshot(),
            "route=messages|group=authentication|severity=warning|title=Warning|body=Your changes are unsaved.|actions=[Acknowledge]|blocking=false|layout=UiShellLayout { group: Authentication, outer_margin: 40.0, panel_gap: 20.0, sidebar_width: 0.0, content_max_width: 1024.0, footer_height: 36.0 }|widgets=[Header, Body, MessageRail, Footer]"
        );
    }

    #[test]
    fn message_error_states_snapshot() {
        assert_eq!(
            messages_screen(MessageSeverity::Error, "Server disconnected.").snapshot(),
            "route=messages|group=authentication|severity=error|title=Error|body=Server disconnected.|actions=[Retry, Exit]|blocking=false|layout=UiShellLayout { group: Authentication, outer_margin: 40.0, panel_gap: 20.0, sidebar_width: 0.0, content_max_width: 1024.0, footer_height: 36.0 }|widgets=[Header, Body, MessageRail, Footer]"
        );
        assert_eq!(
            messages_screen(MessageSeverity::Blocking, "The client cannot continue.").snapshot(),
            "route=messages|group=authentication|severity=blocking|title=Blocking Message|body=The client cannot continue.|actions=[Retry, Exit]|blocking=true|layout=UiShellLayout { group: Authentication, outer_margin: 40.0, panel_gap: 20.0, sidebar_width: 0.0, content_max_width: 1024.0, footer_height: 36.0 }|widgets=[Header, Body, MessageRail, Footer]"
        );
    }
}
