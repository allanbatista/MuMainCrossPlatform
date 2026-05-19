pub mod character_select;
pub mod chat;
pub mod error;
pub mod hotkeys;
pub mod hud;
pub mod i18n;
pub mod layout;
pub mod login;
pub mod messages;
pub mod minimap;
pub mod options;
pub mod routes;
pub mod server_select;
pub mod widgets;

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

mod fixture_routes;

pub use character_select::{
    character_select_screen, CharacterSelectAction, CharacterSelectButton,
    CharacterSelectCharacter, CharacterSelectScreen, CharacterSelectScreenState,
};
pub use chat::{chat_screen, ChatMessageCount, ChatMessageType, ChatScreen, ChatScreenState};
pub use error::ErrorPresentation;
pub use fixture_routes::{fixture_routes, UiFixtureRoute};
pub use hotkeys::{hotkeys_screen, HotkeyBinding, HotkeysScreen, HotkeysScreenState};
pub use hud::{hud_screen, HudButton, HudButtonKind, HudGauge, HudGaugeKind, HudScreen};
pub use i18n::Translator;
pub use layout::UiShellLayout;
pub use login::{login_screen, LoginAction, LoginField, LoginScreen, LoginScreenState};
pub use messages::{messages_screen, MessageAction, MessageSeverity, MessagesScreen};
pub use minimap::{minimap_screen, MiniMapMarker, MiniMapMarkerKind, MiniMapScreen};
pub use mu_assets::TranslationDomain;
pub use options::{
    options_screen, OptionsScreen, OptionsScreenState, OptionsSection, OptionsToggle,
};
pub use routes::{
    UiRoute, UiRouteCatalog, UiRouteDescriptor, UiRouteGroup, UiShellPlugin, UiShellState,
};
pub use server_select::{
    server_select_screen, ServerEntry, ServerSelectAction, ServerSelectScreen,
    ServerSelectScreenState,
};
pub use widgets::{UiShellWidgetKind, UiShellWidgetSet};
