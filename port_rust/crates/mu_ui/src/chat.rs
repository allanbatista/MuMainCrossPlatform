use crate::{UiRoute, UiShellLayout, UiShellWidgetSet};

const CHAT_TITLE: &str = "Chat";
const CHAT_LAYER_DEPTH: f32 = 6.1;
const CHAT_KEY_EVENT_ORDER: f32 = 8.0;
const CHAT_SHOWING_LINES: u8 = 6;
const CHAT_BACK_ALPHA: f32 = 0.6;

const CHAT_MESSAGE_COUNTS: &[ChatMessageCount] = &[
    ChatMessageCount::new(ChatMessageType::All, 12),
    ChatMessageCount::new(ChatMessageType::Chat, 6),
    ChatMessageCount::new(ChatMessageType::Whisper, 2),
    ChatMessageCount::new(ChatMessageType::System, 3),
    ChatMessageCount::new(ChatMessageType::Error, 1),
    ChatMessageCount::new(ChatMessageType::Party, 0),
    ChatMessageCount::new(ChatMessageType::Guild, 0),
    ChatMessageCount::new(ChatMessageType::Union, 0),
    ChatMessageCount::new(ChatMessageType::Gens, 0),
    ChatMessageCount::new(ChatMessageType::GM, 0),
];

const CHAT_FILTERS: &[&str] = &[];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatMessageType {
    All,
    Chat,
    Whisper,
    System,
    Error,
    Party,
    Guild,
    Union,
    Gens,
    GM,
}

impl ChatMessageType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Chat => "chat",
            Self::Whisper => "whisper",
            Self::System => "system",
            Self::Error => "error",
            Self::Party => "party",
            Self::Guild => "guild",
            Self::Union => "union",
            Self::Gens => "gens",
            Self::GM => "gm",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChatMessageCount {
    pub message_type: ChatMessageType,
    pub count: u16,
}

impl ChatMessageCount {
    pub const fn new(message_type: ChatMessageType, count: u16) -> Self {
        Self {
            message_type,
            count,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatScreenState {
    Docked,
}

impl ChatScreenState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Docked => "docked",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChatScreen {
    pub route: UiRoute,
    pub layout: UiShellLayout,
    pub widgets: UiShellWidgetSet,
    pub state: ChatScreenState,
    pub title: &'static str,
    pub layer_depth: f32,
    pub key_event_order: f32,
    pub show_chat_log: bool,
    pub show_frame: bool,
    pub current_message_type: ChatMessageType,
    pub showing_lines: u8,
    pub render_end_line: i32,
    pub back_alpha: f32,
    pub pointed_message_index: Option<u16>,
    pub filters: &'static [&'static str],
    pub message_counts: &'static [ChatMessageCount],
}

pub fn chat_screen() -> ChatScreen {
    ChatScreen {
        route: UiRoute::Chat,
        layout: UiShellLayout::for_route(UiRoute::Chat),
        widgets: UiShellWidgetSet::for_route(UiRoute::Chat),
        state: ChatScreenState::Docked,
        title: CHAT_TITLE,
        layer_depth: CHAT_LAYER_DEPTH,
        key_event_order: CHAT_KEY_EVENT_ORDER,
        show_chat_log: true,
        show_frame: false,
        current_message_type: ChatMessageType::All,
        showing_lines: CHAT_SHOWING_LINES,
        render_end_line: -1,
        back_alpha: CHAT_BACK_ALPHA,
        pointed_message_index: None,
        filters: CHAT_FILTERS,
        message_counts: CHAT_MESSAGE_COUNTS,
    }
}

impl ChatScreen {
    pub fn snapshot(self) -> String {
        format!(
            "route={}|group={}|state={}|title={}|layer_depth={}|key_event_order={}|show_chat_log={}|show_frame={}|current_message_type={}|showing_lines={}|render_end_line={}|back_alpha={}|pointed_message_index={:?}|filters={:?}|message_counts={:?}|layout={:?}|widgets={:?}",
            self.route.slug(),
            self.layout.group.as_str(),
            self.state.as_str(),
            self.title,
            self.layer_depth,
            self.key_event_order,
            self.show_chat_log,
            self.show_frame,
            self.current_message_type.as_str(),
            self.showing_lines,
            self.render_end_line,
            self.back_alpha,
            self.pointed_message_index,
            self.filters,
            self.message_counts,
            self.layout,
            self.widgets.widgets(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::chat_screen;

    #[test]
    fn chat_snapshot_matches_legacy_docked_window() {
        assert_eq!(
            chat_screen().snapshot(),
            "route=chat|group=world|state=docked|title=Chat|layer_depth=6.1|key_event_order=8|show_chat_log=true|show_frame=false|current_message_type=all|showing_lines=6|render_end_line=-1|back_alpha=0.6|pointed_message_index=None|filters=[]|message_counts=[ChatMessageCount { message_type: All, count: 12 }, ChatMessageCount { message_type: Chat, count: 6 }, ChatMessageCount { message_type: Whisper, count: 2 }, ChatMessageCount { message_type: System, count: 3 }, ChatMessageCount { message_type: Error, count: 1 }, ChatMessageCount { message_type: Party, count: 0 }, ChatMessageCount { message_type: Guild, count: 0 }, ChatMessageCount { message_type: Union, count: 0 }, ChatMessageCount { message_type: Gens, count: 0 }, ChatMessageCount { message_type: GM, count: 0 }]|layout=UiShellLayout { group: World, outer_margin: 16.0, panel_gap: 12.0, sidebar_width: 360.0, content_max_width: 1280.0, footer_height: 0.0 }|widgets=[Body, Sidebar, StatusStrip, Overlay]"
        );
    }
}
