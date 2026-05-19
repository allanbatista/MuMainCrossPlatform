pub mod character_select;
pub mod chat;
pub mod duel;
pub mod error;
pub mod events;
pub mod gens;
pub mod hotkeys;
pub mod hud;
pub mod i18n;
pub mod inventory;
pub mod layout;
pub mod login;
pub mod messages;
pub mod minimap;
pub mod npc;
pub mod options;
pub mod player_shop;
pub mod quests;
pub mod routes;
pub mod server_select;
pub mod shop;
pub mod trade;
pub mod widgets;

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

mod fixture_routes;

pub use character_select::{
    character_select_screen, CharacterSelectAction, CharacterSelectButton,
    CharacterSelectCharacter, CharacterSelectScreen, CharacterSelectScreenState,
};
pub use chat::{chat_screen, ChatMessageCount, ChatMessageType, ChatScreen, ChatScreenState};
pub use duel::{duel_screen, DuelAction, DuelChannelCard, DuelScreen, DuelScreenState};
pub use error::ErrorPresentation;
pub use events::{events_screen, EventAction, EventScreen, EventScreenState};
pub use fixture_routes::{fixture_routes, UiFixtureRoute};
pub use gens::{gens_ranking_screen, GensAction, GensRankingScreen, GensRankingScreenState};
pub use hotkeys::{hotkeys_screen, HotkeyBinding, HotkeysScreen, HotkeysScreenState};
pub use hud::{hud_screen, HudButton, HudButtonKind, HudGauge, HudGaugeKind, HudScreen};
pub use i18n::Translator;
pub use inventory::{
    inventory_screen, InventoryAction, InventoryPanel, InventoryScreen, InventoryScreenState,
};
pub use layout::UiShellLayout;
pub use login::{login_screen, LoginAction, LoginField, LoginScreen, LoginScreenState};
pub use messages::{messages_screen, MessageAction, MessageSeverity, MessagesScreen};
pub use minimap::{minimap_screen, MiniMapMarker, MiniMapMarkerKind, MiniMapScreen};
pub use mu_assets::TranslationDomain;
pub use npc::{npc_screen, NpcAction, NpcScreen, NpcScreenState};
pub use options::{
    options_screen, OptionsScreen, OptionsScreenState, OptionsSection, OptionsToggle,
};
pub use player_shop::{
    player_shop_screen, PlayerShopAction, PlayerShopScreen, PlayerShopScreenState,
};
pub use quests::{quests_screen, QuestAction, QuestScreen, QuestScreenState};
pub use routes::{
    UiRoute, UiRouteCatalog, UiRouteDescriptor, UiRouteGroup, UiShellPlugin, UiShellState,
};
pub use server_select::{
    server_select_screen, ServerEntry, ServerSelectAction, ServerSelectScreen,
    ServerSelectScreenState,
};
pub use shop::{shop_screen, ShopAction, ShopScreen, ShopScreenState};
pub use trade::{trade_screen, TradeAction, TradeScreen, TradeScreenState};
pub use widgets::{UiShellWidgetKind, UiShellWidgetSet};
