pub mod character_select;
pub mod error;
pub mod i18n;
pub mod layout;
pub mod login;
pub mod messages;
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
pub use error::ErrorPresentation;
pub use fixture_routes::{fixture_routes, UiFixtureRoute};
pub use i18n::Translator;
pub use layout::UiShellLayout;
pub use login::{login_screen, LoginAction, LoginField, LoginScreen, LoginScreenState};
pub use messages::{messages_screen, MessageAction, MessageSeverity, MessagesScreen};
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
